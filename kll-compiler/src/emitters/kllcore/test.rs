// Copyright 2021-2022 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![cfg(test)]

use crate::emitters::kllcore::{KllCoreData, KllCoreValidation};
use crate::types::KllFile;
use flexi_logger::Logger;
use layouts_rs::Layouts;
use log::*;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

enum LogError {
    CouldNotStartLogger,
}

/// Lite logging setup
fn setup_logging_lite() -> Result<(), LogError> {
    match Logger::try_with_env_or_str("")
        .unwrap()
        .format(flexi_logger::colored_default_format)
        .format_for_files(flexi_logger::colored_detailed_format)
        .duplicate_to_stderr(flexi_logger::Duplicate::All)
        .start()
    {
        Err(_) => Err(LogError::CouldNotStartLogger),
        Ok(_) => Ok(()),
    }
}

#[test]
fn trigger() {
    let test = fs::read_to_string("examples/kllcoretest.kll").unwrap();
    let result = KllFile::from_str(&test);
    let state = result.unwrap().into_struct();

    // Generate trigger guides
    let mut trigger_guides = Vec::new();
    for trigger_list in state.trigger_lists() {
        let mut guide = trigger_list.kll_core_guide();
        trigger_guides.append(&mut guide);
    }

    // TODO Validate
}

#[test]
fn result() {
    let test = fs::read_to_string("examples/kllcoretest.kll").unwrap();
    let result = KllFile::from_str(&test);
    let state = result.unwrap().into_struct();
    let mut layouts = Layouts::from_dir(PathBuf::from("layouts"));

    // Generate result guides
    let mut result_guides = Vec::new();
    for result_list in state.result_lists() {
        let mut guide = result_list.kll_core_guide(&mut layouts);
        result_guides.append(&mut guide);
    }

    // TODO Validate
}

#[test]
fn trigger_result() {
    let test = fs::read_to_string("examples/kllcoretest.kll").unwrap();
    let result = KllFile::from_str(&test);
    let state = result.unwrap().into_struct();
    let mut layouts = Layouts::from_dir(PathBuf::from("layouts"));

    // Trigger and Result deduplication hashmaps
    let mut trigger_hash = HashMap::new();
    let mut result_hash = HashMap::new();

    // Trigger:Result mapping hashmap
    let mut trigger_result_hash = HashMap::new();

    // Generate trigger and result guides as well as the trigger result mapping
    let mut trigger_guides = Vec::new();
    let mut result_guides = Vec::new();
    let mut trigger_result_map: Vec<u16> = Vec::new();
    for (trigger_list, result_list) in state.trigger_result_lists() {
        let mut trigger_guide = trigger_list.kll_core_guide();
        // Determine if trigger guide has already been added
        let trigger_pos = match trigger_hash.try_insert(trigger_guide.clone(), trigger_guide.len())
        {
            Ok(pos) => {
                trigger_guides.append(&mut trigger_guide);
                *pos
            }
            Err(err) => *err.entry.get(),
        };

        let mut result_guide = result_list.kll_core_guide(&mut layouts);
        // Determine if result guide has already been added
        let result_pos = match result_hash.try_insert(result_guide.clone(), result_guide.len()) {
            Ok(pos) => {
                result_guides.append(&mut result_guide);
                *pos
            }
            Err(err) => *err.entry.get(),
        };

        // Add trigger:result mapping
        if trigger_result_hash
            .insert((trigger_guide, result_guide), (trigger_pos, result_pos))
            .is_none()
        {
            trigger_result_map.push(trigger_pos as u16);
            trigger_result_map.push(result_pos as u16);
        }
    }

    // TODO Validate
}

#[test]
fn layer_lookup_simple() {
    setup_logging_lite().ok();

    let test = fs::read_to_string("examples/kllcoretest.kll").unwrap();
    let result = KllFile::from_str(&test);
    let state = result.unwrap().into_struct();
    let mut layers = vec![state];
    dbg!(layers.clone());
    let mut layouts = Layouts::from_dir(PathBuf::from("layouts"));
    let kdata = KllCoreData::new(&mut layers, &mut layouts);

    // TODO - Generate loop conditions using compiler
    let loop_condition_lookup: &[u32] = &[0];

    // Load data structures into kll-core
    const LAYOUT_SIZE: usize = 2;
    let lookup = kll_core::layout::LayerLookup::<LAYOUT_SIZE>::new(
        &kdata.raw_layer_lookup,
        &kdata.trigger_guides,
        &kdata.result_guides,
        &kdata.trigger_result_map,
        loop_condition_lookup,
    );

    // Initialize LayerState
    const STATE_SIZE: usize = 2;
    const MAX_LAYERS: usize = 2;
    const MAX_ACTIVE_LAYERS: usize = 2;
    const MAX_ACTIVE_TRIGGERS: usize = 2;
    const MAX_LAYER_STACK_CACHE: usize = 2;
    const MAX_OFF_STATE_LOOKUP: usize = 2;
    let mut layer_state = kll_core::layout::LayerState::<
        LAYOUT_SIZE,
        STATE_SIZE,
        MAX_LAYERS,
        MAX_ACTIVE_LAYERS,
        MAX_ACTIVE_TRIGGERS,
        MAX_LAYER_STACK_CACHE,
        MAX_OFF_STATE_LOOKUP,
    >::new(lookup, 0);

    // Generate Press event
    let event = kll_core::TriggerEvent::Switch {
        state: kll_core::trigger::Phro::Press,
        index: 0x00,
        last_state: 0,
    };

    // Process Press event
    const LSIZE: usize = 4;
    let ret = layer_state.process_trigger::<LSIZE>(event);
    assert!(ret.is_ok(), "Failed to enqueue: {:?} - {:?}", event, ret);

    // Confirm there are no off state lookups
    assert_eq!(
        layer_state.off_state_lookups().len(),
        0,
        "Unexpected off state lookups"
    );

    // Verify capability event
    let cap_runs = layer_state.finalize_triggers::<LSIZE>();
    trace!("cap_runs: {:?}", cap_runs);
    assert_eq!(
        cap_runs,
        [kll_core::CapabilityRun::HidKeyboard {
            state: kll_core::CapabilityEvent::Initial,
            id: kll_core::kll_hid::Keyboard::Esc,
        }],
        "Unexpected press result {:?}",
        cap_runs
    );

    // Next time iteration
    layer_state.increment_time();

    // Generate Release event
    let event = kll_core::TriggerEvent::Switch {
        state: kll_core::trigger::Phro::Release,
        index: 0x00,
        last_state: 0,
    };

    // Process Release event
    assert!(
        layer_state.process_trigger::<LSIZE>(event).is_ok(),
        "Failed to enqueue: {:?}",
        event
    );

    // Confirm there are no off state lookups
    assert_eq!(
        layer_state.off_state_lookups().len(),
        0,
        "Unexpected off state lookups"
    );

    // Verify capability event
    let cap_runs = layer_state.finalize_triggers::<LSIZE>();
    trace!("cap_runs: {:?}", cap_runs);
    assert_eq!(
        cap_runs,
        [kll_core::CapabilityRun::HidKeyboard {
            state: kll_core::CapabilityEvent::Last,
            id: kll_core::kll_hid::Keyboard::Esc,
        }],
        "Unexpected release result {:?}",
        cap_runs
    );
}

#[test]
fn generate_binary() {
    // todo needs an offset table for the firmware to know where the pointers
    // are
}

#[test]
fn generate_rust() {
    // todo
}

#[test]
fn keystone_basemap_rust() {
    setup_logging_lite().ok();

    let test = fs::read_to_string("examples/keystone_scancode_map.kll").unwrap();
    let result = KllFile::from_str(&test);
    let state = result.unwrap().into_struct();
    let mut layers = vec![state];
    dbg!(layers.clone());
    let mut layouts = Layouts::from_dir(PathBuf::from("layouts"));
    let kdata = KllCoreData::new(&mut layers, &mut layouts);

    const LAYOUT_SIZE: usize = 128;
    let mut kval = KllCoreValidation::<LAYOUT_SIZE>::new(&kdata, layers, layouts);
    kval.validate();
}
