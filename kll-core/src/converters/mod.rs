// Copyright 2021-2022 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

mod analog;
mod animation;
mod layer;
mod led;
mod rotation;
mod switch;

mod convert {
    use crate::converters::{analog, animation, layer, led, rotation, switch};
    use crate::{Capability, CapabilityEvent, CapabilityRun, TriggerCondition, TriggerEvent};

    impl From<TriggerEvent> for CapabilityRun {
        fn from(event: TriggerEvent) -> Self {
            match event {
                TriggerEvent::Switch { .. } => switch::convert(event),
                TriggerEvent::Layer { .. } => layer::convert(event),
                TriggerEvent::HidLed { .. } => led::convert(event),
                TriggerEvent::Animation { .. } => animation::convert(event),
                TriggerEvent::Rotation { .. } => rotation::convert(event),
                TriggerEvent::None => CapabilityRun::NoOp {
                    state: CapabilityEvent::None,
                },
                TriggerEvent::AnalogDistance { .. } => analog::convert(event),
                TriggerEvent::AnalogVelocity { .. } => analog::convert(event),
                TriggerEvent::AnalogAcceleration { .. } => analog::convert(event),
                TriggerEvent::AnalogJerk { .. } => analog::convert(event),
                // TriggerEvent::Sleep =>,
                // TriggerEvent::Resume=>,
                // TriggerEvent::Inactive=>,
                // TriggerEvent::Active=>,
                other => {
                    panic!("*** remove once all events are handled ***\n TriggerEvent {:?} not recognised", other)
                }
            }
        }
    }

    /// Convert TriggerEvent into the u8 identifier
    impl From<TriggerEvent> for u8 {
        fn from(event: TriggerEvent) -> Self {
            match event {
                TriggerEvent::None => 0,
                TriggerEvent::Switch { .. } => 1,
                TriggerEvent::HidLed { .. } => 2,
                TriggerEvent::AnalogDistance { .. } => 3,
                TriggerEvent::AnalogVelocity { .. } => 4,
                TriggerEvent::AnalogAcceleration { .. } => 5,
                TriggerEvent::AnalogJerk { .. } => 6,
                TriggerEvent::Layer { .. } => 7,
                TriggerEvent::Animation { .. } => 8,
                TriggerEvent::Sleep { .. } => 9,
                TriggerEvent::Resume { .. } => 10,
                TriggerEvent::Inactive { .. } => 11,
                TriggerEvent::Active { .. } => 12,
                TriggerEvent::Rotation { .. } => 13,
            }
        }
    }

    /// Convert TriggerCondition into the u8 identifier
    impl From<TriggerCondition> for u8 {
        fn from(cond: TriggerCondition) -> Self {
            match cond {
                TriggerCondition::None => 0,
                TriggerCondition::Switch { .. } => 1,
                TriggerCondition::HidLed { .. } => 2,
                TriggerCondition::AnalogDistance { .. } => 3,
                TriggerCondition::AnalogVelocity { .. } => 4,
                TriggerCondition::AnalogAcceleration { .. } => 5,
                TriggerCondition::AnalogJerk { .. } => 6,
                TriggerCondition::Layer { .. } => 7,
                TriggerCondition::Animation { .. } => 8,
                TriggerCondition::Sleep { .. } => 9,
                TriggerCondition::Resume { .. } => 10,
                TriggerCondition::Inactive { .. } => 11,
                TriggerCondition::Active { .. } => 12,
                TriggerCondition::Rotation { .. } => 13,
            }
        }
    }

    /// Convert TriggerCondition to TriggerEvent
    /// Used for mainly for kll validation, events are time index 0
    /// and are assumed to be immediate with no history
    impl From<TriggerCondition> for TriggerEvent {
        fn from(cond: TriggerCondition) -> Self {
            match cond {
                TriggerCondition::None => TriggerEvent::None,
                TriggerCondition::Switch { state, index, .. } => TriggerEvent::Switch {
                    state,
                    index,
                    last_state: 0,
                },
                _ => {
                    panic!(
                        "TriggerCondition to TriggerEvent not implemented! {:?}",
                        cond
                    );
                }
            }
        }
    }

    /// Convert Capability to CapabilityRun
    /// Used for mainly for kll validation
    impl From<Capability> for CapabilityRun {
        fn from(cap: Capability) -> Self {
            let tevent = TriggerEvent::None;
            match cap {
                Capability::NoOp { state, .. } => CapabilityRun::NoOp {
                    state: state.event(tevent),
                },
                Capability::HidKeyboard { state, id, .. } => CapabilityRun::HidKeyboard {
                    state: state.event(tevent),
                    id,
                },
                _ => {
                    panic!("Capability to CapabilityRun not implemented! {:?}", cap);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{CapabilityEvent, CapabilityRun, TriggerEvent};

    #[test]
    fn non_event_converted_to_noop_run() {
        let expected = CapabilityRun::NoOp {
            state: CapabilityEvent::None,
        };
        let result: CapabilityRun = TriggerEvent::None.into();

        assert_eq!(result, expected);
    }
}
