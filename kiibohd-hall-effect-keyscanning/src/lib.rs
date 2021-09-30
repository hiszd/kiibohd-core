// Copyright 2021 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![no_std]

use embedded_hal::digital::v2::OutputPin;
use kiibohd_hall_effect::{SenseAnalysis, SensorError, Sensors};

/// Handles strobing the Hall Effect sensor matrix
/// ADC reading is handled separately as the current embedded-hal doesn't work
/// well across oneshot, interrupt based and DMA ADC read methods.
/// (usually requires quite a bit of MCU specific setup as well)
///
/// ```rust,ignore
/// const CSIZE: usize = 18; // Number of columns
/// const RSIZE: usize = 6; // Number of rows
/// const MSIZE: usize = RSIZE * CSIZE; // Total matrix size
/// type Matrix = kiibohd_hall_effect_keyscanning::Matrix<PioX<Output<PushPull>>, CSIZE, MSIZE>; // atsam4-hal
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
/// let mut matrix = Matrix::new(cols).unwrap();
/// ```
pub struct Matrix<C: OutputPin, const CSIZE: usize, const MSIZE: usize> {
    cols: [C; CSIZE],
    cur_strobe: usize,
    sensors: Sensors<MSIZE>,
}

impl<C: OutputPin, const CSIZE: usize, const MSIZE: usize> Matrix<C, CSIZE, MSIZE> {
    pub fn new(cols: [C; CSIZE]) -> Result<Self, SensorError> {
        let sensors = Sensors::new()?;
        let res = Self {
            cols,
            cur_strobe: CSIZE - 1,
            sensors,
        };
        Ok(res)
    }

    /// Clears strobes
    /// Resets strobe counter to the last element (so next_strobe starts at 0)
    pub fn clear<'a, E: 'a>(&'a mut self) -> Result<(), E>
    where
        C: OutputPin<Error = E>,
    {
        // Clear strobes
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
        C: OutputPin<Error = E>,
    {
        // Unset current strobe
        self.cols[self.cur_strobe].set_low()?;

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

    /// Record ADC Hall Effect reading for the given the current row/sense index
    /// The sense index is usually 0-5, though it depends on the typical setup
    /// SC: Sample Count - How many samples before computing an analysis for a given index
    pub fn record<const SC: usize>(
        &mut self,
        index: usize,
        value: u16,
    ) -> Result<Option<&SenseAnalysis>, SensorError> {
        self.sensors.add::<SC>(index, value)
    }
}
