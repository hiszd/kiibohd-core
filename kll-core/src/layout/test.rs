// Copyright 2021 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![cfg(test)]

// ----- Crates -----

use super::*;
use flexi_logger::Logger;
use log::*;

// ----- Enumerations -----

enum LogError {
    CouldNotStartLogger,
}

// ----- Functions -----

/// Lite logging setup
fn setup_logging_lite() -> Result<(), LogError> {
    match Logger::with_env_or_str("")
        .format(flexi_logger::colored_default_format)
        .format_for_files(flexi_logger::colored_detailed_format)
        .duplicate_to_stderr(flexi_logger::Duplicate::All)
        .start()
    {
        Err(_) => Err(LogError::CouldNotStartLogger),
        Ok(_) => Ok(()),
    }
}

// ----- Macros -----

/// Convenience macro to generate TriggerGuides using TriggerConditions
/// Generates a const ready byte array
/// NOTE: This macro only works if the enum struct size is 6 bytes
///
/// Form
/// ```
/// const COND_A: &TriggerCondition = &TriggerCondition::Switch {
///     state: trigger::Phro::Press,
///     index: 1200,
///     loop_condition_index: 4,
/// };
///
/// const COND_B: &TriggerCondition = &TriggerCondition::Switch {
///     state: trigger::Phro::Press,
///     index: 2400,
///     loop_condition_index: 0,
/// };
///
/// const TRIGGER_GUIDES: &[u8] = trigger_guide_alt!(
///     // 2 key combo then 1 key combo sequence
///     [[2, COND_A, COND_B], [1, COND_A]],
///     // Just a single key combo
///     [[1, COND_A]],
/// );
/// ```
///
/// TODO (HaaTa): Rust macros aren't good at counting (at least not simple ones like this).
///               Figure out a good way to remove the combo counts
macro_rules! trigger_guide_alt {
    (
        $([
            $([
                $count:expr$(,$tc:expr)*$(,)?
            ]$(,)?)*
        ]$(,)?)*
    ) => {
        unsafe { &[
            $(
                $(
                    $count,
                    $(
                        $tc.bytes()[0],
                        $tc.bytes()[1],
                        $tc.bytes()[2],
                        $tc.bytes()[3],
                        $tc.bytes()[4],
                        $tc.bytes()[5],
                    )*
                )*
            0,)*
        ] }
    }
}

// ----- Tests -----

#[test]
fn trigger_condition_conversion() {
    setup_logging_lite().ok();

    // Verify TriggerCondition
    let cond_a = TriggerCondition::Switch {
        state: trigger::Phro::Press,
        index: 1200,
        loop_condition_index: 4,
    };
    unsafe {
        let cond_b = TriggerCondition::from_bytes(cond_a.bytes());
        assert_eq!(
            cond_a,
            cond_b,
            "Check bytes and from_bytes conversions {:?} -> {:?} -> {:?}",
            cond_a,
            cond_a.bytes(),
            cond_b
        );

        trace!("TriggerCondition::Switch -> {:?}", cond_a.bytes());
        trace!(
            "TriggerCondition -> {:?}",
            TriggerCondition::from_bytes(cond_a.bytes())
        );
    }
}

#[test]
fn trigger_guide_macro() {
    setup_logging_lite().ok();

    // Setup const TriggerConditions to use for the trigger guide
    const COND_A: &TriggerCondition = &TriggerCondition::Switch {
        state: trigger::Phro::Press,
        index: 1200,
        loop_condition_index: 4,
    };
    const COND_B: &TriggerCondition = &TriggerCondition::Switch {
        state: trigger::Phro::Press,
        index: 2400,
        loop_condition_index: 0,
    };

    const TRIGGER_GUIDES: &[u8] = trigger_guide_alt!(
        // 2 key combo then 1 key combo sequence
        [[2, COND_A, COND_B], [1, COND_A]],
        // Just a single key combo
        [[1, COND_A]],
    );

    #[rustfmt::skip]
    const TRIGGER_GUIDE_COMPARE: &[u8] = &[
        // COND_A + COND_B
        2, 1, 1, 176, 4, 4, 0, 1, 1, 96, 9, 0, 0,
        // COND_A
        1, 1, 1, 176, 4, 4, 0,
        // END
        0,
        // COND_A
        1, 1, 1, 176, 4, 4, 0,
        // END
        0,
    ];

    assert_eq!(
        TRIGGER_GUIDES, TRIGGER_GUIDE_COMPARE,
        "trigger_guide macro check failed: {:?} vs {:?}",
        TRIGGER_GUIDES, TRIGGER_GUIDE_COMPARE
    );

    // Simple inline guide
    const TRIGGER_GUIDES2: &[u8] = kll_macros::trigger_guide!([
        [
            TriggerCondition::Switch {
                state: trigger::Phro::Press,
                index: 15,
                loop_condition_index: 0
            },
            TriggerCondition::Switch {
                state: trigger::Phro::Press,
                index: 16,
                loop_condition_index: 1
            }
        ],
        [TriggerCondition::Switch {
            state: trigger::Phro::Release,
            index: 15,
            loop_condition_index: 0
        }]
    ]);

    #[rustfmt::skip]
    const TRIGGER_GUIDE_COMPARE2: &[u8] = &[
        // 15Press + 16Press
        2, 1, 1, 15, 0, 0, 0, 1, 1, 16, 0, 1, 0,
        // 15Release
        1, 1, 3, 15, 0, 0, 0,
        // END
        0
    ];

    assert_eq!(
        TRIGGER_GUIDES2, TRIGGER_GUIDE_COMPARE2,
        "trigger_guide macro check failed: {:?} vs {:?}",
        TRIGGER_GUIDES2, TRIGGER_GUIDE_COMPARE2
    );
}

#[test]
fn result_guide_macro() {
    setup_logging_lite().ok();

    const RESULT_GUIDES: &[u8] = kll_macros::result_guide!(
        // Press A + Shift
        [[
            Capability::HidKeyboard {
                state: CapabilityState::Initial,
                loop_condition_index: 0,
                id: kll_hid::Keyboard::A,
            },
            Capability::HidKeyboard {
                state: CapabilityState::Initial,
                loop_condition_index: 0,
                id: kll_hid::Keyboard::LeftShift,
            },
        ],],
        // Release B
        [[Capability::HidKeyboard {
            state: CapabilityState::Last,
            loop_condition_index: 0,
            id: kll_hid::Keyboard::B,
        },]],
    );

    #[rustfmt::skip]
    const RESULT_GUIDE_COMPARE: &[u8] = &[
        // A + Shift
        2, 6, 1, 0, 0, 4, 0, 0, 0, 6, 1, 0, 0, 225, 0, 0, 0,
        // B
        1, 6, 2, 0, 0, 5, 0, 0, 0,
        // End
        0,
    ];

    assert_eq!(
        RESULT_GUIDES, RESULT_GUIDE_COMPARE,
        "result_guide macro check failed: {:?} vs {:?}",
        RESULT_GUIDES, RESULT_GUIDE_COMPARE
    );
}

#[test]
fn basic_init_test() {
    setup_logging_lite().ok();

    // Layer Lookup (TriggerLists)
    // LAYER_LOOKUP -> TRIGGER_RESULT_MAPPING -> (Trigger, Result)
    //   Trigger => TRIGGER_GUIDES
    //   Result => RESULT_GUIDES
    #[rustfmt::skip]
    const LAYER_LOOKUP: &[u8] = kll_macros::layer_lookup!(
        // Layer 0, Switch Type (1), Index 5, No Triggers
        0, 1, 5, [],
        // Layer 0, Switch Type (1), Index 6, 2 triggers indices: 0 2
        0, 1, 6, [0, 2],
        // Layer 0, Switch Type (1), Index 7, 1 trigger index: 0
        0, 1, 7, [0],
        // Layer 1, None Type (0), Index 2, No Triggers
        1, 0, 2, [],
        // Layer 1, Layer Type (7), Layer(index) 3, 1 trigger index: 4
        1, 7, 3, [4],
        // Layer 2, AnalogDistance Type (3), Index 8, 1 trigger index: 6
        2, 3, 8, [6],
        // Layer 2, Switch Type (1), Index 6, 1 trigger index: 8
        2, 1, 6, [8],
    );

    // TriggerResult Mapping
    const TRIGGER_RESULT_MAPPING: &[u16] = &[
        // index: TriggerGuideIndex => ResultGuideIndex
        0, 0, // 0: 0 => 0
        13, 35, // 2: 13 => 35
        20, 44, // 4: 20 => 44
        27, 0, // 6: 27 => 0
        13, 44, // 8: 13 => 44
    ];

    // TriggerGuide layout
    // <combo size>, <TriggerCondition>.., <combo size>, ...
    // If a combo size is 0, then the sequence has ended (handled by macro)
    const TRIGGER_GUIDES: &[u8] = kll_macros::trigger_guide!(
        // Index: 0
        [[
            TriggerCondition::Switch {
                state: trigger::Phro::Hold,
                index: 6,
                loop_condition_index: 0,
            },
            TriggerCondition::Switch {
                state: trigger::Phro::Hold,
                index: 7,
                loop_condition_index: 0,
            },
        ]],
        // Index: 13
        [[TriggerCondition::Switch {
            state: trigger::Phro::Hold,
            index: 6,
            loop_condition_index: 0,
        },]],
        // Index: 20
        [[TriggerCondition::Layer {
            state: trigger::LayerState::ShiftActivate,
            layer: 3,
            loop_condition_index: 0,
        },]],
        // Index: 27
        [[TriggerCondition::AnalogDistance {
            reserved: 0,
            index: 8,
            val: 1500,
        },]],
    );
    trace!("TRIGGER_GUIDES: {:?}", TRIGGER_GUIDES);

    // ResultGuide layout
    const RESULT_GUIDES: &[u8] = kll_macros::result_guide!(
        // Press Shift + A; Release Shift; Release A
        // Index: 0
        [
            [
                Capability::HidKeyboard {
                    state: CapabilityState::Initial,
                    loop_condition_index: 0,
                    id: kll_hid::Keyboard::A,
                },
                Capability::HidKeyboard {
                    state: CapabilityState::Initial,
                    loop_condition_index: 0,
                    id: kll_hid::Keyboard::LeftShift,
                },
            ],
            [Capability::HidKeyboard {
                state: CapabilityState::Last,
                loop_condition_index: 0,
                id: kll_hid::Keyboard::LeftShift,
            },],
            [Capability::HidKeyboard {
                state: CapabilityState::Last,
                loop_condition_index: 0,
                id: kll_hid::Keyboard::A,
            },],
        ],
        // Press B
        // Index: 35
        [[Capability::HidKeyboard {
            state: CapabilityState::Initial,
            loop_condition_index: 0,
            id: kll_hid::Keyboard::B,
        },]],
        // Release B
        // Index: 44
        [[Capability::HidKeyboard {
            state: CapabilityState::Last,
            loop_condition_index: 0,
            id: kll_hid::Keyboard::B,
        },]],
    );
    trace!("RESULT_GUIDES: {:?}", RESULT_GUIDES);

    // Loop Condition Lookup
    // NOTE: This lookup must always have at least 1 entry.
    const LOOP_CONDITION_LOOKUP: &[u32] = &[0];

    // Build lookup
    let lookup = LayerLookup::<256>::new(
        LAYER_LOOKUP,
        TRIGGER_GUIDES,
        RESULT_GUIDES,
        TRIGGER_RESULT_MAPPING,
        LOOP_CONDITION_LOOKUP,
    );

    // Print out valid lookups
    trace!("layer_lookup: {:?}", LAYER_LOOKUP);
    for ((layer, ttype, index), mlookup) in &lookup.layer_lookup {
        trace!("({}, {}, {}), {}", layer, ttype, index, mlookup);
    }

    // Validate trigger lists
    // Extra 0's in triggerlist are ignored (uses size of the source for length)
    trace!("TriggerList Lookup");
    for (key, triggerlist) in [
        ((0, 1, 6), [0, 0, 2, 0]),
        ((1, 7, 3), [4, 0, 0, 0]),
        ((2, 3, 8), [6, 0, 0, 0]),
    ] {
        match lookup.trigger_list(key) {
            Some(mlookup) => {
                trace!("{:?} -> {:?}", key, mlookup);
                assert!(
                    mlookup == &triggerlist[..mlookup.len()],
                    "{:?} -> {:?} != {:?}",
                    key,
                    mlookup,
                    triggerlist
                );
            }
            None => {
                panic!("Missing key: {:?}", key);
            }
        }
    }

    // Invalid lookup
    assert!(
        lookup.trigger_list((100, 100, 100)).is_none(),
        "Failed 'Invalid' (100, 100, 100) lookup"
    );

    // Test trigger and result guide lookups
    for (key, triggers, results) in [(
        (0, 1, 6),
        [
            Some([
                TriggerCondition::Switch {
                    state: trigger::Phro::Hold,
                    index: 6,
                    loop_condition_index: 0,
                },
                TriggerCondition::Switch {
                    state: trigger::Phro::Hold,
                    index: 7,
                    loop_condition_index: 0,
                },
            ]),
            Some([
                TriggerCondition::Switch {
                    state: trigger::Phro::Hold,
                    index: 6,
                    loop_condition_index: 0,
                },
                TriggerCondition::None,
            ]),
        ],
        [
            Some([
                Capability::HidKeyboard {
                    state: CapabilityState::Initial,
                    loop_condition_index: 0,
                    id: kll_hid::Keyboard::A,
                },
                Capability::HidKeyboard {
                    state: CapabilityState::Initial,
                    loop_condition_index: 0,
                    id: kll_hid::Keyboard::LeftShift,
                },
            ]),
            Some([
                Capability::HidKeyboard {
                    state: CapabilityState::Initial,
                    loop_condition_index: 0,
                    id: kll_hid::Keyboard::B,
                },
                Capability::NoOp {
                    state: CapabilityState::Any,
                    loop_condition_index: 0,
                },
            ]),
        ],
    )] {
        for (index, mapping) in lookup.lookup_guides::<10>(key).iter().enumerate() {
            let trigger = lookup.trigger_guide(*mapping, 0);
            let result = lookup.result_guide(*mapping, 0);
            trace!(
                "Mapping {:?}  Trigger: {:?}  Result: {:?}",
                mapping,
                trigger,
                result
            );
            assert_eq!(
                trigger.unwrap(),
                &triggers[index].unwrap()[0..trigger.unwrap().len()],
                "TriggerGuide did not match"
            );
            assert_eq!(
                result.unwrap(),
                &results[index].unwrap()[0..result.unwrap().len()],
                "ResultGuide did not match"
            );
        }
    }
}

// TODO Tests
// - Basic trigger -> result capability validation test
// - Import KLL file and do a handful of manual validation (positive test cases)
// - Import KLL file and do a handful of manual validation (negative test cases)
// - Import KLL, retrieve json (or similar datastructure) and automate all triggers and make sure
// all results are reached
