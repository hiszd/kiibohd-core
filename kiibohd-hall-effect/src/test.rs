// Copyright 2021-2023 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![cfg(test)]

// ----- Crates -----

use super::*;
use flexi_logger::Logger;

// ----- Types -----

// --- NOTE ---
// These thresholds were calculated on a Keystone v1.00 TKL pcb

// Max sample deviation
const MAX_DEVIATION: usize = 16;

// Calibration Mode Thresholds
const MIN_OK_THRESHOLD: usize = 1350;
// U1350 - b10101000110 - Switch not pressed (not 100% guaranteed, but the minimum range we can work withA
// Some sensors will have default values up to 1470 without any magnet and that is within the specs
// of the datasheet.

const MAX_OK_THRESHOLD: usize = 2500;
// U2500 - b100111000100 - Switch fully pressed

const NO_SENSOR_THRESHOLD: usize = 1000;
// Likely invalid ADC level from non-existent sensor (or very low magnet)

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

// ----- Tests -----

#[test]
fn invalid_index() {
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

    // Add data to an invalid location
    assert!(sensors
        .add_test::<1, MAX_DEVIATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(1, 0)
        .is_err());

    // Retrieve data from an invalid location
    assert!(sensors.get_data(1).is_err());
}

#[test]
fn not_ready() {
    setup_logging_lite().ok();
    trace!(
        "MAX_DEVIATION: {}  MIN_OK_THRESHOLD: {}  MAX_OK_THRESHOLD: {}  NO_SENSOR_THRESHOLD: {}",
        MAX_DEVIATION,
        MIN_OK_THRESHOLD,
        MAX_OK_THRESHOLD,
        NO_SENSOR_THRESHOLD
    );

    // Allocate a single sensor
    let sensors = Sensors::<1>::new().unwrap();

    // Retrieve before sending any data
    let state = sensors.get_data(0);
    if let Err(SensorError::CalibrationError(data)) = state.clone() {
        if data.cal == CalibrationStatus::NotReady {
            return;
        }
    }
    panic!("Unexpected state: {:?}", state);
}

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
