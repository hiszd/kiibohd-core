// Copyright 2021 Zion Koyl
// Copyright 2021-2022 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use core::ops::Not;

#[derive(PartialEq, Eq, Copy, Clone, Debug, defmt::Format)]
pub enum State {
    On,
    Off,
}

impl Not for State {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            State::On => State::Off,
            State::Off => State::On,
        }
    }
}

/// The KeyState handles all of the decision making and state changes based on a high or low signal from a GPIO pin
#[derive(Copy, Clone)]
pub struct KeyState<
    const CSIZE: usize,
    const SCAN_PERIOD_US: u32,
    const DEBOUNCE_US: u32,
    const IDLE_MS: u32,
> {
    /// Most recently GPIO reading (not debounced)
    raw_state: State,

    /// True key state after debounce processing (debounced)
    state: State,

    /// Used to determine if the key is idle (in Off state for IDLE_MS)
    idle: bool,

    /// Tracking bounce
    debounce_tracking: bool,

    /// Used to determine the state after debounce
    /// Increments if a value is not what's set as state.
    /// Decrements if the value is already set as state.
    /// If positive set to On
    /// If negative set to Off
    /// This value is reset to 0 after setting the new state
    raw_state_average: i32,

    /// Used to track the number of cycles since state has changed.
    cycles_since_state_change: u32,

    /// This is used to track the list GPIO read bounce
    ///
    /// If cycles * scan_period > DEBOUNCE_US then raw_state is assigned to state.
    cycles_since_last_bounce: u32,
}

impl<const CSIZE: usize, const SCAN_PERIOD_US: u32, const DEBOUNCE_US: u32, const IDLE_MS: u32>
    KeyState<CSIZE, SCAN_PERIOD_US, DEBOUNCE_US, IDLE_MS>
{
    pub fn new() -> Self {
        Self {
            raw_state: State::Off,
            state: State::Off,
            idle: false,
            debounce_tracking: false,
            raw_state_average: 0,
            cycles_since_state_change: 0,
            cycles_since_last_bounce: 0,
        }
    }

    /// Record the GPIO read event and adjust debounce state machine accordingly
    ///
    /// Returns:
    /// (State, idle, cycles_since_state_change)
    pub fn record(&mut self, on: bool) -> (State, bool, u32) {
        // Track raw state average
        // This is used to set the new state
        if self.debounce_tracking {
            if on && self.state == State::Off || !on && self.state == State::On {
                self.raw_state_average += 1;
            } else {
                self.raw_state_average -= 1;
            }
        }

        // Update the raw state as a bounce event if not the same as the previous scan iteration
        // e.g. GPIO read value has changed since the last iteration
        if on && self.raw_state == State::Off || !on && self.raw_state == State::On {
            // Update raw state
            self.raw_state = if on { State::On } else { State::Off };

            // Reset bounce cycle counter
            self.cycles_since_last_bounce = 0;

            // Start debounce tracking (if we haven't already started)
            self.debounce_tracking = true;
            self.raw_state_average += 1;

            // Return current state
            return self.state();
        }

        // Increment debounce cycle counter
        self.cycles_since_last_bounce += 1;

        // Update the debounced state if it has changed and exceeded the debounce timer
        // (debounce timer resets if there is any bouncing during the debounce interval).
        if self.cycles_since_last_bounce * SCAN_PERIOD_US * CSIZE as u32 >= DEBOUNCE_US {
            // Since we have hit the cycles_since_last_bounce threshold, we can keep it here
            self.cycles_since_last_bounce -= 1;

            if self.raw_state != self.state && self.raw_state_average != 0 {
                // Update state
                // If the average is greater than 0, change the state
                let new_state = if self.raw_state_average > 0 {
                    !self.state
                } else {
                    self.state
                };

                // No longer idle
                self.idle = false;

                // Stop debounce tracking
                self.debounce_tracking = false;
                self.raw_state_average = 0;

                // Reset state transition cycle counter
                // and update state if it has changed.
                if new_state != self.state {
                    self.state = new_state;
                    self.cycles_since_state_change = 0;
                }

                // Return current state
                return self.state();
            }
        }

        // Increment state cycle counter
        self.cycles_since_state_change += 1;

        // Determine if key is idle
        // Must be both in the off state and have been off >= IDLE_MS
        self.idle = self.state == State::Off
            && self.cycles_since_state_change * SCAN_PERIOD_US * CSIZE as u32 / 1000 >= IDLE_MS;

        // Return current state
        self.state()
    }

    /// Returns thet current state and cycles since the state changed
    ///
    /// (State, idle, cycles_since_state_change)
    pub fn state(&self) -> (State, bool, u32) {
        (self.state, self.idle, self.cycles_since_state_change)
    }

    /// Number of cycles since the last state change
    /// 0 when the state first changes
    pub fn cycles_since_state_change(&self) -> u32 {
        self.cycles_since_state_change
    }

    /// True if the switch is idle
    /// idle indicates the switch is off and there have been no events
    pub fn idle(&self) -> bool {
        self.idle
    }
}

impl<const CSIZE: usize, const SCAN_PERIOD_US: u32, const DEBOUNCE_US: u32, const IDLE_MS: u32>
    Default for KeyState<CSIZE, SCAN_PERIOD_US, DEBOUNCE_US, IDLE_MS>
{
    fn default() -> Self {
        Self::new()
    }
}
