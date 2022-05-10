// Copyright 2021 Zion Koyl
// Copyright 2021-2022 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![no_std]

pub mod state;

pub use self::state::{KeyState, State};
use embedded_hal::digital::v2::{InputPin, IoPin, OutputPin, PinState};

/// Records momentary push button events
///
/// Cycles can be converted to time by multiplying by the scan period (Matrix::period())
#[derive(Copy, Clone, Debug, PartialEq, defmt::Format)]
pub enum KeyEvent {
    On {
        /// Cycles since the last state change
        cycles_since_state_change: u32,
    },
    Off {
        /// Key is idle (a key can only be idle in the off state)
        idle: bool,
        /// Cycles since the last state change
        cycles_since_state_change: u32,
    },
}

/// This struct handles scanning and strobing of the key matrix.
///
/// It also handles the debouncing of key input to ensure acurate keypresses are being read.
/// OutputPin's are passed as columns (cols) which are strobed.
/// IoPins are functionally InputPins (rows) which are read. Rows are IoPins in order to drain the
/// row/sense between strobes to prevent stray capacitance.
///
/// ```rust,ignore
/// const CSIZE: usize = 18; // Number of columns
/// const RSIZE: usize = 6; // Number of rows
/// const MSIZE: usize = RSIZE * CSIZE; // Total matrix size
/// // Period of time it takes to re-scan a column (everything must be constant time!)
/// const SCAN_PERIOD_US = 40;
/// // Debounce timer in us. Can only be as precise as a multiple of SCAN_PERIOD_US.
/// // Per-key timer is reset if the raw gpio reading changes for any reason.
/// const DEBOUNCE_US = 5000; // 5 ms
/// // Idle timer in ms. Only valid if the switch is in the off state.
/// const IDLE_MS = 600_0000; // 600 seconds or 10 minutes
///
/// let cols = [
///     pins.strobe1.downgrade(),
///     pins.strobe2.downgrade(),
///     pins.strobe3.downgrade(),
///     pins.strobe4.downgrade(),
///     pins.strobe5.downgrade(),
///     pins.strobe6.downgrade(),
///     pins.strobe7.downgrade(),
///     pins.strobe8.downgrade(),
///     pins.strobe9.downgrade(),
///     pins.strobe10.downgrade(),
///     pins.strobe11.downgrade(),
///     pins.strobe12.downgrade(),
///     pins.strobe13.downgrade(),
///     pins.strobe14.downgrade(),
///     pins.strobe15.downgrade(),
///     pins.strobe16.downgrade(),
///     pins.strobe17.downgrade(),
///     pins.strobe18.downgrade(),
/// ];
///
/// let rows = [
///     pins.sense1.downgrade(),
///     pins.sense2.downgrade(),
///     pins.sense3.downgrade(),
///     pins.sense4.downgrade(),
///     pins.sense5.downgrade(),
///     pins.sense6.downgrade(),
/// ];
///
/// let mut matrix = Matrix::<OutputPin, InputPin, CSIZE, RSIZE, MSIZE, SCAN_PERIOD_US, DEBOUNCE_US,
/// IDLE_MS>::new(cols, rows);
///
/// // Prepare first strobe
/// matrix.next_strobe().unwrap();
///
/// // --> This next part must be done in constant time (SCAN_PERIOD_US) <--
/// let state = matrix.sense().unwrap();
/// matrix.next_strobe().unwrap();
/// ```
pub struct Matrix<
    C: OutputPin,
    R: InputPin,
    const CSIZE: usize,
    const RSIZE: usize,
    const MSIZE: usize,
    const SCAN_PERIOD_US: u32,
    const DEBOUNCE_US: u32,
    const IDLE_MS: u32,
> {
    /// Strobe GPIOs (columns)
    cols: [C; CSIZE],
    /// Sense GPIOs (rows)
    rows: [R; RSIZE],
    /// Current GPIO column being strobed
    cur_strobe: usize,
    /// Recorded state of the entire matrix
    state_matrix: [KeyState<CSIZE, SCAN_PERIOD_US, DEBOUNCE_US, IDLE_MS>; MSIZE],
}

impl<
        C: OutputPin,
        R: InputPin,
        const CSIZE: usize,
        const RSIZE: usize,
        const MSIZE: usize,
        const SCAN_PERIOD_US: u32,
        const DEBOUNCE_US: u32,
        const IDLE_MS: u32,
    > Matrix<C, R, CSIZE, RSIZE, MSIZE, SCAN_PERIOD_US, DEBOUNCE_US, IDLE_MS>
{
    pub fn new<'a, E: 'a>(cols: [C; CSIZE], rows: [R; RSIZE]) -> Result<Self, E>
    where
        C: OutputPin<Error = E>,
        E: core::convert::From<<C as OutputPin>::Error>,
    {
        let state_matrix = [KeyState::<CSIZE, SCAN_PERIOD_US, DEBOUNCE_US, IDLE_MS>::new(); MSIZE];
        let mut res = Self {
            cols,
            rows,
            cur_strobe: CSIZE - 1,
            state_matrix,
        };

        // Reset strobe position and make sure all strobes are off
        res.clear()?;
        Ok(res)
    }

    /// Clears strobes
    /// Resets strobe counter to the last element (so next_strobe starts at 0)
    pub fn clear<'a, E: 'a>(&'a mut self) -> Result<(), E>
    where
        C: OutputPin<Error = E>,
    {
        // Clear all strobes
        for c in self.cols.iter_mut() {
            c.set_low()?;
        }

        // Reset strobe position
        self.cur_strobe = CSIZE - 1;
        Ok(())
    }

    /// Next strobe
    pub fn next_strobe<'a, E: 'a>(&'a mut self) -> Result<usize, E>
    where
        C: OutputPin<Error = E> + IoPin<R, C>,
        R: InputPin<Error = E> + IoPin<R, C>,
        E: core::convert::From<<R as IoPin<R, C>>::Error>
            + core::convert::From<<C as IoPin<R, C>>::Error>,
    {
        // Unset current strobe
        self.cols[self.cur_strobe].set_low()?;

        // Drain stray potential from sense lines
        // NOTE: This is unsafe because the gpio are stored in an array and (likely) do not implement
        //       copy or clone. Since they are in an array, we can't move them either.
        //       Since we're just temporarily sinking the pin and putting it back, this is safe to
        //       do.
        for s in self.rows.iter_mut() {
            let ptr = s as *const R;
            unsafe {
                let row = core::ptr::read(ptr);
                // Temporarily sink sense gpios and reset to sense/read gpio
                row.into_output_pin(PinState::Low)?.into_input_pin()?;
            }
        }

        // Check for roll-over condition
        if self.cur_strobe >= CSIZE - 1 {
            self.cur_strobe = 0;
        } else {
            self.cur_strobe += 1;
        }

        // Set new strobe
        self.cols[self.cur_strobe].set_high()?;

        Ok(self.cur_strobe)
    }

    /// Current strobe
    pub fn strobe(&self) -> usize {
        self.cur_strobe
    }

    /// Sense a column of switches
    ///
    /// Returns the results of each row for the currently strobed column and the measured strobe
    pub fn sense<'a, E: 'a>(&'a mut self) -> Result<([KeyEvent; RSIZE], usize), E>
    where
        E: core::convert::From<<R as InputPin>::Error>,
    {
        let mut res = [KeyEvent::Off {
            idle: false,
            cycles_since_state_change: 0,
        }; RSIZE];

        for (i, r) in self.rows.iter().enumerate() {
            // Read GPIO
            let on = r.is_high()?;
            // Determine matrix index
            let index = self.cur_strobe * RSIZE + i;
            // Record GPIO event and determine current status after debouncing algorithm
            let (keystate, idle, cycles_since_state_change) = self.state_matrix[index].record(on);

            // Assign KeyEvent using the output keystate
            res[i] = if keystate == State::On {
                KeyEvent::On {
                    cycles_since_state_change,
                }
            } else {
                KeyEvent::Off {
                    idle,
                    cycles_since_state_change,
                }
            };
        }

        Ok((res, self.cur_strobe))
    }

    /// Return the KeyState for a given index
    pub fn state(
        &self,
        index: usize,
    ) -> Option<KeyState<CSIZE, SCAN_PERIOD_US, DEBOUNCE_US, IDLE_MS>> {
        if index >= self.state_matrix.len() {
            None
        } else {
            Some(self.state_matrix[index])
        }
    }

    /// Generate event from KeyState
    /// Useful when trying to determine if a key has not been pressed
    pub fn generate_event(&self, index: usize) -> Option<KeyEvent> {
        let state = self.state(index);

        state.map(|state| match state.state().0 {
            State::On => KeyEvent::On {
                cycles_since_state_change: state.cycles_since_state_change(),
            },
            State::Off => KeyEvent::Off {
                idle: state.idle(),
                cycles_since_state_change: state.cycles_since_state_change(),
            },
        })
    }
}

#[cfg(feature = "kll-core")]
mod converters {
    use crate::KeyEvent;
    use heapless::Vec;
    use kll_core::TriggerEvent;

    impl KeyEvent {
        pub fn trigger_event(&self, index: usize) -> Vec<TriggerEvent, 4> {
            Vec::from_slice(&[match self {
                KeyEvent::On {
                    cycles_since_state_change,
                } => {
                    if *cycles_since_state_change == 0 {
                        defmt::trace!("Reading: {} {}", index, self);
                        kll_core::TriggerEvent::Switch {
                            state: kll_core::trigger::Phro::Press,
                            index: index as u16,
                            last_state: 0,
                        }
                    } else {
                        kll_core::TriggerEvent::Switch {
                            state: kll_core::trigger::Phro::Hold,
                            index: index as u16,
                            last_state: *cycles_since_state_change,
                        }
                    }
                }
                KeyEvent::Off {
                    idle: _,
                    cycles_since_state_change,
                } => {
                    if *cycles_since_state_change == 0 {
                        defmt::trace!("Reading: {} {}", index, self);
                        kll_core::TriggerEvent::Switch {
                            state: kll_core::trigger::Phro::Release,
                            index: index as u16,
                            last_state: 0,
                        }
                    } else {
                        kll_core::TriggerEvent::Switch {
                            state: kll_core::trigger::Phro::Off,
                            index: index as u16,
                            last_state: *cycles_since_state_change,
                        }
                    }
                }
            }])
            .unwrap()
        }
    }
}
