// Copyright 2021 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use proc_macro::{TokenStream, TokenTree};
use std::iter::FromIterator;

/// Takes a list of sequences of combos and turns it into a u8 array
/// that can be stored in memory as a contiguous piece of data.
/// This is necessary to store the trigger guide independently of rust compilation.
///
/// ```
/// use kll_core::{Capability, CapabilityState, TriggerCondition, trigger};
///
/// const TRIGGER_GUIDES: &'static [u8] = kll_macros::trigger_guide!(
///     [[
///         TriggerCondition::Switch {
///             state: trigger::Phro::Hold,
///             index: 6,
///             loop_condition_index: 0,
///         },
///         TriggerCondition::Switch {
///             state: trigger::Phro::Hold,
///             index: 7,
///             loop_condition_index: 0,
///         },
///     ]],
///     [[
///         TriggerCondition::Switch {
///             state: trigger::Phro::Hold,
///             index: 6,
///             loop_condition_index: 0,
///         },
///     ]],
///     [[
///         TriggerCondition::Layer {
///             state: trigger::LayerState::ShiftActivate,
///             layer: 3,
///             loop_condition_index: 0,
///         },
///     ]],
///     [[
///         TriggerCondition::AnalogDistance {
///             reserved: 0,
///             index: 8,
///             val: 1500,
///         },
///     ]],
/// );
/// ```
#[proc_macro]
pub fn trigger_guide(input: TokenStream) -> TokenStream {
    let mut output: Vec<String> = vec!["unsafe { &[".to_string()];

    for sequence in input {
        match sequence {
            TokenTree::Group(sequence) => {
                for combo in sequence.stream() {
                    match combo {
                        TokenTree::Group(combo) => {
                            let mut combo_output: Vec<String> = Vec::new();
                            let mut prefix_output: Vec<String> = Vec::new();
                            let mut elem_count = 0;
                            // Largest enum is 6 bytes
                            // Adjusts the size depending on the enum
                            let mut byte_count = 0;
                            let byte_max_count = 6;
                            for elem in combo.stream() {
                                // Detect TriggerCondition
                                match elem.clone() {
                                    TokenTree::Ident(ident) => {
                                        // Check for hardcoded byte count
                                        // NOTE: It may be possible to use std::mem::size_of
                                        //       if we make a new struct crate that this macro
                                        //       crate can depend on. Determining byte count is the
                                        //       most important feature of this macro. This is
                                        //       needed to prevent undefined struct memory accesses
                                        //       when building the const u8 array.
                                        match ident.to_string().as_str() {
                                            "TriggerCondition" => {
                                                // New element
                                                elem_count += 1;
                                                prefix_output = Vec::new();
                                            }
                                            "Switch" | "Animation" => {
                                                byte_count = 6;
                                            }
                                            "AnalogDistance" | "AnalogVelocity"
                                            | "AnalogAcceleration" | "AnalogJerk" => {
                                                byte_count = 6;
                                            }
                                            "HidLed" | "Layer" | "Rotation" => {
                                                byte_count = 5;
                                            }
                                            "Sleep" | "Resume" | "Inactive" | "Active" => {
                                                byte_count = 4;
                                            }
                                            _ => {
                                                panic!("Unknown elem ident: {:?}", ident);
                                            }
                                        }
                                    }
                                    TokenTree::Punct(_) => {}
                                    TokenTree::Group(group) => {
                                        // Finished element, prepare additions
                                        for n in 0..byte_count {
                                            combo_output.append(&mut prefix_output.clone());
                                            combo_output.push(group.to_string());
                                            combo_output.push(".bytes()[".to_string());
                                            combo_output.push(n.to_string());
                                            combo_output.push("],".to_string());
                                        }
                                        // Fill empty bytes (to prevent undefined struct access)
                                        for _ in byte_count..byte_max_count {
                                            combo_output.push("0,".to_string());
                                        }
                                    }
                                    _ => {}
                                }
                                prefix_output.push(elem.to_string());
                            }

                            // Add combo element count
                            output.push(elem_count.to_string());
                            output.push(",".to_string());
                            // Add combo
                            output.append(&mut combo_output);
                        }
                        TokenTree::Punct(_) => {}
                        _ => {
                            panic!("Invalid combo element: {:?}", combo);
                        }
                    }
                }
            }
            TokenTree::Punct(_) => {}
            _ => {
                panic!("Invalid sequence element: {:?}", sequence);
            }
        }
    }

    // Final 0 length sequence to indicate finished
    output.push("0 ] }".to_string());
    String::from_iter(output).parse().unwrap()
}

/// Takes a list of sequences of combos of Capabilities and turns it into a u8 array
/// that can be stored in memory as a contiguous piece of data.
/// This is necessary to store the result guide independently of rust compilation.
///
/// ```
/// use kll_core::{Capability, CapabilityState};
///
/// const RESULT_GUIDES: &'static [u8] = kll_macros::result_guide!(
///     [
///         // Press Shift + A; Release Shift; Release A
///         [
///             Capability::HidKeyboard {
///                 state: CapabilityState::Initial,
///                 loop_condition_index: 0,
///                 id: kll_hid::Keyboard::A,
///             },
///             Capability::HidKeyboard {
///                 state: CapabilityState::Initial,
///                 loop_condition_index: 0,
///                 id: kll_hid::Keyboard::LeftShift,
///             },
///         ],
///         [Capability::HidKeyboard {
///             state: CapabilityState::Last,
///             loop_condition_index: 0,
///             id: kll_hid::Keyboard::LeftShift,
///         },],
///         [Capability::HidKeyboard {
///             state: CapabilityState::Last,
///             loop_condition_index: 0,
///             id: kll_hid::Keyboard::A,
///         },],
///     ],
///     // Press B
///     [[Capability::HidKeyboard {
///         state: CapabilityState::Initial,
///         loop_condition_index: 0,
///         id: kll_hid::Keyboard::B,
///     },]],
///     // Release B
///     [[Capability::HidKeyboard {
///         state: CapabilityState::Last,
///         loop_condition_index: 0,
///         id: kll_hid::Keyboard::B,
///     },]],
/// );
/// ```
#[proc_macro]
pub fn result_guide(input: TokenStream) -> TokenStream {
    let mut output: Vec<String> = vec!["unsafe { &[".to_string()];

    for sequence in input {
        match sequence {
            TokenTree::Group(sequence) => {
                for combo in sequence.stream() {
                    match combo {
                        TokenTree::Group(combo) => {
                            let mut combo_output: Vec<String> = Vec::new();
                            let mut prefix_output: Vec<String> = Vec::new();
                            let mut elem_count = 0;
                            // Largest enum is 8 bytes
                            // Adjusts the size depending on the enum
                            let mut byte_count = 0;
                            let byte_max_count = 8;
                            for elem in combo.stream() {
                                // Detect TriggerCondition
                                match elem.clone() {
                                    TokenTree::Ident(ident) => {
                                        // Check for hardcoded byte count
                                        // NOTE: It may be possible to use std::mem::size_of
                                        //       if we make a new struct crate that this macro
                                        //       crate can depend on. Determining byte count is the
                                        //       most important feature of this macro. This is
                                        //       needed to prevent undefined struct memory accesses
                                        //       when building the const u8 array.
                                        match ident.to_string().as_str() {
                                            "Capability" => {
                                                // New element
                                                elem_count += 1;
                                                prefix_output = Vec::new();
                                            }
                                            "LayerClear" | "McuFlashMode" | "NoOp" => {
                                                byte_count = 4;
                                            }
                                            "HidKeyboard"
                                            | "HidProtocol"
                                            | "HidLed"
                                            | "HidSystemControl"
                                            | "LayerRotate"
                                            | "PixelAnimationControl"
                                            | "PixelFadeLayer"
                                            | "PixelGammaControl" => {
                                                byte_count = 5;
                                            }
                                            "HidioOpenUrl"
                                            | "HidioUnicodeString"
                                            | "HidConsumerControl"
                                            | "HidKeyboardState"
                                            | "LayerState"
                                            | "PixelAnimationIndex"
                                            | "PixelLedControl"
                                            | "Rotate" => {
                                                byte_count = 6;
                                            }
                                            "PixelFadeIndex" | "PixelFadeSet" | "PixelTest" => {
                                                byte_count = 7;
                                            }
                                            "HidioUnicodeState" => {
                                                byte_count = 8;
                                            }
                                            _ => {
                                                panic!("Unknown elem ident: {:?}", ident);
                                            }
                                        }
                                    }
                                    TokenTree::Punct(_) => {}
                                    TokenTree::Group(group) => {
                                        // Finished element, prepare additions
                                        for n in 0..byte_count {
                                            combo_output.append(&mut prefix_output.clone());
                                            combo_output.push(group.to_string());
                                            combo_output.push(".bytes()[".to_string());
                                            combo_output.push(n.to_string());
                                            combo_output.push("],".to_string());
                                        }
                                        // Fill empty bytes (to prevent undefined struct access)
                                        for _ in byte_count..byte_max_count {
                                            combo_output.push("0,".to_string());
                                        }
                                    }
                                    _ => {}
                                }
                                prefix_output.push(elem.to_string());
                            }

                            // Add combo element count
                            output.push(elem_count.to_string());
                            output.push(",".to_string());
                            // Add combo
                            output.append(&mut combo_output);
                        }
                        TokenTree::Punct(_) => {}
                        _ => {
                            panic!("Invalid combo element: {:?}", combo);
                        }
                    }
                }
            }
            TokenTree::Punct(_) => {}
            _ => {
                panic!("Invalid sequence element: {:?}", sequence);
            }
        }
    }

    // Final 0 length sequence to indicate finished
    output.push("0 ] }".to_string());
    String::from_iter(output).parse().unwrap()
}

enum LayerLookupState {
    Layer,
    LayerComma,
    Type,
    TypeComma,
    Index,
    IndexComma,
    Triggers,
    TriggersComma,
}

/// Takes data in the following format and turns it into a byte array.
/// The trigger count is automatically calculated.
/// Triggers are u16, all the other fields are u8.
///
/// ```
/// const LAYER_LOOKUP: &'static [u8] = kll_macros::layer_lookup!(
///     // Layer 0, Switch Type (1), Index 5, No Triggers
///     0, 1, 5, [],
///     // Layer 0, Switch Type (1), Index 6, 2 Triggers: 0 14
///     0, 1, 6, [0, 14],
///     // Layer 0, Switch Type (1), Index 7, 1 Trigger: 0
///     0, 1, 7, [0],
///     // Layer 1, None Type (0), Index 2, No Triggers
///     1, 0, 2, [],
///     // Layer 1, Layer Type (7), Layer(index) 3, 1 Trigger: A
///     1, 7, 3, [0xA],
///     // Layer 2, AnalogDistance Type (3), Index 8, 1 Trigger: A
///     2, 3, 8, [0xA],
///     // Layer 2, Switch Type (1), Index 6, 1 Trigger: 14
///     2, 1, 6, [14],
/// );
/// ```
///
#[proc_macro]
pub fn layer_lookup(input: TokenStream) -> TokenStream {
    let mut state = LayerLookupState::Layer;

    let mut triggers: Vec<u16> = Vec::new();

    let mut output: Vec<String> = vec!["&".to_string(), "[".to_string()];

    // TODO Add error checking for syntax
    for token in input {
        match state {
            LayerLookupState::Layer => {
                output.push(token.to_string());
                state = LayerLookupState::LayerComma;
            }
            LayerLookupState::LayerComma => {
                output.push(token.to_string());
                state = LayerLookupState::Type;
            }
            LayerLookupState::Type => {
                output.push(token.to_string());
                state = LayerLookupState::TypeComma;
            }
            LayerLookupState::TypeComma => {
                output.push(token.to_string());
                state = LayerLookupState::Index;
            }
            LayerLookupState::Index => {
                match token {
                    TokenTree::Literal(literal) => {
                        // Check if this is a hex value
                        let val = if literal.to_string().contains("0x") {
                            u16::from_str_radix(literal.to_string().trim_start_matches("0x"), 16)
                                .unwrap()
                        } else {
                            literal.to_string().parse::<u16>().unwrap()
                        };

                        // Push index as two bytes
                        for num in val.to_le_bytes() {
                            output.push(format!("{}, ", num));
                        }
                    }
                    _ => {
                        panic!("Invalid token, expected index token: {:?}", token);
                    }
                }
                state = LayerLookupState::IndexComma;
            }
            LayerLookupState::IndexComma => {
                // Comma has already been added due to the index
                state = LayerLookupState::Triggers;
            }
            LayerLookupState::Triggers => {
                match token {
                    TokenTree::Group(group) => {
                        for subtoken in group.stream() {
                            match subtoken.clone() {
                                TokenTree::Punct(_) => {}
                                TokenTree::Literal(literal) => {
                                    // Check if this is a hex value
                                    if literal.to_string().contains("0x") {
                                        triggers.push(
                                            u16::from_str_radix(
                                                literal.to_string().trim_start_matches("0x"),
                                                16,
                                            )
                                            .unwrap(),
                                        );
                                    } else {
                                        triggers.push(literal.to_string().parse::<u16>().unwrap());
                                    }
                                }
                                _ => {
                                    panic!("Invalid trigger list token: {:?}", subtoken);
                                }
                            }
                        }

                        // Finished gathering triggers
                        // 1. Add the count
                        output.push(format!("{},", triggers.len()));

                        // 2. Add each of the triggers as a little endian u16
                        if !triggers.is_empty() {
                            for trigger in triggers {
                                for num in trigger.to_le_bytes() {
                                    output.push(format!("{},", num));
                                }
                            }
                            triggers = Vec::new();
                        }
                        state = LayerLookupState::TriggersComma;
                    }
                    _ => {
                        panic!("Invalid trigger token group: {:?}", token);
                    }
                }
            }
            LayerLookupState::TriggersComma => {
                state = LayerLookupState::Layer;
            }
        }
    }

    output.push("]".to_string());
    String::from_iter(output).parse().unwrap()
}
