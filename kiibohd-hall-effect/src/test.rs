// Copyright 2021-2023 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![cfg(test)]

// TODO
// General tests
// Normal
// - Test ok case
// - Test idle recalibrate
// - Test below min-ok
// - Test above max-ok
// Test
// - Test ok case
// - Test idle recalibrate
// - Test below min-ok
// - Test above max-ok
// - Test sensor missing
// - Test sensor broken
// - Test sensor wrong pole

// ----- Crates -----

use super::*;
use flexi_logger::Logger;

// ----- Types -----

// TODO calibrate these values

// Activation threshold
const ACTIVATION: i16 = 1000;

// Deactivation threshold
const DEACTIVATION: i16 = 800;

// Idle limit, used to determine when to reset calibration
const IDLE_LIMIT: usize = 50;

// ----- Enumerations -----

enum LogError {
    CouldNotStartLogger,
}

// ----- Functions -----

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

fn print_minimal_entry(entry: &'static lookup::Entry) {
    trace!("min_ok_value: {}", entry.min_ok_value);
    trace!("max_ok_value: {}", entry.max_ok_value);
    trace!("sensor_zero: {}", entry.sensor_zero);
    trace!("min_idle_value: {}", entry.min_idle_value);
    trace!("max_idle_value: {}", entry.max_idle_value);
}

// ----- General Tests -----

#[test]
fn invalid_index() {
    setup_logging_lite().ok();
    print_minimal_entry(&lookup::SILO_ATSAM4S_LC605_GAIN_4X);

    // Allocate a single sensor
    let mut sensors = Sensors::<1>::new(
        SensorMode::Normal(&lookup::SILO_ATSAM4S_LC605_GAIN_4X),
        ACTIVATION,
        DEACTIVATION,
    )
    .unwrap();

    // Add data to an invalid location
    assert!(sensors.add::<IDLE_LIMIT>(1, 0).is_err());

    // Retrieve data from an invalid location
    assert!(sensors.get_data(1).is_err());
}

// Updating mode
// - Setup initial mode
// - Switch mode
//   * Update activation/deactivation
//   * Update mode
// - Check mode
#[test]
fn mode_switch() {
    setup_logging_lite().ok();
    print_minimal_entry(&lookup::SILO_ATSAM4S_LC605_GAIN_2X);
    print_minimal_entry(&lookup::SILO_ATSAM4S_LC605_GAIN_4X);

    // Allocate a single sensor in normal mode
    let mode = SensorMode::Normal(&lookup::SILO_ATSAM4S_LC605_GAIN_4X);
    let mut sensors = Sensors::<1>::new(mode, ACTIVATION, DEACTIVATION).unwrap();
    assert!(sensors.mode().unwrap() == mode);

    // Switch to low-latency mode
    let mode = SensorMode::LowLatency(&lookup::SILO_ATSAM4S_LC605_GAIN_4X);
    sensors.update_mode(mode);
    assert!(sensors.mode().unwrap() == mode);

    // Switch to test mode
    let mode = SensorMode::Test(&lookup::SILO_ATSAM4S_LC605_GAIN_2X);
    sensors.update_mode(mode);
    assert!(sensors.mode().unwrap() == mode);
}

// ----- Normal Mode Tests -----

#[test]
fn normal_not_ready() {
    setup_logging_lite().ok();
    print_minimal_entry(&lookup::SILO_ATSAM4S_LC605_GAIN_4X);

    // Allocate a single sensor
    let sensors = Sensors::<1>::new(
        SensorMode::Normal(&lookup::SILO_ATSAM4S_LC605_GAIN_4X),
        ACTIVATION,
        DEACTIVATION,
    )
    .unwrap();

    // Retrieve before sending any data
    let state = sensors.get_data(0);
    if let Err(SensorError::CalibrationError(data)) = state.clone() {
        if data.cal == CalibrationStatus::NotReady {
            return;
        }
    }
    panic!("Unexpected state: {:?}", state);
}

// ----- Test Mode Tests -----

#[test]
fn test_not_ready() {
    setup_logging_lite().ok();
    print_minimal_entry(&lookup::SILO_ATSAM4S_LC605_GAIN_4X);

    // Allocate a single sensor
    let sensors = Sensors::<1>::new(
        SensorMode::Test(&lookup::SILO_ATSAM4S_LC605_GAIN_4X),
        ACTIVATION,
        DEACTIVATION,
    )
    .unwrap();

    // Retrieve before sending any data
    let state = sensors.get_data(0);
    if let Err(SensorError::CalibrationError(data)) = state.clone() {
        if data.cal == CalibrationStatus::NotReady {
            return;
        }
    }
    panic!("Unexpected state: {:?}", state);
}

/*
#[test]
fn deviation_check() {
    setup_logging_lite().ok();
    trace!(
        "MAX_DEVIATION: {}  MIN_OK_THRESHOLD: {}  MAX_OK_THRESHOLD: {}  NO_SENSOR_THRESHOLD: {}",
        MAX_DEVIATION,
        MIN_OK_THRESHOLD,
        MAX_OK_THRESHOLD,
        NO_SENSOR_THRESHOLD
    );

    // Allocate a single sensor
    let mut sensors = Sensors::<1>::new().unwrap();

    // Add two sensor samples, with a deviation between them of 20
    // (200 and 220)
    assert!(sensors
        .add_test::<2, MAX_DEVIATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0, 200
        )
        .is_ok());
    let state = sensors
        .add_test::<2, MAX_DEVIATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0, 220,
        );

    // We're expecting Ok(None) as the samples should be discarded
    if let Ok(None) = state.clone() {
        return;
    }
    panic!("Unexpected state: {:?}", state);
}

#[test]
fn sensor_missing() {
    setup_logging_lite().ok();
    trace!(
        "MAX_DEVIATION: {}  MIN_OK_THRESHOLD: {}  MAX_OK_THRESHOLD: {}  NO_SENSOR_THRESHOLD: {}",
        MAX_DEVIATION,
        MIN_OK_THRESHOLD,
        MAX_OK_THRESHOLD,
        NO_SENSOR_THRESHOLD
    );

    // Allocate a single sensor
    let mut sensors = Sensors::<1>::new().unwrap();

    // Add a sensor value of 0
    // (needs 2 samples to finish averaging)
    // Once averaging is complete, we'll get a result
    assert!(sensors
        .add_test::<2, MAX_DEVIATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0,
            NO_SENSOR_THRESHOLD as u16 - 1
        )
        .is_ok());
    let state = sensors
        .add_test::<2, MAX_DEVIATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0,
            NO_SENSOR_THRESHOLD as u16 - 1,
        );

    if let Err(SensorError::CalibrationError(data)) = state.clone() {
        if data.cal == CalibrationStatus::SensorMissing {
            return;
        }
    }
    panic!("Unexpected state: {:?}", state);
}

#[test]
fn sensor_broken() {
    setup_logging_lite().ok();
    trace!(
        "MAX_DEVIATION: {}  MIN_OK_THRESHOLD: {}  MAX_OK_THRESHOLD: {}  NO_SENSOR_THRESHOLD: {}",
        MAX_DEVIATION,
        MIN_OK_THRESHOLD,
        MAX_OK_THRESHOLD,
        NO_SENSOR_THRESHOLD
    );

    // Allocate a single sensor
    let mut sensors = Sensors::<1>::new().unwrap();

    // Add max sensor value
    // (needs 2 samples to finish averaging)
    // Once averaging is complete, we'll get a result
    assert!(sensors
        .add_test::<2, MAX_DEVIATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0, 0xFFFF,
        )
        .is_ok());
    let state = sensors
        .add_test::<2, MAX_DEVIATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0, 0xFFFF,
        );

    if let Err(SensorError::CalibrationError(data)) = state.clone() {
        if data.cal == CalibrationStatus::SensorBroken {
            return;
        }
    }
    panic!("Unexpected state: {:?}", state);
}

#[test]
fn magnet_missing() {
    setup_logging_lite().ok();
    trace!(
        "MAX_DEVIATION: {}  MIN_OK_THRESHOLD: {}  MAX_OK_THRESHOLD: {}  NO_SENSOR_THRESHOLD: {}",
        MAX_DEVIATION,
        MIN_OK_THRESHOLD,
        MAX_OK_THRESHOLD,
        NO_SENSOR_THRESHOLD
    );

    // Allocate a single sensor
    let mut sensors = Sensors::<1>::new().unwrap();

    // Add max sensor value
    // (needs 2 samples to finish averaging)
    // Once averaging is complete, we'll get a result
    assert!(sensors
        .add_test::<2, MAX_DEVIATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0,
            MIN_OK_THRESHOLD as u16 - 1
        )
        .is_ok());
    let state = sensors
        .add_test::<2, MAX_DEVIATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0,
            MIN_OK_THRESHOLD as u16 - 1,
        );

    if let Err(SensorError::CalibrationError(data)) = state.clone() {
        if data.cal == CalibrationStatus::MagnetWrongPoleOrMissing {
            return;
        }
    }
    panic!("Unexpected state: {:?}", state);
}

fn magnet_check_calibration<const U: usize>(sensors: &mut Sensors<U>) {
    // Add two values, larger MIN_OK_THRESHOLD
    let val = MIN_OK_THRESHOLD as u16 + MAX_DEVIATION as u16;
    // (needs 2 samples to finish averaging)
    // Once averaging is complete, we'll get a result
    assert!(sensors
        .add_test::<2, MAX_DEVIATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0, val
        )
        .is_ok());
    let state = sensors
        .add_test::<2, MAX_DEVIATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0, val,
        );

    let mut test = false;
    if let Ok(Some(rval)) = state.clone() {
        if rval.raw == val {
            test = true;
        }
    }
    assert!(test, "Unexpected state: {:?}", state);

    // Check calibration
    let mut test = false;
    let state = sensors.get_data(0);
    if let Ok(val) = state {
        if val.cal == CalibrationStatus::MagnetDetected {
            test = true;
        }
    }
    assert!(test, "Unexpected state: {:?}", state);
}

fn magnet_check_normal<const U: usize>(sensors: &mut Sensors<U>) {
    // Add two values, larger MIN_OK_THRESHOLD
    let val = MIN_OK_THRESHOLD as u16 + MAX_DEVIATION as u16;
    // (needs 2 samples to finish averaging)
    // Once averaging is complete, we'll get a result
    assert!(sensors
        .add_test::<2, MAX_DEVIATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0, val
        )
        .is_ok());
    let state = sensors
        .add_test::<2, MAX_DEVIATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0, val,
        );

    let mut test = false;
    if let Ok(Some(rval)) = state.clone() {
        if rval.raw == val {
            test = true;
        }
    }
    assert!(test, "Unexpected state: {:?}", state);

    // Check calibration
    let mut test = false;
    let state = sensors.get_data(0);
    if let Ok(val) = state {
        if val.cal == CalibrationStatus::MagnetDetected {
            test = true;
        }
    }
    assert!(test, "Unexpected state: {:?}", state);
}

fn magnet_calibrate<const U: usize>(sensors: &mut Sensors<U>) {
    // Calibrate sensor
    magnet_check_calibration::<U>(sensors);

    // Check again with normal operation
    magnet_check_normal::<U>(sensors);
}

#[test]
fn magnet_detected() {
    setup_logging_lite().ok();
    trace!(
        "MAX_DEVIATION: {}  MIN_OK_THRESHOLD: {}  MAX_OK_THRESHOLD: {}  NO_SENSOR_THRESHOLD: {}",
        MAX_DEVIATION,
        MIN_OK_THRESHOLD,
        MAX_OK_THRESHOLD,
        NO_SENSOR_THRESHOLD
    );

    // Allocate a single sensor
    let mut sensors = Sensors::<1>::new().unwrap();

    // Two sets of samples that will put the sensor into normal mode (and check both MagnetDetected
    // states)
    magnet_calibrate::<1>(&mut sensors);
}

#[test]
fn sensor_min_adjust() {
    setup_logging_lite().ok();
    trace!(
        "MAX_DEVIATION: {}  MIN_OK_THRESHOLD: {}  MAX_OK_THRESHOLD: {}  NO_SENSOR_THRESHOLD: {}",
        MAX_DEVIATION,
        MIN_OK_THRESHOLD,
        MAX_OK_THRESHOLD,
        NO_SENSOR_THRESHOLD
    );

    // Allocate a single sensor
    let mut sensors = Sensors::<1>::new().unwrap();

    // Baseline
    magnet_check_calibration::<1>(&mut sensors);
    magnet_check_normal::<1>(&mut sensors);

    // Send a lower value than the min calibration and make sure it was set
    let prev_avg = sensors.get_data(0).unwrap().data.prev_average;
    let val = prev_avg - (MAX_DEVIATION * 2) as u16;
    let calc_new_min = (val + prev_avg) / 2;

    assert!(sensors
        .add_test::<2, MAX_DEVIATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0, val
        )
        .is_ok());
    let state = sensors
        .add_test::<2, MAX_DEVIATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0, val,
        );
    let mut test = false;
    if let Ok(Some(rval)) = state.clone() {
        if rval.raw == calc_new_min {
            test = true;
        }
    }
    assert!(
        test,
        "Unexpected state: {:?} != {} (prev_avg: {})",
        state, val, prev_avg
    );

    // Check min calibration
    let new_min = sensors.get_data(0).unwrap().stats.min;
    assert!(
        calc_new_min == new_min,
        "Unexpected min: {} != {}",
        calc_new_min,
        new_min
    );
}

#[test]
fn increase_decrease_flip() {
    setup_logging_lite().ok();
    trace!(
        "MAX_DEVIATION: {}  MIN_OK_THRESHOLD: {}  MAX_OK_THRESHOLD: {}  NO_SENSOR_THRESHOLD: {}",
        MAX_DEVIATION,
        MIN_OK_THRESHOLD,
        MAX_OK_THRESHOLD,
        NO_SENSOR_THRESHOLD
    );

    // Allocate a single sensor
    let mut sensors = Sensors::<1>::new().unwrap();

    // Two sets of samples that will put the sensor into normal mode (and check both MagnetDetected
    // states)
    info!("Start magnet_calibrate");
    magnet_calibrate::<1>(&mut sensors);
    info!("End magnet_calibrate");

    // Add two sensor samples (no deviation), a deviation higer than the current
    let val = MIN_OK_THRESHOLD as u16 + MAX_DEVIATION as u16 * 2;
    let prev_avg = sensors.get_data(0).unwrap().data.prev_average;
    let calc_val = (prev_avg + val) / 2;

    info!("Higher value: {} start (averaged: {})", val, calc_val);
    assert!(sensors
        .add_test::<2, MAX_DEVIATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0, val
        )
        .is_ok());
    let state = sensors
        .add_test::<2, MAX_DEVIATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0, val,
        );
    let mut test = false;
    if let Ok(Some(rval)) = state.clone() {
        if rval.raw == calc_val {
            test = true;
        }
    }
    assert!(test, "Unexpected state: {:?} != {}", state, calc_val);
    info!("Higher value: {} end", val);

    // Decrease input value, but by less than a deviation
    let val = calc_val - MAX_DEVIATION as u16 / 2;
    info!("Lower value (fail): {} start", val);
    assert!(sensors
        .add_test::<2, MAX_DEVIATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0, val
        )
        .is_ok());
    let state = sensors
        .add_test::<2, MAX_DEVIATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0, val,
        );

    // We're expecting Ok(calc_val) as the samples should be discarded
    let mut test = false;
    if let Ok(Some(rval)) = state.clone() {
        if rval.raw == calc_val {
            test = true;
        }
    }
    assert!(test, "Unexpected state: {:?}", state);
    info!("Lower value (fail): {} end", val);

    // Decrease input value, but by more than a deviation
    let val = calc_val + MAX_DEVIATION as u16 - 1;
    let calc_val = (val + calc_val) / 2;
    info!("Lower value (pass): {} start (averaged: {})", val, calc_val);
    assert!(sensors
        .add_test::<2, MAX_DEVIATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0, val
        )
        .is_ok());
    let state = sensors
        .add_test::<2, MAX_DEVIATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0, val,
        );
    let mut test = false;
    if let Ok(Some(rval)) = state.clone() {
        if rval.raw == calc_val {
            test = true;
        }
    }
    assert!(test, "Unexpected state: {:?} != {}", state, calc_val);
    info!("Lower value (pass): {} end", val);
}
*/
