// Copyright 2021-2023 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// ----- Modules -----

#![no_std]

mod rawlookup;
mod test;

// ----- Crates -----

#[cfg(feature = "defmt")]
use defmt::*;
use heapless::Vec;
#[cfg(not(feature = "defmt"))]
use log::*;

// TODO Use features to determine which lookup table to use
use rawlookup::MODEL;

// ----- Sense Data -----

/// Calibration status indicates if a sensor position is ready to send
/// analysis for a particular key.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CalibrationStatus {
    NotReady = 0,                 // Still trying to determine status (from power-on)
    SensorMissing = 1,            // ADC value at 0
    SensorBroken = 2, // Reading higher than ADC supports (invalid), or magnet is too strong
    MagnetDetected = 3, // Magnet detected, min calibrated, positive range
    MagnetWrongPoleOrMissing = 4, // Magnet detected, wrong pole direction
    InvalidIndex = 5, // Invalid index
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SensorError {
    CalibrationError(SenseData),
    FailedToResize(usize),
    InvalidSensor(usize),
}

/// Calculations:
///  d = linearized(adc sample) --> distance
///  v = (d - d_prev) / 1       --> velocity
///  a = (v - v_prev) / 2       --> acceleration
///  j = (a - a_prev) / 3       --> jerk
///
/// These calculations assume constant time delta of 1
#[repr(C)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SenseAnalysis {
    pub raw: u16,          // Raw ADC reading
    pub distance: i16,     // Distance value (lookup + min/max alignment)
    pub velocity: i16,     // Velocity calculation (*)
    pub acceleration: i16, // Acceleration calculation (*)
    pub jerk: i16,         // Jerk calculation (*)
}

impl SenseAnalysis {
    /// Using the raw value do calculations
    /// Requires the previous analysis
    pub fn new(raw: u16, data: &SenseData) -> SenseAnalysis {
        // Do raw lookup (we've already checked the bounds)
        let initial_distance = MODEL[raw as usize];

        /*
        // Min/max adjustment
        let distance_offset = match data.cal {
            CalibrationStatus::MagnetDetected => {
                // Subtract the min lookup
                // Lookup table has negative values for unexpectedly
                // small values (greater than sensor center)
                MODEL[data.stats.min as usize]
            }
            _ => {
                // Invalid reading
                return SenseAnalysis::null();
            }
        };
        */
        let distance_offset = MODEL[data.stats.min as usize];
        let distance = initial_distance - distance_offset;
        let velocity = distance - data.analysis.distance; // / 1
        let acceleration = (velocity - data.analysis.velocity) / 2;
        // NOTE: To use jerk, the compile-time thresholds will need to be
        //       multiplied by 3 (to account for the missing / 3)
        let jerk = acceleration - data.analysis.acceleration;
        SenseAnalysis {
            raw,
            distance,
            velocity,
            acceleration,
            jerk,
        }
    }

    /// Null entry
    pub fn null() -> SenseAnalysis {
        SenseAnalysis {
            raw: 0,
            distance: 0,
            velocity: 0,
            acceleration: 0,
            jerk: 0,
        }
    }
}

/// Keeps track of the direction of the adc values
/// The direction changes when the ADC exceeds the value of MAX_DEV in the opposite direction
/// The scratch value is immediately updated when moving in the same direction.
/// If the next value is in the opposite direction, the scratch value will only be updated if it
/// exceeds MAX_DEV.
/// This should greatly stabilize ADCs while still allowing for high sensitivity.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum Direction {
    Increase = 0,
    Decrease = 1,
}

/// Stores incoming raw samples
#[repr(C)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RawData {
    scratch: u32,
    prev_average: u16,
    deviation: i16,
    scratch_samples: u8,
    direction: Direction,
}

impl RawData {
    fn new() -> RawData {
        RawData {
            scratch: 0,
            prev_average: 0,
            deviation: 0,
            scratch_samples: 0,
            direction: Direction::Increase,
        }
    }

    /// Adds to the internal scratch location
    /// Designed to accumulate until a set number of readings added
    /// SC: specifies the number of scratch samples until ready to average
    ///     Should be a power of two (1, 2, 4, 8, 16...) for the compiler to
    ///     optimize.
    fn add<const SC: usize, const MAX_DEV: usize>(&mut self, reading: u16) -> Option<u16> {
        // If the previous sample deviates more than MAX_DEV, then reject
        // all the samples in the set. This is to work around ADC noise that can be
        // difficult to filter out.
        // In general this should be imperceptible to the user and give a more consistent
        // response to events.
        //
        // Make sure we have an even number of samples, otherwise ignore the last one
        if !(SC & 1 == 1 && self.scratch_samples == SC as u8 - 1) {
            // Check if even or odd
            // Even - Add to deviation
            // Odd - Subtract from deviation
            if self.scratch_samples & 1 == 0 {
                // Even
                self.deviation += reading as i16;
            } else {
                // Odd
                self.deviation -= reading as i16;
            }
        }

        self.scratch += reading as u32;
        self.scratch_samples += 1;
        trace!(
            "Reading: {}  Sample: {}/{}",
            reading,
            self.scratch_samples,
            SC as u8
        );

        if self.scratch_samples == SC as u8 {
            let mut cur_average: u16 = (self.scratch / SC as u32) as u16;
            trace!("Averaging: {} / {} = {}", self.scratch, SC, cur_average);

            // Check deviation, and ignore this sample set if it's too high
            if SC > 1 && self.deviation.abs() > MAX_DEV as i16 {
                // Reset scratch
                self.scratch = 0;
                self.scratch_samples = 0;
                self.deviation = 0;
                return None;
            }

            let change = cur_average as i32 - self.prev_average as i32;

            // Check direction
            match self.direction {
                Direction::Increase => {
                    if change < 0 {
                        if change.abs() > MAX_DEV as i32 {
                            // Direction changed
                            self.direction = Direction::Decrease;
                        } else {
                            // Not enough change to change direction, so we use the previous
                            // samples
                            cur_average = self.prev_average;
                        }
                    }
                }
                Direction::Decrease => {
                    if change > 0 {
                        if change.abs() > MAX_DEV as i32 {
                            // Direction changed
                            self.direction = Direction::Increase;
                        } else {
                            // Not enough change to change direction, so we use the previous
                            // samples
                            cur_average = self.prev_average;
                        }
                    }
                }
            }

            let val = if self.prev_average == 0 {
                cur_average
            } else {
                // Average previous value if non-zero
                let val = (cur_average + self.prev_average) / 2;
                trace!(
                    "Averaging prev: ({} + {}) / 2 = {}",
                    cur_average,
                    self.prev_average,
                    val
                );
                val
            };

            self.prev_average = val;
            self.scratch = 0;
            self.scratch_samples = 0;
            self.deviation = 0;
            trace!("Result: {}", val);
            Some(val)
        } else {
            None
        }
    }

    /// Reset data, used when transitioning between calibration and normal modes
    fn reset(&mut self) {
        self.scratch = 0;
        self.scratch_samples = 0;
        self.prev_average = 0;
        self.deviation = 0;
    }
}

/// Sense stats include statistically information about the sensor data
#[repr(C)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SenseStats {
    pub min: u16,     // Minimum raw value (reset when out of calibration)
    pub max: u16,     // Maximum raw value (reset when out of calibration)
    pub samples: u32, // Total number of samples (does not reset)
}

impl SenseStats {
    fn new() -> SenseStats {
        SenseStats {
            min: 0xFFFF,
            max: 0x0000,
            samples: 0,
        }
    }

    /// Reset, resettable stats (e.g. min, max, but not samples)
    fn reset(&mut self) {
        self.min = 0xFFFF;
        self.max = 0x0000;
    }
}

/// Sense data is store per ADC source element (e.g. per key)
/// The analysis is stored in a queue, where old values expire out
/// min/max is used to handle offsets from the distance lookups
/// Higher order calculations assume a constant unit of time between measurements
/// Any division is left to compile-time comparisions as it's not necessary
/// to actually compute the final higher order values in order to make a decision.
/// This diagram can give a sense of how the incoming data is used.
/// The number represents the last ADC sample required to calculate the value.
///
/// ```text,ignore
///
///            4  5 ... <- Jerk (e.g. m/2^3)
///          / | /|
///         3  4  5 ... <- Acceleration (e.g. m/2^2)
///       / | /| /|
///      2  3  4  5 ... <- Velocity (e.g. m/s)
///    / | /| /| /|
///   1  2  3  4  5 ... <- Distance (e.g. m)
///  ----------------------
///   1  2  3  4  5 ... <== ADC Averaged Sample
///
/// ```
///
/// Distance     => Min/Max adjusted lookup
/// Velocity     => (d_current - d_previous) / 1 (constant time)
///                 There is 1 time unit between samples 1 and 2
/// Acceleration => (v_current - v_previous) / 2 (constant time)
///                 There are 2 time units between samples 1 and 3
/// Jerk         => (a_current - a_previous) / 3 (constant time)
///                 There are 3 time units between samples 1 and 4
///
/// NOTE: Division is computed at compile time for jerk (/ 3)
///
/// Time is simplified to 1 unit (normally sampling will be at a constant time-rate, so this should be somewhat accurate).
///
/// A variety of thresholds are used during calibration and normal operating modes.
/// These values are generics as there's no reason to store each of the thresholds at runtime for
/// each sensor (wastes precious sram per sensor).
///
/// Calibration Mode:
/// * MNOK: Min valid calibration (Wrong magnet direction; wrong pole, less than a specific value)
/// * MXOK: Max valid calibration (Bad Sensor threshold; sensor is bad if reading is higher than this value)
/// * NS: No sensor detected (less than a specific value)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SenseData {
    pub analysis: SenseAnalysis,
    pub cal: CalibrationStatus,
    pub data: RawData,
    pub stats: SenseStats,
}

impl SenseData {
    pub fn new() -> SenseData {
        SenseData {
            analysis: SenseAnalysis::null(),
            cal: CalibrationStatus::NotReady,
            data: RawData::new(),
            stats: SenseStats::new(),
        }
    }

    /// Acculumate a new sensor reading
    /// Once the required number of samples is retrieved, do analysis
    /// Analysis does a few more addition, subtraction and comparisions
    /// so it's a more expensive operation.
    /// Normal mode
    fn add<const SC: usize, const MAX_DEV: usize>(
        &mut self,
        reading: u16,
    ) -> Result<Option<&SenseAnalysis>, SensorError> {
        // Add value to accumulator
        if let Some(data) = self.data.add::<SC, MAX_DEV>(reading) {
            // Check min/max values
            if data > self.stats.max {
                self.stats.max = data;
            }
            if data < self.stats.min {
                self.stats.min = data;
            }
            trace!(
                "Reading: {}  Result: {}  Stats: {:?}",
                reading,
                data,
                self.stats
            );

            // As soon as we have enough values accumulated, set magnet as detected in normal mode
            self.cal = CalibrationStatus::MagnetDetected;

            // Calculate new analysis (requires previous results + min/max)
            self.analysis = SenseAnalysis::new(data, self);
            Ok(Some(&self.analysis))
        } else {
            Ok(None)
        }
    }

    /// Acculumate a new sensor reading
    /// Once the required number of samples is retrieved, do analysis
    /// Analysis does a few more addition, subtraction and comparisions
    /// so it's a more expensive operation.
    /// Test mode
    fn add_test<
        const SC: usize,
        const MAX_DEV: usize,
        const MNOK: usize,
        const MXOK: usize,
        const NS: usize,
    >(
        &mut self,
        reading: u16,
    ) -> Result<Option<&SenseAnalysis>, SensorError> {
        // Add value to accumulator
        if let Some(data) = self.data.add::<SC, MAX_DEV>(reading) {
            // Check min/max values
            if data > self.stats.max {
                self.stats.max = data;
            }
            if data < self.stats.min {
                self.stats.min = data;
            }

            // Check calibration
            self.cal = self.check_calibration::<MNOK, MXOK, NS>(data);
            trace!(
                "Reading: {}  Result: {}  Cal: {:?}  Stats: {:?}",
                reading,
                data,
                self.cal,
                self.stats
            );
            match self.cal {
                CalibrationStatus::MagnetDetected => {}
                // Don't bother doing calculations if magnet+sensor isn't ready
                _ => {
                    // Reset min/max
                    self.stats.reset();
                    // Reset averaging
                    self.data.reset();
                    // Clear analysis, only set raw
                    self.analysis = SenseAnalysis::null();
                    self.analysis.raw = data;
                    return Err(SensorError::CalibrationError(self.clone()));
                }
            }

            // Calculate new analysis (requires previous results + min/max)
            self.analysis = SenseAnalysis::new(data, self);
            Ok(Some(&self.analysis))
        } else {
            Ok(None)
        }
    }

    /// Update calibration state
    /// Calibration is different depending on whether or not we've already been successfully
    /// calibrated. Gain and offset are set differently depending on whether the sensor has been
    /// calibrated. Uncalibrated sensors run at a lower gain to gather more details around voltage
    /// limits. Wherease calibrated sensors run at higher gain (and likely an offset) to maximize
    /// the voltage range of the desired sensor range.
    /// NOTE: This implementation (currently) only works for a single magnet pole of a bipolar sensor.
    fn check_calibration<const MNOK: usize, const MXOK: usize, const NS: usize>(
        &self,
        data: u16,
    ) -> CalibrationStatus {
        // Value too high, likely a bad sensor or bad soldering on the pcb
        // Magnet may also be too strong.
        if data > MXOK as u16 {
            return CalibrationStatus::SensorBroken;
        }
        // No sensor detected
        if data < NS as u16 {
            return CalibrationStatus::SensorMissing;
        }
        // Wrong pole (or magnet may be too weak)
        if data < MNOK as u16 {
            return CalibrationStatus::MagnetWrongPoleOrMissing;
        }

        CalibrationStatus::MagnetDetected
    }
}

impl Default for SenseData {
    fn default() -> Self {
        SenseData::new()
    }
}

// ----- Hall Effect Interface ------

pub struct Sensors<const S: usize> {
    sensors: Vec<SenseData, S>,
}

impl<const S: usize> Sensors<S> {
    /// Initializes full Sensor array
    /// Only fails if static allocation fails (very unlikely)
    pub fn new() -> Result<Sensors<S>, SensorError> {
        let mut sensors = Vec::new();
        if sensors.resize_default(S).is_err() {
            Err(SensorError::FailedToResize(S))
        } else {
            Ok(Sensors { sensors })
        }
    }

    /// Add sense data for a specific sensor
    pub fn add<const SC: usize, const MAX_DEV: usize>(
        &mut self,
        index: usize,
        reading: u16,
    ) -> Result<Option<&SenseAnalysis>, SensorError> {
        trace!("Index: {}  Reading: {}", index, reading);
        if index < self.sensors.len() {
            self.sensors[index].add::<SC, MAX_DEV>(reading)
        } else {
            Err(SensorError::InvalidSensor(index))
        }
    }

    /// Add sense data for a specific sensor
    /// Test mode
    pub fn add_test<
        const SC: usize,
        const MAX_DEV: usize,
        const MNOK: usize,
        const MXOK: usize,
        const NS: usize,
    >(
        &mut self,
        index: usize,
        reading: u16,
    ) -> Result<Option<&SenseAnalysis>, SensorError> {
        trace!("Index: {}  Reading: {}", index, reading);
        if index < self.sensors.len() {
            self.sensors[index].add_test::<SC, MAX_DEV, MNOK, MXOK, NS>(reading)
        } else {
            Err(SensorError::InvalidSensor(index))
        }
    }

    pub fn get_data(&self, index: usize) -> Result<&SenseData, SensorError> {
        if index < self.sensors.len() {
            if self.sensors[index].cal == CalibrationStatus::NotReady {
                Err(SensorError::CalibrationError(self.sensors[index].clone()))
            } else {
                Ok(&self.sensors[index])
            }
        } else {
            Err(SensorError::InvalidSensor(index))
        }
    }

    /// Max number of sensors
    pub fn len(&self) -> usize {
        S
    }

    pub fn is_empty(&self) -> bool {
        S == 0
    }
}

#[cfg(feature = "kll-core")]
mod converters {
    use crate::{CalibrationStatus, SenseAnalysis, SenseData};
    use heapless::Vec;
    use kll_core::TriggerEvent;

    impl SenseAnalysis {
        /// Convert SenseData to a TriggerEvent
        /// Criteria used to generate the event (an event may not be ready yet)
        /// - Distance movement must be non-zero (velocity)
        /// - Enough samples must be generated for each kind of event
        ///   This only matters when initializing the datastructures, steady-state always has
        ///   enough samples
        ///   * 1 sample for distance
        ///   * 2 samples for velocity
        ///   * 3 samples for acceleration
        ///   * 4 samples for jerk
        pub fn trigger_event(&self, index: usize, ignore_off: bool) -> Vec<TriggerEvent, 4> {
            let index: u16 = index as u16;
            // Make sure there has been distance movement
            if self.velocity != 0 || ignore_off {
                Vec::from_slice(&[
                    TriggerEvent::AnalogDistance {
                        index,
                        val: self.distance,
                    },
                    TriggerEvent::AnalogVelocity {
                        index,
                        val: self.velocity,
                    },
                    TriggerEvent::AnalogAcceleration {
                        index,
                        val: self.acceleration,
                    },
                    TriggerEvent::AnalogJerk {
                        index,
                        val: self.jerk,
                    },
                ])
                .unwrap()
            } else {
                Vec::new()
            }
        }
    }

    impl SenseData {
        /// Conveniece conversion, uses earlier analysis
        /// Also validates calibration status
        pub fn trigger_event(&self, index: usize, ignore_off: bool) -> Vec<TriggerEvent, 4> {
            // Validate calibration
            if self.cal != CalibrationStatus::MagnetDetected {
                Vec::new()
            } else {
                self.analysis.trigger_event(index, ignore_off)
            }
        }
    }
}
