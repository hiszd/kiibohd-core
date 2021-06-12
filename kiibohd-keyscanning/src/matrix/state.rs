#[allow(unused_imports)]

use core::convert::Infallible;
use embedded_time::{duration::*, rate::*};

#[derive(Copy, Clone)]
pub enum State {
    Pressed,
    Bouncing,
    Released,
    Idle,
    Held,
}

#[derive(Copy, Clone)]
pub struct StateReturn {
    state_change: bool,
    ending_state: State,
}

/// The KeyState handles all of the decision making and state changes based on a high signal from the GPIO, or a low signal
#[derive(Copy, Clone)]
pub struct KeyState {
    state: State,
    prev_state: State,
    pressed_dur: u32,
    bouncing_dur: u32,
    released_dur: u32,
    held_dur: u32,
    bounce_limit: u32,
    idle_time: u32,
    scan_period: u32,
    held_limit: u32,
    idle_limit: u32,
}

impl KeyState {
    pub fn new(bounce_limit: &u32, held_limit: &u32, idle_limit: &u32, scan_period: &u32) -> KeyState {
        KeyState {
            state: State::Released, // Current key state
            prev_state: State::Released, // Previous key state
            pressed_dur: *Milliseconds(0_u32).integer(), // Duration(ms) the key has been pressed(after debouncing)
            bouncing_dur: *Milliseconds(0_u32).integer(), // Duration(ms) the key has been debouncing
            released_dur: *Milliseconds(0_u32).integer(), // Duration(ms) the key has been released
            held_dur: *Milliseconds(0_u32).integer(), // Duration(ms) the key has been held
            bounce_limit: *bounce_limit, // Duration(ms) that the key has to be high to be considered debounced
            idle_time: *Milliseconds(0_u32).integer(), // Duration(ms) the key has been idle
            scan_period: *scan_period, // the period of time(microseconds) that the scan takes
            held_limit: *held_limit, // Duration(ms) that the key needs to be pressed to be considered held
            idle_limit: *idle_limit, // Duration(ms) that the key needs to be released to be considered idle
        }
    }

    pub fn get_state(&self) -> State {
        self.state
    }
    
    /// The result of the last scan gets sent here and thie function determines the actual state change of the key.
    /// This function returns (state change, ending state)
    pub fn poll_update(&mut self, high: bool) -> StateReturn {
        let zero: u32 = *0_u32.milliseconds().integer();
        let scan_period: u32 = self.scan_period;
        let mut state_change: bool = false;
        self.prev_state = self.state;
        if high == false { // if the GPIO reads the input pin low
            match self.prev_state {
                State::Pressed => { // if the previous state was pressed and the input is read as low
                    self.state = State::Released;
                    self.pressed_dur = zero;
                    state_change = true;
                },
                State::Bouncing => { // the previous state was bouncing and the output was read as low
                    self.state = State::Released;
                    self.bouncing_dur = zero;
                    state_change = true;
                },
                State::Released => { // if the previous state was released and the input is still low
                    if self.released_dur >= self.idle_limit { // The key was Released and is now considered idle
                        self.state = State::Idle;
                        self.released_dur = zero;
                        state_change = true;
                    } else { // The key was released, but is not yet idle
                        self.state = State::Released;
                        self.released_dur = self.released_dur + scan_period;
                    }
                },
                State::Idle => { // if the previous state was idle and the input is still read as low
                    self.state = State::Idle;
                    self.idle_time = self.idle_time + scan_period;
                },
                State::Held => {
                    self.state = State::Released;
                    self.held_dur = zero;
                    state_change = true;
                }
            }
        } else if high == true { // if the GPIO reads the input pin high
            match self.prev_state {
                State::Pressed => {
                    if self.pressed_dur >= self.held_limit { // if the key was pressed
                        self.state = State::Held;
                        state_change = true;
                    } else {
                        self.state = State::Pressed;
                        self.pressed_dur = self.pressed_dur + scan_period;
                    }
                },
                State::Bouncing => {
                    if self.bouncing_dur >= self.bounce_limit { // The key has been pressed longer than the debounce limit and is officialyl pressed
                        self.state = State::Pressed;
                        self.bouncing_dur = zero;
                        state_change = true;
                    } else { // The key is still in the debounce phase
                        self.state = State::Bouncing;
                        self.bouncing_dur = self.bouncing_dur + scan_period;
                    }
                },
                State::Released => { // The key was released, but now the GPIO reads high
                    self.state = State::Bouncing;
                    self.released_dur = zero;
                    state_change = true;
                },
                State::Idle => { // The key was idle, but is now bouncing
                    self.state = State::Bouncing;
                    self.idle_time = zero;
                    state_change = true;
                },
                State::Held => { // The key was held, and is still held
                    self.state = State::Held;
                    self.held_dur = self.held_dur + scan_period;
                }
            }
        }

        return StateReturn{state_change: state_change, ending_state: self.state};
    }
}