// Copyright 2022 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::error;
use crate::{CapabilityEvent, CapabilityRun, TriggerEvent};

/// Converts the passed analog `TriggerEvent` into a `CapabilityRun::Analog` with Passthrough
///
/// # Arguments
///
/// * `event`: The TriggerEvent to convert.  This should always be one of:
/// `TriggerEvent::AnalogDistance`, `TriggerEvent::AnalogVelocity`,
/// `TriggerEvent::AnalogAcceleration` or `TriggerEvent::AnalogJerk`.
/// if it is anything else a CapabilityRun::NoOp will be returned
///
/// returns: CapabilityRun::Analog
pub(super) fn convert(event: TriggerEvent) -> CapabilityRun {
    match event {
        TriggerEvent::AnalogDistance { .. }
        | TriggerEvent::AnalogVelocity { .. }
        | TriggerEvent::AnalogAcceleration { .. }
        | TriggerEvent::AnalogJerk { .. } => CapabilityRun::Analog {
            state: CapabilityEvent::Passthrough(event),
        },
        _ => {
            error!("Unexpected event {:?}", event);
            CapabilityRun::NoOp {
                state: CapabilityEvent::None,
            }
        }
    }
}
