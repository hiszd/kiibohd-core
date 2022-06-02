// Copyright 2021-2022 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

mod test;

// ----- Crates -----

use super::*;
use core::cmp::Ordering;
use heapless::{FnvIndexMap, Vec};

// ----- Enums -----

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum LayerProcessMode {
    Layer,
    TriggerType,
    IndexA,
    IndexB,
    TriggerSize,
    Triggers(u8),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum StateStatus {
    /// TriggerCondition + u8 offset position
    TriggerPos {
        /// Time instance when the offset is updated (on combo increment).
        time_instance: u32,
        /// Next offset in the TriggerGuide
        /// This is the offset inside the datastructure so it can be any number
        /// even at the start of the TriggerGuide.
        offset: u16,
    },
    /// Capability + u8 offset position + last TriggerEvent
    ResultPos {
        /// Time instance when the offset is updated (on combo increment).
        /// This value is set on increment and is set for the first combo eval
        time_instance: u32,
        /// TriggerEvent that initiated the Result Capability
        event: TriggerEvent,
        /// Next offset in the ResultGuide
        /// This is the offset inside the datastructure so it can be any number
        /// even at the start of the ResultGuide.
        offset: u16,
    },
    /// Done is set when the Capabilities are finished and the entry should be reaped
    Done,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProcessError {
    /// MAX_OFF_STATE_LOOKUP is too small
    FailedOffStatePush,
    /// STATE_SIZE is too small
    FailedLookupStateInsert,
    /// MAX_ACTIVE_TRIGGERS is too small
    FailedTriggerComboEvalStateInsert,
}

// ----- Structs -----

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
struct Layer {
    state: layer::State,
    /// Last operation that touched this layer state
    last_time_instance: u32,
}

pub struct LayerState<
    'a,
    const LAYOUT_SIZE: usize,
    const STATE_SIZE: usize,
    const MAX_LAYERS: usize,
    const MAX_ACTIVE_LAYERS: usize,
    const MAX_ACTIVE_TRIGGERS: usize,
    const MAX_LAYER_STACK_CACHE: usize,
    const MAX_OFF_STATE_LOOKUP: usize,
> {
    /// KLL guide lookup
    layer_lookup: LayerLookup<'a, LAYOUT_SIZE>,
    /// Stores the trigger:result mapping state position, tracks the macro position
    lookup_state: FnvIndexMap<(u16, u16), StateStatus, STATE_SIZE>,
    /// Stores the current state of every possible layer
    layer: Vec<Layer, MAX_LAYERS>,
    /// Current evaluation order of each layer
    /// Each layer is only in the stack once
    /// Whenever layer::State::Off is set the layer is removed from the stack
    /// Changing the state of a layer does not change the priority order of the stack
    layer_stack: Vec<u8, MAX_ACTIVE_LAYERS>,
    /// Whenever there is a layer lookup for "initial" actions cache the result of the lookup
    /// This initial action always does a clean lookup.
    /// The reason for this is to handle the situation where a layer is activated, a key is pressed
    /// then the layer is deactivated and the key is released. You want the action that as
    /// previously activated on the deactivated layer to deactivate, not whatever is on the
    /// effective new layer in the stack.
    /// (ttype, index) -> (layer index, Layer {layer state, time instance})
    layer_stack_cache: FnvIndexMap<(u8, u16), (u8, Layer), MAX_LAYER_STACK_CACHE>,
    /// Maintains the combo state when evaluating a list of TriggerEvents
    /// This hash table is cleared when finalizing a scan loop
    /// Maps (trigger_guide, result_guide) -> (combo evaluations remaining)
    trigger_combo_eval_state: FnvIndexMap<(u16, u16), u8, MAX_ACTIVE_TRIGGERS>,
    /// time_instance is a dumb counter used to keep track of processing instances.
    /// Yes, the counter will rollover but generally this shouldn't matter
    /// Used to calculate produced Layer TriggerEvents, is generally set once per processing loop
    time_instance: u32,
    /// Off state lookups
    /// Used to keep track of possibly off-states that need a reverse lookup
    /// Cleared each processing loop.
    /// ((trigger_guide, result_guide), ttype, index)
    off_state_lookups: Vec<((u16, u16), u8, u16), MAX_OFF_STATE_LOOKUP>,
}

impl<
        'a,
        const LAYOUT_SIZE: usize,
        const STATE_SIZE: usize,
        const MAX_LAYERS: usize,
        const MAX_ACTIVE_LAYERS: usize,
        const MAX_ACTIVE_TRIGGERS: usize,
        const MAX_LAYER_STACK_CACHE: usize,
        const MAX_OFF_STATE_LOOKUP: usize,
    >
    LayerState<
        'a,
        LAYOUT_SIZE,
        STATE_SIZE,
        MAX_LAYERS,
        MAX_ACTIVE_LAYERS,
        MAX_ACTIVE_TRIGGERS,
        MAX_LAYER_STACK_CACHE,
        MAX_OFF_STATE_LOOKUP,
    >
{
    pub fn new(layer_lookup: LayerLookup<'a, LAYOUT_SIZE>, time_instance: u32) -> Self {
        // Allocate trigger:result mapping state hashtable
        // Used to keep track of the guide offset
        // Mapping
        //   (trigger guide, result guide) -> Type(offset)
        let lookup_state = FnvIndexMap::<(u16, u16), StateStatus, STATE_SIZE>::new();

        let mut layer = Vec::new();
        layer
            .resize(
                layer_lookup.max_layers() as usize,
                Layer {
                    state: layer::State::Off,
                    last_time_instance: 0u32,
                },
            )
            .unwrap();

        // Layer 0 is always enabled by default
        layer[0].state = layer::State::Shift;

        let layer_stack = Vec::new();
        let layer_stack_cache = FnvIndexMap::<(u8, u16), (u8, Layer), MAX_LAYER_STACK_CACHE>::new();
        let trigger_combo_eval_state = FnvIndexMap::<(u16, u16), u8, MAX_ACTIVE_TRIGGERS>::new();
        let off_state_lookups = Vec::new();

        Self {
            layer_lookup,
            lookup_state,
            layer,
            layer_stack,
            layer_stack_cache,
            trigger_combo_eval_state,
            time_instance,
            off_state_lookups,
        }
    }

    /// Determine if layer is in the stack
    fn is_layer_in_stack(&self, layer: u8) -> bool {
        self.layer_stack.contains(&layer)
    }

    /// Used to set the current time instance used for produced Layer TriggerEvents
    pub fn set_time(&mut self, val: u32) {
        self.time_instance = val;
    }

    /// Set layer state
    /// If layer already has the state enable, disable and vice versa
    pub fn set_layer(&mut self, layer: u8, state: layer::State) -> TriggerEvent {
        // Make sure the layer is valid
        assert!(
            layer as usize >= self.layer.len(),
            "Invalid layer set: {} {:?}",
            layer,
            state,
        );

        // Cannot set layer 0
        assert!(layer != 0, "Cannot change layer 0 state");

        // Check to see if the layer is already in the stack, add it if not
        let layer_in_stack = self.is_layer_in_stack(layer);
        if !layer_in_stack {
            self.layer_stack.push(layer).unwrap();
        }

        // Store previous state for event generation
        let prev_state = self.layer[layer as usize].state;

        // Set the layer if not already enabled
        if !self.layer[layer as usize].state.is_set(state) {
            self.layer[layer as usize].state.add(state);
        } else {
            self.layer[layer as usize].state.remove(state);
        }

        // Current state
        let cur_state = self.layer[layer as usize].state;

        // Determine Aodo state
        let activity_state = trigger::Aodo::from_state(prev_state.active(), cur_state.active());

        // Update the time instance
        self.layer[layer as usize].last_time_instance = self.time_instance;

        // Remove the layer from the stack if state is Off
        if self.layer[layer as usize].state == layer::State::Off {
            let mut offset = 0;
            for (index, val) in self.layer_stack.clone().iter().enumerate() {
                // Search for index of the layer
                if *val == layer {
                    offset = 1;
                } else {
                    // Once index of the layer has been located, shift all stack elements
                    if offset > 0 {
                        self.layer_stack[index - offset] = *val;
                    }
                }
            }

            // Reduce the length by one
            self.layer_stack.truncate(self.layer_stack.len() - 1);
        }

        // Build layer trigger event
        let state = trigger::LayerState::from_layer(cur_state, activity_state);

        // Send signal for layer state change
        TriggerEvent::Layer {
            state,
            layer,
            last_state: 0u32, // Initial events always start at 0
        }
    }

    /// Attempts to lookup a trigger list given a layer and given state
    fn layer_lookup_search<const LSIZE: usize>(
        &self,
        ttype: u8,
        index: u16,
    ) -> Option<(u8, heapless::Vec<(u16, u16), LSIZE>)> {
        // Start from the top of the stack
        for (layer, state) in self.layer.iter().rev().enumerate() {
            let layer = layer as u8;
            // Check if effective state is valid
            if state.state.effective() {
                let guides = self
                    .layer_lookup
                    .lookup_guides::<LSIZE>((layer, ttype, index));
                // If guides were found, we can stop here
                if !guides.is_empty() {
                    return Some((layer, guides));
                }
            }
        }

        // No matches
        None
    }

    /// Lookup effective layer for scancode
    /// Depending on the incoming state use either a full-lookup or cached value
    ///
    /// Returns None if no lookup was successful
    /// Otherwise returns a list of Trigger::Result mappings to process
    pub fn lookup<const LSIZE: usize>(
        &mut self,
        event: TriggerEvent,
    ) -> Option<(u8, heapless::Vec<(u16, u16), LSIZE>)> {
        let cache_lookup = (u8::from(event), event.index());
        let cache_hit = self.layer_stack_cache.get(&cache_lookup);
        trace!("Lookup cache hit: {:?}", cache_hit);

        // Convert to CapabilityRun to determine how to evaluate trigger
        let capability: CapabilityRun = event.into();
        let capability_state = capability.state();
        trace!("Converted capability_state: {:?}", capability_state);

        // Do cached lookup if not the initial event for the trigger and present in the cache
        let layer_guides = if capability_state != CapabilityEvent::Initial && let Some((layer, _layer_state)) = cache_hit {
            // Retrieve layer, and build guide lookup
            let guide_lookup = (*layer, cache_lookup.0, cache_lookup.1);

            // We can do a direct lookup as we're hitting a cache
            let guides = self.layer_lookup.lookup_guides::<LSIZE>(guide_lookup);

            Some((*layer, guides))

        // Do full lookup if this is the initial event for the trigger or was not in the cache
        } else {
            self.layer_lookup_search::<LSIZE>(cache_lookup.0, cache_lookup.1)
        };
        trace!("layer_guides: {:?}", layer_guides);

        // If this is a final event, remove the trigger from the layer cache
        if capability_state == CapabilityEvent::Last {
            self.layer_stack_cache.remove(&cache_lookup);

        // Otherwise update/insert the key if we don't have one already
        } else if cache_hit.is_none() && layer_guides.is_some() {
            let layer = layer_guides.as_ref().unwrap().0;
            // Build cache key by looking up identified layer state
            // The layer state is needed so we can remember what to do if the layer is deactivated
            // during the middle of an action
            let cache_key = (
                layer,
                Layer {
                    state: self.layer[layer as usize].state,
                    last_time_instance: self.time_instance,
                },
            );

            self.layer_stack_cache
                .insert(cache_lookup, cache_key)
                .unwrap();
        }

        layer_guides
    }

    /// Increment time instance
    /// Per the design of KLL, each processing loop of events takes place in a single instance.
    /// Before processing any events, make sure to call this function to increment the internal
    /// time state which is needed to properly schedule generated events.
    pub fn increment_time(&mut self) {
        self.time_instance = self.time_instance.wrapping_add(1u32);
    }

    /// Process incoming triggers
    pub fn process_trigger<const LSIZE: usize>(
        &mut self,
        event: TriggerEvent,
    ) -> Result<(), ProcessError> {
        trace!("Event: {:?}", event);
        // Lookup guide
        if let Some((_layer, guides)) = self.lookup::<LSIZE>(event) {
            trace!("Event guides: {:?}", guides);
            // Process each of the guides
            for guide in guides {
                // Lookup the state of each of the guides
                let state = if let Some(state) = self.lookup_state.get(&guide) {
                    *state
                } else {
                    StateStatus::TriggerPos {
                        time_instance: self.time_instance,
                        offset: 0,
                    }
                };

                // Determine if this trigger is valid
                // If we have a new trigger on a state that is processing a result, ignore this
                // event. We don't ignore result events, they are just queued up.
                let pos = match state {
                    StateStatus::TriggerPos { offset, .. } => offset,
                    _ => {
                        continue;
                    }
                };

                // Lookup trigger guide
                if let Some(trigger_guide) = self.layer_lookup.trigger_guide(guide, pos) {
                    // Check for already evaluated trigger state for this processing loop
                    let mut remaining =
                        if let Some(remaining) = self.trigger_combo_eval_state.get(&guide) {
                            *remaining
                        } else {
                            // Lookup size of this trigger list combo
                            trigger_guide.len() as u8
                        };

                    // Verify that we actually match the condition
                    // e.g. Press vs. Release
                    let mut removed_lookup_state = false;
                    for cond in trigger_guide {
                        match cond.evaluate(event, self.layer_lookup.loop_condition_lookup) {
                            Vote::Positive => {
                                remaining -= 1;
                            }
                            Vote::Negative => {
                                // Remove lookup state entry, continue to next guide
                                self.lookup_state.remove(&guide);
                                removed_lookup_state = true;
                                break;
                            }
                            Vote::Insufficient => {} // Do nothing
                            Vote::OffState => {
                                // Attempt to push a reverse lookup query
                                // The results of the query will be another set of TriggerEvents
                                if self
                                    .off_state_lookups
                                    .push((guide, u8::from(*cond), cond.index()))
                                    .is_err()
                                {
                                    return Err(ProcessError::FailedOffStatePush);
                                }
                            }
                        }
                    }

                    // Don't insert a new lookup_state entry if we're removed it on purpose
                    if removed_lookup_state {
                        continue;
                    }

                    // Check if there are no remaining evaluations
                    if remaining == 0 {
                        // Determine the next offset
                        let next_status = if let Some(next_offset) =
                            self.layer_lookup.next_trigger_combo(guide, pos)
                        {
                            StateStatus::TriggerPos {
                                time_instance: self.time_instance,
                                offset: next_offset,
                            }
                        } else {
                            StateStatus::ResultPos {
                                time_instance: self.time_instance,
                                event,
                                offset: 0,
                            }
                        };

                        // Update lookup state
                        if self.lookup_state.insert(guide, next_status).is_err() {
                            return Err(ProcessError::FailedLookupStateInsert);
                        }
                    } else {
                        // Update trigger_combo_eval_state
                        if self
                            .trigger_combo_eval_state
                            .insert(guide, remaining)
                            .is_err()
                        {
                            return Err(ProcessError::FailedTriggerComboEvalStateInsert);
                        }
                    }
                }
            }
        } else {
            trace!("No event mapping for: {:?}", event);
        }

        Ok(())
    }

    /// Off state lookups
    /// Used to keep track of possibly off-states that need a reverse lookup
    /// Cleared each processing loop.
    /// ((trigger_guide, result_guide), ttype, index)
    pub fn off_state_lookups(&self) -> &[((u16, u16), u8, u16)] {
        &self.off_state_lookups
    }

    /// Process off state lookups
    /// To maintain state use a callback function to evaluate input off states
    pub fn process_off_state_lookups<const MAX_LAYER_LOOKUP_SIZE: usize>(
        &mut self,
        generate_event: &dyn Fn(usize) -> Vec<TriggerEvent, 4>,
    ) {
        let mut events: heapless::Vec<TriggerEvent, MAX_LAYER_LOOKUP_SIZE> = heapless::Vec::new();
        for lookup in &self.off_state_lookups {
            // TODO support non-keyboard TriggerConditions
            assert!(
                lookup.1 == 1,
                "Currently only keyboard TriggerConditions are supported"
            );
            for gen_event in generate_event(lookup.2.into()) {
                events.push(gen_event).unwrap();
            }
        }

        for event in events {
            let ret = self.process_trigger::<MAX_LAYER_LOOKUP_SIZE>(event);
            assert!(
                ret.is_ok(),
                "Failed to enqueue offstate: {:?} - {:?}",
                event,
                ret
            );
        }
    }

    /// Finalize incoming triggers, update internal state and generate outgoing results
    pub fn finalize_triggers<const LSIZE: usize>(&mut self) -> heapless::Vec<CapabilityRun, LSIZE> {
        let mut results = heapless::Vec::<_, LSIZE>::new();

        // Iterate over lookup_state, looking for ResultPos entries
        for (guide, status) in self.lookup_state.iter_mut() {
            // Process results
            if let StateStatus::ResultPos {
                time_instance,
                event,
                offset,
            } = status
            {
                // Time offset, used to compare against the timing conditions
                let time_offset = self.time_instance - *time_instance;

                // Lookup ResultGuide
                if let Some(result_guide) = self.layer_lookup.result_guide(*guide, *offset) {
                    // Keeps track of completed conditions inside the combination
                    let mut completed_cond = 0;

                    // For each element in the combo
                    for cap in result_guide {
                        let time_cond = self.layer_lookup.loop_condition_lookup
                            [cap.loop_condition_index() as usize];
                        match time_offset.cmp(&time_cond) {
                            Ordering::Equal => {
                                // Convert the Capability into a CapabilityRun and enqueue it
                                if results
                                    .push(
                                        cap.generate(
                                            *event,
                                            self.layer_lookup.loop_condition_lookup,
                                        ),
                                    )
                                    .is_err()
                                {
                                    panic!("finalize_triggers LSIZE is too small!");
                                }

                                // Increment completion
                                completed_cond += 1;
                            }
                            Ordering::Greater => {
                                // Capability has already been scheduled, mark as completed
                                completed_cond += 1;
                            }
                            _ => {}
                        }
                    }

                    // Update status position
                    // Check to see if the time_instance is 0, so we can set it
                    if *offset == 0 {
                        *status = StateStatus::ResultPos {
                            time_instance: self.time_instance,
                            event: *event,
                            offset: *offset,
                        };
                    } else {
                        // Only increment combo if combo has been fully executed/processed
                        if completed_cond == result_guide.len() {
                            if let Some(next_pos) =
                                self.layer_lookup.next_result_combo(*guide, *offset)
                            {
                                *status = StateStatus::ResultPos {
                                    time_instance: 0, // Set to 0, indicates new combo
                                    event: *event,
                                    offset: next_pos,
                                };
                            } else {
                                // No more combos, remove entry
                                *status = StateStatus::Done;
                            }
                        }
                    }
                }
            }
        }

        // Clear out StateStatus::Done entries
        // TODO(HaaTa): Is this optimal?
        for (guide, status) in self.lookup_state.clone().iter() {
            if status == &StateStatus::Done {
                self.lookup_state.remove(guide);
            }
        }

        // Clear the trigger_combo_eval_state for the next scan iteration
        self.trigger_combo_eval_state.clear();

        // Clear the off_state_lookups for the next scan iteration
        self.off_state_lookups.clear();

        results
    }
}

/// The LayerLookup struct is used as a guide for the KLL state machine
/// It is a (mostly) constant lookup table which can give you all possible
/// TriggerGuides for a specified input.
/// Each TriggerGuide has a connected ResultGuide which is also stored in this datastructure.
///
/// In most cases a (layer, ttype, index) tuple is provided and a list of TriggerGuide:ResultGuide
/// mappings
/// is provided. See lookup_guides().
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayerLookup<'a, const LAYOUT_SIZE: usize> {
    layer_lookup: FnvIndexMap<(u8, u8, u16), usize, LAYOUT_SIZE>,
    raw_layer_lookup: &'a [u8],
    trigger_guides: &'a [u8],
    result_guides: &'a [u8],
    trigger_result_mapping: &'a [u16],
    loop_condition_lookup: &'a [u32],
    max_layer: u8,
}

impl<'a, const LAYOUT_SIZE: usize> LayerLookup<'a, LAYOUT_SIZE> {
    pub fn new(
        raw_layer_lookup: &'a [u8],
        trigger_guides: &'a [u8],
        result_guides: &'a [u8],
        trigger_result_mapping: &'a [u16],
        loop_condition_lookup: &'a [u32],
    ) -> Self {
        // Build layer lookup from array
        // The purpose of this hash table is to quickly find the trigger list in LAYER_LOOKUP
        // Mapping
        //   (<layer>, <ttype>, <index>) -> LAYER_LOOKUP index
        let mut layer_lookup = FnvIndexMap::<(u8, u8, u16), usize, LAYOUT_SIZE>::new();

        let mut max_layer = 0;

        let mut mode = LayerProcessMode::Layer;
        let mut layer = 0;
        let mut ttype = 0;
        let mut index: u16 = 0;
        for (i, val) in raw_layer_lookup.iter().enumerate() {
            match mode {
                LayerProcessMode::Layer => {
                    layer = *val;
                    if layer > max_layer {
                        max_layer = layer;
                    }
                    mode = LayerProcessMode::TriggerType;
                }
                LayerProcessMode::TriggerType => {
                    ttype = *val;
                    mode = LayerProcessMode::IndexA;
                }
                LayerProcessMode::IndexA => {
                    index = *val as u16;
                    mode = LayerProcessMode::IndexB;
                }
                LayerProcessMode::IndexB => {
                    index |= (*val as u16) << 8;
                    mode = LayerProcessMode::TriggerSize;
                }
                LayerProcessMode::TriggerSize => {
                    let size = *val;
                    let lookup = i;
                    // We only add to the hash table if triggers actually exist
                    // The KLL compiler should optimize these out, but it's still valid array syntax
                    mode = if size > 0 {
                        // Attempt to insert the key
                        match layer_lookup.insert((layer, ttype, index), lookup) {
                            // Success, no existing key
                            Ok(None) => {}
                            // Success, replace existing key (this is bad, warn)
                            Ok(Some(old_lookup)) => {
                                warn!("Duplicate layer lookup key! ({}, {}). {} has been replaced by {}", layer, index, old_lookup, lookup);
                            }
                            Err(e) => {
                                error!(
                                    "Failed to add lookup key ({}, {}) -> {}: {:?}; Size:{:?} Capacity:{:?}",
                                    layer, index, lookup, e, layer_lookup.len(), LAYOUT_SIZE,
                                );
                            }
                        }
                        // Triggers are u16, so multiple by 2
                        LayerProcessMode::Triggers(size * 2)
                    } else {
                        LayerProcessMode::Layer
                    }
                }
                LayerProcessMode::Triggers(size) => {
                    mode = if size <= 1 {
                        LayerProcessMode::Layer
                    } else {
                        LayerProcessMode::Triggers(size - 1)
                    };
                }
            }
        }
        trace!("trigger_guides: {:?}", trigger_guides);
        trace!("trigger_result_mapping: {:?}", trigger_result_mapping);
        Self {
            layer_lookup,
            raw_layer_lookup,
            trigger_guides,
            result_guides,
            trigger_result_mapping,
            loop_condition_lookup,
            max_layer,
        }
    }

    /// Retrieves a TriggerList
    /// A TriggerList is a list of indices that correspond to a specific TriggerGuide -> ResultGuide
    /// mapping.
    pub fn trigger_list(&self, (layer, ttype, index): (u8, u8, u16)) -> Option<&'a [u8]> {
        trace!("layer_lookup: {:?}", self.layer_lookup);
        match self.layer_lookup.get(&(layer, ttype, index)) {
            Some(lookup) => {
                // Determine size of trigger list
                trace!("raw_layer_lookup: {:?}", self.raw_layer_lookup);
                let size: usize = self.raw_layer_lookup[*lookup].into();

                // If the size is 0, just return None
                if size == 0 {
                    return None;
                }

                // Each trigger list id is a u16
                let size = size * 2;

                // Build TriggerList slice
                let initial: usize = lookup + 1;
                Some(&self.raw_layer_lookup[initial..initial + size])
            }
            None => None,
        }
    }

    /// Retrieves a list of TriggerGuide:ResultGuide mappings
    /// Will need to be called for every new TriggerEvent.
    pub fn lookup_guides<const LSIZE: usize>(
        &self,
        (layer, ttype, index): (u8, u8, u16),
    ) -> heapless::Vec<(u16, u16), LSIZE> {
        let mut guides = heapless::Vec::<_, LSIZE>::new();

        // Lookup TriggerList
        match self.trigger_list((layer, ttype, index)) {
            Some(mlookup) => {
                // Iterate over each trigger to locate guides
                // Each value is a u16 (hence chunking by 2)
                trace!("mlookup: {:?}", mlookup);
                for chunk in mlookup.chunks_exact(2) {
                    // Determine guide lookup index
                    let index = u16::from_le_bytes([chunk[0], chunk[1]]) as usize;

                    // Push guide pair
                    assert!(
                        guides
                            .push((
                                self.trigger_result_mapping[index],
                                self.trigger_result_mapping[index + 1]
                            ))
                            .is_ok(),
                        "lookup_guides vector is full, increase LSIZE: {}",
                        LSIZE
                    );
                }
                trace!("guides: {:?}", guides);
                guides
            }
            None => guides,
        }
    }

    /// Retrieves the TriggerGuide for a given TriggerGuide:ResultGuide pair
    ///
    /// offset indicates the number of u8 positions the sequence is currently at.
    /// trigger + offset will always point to the start of a combination
    pub fn trigger_guide(
        &self,
        (trigger, _result): (u16, u16),
        offset: u16,
    ) -> Option<&[TriggerCondition]> {
        // Determine size of offset combo in the sequence
        let count = self.trigger_guides[trigger as usize + offset as usize] as usize;
        if count == 0 {
            return None;
        }

        // Determine starting position of combo
        let start = trigger as usize + offset as usize + 1;

        // Convert u8 combo list to TriggerCondition list
        let ptr: *const u8 =
            self.trigger_guides[start..start + core::mem::size_of::<TriggerCondition>()].as_ptr();
        let cond = unsafe { core::slice::from_raw_parts(ptr as *const TriggerCondition, count) };
        Some(cond)
    }

    /// Retrieves the ResultGuide for a given TriggerGuide:ResultGuide pair
    ///
    /// offset indicates the number of u8 positions the sequence is currently at.
    /// result + offset will always point to the start of a combination
    pub fn result_guide(
        &self,
        (_trigger, result): (u16, u16),
        offset: u16,
    ) -> Option<&[Capability]> {
        // Determine size of offset combo in the sequence
        let count = self.result_guides[result as usize + offset as usize] as usize;
        if count == 0 {
            return None;
        }

        // Determine starting position of combo
        let start = result as usize + offset as usize + 1;

        // Convert u8 combo list to Capability list
        let ptr: *const u8 =
            self.result_guides[start..start + core::mem::size_of::<Capability>()].as_ptr();
        let cond = unsafe { core::slice::from_raw_parts(ptr as *const Capability, count) };
        Some(cond)
    }

    /// Determines the next trigger guide combo offset
    /// Returns Some if there is a next offset, None if the next combo is 0 length
    /// Will also return None if the current offset is also 0 (shouldn't be a common use case)
    pub fn next_trigger_combo(&self, (trigger, _result): (u16, u16), offset: u16) -> Option<u16> {
        // Determine size of offset combo in the sequence
        let count = self.trigger_guides[trigger as usize + offset as usize] as usize;
        if count == 0 {
            return None;
        }

        // New offset position
        // +1 is added as the combo length count uses 1 byte
        let offset = offset as usize + count * core::mem::size_of::<TriggerCondition>() + 1;

        // Determine size of next combo
        let count = self.trigger_guides[trigger as usize + offset as usize] as usize;
        if count == 0 {
            None
        } else {
            Some(offset as u16)
        }
    }

    /// Determine the next result guide combo offset
    /// Returns Some if there is a next offset, None if the next combo is 0 length
    /// Will also return None if the current offset is also 0 (shouldn't be a common use case)
    pub fn next_result_combo(&self, (_trigger, result): (u16, u16), offset: u16) -> Option<u16> {
        // Determine size of offset combo in the sequence
        let count = self.result_guides[result as usize + offset as usize] as usize;
        if count == 0 {
            return None;
        }

        // New offset position
        // +1 is added as the combo length count uses 1 byte
        let offset = offset as usize + count * core::mem::size_of::<Capability>() + 1;

        // Determine size of next combo
        let count = self.result_guides[result as usize + offset as usize] as usize;
        if count == 0 {
            None
        } else {
            Some(offset as u16)
        }
    }

    /// Convience access for layer_lookup
    /// Useful when trying to get a list of all possible triggers
    pub fn layer_lookup(&self) -> &FnvIndexMap<(u8, u8, u16), usize, LAYOUT_SIZE> {
        &self.layer_lookup
    }

    /// Determine the max number of layers
    pub fn max_layers(&self) -> u8 {
        self.max_layer + 1
    }
}
