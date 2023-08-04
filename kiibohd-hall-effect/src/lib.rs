// Copyright 2021-2023 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// ----- Modules -----

#![no_std]

pub mod lookup;
mod test;

// ----- Crates -----

#[cfg(feature = "defmt")]
use defmt::*;
use heapless::Vec;
#[cfg(not(feature = "defmt"))]
use log::*;

// ----- Sense Data -----

// Make sure there are at least 5 samples before trying to determine the sensor state (from NotReady)
const MIN_IDLE_LIMIT: usize = 5;

/// Indicates mode of the sensor
/// Used to specify a different lookup table and data processing behaviour
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SensorMode {
    /// Run ADC in reduced precision mode to collect additional calibration data
    /// Generally uses an alternate lookup table
    Test(&'static lookup::Entry),
    /// Low latency mode (usually the same as NormalMode)
    LowLatency(&'static lookup::Entry),
    /// Normal mode for ADC
    Normal(&'static lookup::Entry),
}

impl SensorMode {
    pub fn entry(&self) -> &lookup::Entry {
        match self {
            SensorMode::Test(entry) => entry,
            SensorMode::LowLatency(entry) => entry,
            SensorMode::Normal(entry) => entry,
        }
    }
}

/// Calibration status indicates if a sensor position is ready to send
/// analysis for a particular key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CalibrationStatus {
    /// Still trying to determine status (from power-on)
    /// Raw value must settle within a range before the sensor is considered ready
    /// This range is determined by the ADC mode that is currently set
    /// (e.g. 600-800 for at least 5 seconds)
    NotReady = 0,
    /// ADC value at 0 (test mode only)
    SensorMissing = 1,
    /// Reading higher than ADC supports (invalid), or magnet is too strong (test mode only)
    SensorBroken = 2,
    /// Sensor value low (not enough data to quantify further)
    SensorLow = 6,
    /// Magnet detected, min calibrated, positive range
    MagnetDetected = 3,
    /// Magnet detected, wrong pole direction (test mode only)
    MagnetWrongPole = 4,
    /// Invalid index
    InvalidIndex = 5,
}

impl CalibrationStatus {
    /// Update calibration status
    /// Returns true if the sensor is ready/calibrated
    pub fn update_calibration(&mut self, reading: u16, mode: SensorMode) -> bool {
        let entry = mode.entry();
        // Make sure reading isn't too low
        if reading < entry.min_ok_value {
            // Don't try to determine the true state, we'll do that later
            *self = CalibrationStatus::NotReady;
        }
        self.is_calibrated()
    }

    /// Easy check whether or not the sensor is ready
    pub fn is_calibrated(&self) -> bool {
        matches!(self, CalibrationStatus::MagnetDetected)
    }

    /// Detailed calibration status
    /// Returns a more detailed calibration status (takes a few more steps and is not necessary
    /// during normal operation)
    pub fn detailed_calibration(&self, data: &SenseData) -> CalibrationStatus {
        match self {
            CalibrationStatus::MagnetDetected => *self,
            _ => {
                match data.mode {
                    // More detailed analysis due to additional ADC range
                    SensorMode::Test(entry) => {
                        if data.data.value == 0 {
                            CalibrationStatus::SensorMissing
                        } else if data.data.value > entry.max_ok_value {
                            CalibrationStatus::SensorBroken
                        } else if data.data.value < entry.min_ok_value {
                            CalibrationStatus::MagnetWrongPole
                        } else if data.data.value < entry.min_idle_value {
                            CalibrationStatus::SensorLow
                        } else {
                            CalibrationStatus::NotReady
                        }
                    }
                    // Simplified analysis due to optimized ADC range
                    SensorMode::LowLatency(entry) | SensorMode::Normal(entry) => {
                        if data.data.value < entry.min_idle_value {
                            CalibrationStatus::SensorLow
                        } else {
                            CalibrationStatus::NotReady
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SensorError {
    CalibrationError(SenseData),
    FailedToResize(usize),
    InvalidSensor(usize),
    NoSensors,
}

/// Records momentary push button events
///
/// Cycles can be converted to time by multiplying by the scan period (Matrix::period())
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeyState {
    /// Passed activation point
    On {
        /// Cycles since the last state change
        cycles_since_state_change: u32,
    },
    /// Passed deactivation point
    Off {
        /// Cycles since the last state change
        cycles_since_state_change: u32,
    },
}

/// Calculations:
///  d = linearized(adc sample) --> distance
///  v = (d - d_prev) / 1       --> velocity
///  a = (v - v_prev) / 2       --> acceleration
///  j = (a - a_prev) / 3       --> jerk
///
/// These calculations assume constant time delta of 1
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SenseAnalysis {
    /// Threshold state
    pub state: KeyState,
    /// Distance value (lookup + min/max alignment)
    pub distance: i16,
    /// Velocity calculation (*)
    pub velocity: i16,
    /// Acceleration calculation (*)
    pub acceleration: i16,
    /// Jerk calculation (*)
    pub jerk: i16,
}

impl SenseAnalysis {
    /// Using the raw value do calculations
    pub fn new(data: &SenseData) -> Self {
        // Lookup distance
        let entry = data.mode.entry();
        let distance = entry.lookup(data.data.value, data.raw_offset);

        // Update key state
        let state = match data.analysis.state {
            KeyState::On {
                cycles_since_state_change,
            } => {
                if distance <= data.deactivation {
                    // Key is now off
                    KeyState::Off {
                        cycles_since_state_change: 0,
                    }
                } else {
                    // Key is still on
                    KeyState::On {
                        cycles_since_state_change: cycles_since_state_change.saturating_add(1),
                    }
                }
            }
            KeyState::Off {
                cycles_since_state_change,
            } => {
                if distance >= data.activation {
                    // Key is now on
                    KeyState::On {
                        cycles_since_state_change: 0,
                    }
                } else {
                    // Key is still off
                    KeyState::Off {
                        cycles_since_state_change: cycles_since_state_change.saturating_add(1),
                    }
                }
            }
        };

        // Calculate velocity/acceleration/jerk
        let velocity = distance - data.analysis.distance; // / 1
        let acceleration = (velocity - data.analysis.velocity) / 2;
        // NOTE: To use jerk, the compile-time thresholds will need to be
        //       multiplied by 3 (to account for the missing / 3)
        let jerk = acceleration - data.analysis.acceleration;
        SenseAnalysis {
            state,
            distance,
            velocity,
            acceleration,
            jerk,
        }
    }

    /// Null entry
    pub fn null() -> SenseAnalysis {
        SenseAnalysis {
            state: KeyState::Off {
                cycles_since_state_change: u32::MAX,
            },
            distance: 0,
            velocity: 0,
            acceleration: 0,
            jerk: 0,
        }
    }
}

/// Stores incoming raw samples
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RawData {
    value: u16,
    average: u16,
    idle_count: u32,
}

impl RawData {
    /// Create a RawData instance
    pub fn new() -> Self {
        Self {
            value: 0,
            average: 0,
            idle_count: 0,
        }
    }

    /// Updates the raw value with a new reading.
    /// - Updates the running average
    /// - Increments the idle if average is within specified range
    /// - If idle count exceeds the specified threshold, the average is returned
    ///   This average is used to calibrate the minimum distance
    pub fn update<const IDLE_LIMIT: usize>(&mut self, value: u16, mode: SensorMode) -> Option<u16> {
        self.value = value;
        // Update average
        self.average = (self.average + value) / 2;

        // Update idle count
        let entry = mode.entry();
        if self.average >= entry.min_idle_value && self.average <= entry.max_idle_value {
            self.idle_count += 1;
        } else {
            self.idle_count = 0;
        }
        trace!(
            "RawData::update: value: {}, average: {}, idle_count: {} ({}..{}:{})",
            self.value,
            self.average,
            self.idle_count,
            entry.min_idle_value,
            entry.max_idle_value,
            entry.sensor_zero,
        );

        // Return average if idle count exceeds threshold
        if self.idle_count > IDLE_LIMIT as u32 {
            Some(self.average)
        } else {
            None
        }
    }

    /// Returns the current value
    pub fn value(&self) -> u16 {
        self.value
    }

    /// Returns the current average
    pub fn average(&self) -> u16 {
        self.average
    }

    /// Reset data, used when transitioning between calibration and normal modes
    pub fn reset(&mut self) {
        self.value = 0;
        self.average = 0;
        self.idle_count = 0;
    }
}

impl Default for RawData {
    fn default() -> Self {
        Self::new()
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
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SenseData {
    /// Computed lookup for the raw ADC data
    analysis: SenseAnalysis,
    /// Calibration status of the sensor
    cal: CalibrationStatus,
    /// Raw data tracking of the sensor
    data: RawData,
    /// Temperature/humidity compensation for the ADC->distance lookup
    raw_offset: i16,
    /// Processing mode + lookup table for ADC
    mode: SensorMode,
    /// Activation point (distance, push direction)
    /// TODO - The logic doesn't handle negative distance values yet where activation is less than
    /// deactivation.
    activation: i16,
    /// Deactivation point (distance, release direction)
    deactivation: i16,
}

impl SenseData {
    /// Create a new SenseData instance
    /// - mode: Sensor mode
    /// - activation: Activation point (distance, push direction)
    /// - deactivation: Deactivation point (distance, release direction)
    pub fn new(mode: SensorMode, activation: i16, deactivation: i16) -> SenseData {
        SenseData {
            analysis: SenseAnalysis::null(),
            cal: CalibrationStatus::NotReady,
            data: RawData::new(),
            raw_offset: 0, // Starts in NotReady mode, so this value is ignored
            mode,
            activation,
            deactivation,
        }
    }

    /// Add new sensor reading
    /// Only returns a value once the sensor has been properly calibrated and is within
    /// the expected range.
    pub fn add<const IDLE_LIMIT: usize>(&mut self, reading: u16) -> Option<&SenseData> {
        // Update raw data
        let entry = self.mode.entry();
        if let Some(average) = self.data.update::<IDLE_LIMIT>(reading, self.mode) {
            // New minimum value detected
            // Due to temperature and humidity, the sensor may drift
            //
            // Calculate the offset from the pre-computed lookup table
            self.raw_offset = average as i16 - entry.sensor_zero as i16;

            // When we have a new valid minimum value calibration is complete
            self.cal = CalibrationStatus::MagnetDetected;
        } else if self.cal == CalibrationStatus::NotReady
            && reading > entry.min_idle_value
            && reading < entry.max_idle_value
            && self.data.idle_count > MIN_IDLE_LIMIT as u32
        {
            // To allow the keyboard to be used immediately, use this intial value for the offset
            // And then use the average calculated later to better tune the switches
            self.raw_offset = reading as i16 - entry.sensor_zero as i16;
            self.cal = CalibrationStatus::MagnetDetected;
        }

        // If sensor is calibrated compute SenseAnalysis
        // Make sure the incoming value doesn't break the calibration
        if self.cal.update_calibration(reading, self.mode) {
            // Update analysis
            self.analysis = SenseAnalysis::new(self);
            Some(self)
        } else {
            // If key state was active before, deactivate it and send an event
            match self.analysis.state {
                KeyState::On { .. } => {
                    self.analysis = SenseAnalysis::null();
                    Some(self)
                }
                _ => None,
            }
        }
    }

    /// Current sensor analysis
    pub fn analysis(&self) -> Option<&SenseAnalysis> {
        if self.cal.is_calibrated() {
            Some(&self.analysis)
        } else {
            None
        }
    }

    /// Current sensor calibration status
    pub fn calibration(&self) -> CalibrationStatus {
        self.cal
    }

    /// Current raw sensor data
    pub fn data(&self) -> &RawData {
        &self.data
    }

    /// Raw offset value
    pub fn raw_offset(&self) -> i16 {
        self.raw_offset
    }

    /// Current sensor mode
    pub fn mode(&self) -> SensorMode {
        self.mode
    }

    /// Current activation point
    pub fn activation(&self) -> i16 {
        self.activation
    }

    /// Current deactivation point
    pub fn deactivation(&self) -> i16 {
        self.deactivation
    }

    /// Update activation/deactivation points
    pub fn update_activation(&mut self, activation: i16, deactivation: i16) {
        self.activation = activation;
        self.deactivation = deactivation;
    }

    /// Change sensor mode
    pub fn update_mode(&mut self, mode: SensorMode) {
        self.mode = mode;
    }
}

// ----- Hall Effect Interface ------

pub struct Sensors<const S: usize> {
    /// Sensors in this collection
    sensors: Vec<SenseData, S>,
}

impl<const S: usize> Sensors<S> {
    /// Initializes full Sensor array
    /// Only fails if static allocation fails (very unlikely)
    /// - mode: Sensor mode
    /// - activation: Activation point (distance, push direction)
    /// - deactivation: Deactivation point (distance, release direction)
    pub fn new(
        mode: SensorMode,
        activation: i16,
        deactivation: i16,
    ) -> Result<Sensors<S>, SensorError> {
        let mut sensors = Vec::new();
        if sensors
            .resize(S, SenseData::new(mode, activation, deactivation))
            .is_err()
        {
            Err(SensorError::FailedToResize(S))
        } else {
            Ok(Sensors { sensors })
        }
    }

    /// Add sense data for a specific sensor
    pub fn add<const IDLE_LIMIT: usize>(
        &mut self,
        index: usize,
        reading: u16,
    ) -> Result<Option<&SenseData>, SensorError> {
        if index < self.sensors.len() {
            Ok(self.sensors[index].add::<IDLE_LIMIT>(reading))
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

    /// Retrieve mode from the first entry
    /// NOTE: Modes generally require ADC configuration and it's usually
    ///       infeasible (due to calibration) to switch modes for different
    ///       sensors. It can work, but it's usually quite complicated and
    ///       chip/board specific (which is why we don't consider it)
    pub fn mode(&self) -> Result<SensorMode, SensorError> {
        if self.is_empty() {
            return Err(SensorError::NoSensors);
        }
        Ok(self.sensors[0].mode())
    }

    /// Update all sensors to use the specified mode
    pub fn update_mode(&mut self, mode: SensorMode) {
        for sensor in self.sensors.iter_mut() {
            sensor.update_mode(mode);
        }
    }
}

#[cfg(feature = "kll-core")]
mod converters {
    #[cfg(feature = "defmt")]
    use defmt::*;
    #[cfg(not(feature = "defmt"))]
    use log::*;

    use crate::{CalibrationStatus, KeyState, SenseData, SensorMode};
    use heapless::Vec;
    use kll_core::layout::TriggerEventIterator;

    impl SenseData {
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
        ///
        /// In LowLatency mode only PressHoldReleaseOff events are generated using the per key
        /// activation point configuration
        pub fn trigger_events<const MAX_EVENTS: usize>(
            &self,
            index: usize,
            ignore_off: bool,
        ) -> TriggerEventIterator<MAX_EVENTS> {
            let mut events = Vec::new();

            // Only create events if the sensor is calibrated
            if self.cal == CalibrationStatus::MagnetDetected {
                // Handle on/off events
                match self.analysis.state {
                    KeyState::On {
                        cycles_since_state_change,
                    } => {
                        if cycles_since_state_change == 0 {
                            trace!(
                                "Reading: {} {:?} {} {}",
                                index,
                                self.analysis.state,
                                self.analysis.distance,
                                self.activation
                            );
                            events
                                .push(kll_core::TriggerEvent::Switch {
                                    state: kll_core::trigger::Phro::Press,
                                    index: index as u16,
                                    last_state: 0,
                                })
                                .unwrap();
                        } else {
                            events
                                .push(kll_core::TriggerEvent::Switch {
                                    state: kll_core::trigger::Phro::Hold,
                                    index: index as u16,
                                    last_state: cycles_since_state_change,
                                })
                                .unwrap();
                        }
                    }
                    KeyState::Off {
                        cycles_since_state_change,
                    } => {
                        if cycles_since_state_change == 0 {
                            trace!(
                                "Reading: {} {:?} {} {}",
                                index,
                                self.analysis.state,
                                self.analysis.distance,
                                self.deactivation
                            );
                            events
                                .push(kll_core::TriggerEvent::Switch {
                                    state: kll_core::trigger::Phro::Release,
                                    index: index as u16,
                                    last_state: 0,
                                })
                                .unwrap();
                        // Ignore off events unless ignore_off is set
                        } else if !ignore_off {
                            events
                                .push(kll_core::TriggerEvent::Switch {
                                    state: kll_core::trigger::Phro::Off,
                                    index: index as u16,
                                    last_state: cycles_since_state_change,
                                })
                                .unwrap();
                        }
                    }
                }

                // Handle analog events
                match self.mode() {
                    SensorMode::Test(_) | SensorMode::Normal(_) => {
                        if self.analysis.velocity != 0 || ignore_off {
                            events
                                .extend_from_slice(&[
                                    kll_core::TriggerEvent::AnalogDistance {
                                        index: index as u16,
                                        val: self.analysis.distance,
                                    },
                                    kll_core::TriggerEvent::AnalogVelocity {
                                        index: index as u16,
                                        val: self.analysis.velocity,
                                    },
                                    kll_core::TriggerEvent::AnalogAcceleration {
                                        index: index as u16,
                                        val: self.analysis.acceleration,
                                    },
                                    kll_core::TriggerEvent::AnalogJerk {
                                        index: index as u16,
                                        val: self.analysis.jerk,
                                    },
                                ])
                                .unwrap()
                        }
                    }
                    _ => {}
                }
            }
            TriggerEventIterator::new(events)
        }
    }
}
