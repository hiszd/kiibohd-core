// Copyright 2021-2022 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![no_std]
#![feature(const_ptr_read)]
#![feature(let_chains)]

#[macro_use]
extern crate static_assertions;
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

mod converters;
pub mod layout;
pub mod macros;
pub use kll_hid;

#[cfg(feature = "defmt")]
use defmt::{error, trace, warn};
#[cfg(not(feature = "defmt"))]
use log::{error, trace, warn};

pub mod hid {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[repr(u8)]
    pub enum Protocol {
        /// HID boot mode protocol
        Boot = 0,
        /// HID Application / NKRO mode protocol
        Application = 1,
        /// Toggle between Boot and Application modes
        Toggle = 3,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[repr(u8)]
    pub enum State {
        /// Control is enabled / pressed
        Active = 0,
        /// Control is disabled / released
        Inactive = 1,
    }
}

pub mod layer {
    use core::ops::{BitAnd, BitAndAssign, BitOrAssign, Not};
    use num_traits::FromPrimitive;

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[repr(u8)]
    pub enum Direction {
        /// Next layer
        Next = 0,
        /// Previous layer
        Previous = 1,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Primitive)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[repr(u8)]
    pub enum State {
        /// No layer state
        Off = 0x00,
        /// Shift state
        Shift = 0x01,
        /// Latch state
        Latch = 0x02,
        /// Shift+Latch state
        ShiftLatch = 0x03,
        /// Lock state
        Lock = 0x04,
        /// Shift+Lock state
        ShiftLock = 0x05,
        /// Latch+Lock state
        LatchLock = 0x06,
        /// Shift+Latch+Lock state
        ShiftLatchLock = 0x07,
    }

    impl State {
        /// Adds the given state to this state
        /// This is a bitwise or operation
        pub fn add(mut self, state: State) {
            self |= state;
        }

        /// Removes the given state from this state
        /// This is a bitwise nand operation
        pub fn remove(mut self, state: State) {
            self &= !(state);
        }

        /// Determine if the given state is present in this state
        pub fn is_set(&self, state: State) -> bool {
            if state != State::Off {
                *self & state != State::Off
            } else {
                *self == state
            }
        }

        /// Check if layer is active (e.g. not Off)
        pub fn active(&self) -> bool {
            *self != State::Off
        }

        /// Effective state
        /// If the state is Off or two states are set, set as disabled
        /// (e.g. Lock + Shift disables the state)
        pub fn effective(&self) -> bool {
            match self {
                State::Off => false,
                State::Shift => true,
                State::Latch => true,
                State::Lock => true,
                State::ShiftLatch => false,
                State::ShiftLock => false,
                State::LatchLock => false,
                State::ShiftLatchLock => true,
            }
        }
    }

    impl BitAnd for State {
        type Output = Self;

        fn bitand(self, rhs: Self) -> Self::Output {
            State::from_u32(self as u32 & rhs as u32).unwrap()
        }
    }

    impl BitAndAssign for State {
        fn bitand_assign(&mut self, rhs: Self) {
            *self = State::from_u32(*self as u32 & rhs as u32).unwrap()
        }
    }

    impl BitOrAssign for State {
        fn bitor_assign(&mut self, rhs: Self) {
            *self = State::from_u32(*self as u32 | rhs as u32).unwrap()
        }
    }

    impl Not for State {
        type Output = Self;

        fn not(self) -> Self::Output {
            State::from_u32(!(self as u32)).unwrap()
        }
    }
}

pub mod pixel {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[repr(u8)]
    pub enum GammaControl {
        /// Disable gamma correction
        Disable = 0,
        /// Enable gamma correction
        Enable = 1,
        /// Toggle gamma correction
        Toggle = 3,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[repr(u8)]
    pub enum AnimationControl {
        /// Toggles between pause/resume
        /// Only pauses if the state is either Forward or ForwardOne
        PauseResume = 0,
        /// Iterate a single frame
        ForwardOne = 1,
        /// Play animations
        Forward = 2,
        /// Stop all animations and clear all state
        Stop = 3,
        /// Restarts all animations
        Reset = 4,
        /// Pause all animations and clear all pixel state
        WipePause = 5,
        /// Pause
        Pause = 6,
        /// Clear all pixels (does not stop or pause)
        Clear = 7,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[repr(u8)]
    pub enum FadeCommand {
        /// Resets fade settings to default
        Reset = 0,
        /// Resets all profiles to defaults
        ResetAll = 1,
        /// Set fade brightness
        BrightnessSet = 2,
        /// Set fade brightness increment
        BrightnessIncrement = 3,
        /// Set fade brightness decrement
        BrightnessDecrement = 4,
        /// Reset to brightness default
        BrightnessDefault = 5,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[repr(u8)]
    pub enum PixelTest {
        /// Disable pixel test mode
        Off = 0,
        /// Enable a single channel
        ChannelSingle = 1,
        /// Enable a single channel rotate forward (index is used for jump amount)
        ChannelSingleRotate = 2,
        /// Enable a single channel rotate in reverse (index is used for jump amount)
        ChannelSingleRotateReverse = 3,
        ChannelFlashAll = 4,
        ChannelRoll = 5,
        ChannelAllOn = 6,
        PixelSingle = 7,
        PixelSingleRotate = 8,
        PixelSingleRotateReverse = 9,
        PixelFlashAll = 10,
        PixelRoll = 11,
        PixelAllOn = 12,
        ScanCodeSingle = 13,
        ScanCodeSingleRotate = 14,
        ScanCodeSingleRotateReverse = 15,
        ScanCodeFlashAll = 16,
        ScanCodeRoll = 17,
        ScanCodeAllOn = 18,
        PositionSingle = 19,
        PositionSingleRotate = 20,
        PositionSingleRotateReverse = 21,
        PositionFlashAll = 22,
        PositionRoll = 23,
        PositionAllOn = 24,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[repr(u8)]
    pub enum LedControl {
        /// Decrease LED brightness
        BrightnessDecrease = 0,
        /// Increase LED brightness
        BrightnessIncrease = 1,
        /// Set brightness
        BrightnessSet = 2,
        /// Default brightness
        BrightnessDefault = 3,
        /// Enable LEDs
        EnableLeds = 4,
        /// Disable LEDs
        DisableLeds = 5,
        /// Toggle LEDs On/Off
        ToggleLeds = 6,
        /// Set FPS target
        FpsSet = 7,
        /// Increase FPS target
        FpsIncrease = 8,
        /// Decrease FPS target
        FpsDecrease = 9,
        /// Default FPS target
        FpsDefault = 10,
    }
}

/// Global capability list for KLL
/// NOTE: Changing parameters and removing entries will require a firmware reflash.
///       At worst, KLL file and compiler definitions may also need to be updated.
///       Please avoid these kinds of changes if possible.
///       Adding new entries is safe.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Capability {
    /// No-op / None action
    /// 4 bytes
    NoOp {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
    } = 0,
    /// Rotation event trigger
    /// 6 bytes
    Rotate {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        index: u8,
        increment: i8,
    } = 1,

    /// Clears all layer states
    /// NOTE: Does not send trigger events
    /// 4 bytes
    LayerClear {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
    } = 2,
    /// Updates layer to the specified state
    /// 6 bytes
    LayerState {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        layer: u8,
        layer_state: layer::State,
    } = 3,
    /// Rotates through possible layers given the direction
    /// Uses internal state to keep track of the current layer
    /// 5 bytes
    LayerRotate {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        direction: layer::Direction,
    } = 4,

    /// HID Protocol Mode
    /// 5 bytes
    HidProtocol {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        mode: hid::Protocol,
    } = 5,
    /// USB HID keyboard event
    /// Handles press/released based on incoming state
    /// 5 bytes
    HidKeyboard {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        id: kll_hid::Keyboard,
    } = 6,
    /// USB HID keyboard event
    /// Force state event
    /// 6 bytes
    HidKeyboardState {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        id: kll_hid::Keyboard,
        key_state: hid::State,
    } = 7,
    /// USB HID Consumer Control Event
    /// Handles press/released based on incoming state
    /// 6 bytes
    HidConsumerControl {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        id: kll_hid::ConsumerControl,
    } = 8,
    /// USB HID System Control Event
    /// Handles press/released based on incoming state
    /// 5 bytes
    HidSystemControl {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        id: kll_hid::SystemControl,
    } = 9,

    // TODO Mouse Control
    // TODO Joystick Control
    /// Enter Flash Mode
    /// Usually jumps to the bootloader
    /// 4 bytes
    McuFlashMode {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
    } = 10,

    /// Overall animation control
    /// 5 bytes
    PixelAnimationControl {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        mode: pixel::AnimationControl,
    },
    /// Activates the given indexed Animation
    /// 6 bytes
    PixelAnimationIndex {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        index: u16,
    },
    /// Fade control
    /// 7 bytes
    PixelFadeControl {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        profile: u8,
        command: pixel::FadeCommand,
        arg: u8,
    },
    /// Layer fade
    /// 5 bytes
    PixelFadeLayer {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        layer: u8,
    },
    /// Fade set profile
    /// 7 bytes
    PixelFadeSet {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        profile: u8,
        config: u8,
        period: u8,
    },
    /// Enable/Disable/Toggle gamma correction
    /// 5 bytes
    PixelGammaControl {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        mode: pixel::GammaControl,
    },
    /// LED Control
    /// 6 bytes
    PixelLedControl {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        mode: pixel::LedControl,
        amount: u8,
    },
    /// Pixel test
    /// 7 bytes
    PixelTest {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        test: pixel::PixelTest,
        index: u16,
    },

    /// Sends URL (using index stored unicode string) to host computer web browser
    /// 6 bytes
    HidioOpenUrl {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        index: u16,
    },
    /// Sends Unicode string (using index stored unicode string) to host computer
    /// 6 bytes
    HidioUnicodeString {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        index: u16,
    },
    /// Sends Unicode character with state (Press or Release)
    /// 8 bytes
    HidioUnicodeState {
        /// Capability state
        state: CapabilityState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        unicode: char,
    },
}

impl Capability {
    /// Generate a CapabilityRun using a Capability + TriggerEvent
    /// The TriggerEvent is only important when CapabilityState::Passthrough is set.
    pub fn generate(&self, event: TriggerEvent, _loop_condition_lookup: &[u32]) -> CapabilityRun {
        // TODO: Handle loop_condition_index
        match self {
            Capability::NoOp { state, .. } => CapabilityRun::NoOp {
                state: state.event(event),
            },
            Capability::HidKeyboard { state, id, .. } => CapabilityRun::HidKeyboard {
                state: state.event(event),
                id: *id,
            },
            _ => {
                panic!(
                    "Missing implementation for Capability::generate: {:?}",
                    self
                );
            }
        }
    }

    /// Lookup loop_condition_index
    pub fn loop_condition_index(&self) -> u16 {
        match self {
            Capability::NoOp {
                loop_condition_index,
                ..
            } => *loop_condition_index,
            Capability::Rotate {
                loop_condition_index,
                ..
            } => *loop_condition_index,
            Capability::LayerClear {
                loop_condition_index,
                ..
            } => *loop_condition_index,
            Capability::LayerState {
                loop_condition_index,
                ..
            } => *loop_condition_index,
            Capability::LayerRotate {
                loop_condition_index,
                ..
            } => *loop_condition_index,
            Capability::HidProtocol {
                loop_condition_index,
                ..
            } => *loop_condition_index,
            Capability::HidKeyboard {
                loop_condition_index,
                ..
            } => *loop_condition_index,
            Capability::HidKeyboardState {
                loop_condition_index,
                ..
            } => *loop_condition_index,
            Capability::HidConsumerControl {
                loop_condition_index,
                ..
            } => *loop_condition_index,
            Capability::HidSystemControl {
                loop_condition_index,
                ..
            } => *loop_condition_index,
            Capability::McuFlashMode {
                loop_condition_index,
                ..
            } => *loop_condition_index,
            Capability::PixelAnimationControl {
                loop_condition_index,
                ..
            } => *loop_condition_index,
            Capability::PixelAnimationIndex {
                loop_condition_index,
                ..
            } => *loop_condition_index,
            Capability::PixelFadeControl {
                loop_condition_index,
                ..
            } => *loop_condition_index,
            Capability::PixelFadeLayer {
                loop_condition_index,
                ..
            } => *loop_condition_index,
            Capability::PixelFadeSet {
                loop_condition_index,
                ..
            } => *loop_condition_index,
            Capability::PixelGammaControl {
                loop_condition_index,
                ..
            } => *loop_condition_index,
            Capability::PixelLedControl {
                loop_condition_index,
                ..
            } => *loop_condition_index,
            Capability::PixelTest {
                loop_condition_index,
                ..
            } => *loop_condition_index,
            Capability::HidioOpenUrl {
                loop_condition_index,
                ..
            } => *loop_condition_index,
            Capability::HidioUnicodeString {
                loop_condition_index,
                ..
            } => *loop_condition_index,
            Capability::HidioUnicodeState {
                loop_condition_index,
                ..
            } => *loop_condition_index,
        }
    }
}

/// CapabilityRun
/// Used to run capabilities rather than map them out in a result guide
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CapabilityRun {
    /// No-op / None action
    /// 4 bytes
    NoOp { state: CapabilityEvent } = 0,
    /// Rotation event trigger
    /// 6 bytes
    Rotate {
        state: CapabilityEvent,
        index: u8,
        increment: i8,
    } = 1,

    /// Clears all layer states
    /// NOTE: Does not send trigger events
    /// 4 bytes
    LayerClear { state: CapabilityEvent } = 2,
    /// Updates layer to the specified state
    /// 6 bytes
    LayerState {
        state: CapabilityEvent,
        layer: u8,
        layer_state: layer::State,
    } = 3,
    /// Rotates through possible layers given the direction
    /// Uses internal state to keep track of the current layer
    /// 5 bytes
    LayerRotate {
        state: CapabilityEvent,
        direction: layer::Direction,
    } = 4,

    /// HID Protocol Mode
    /// 5 bytes
    HidProtocol {
        state: CapabilityEvent,
        mode: hid::Protocol,
    } = 5,
    /// USB HID keyboard event
    /// Handles press/released based on incoming state
    /// 5 bytes
    HidKeyboard {
        state: CapabilityEvent,
        id: kll_hid::Keyboard,
    } = 6,
    /// USB HID keyboard event
    /// Force state event
    /// 6 bytes
    HidKeyboardState {
        state: CapabilityEvent,
        id: kll_hid::Keyboard,
        key_state: hid::State,
    } = 7,
    /// USB HID Consumer Control Event
    /// Handles press/released based on incoming state
    /// 6 bytes
    HidConsumerControl {
        state: CapabilityEvent,
        id: kll_hid::ConsumerControl,
    } = 8,
    /// USB HID System Control Event
    /// Handles press/released based on incoming state
    /// 5 bytes
    HidSystemControl {
        state: CapabilityEvent,
        id: kll_hid::SystemControl,
    } = 9,

    // TODO Mouse Control
    // TODO Joystick Control
    /// Enter Flash Mode
    /// Usually jumps to the bootloader
    /// 4 bytes
    McuFlashMode { state: CapabilityEvent } = 10,

    /// USB HID Led event
    /// Handles press/released based on incoming state
    /// 5 bytes
    HidLed {
        state: CapabilityEvent,
        id: kll_hid::LedIndicator,
    } = 11,

    /// Overall animation control
    /// 5 bytes
    PixelAnimationControl {
        state: CapabilityEvent,
        mode: pixel::AnimationControl,
    },
    /// Activates the given indexed Animation
    /// 6 bytes
    PixelAnimationIndex { state: CapabilityEvent, index: u16 },
    /// Fade control
    /// 7 bytes
    PixelFadeControl {
        state: CapabilityEvent,
        profile: u8,
        command: pixel::FadeCommand,
        arg: u8,
    },
    /// Layer fade
    /// 5 bytes
    PixelFadeLayer { state: CapabilityEvent, layer: u8 },
    /// Fade set profile
    /// 7 bytes
    PixelFadeSet {
        state: CapabilityEvent,
        profile: u8,
        config: u8,
        period: u8,
    },
    /// Enable/Disable/Toggle gamma correction
    /// 5 bytes
    PixelGammaControl {
        state: CapabilityEvent,
        mode: pixel::GammaControl,
    },
    /// LED Control
    /// 6 bytes
    PixelLedControl {
        state: CapabilityEvent,
        mode: pixel::LedControl,
        amount: u8,
    },
    /// Pixel test
    /// 7 bytes
    PixelTest {
        state: CapabilityEvent,
        test: pixel::PixelTest,
        index: u16,
    },

    /// Analog Event
    /// Always a passthrough event
    /// 4 bytes
    Analog { state: CapabilityEvent },

    /// Sends URL (using index stored unicode string) to host computer web browser
    /// 6 bytes
    HidioOpenUrl { state: CapabilityEvent, index: u16 },
    /// Sends Unicode string (using index stored unicode string) to host computer
    /// 6 bytes
    HidioUnicodeString { state: CapabilityEvent, index: u16 },
    /// Sends Unicode character with state (Press or Release)
    /// 8 bytes
    HidioUnicodeState {
        state: CapabilityEvent,
        unicode: char,
    },
}

impl CapabilityRun {
    pub fn state(&self) -> CapabilityEvent {
        match self {
            CapabilityRun::NoOp { state } => *state,
            CapabilityRun::Rotate { state, .. } => *state,
            CapabilityRun::LayerClear { state, .. } => *state,
            CapabilityRun::LayerState { state, .. } => *state,
            CapabilityRun::LayerRotate { state, .. } => *state,
            CapabilityRun::HidProtocol { state, .. } => *state,
            CapabilityRun::HidKeyboard { state, .. } => *state,
            CapabilityRun::HidKeyboardState { state, .. } => *state,
            CapabilityRun::HidConsumerControl { state, .. } => *state,
            CapabilityRun::HidSystemControl { state, .. } => *state,
            CapabilityRun::McuFlashMode { state, .. } => *state,
            CapabilityRun::HidLed { state, .. } => *state,
            CapabilityRun::PixelAnimationControl { state, .. } => *state,
            CapabilityRun::PixelFadeControl { state, .. } => *state,
            CapabilityRun::PixelFadeLayer { state, .. } => *state,
            CapabilityRun::PixelFadeSet { state, .. } => *state,
            CapabilityRun::PixelGammaControl { state, .. } => *state,
            CapabilityRun::PixelLedControl { state, .. } => *state,
            CapabilityRun::PixelTest { state, .. } => *state,
            CapabilityRun::Analog { state } => *state,
            CapabilityRun::HidioOpenUrl { state, .. } => *state,
            CapabilityRun::HidioUnicodeString { state, .. } => *state,
            CapabilityRun::HidioUnicodeState { state, .. } => *state,
            _ => {
                panic!("CapabilityRun type not handled for state({:?})", self)
            }
        }
    }
}

// Size validation for Capability
// DO NOT CHANGE THIS: Will invalidate existing generated KLL layouts
const_assert_eq!(core::mem::size_of::<Capability>(), 8);

// NOTE: It's not possible to make this a trait (yet)
impl Capability {
    /// Convert enum to an array of bytes
    /// # Safety
    pub const unsafe fn bytes(&self) -> &[u8] {
        &*core::ptr::slice_from_raw_parts(
            (self as *const Self) as *const u8,
            core::mem::size_of::<Self>(),
        )
    }

    /// Convert array of bytes to enum
    /// # Safety
    pub const unsafe fn from_byte_array(bytes: [u8; core::mem::size_of::<Self>()]) -> Self {
        core::mem::transmute(bytes)
    }

    /// Convert slice of bytes to enum
    /// Aggressively casts the provide u8 slice to retrieve a Capability
    /// # Safety
    pub const unsafe fn from_bytes(bytes: &[u8]) -> Self {
        core::ptr::read(bytes.as_ptr() as *const &[u8] as *const Self)
    }
}

pub enum Vote {
    /// Successful comparison
    Positive,
    /// Negative comparison, should stop this and all future voting for this trigger guide
    Negative,
    /// No match, but doesn't exclude future comparisons (e.g. hold event)
    Insufficient,
    /// Indicate that this is an off state that need to be processed separately (or later)
    OffState,
}

pub mod trigger {
    use super::*;
    use num_traits::FromPrimitive;

    /// PHRO - Press/Hold/Release/Off
    /// Generally used for momentary switches
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[repr(u8)]
    pub enum Phro {
        Press = 1,
        Hold = 2,
        Release = 3,
        Off = 0,

        /// Passthrough TriggerEvent state
        /// Only used for TriggerConditions
        Passthrough = 8,
    }

    impl Phro {
        /// Given the previous state and current state determine the correct Phro state
        pub fn from_state(prev_state: bool, cur_state: bool) -> Self {
            // Off -> On
            if !prev_state && cur_state {
                Phro::Press
            // On -> On
            } else if prev_state && cur_state {
                Phro::Hold
            // On -> Off
            } else if prev_state && !cur_state {
                Phro::Release
            // Off -> Off
            } else {
                Phro::Off
            }
        }

        /// Compare states including time base
        /// Used when comparing TriggerEvents to TriggerConditions and whether the event
        /// satisfies the condition
        pub fn compare(&self, cond_time: u32, event_state: Self, event_time: u32) -> Vote {
            // Make sure states match
            if *self != event_state {
                // When the condition is an Off state and the event is not
                // We need to return this status back so we can do a reverse lookup to retrieve
                // any off state events
                if *self == Phro::Off {
                    return Vote::OffState;
                } else {
                    return Vote::Insufficient;
                }
            }

            // Evaluate timing
            match self {
                Phro::Press => {
                    if event_time >= cond_time {
                        Vote::Positive
                    } else {
                        Vote::Negative
                    }
                }
                Phro::Hold => {
                    if event_time >= cond_time {
                        Vote::Positive
                    } else {
                        Vote::Insufficient
                    }
                }
                Phro::Release => {
                    if event_time <= cond_time {
                        Vote::Positive
                    } else {
                        Vote::Negative
                    }
                }
                Phro::Off => {
                    if event_time >= cond_time {
                        Vote::Positive
                    } else {
                        Vote::Negative
                    }
                }
                // Not enough information to determine a resolution
                _ => Vote::Insufficient,
            }
        }
    }

    /// AODO - Activate/On/Deactivate/Off
    /// Generally used for maintained switches
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[repr(u8)]
    pub enum Aodo {
        Activate = 1,
        On = 2,
        Deactivate = 3,
        Off = 0,

        /// Passthrough TriggerEvent state
        /// Only used for TriggerConditions
        Passthrough = 8,
    }

    impl Aodo {
        /// Given the previous state and current state determine the correct Aodo state
        pub fn from_state(prev_state: bool, cur_state: bool) -> Self {
            // Off -> On
            if !prev_state && cur_state {
                Aodo::Activate
            // On -> On
            } else if prev_state && cur_state {
                Aodo::On
            // On -> Off
            } else if prev_state && !cur_state {
                Aodo::Deactivate
            // Off -> Off
            } else {
                Aodo::Off
            }
        }
    }

    /// DRO - Done/Repeat/Off
    /// Generally used for an abstract process, such as an animation sequence.
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[repr(u8)]
    pub enum Dro {
        Off = 0,
        Done = 1,
        Repeat = 3,

        /// Passthrough TriggerEvent state
        /// Only used for TriggerConditions
        Passthrough = 8,
    }

    /// LayerState - AODO + Layer Info
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Primitive)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[repr(u8)]
    pub enum LayerState {
        ShiftActivate = 0x11,
        LatchActivate = 0x21,
        ShiftLatchActivate = 0x31,
        LockActivate = 0x41,
        ShiftLockActivate = 0x51,
        LatchLockActivate = 0x61,
        ShiftLatchLockActivate = 0x71,

        ShiftOn = 0x12,
        LatchOn = 0x22,
        ShiftLatchOn = 0x32,
        LockOn = 0x42,
        ShiftLockOn = 0x52,
        LatchLockOn = 0x62,
        ShiftLatchLockOn = 0x72,

        ShiftDeactivate = 0x13,
        LatchDeactivate = 0x23,
        ShiftLatchDeactivate = 0x33,
        LockDeactivate = 0x43,
        ShiftLockDeactivate = 0x53,
        LatchLockDeactivate = 0x63,
        ShiftLatchLockDeactivate = 0x73,

        ShiftOff = 0x10,
        LatchOff = 0x20,
        ShiftLatchOff = 0x30,
        LockOff = 0x40,
        ShiftLockOff = 0x50,
        LatchLockOff = 0x60,
        ShiftLatchLockOff = 0x70,

        /// Passthrough TriggerEvent state
        /// Only used for TriggerConditions
        Passthrough = 0x08,
    }

    impl LayerState {
        /// Mergers layer::State and Aodo for TriggerEvent::LayerState
        pub fn from_layer(layer_state: layer::State, activity_state: Aodo) -> Self {
            LayerState::from_u32(((layer_state as u32) << 1) | activity_state as u32).unwrap()
        }
    }
}

/// Trigger event definitions
///
/// last_state is an incrementing counter that increases on every scan loop while the state has not
/// changed (e.g. holding a key).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum TriggerEvent {
    None = 0,
    Switch {
        /// Switch state
        state: trigger::Phro,
        /// Switch identification index
        index: u16,
        /// Scanning loops since the last state change (can be 0 if the state just changed)
        last_state: u32,
    } = 1,
    HidLed {
        /// LED state
        state: trigger::Aodo,
        /// HID LED identification (from USB HID spec, e.g. CapsLock)
        index: u8,
        /// Scanning loops since the last state change (can be 0 if the state just changed)
        last_state: u32,
    } = 2,
    AnalogDistance {
        index: u16,
        val: i16,
    } = 3,
    AnalogVelocity {
        index: u16,
        val: i16,
    } = 4,
    AnalogAcceleration {
        index: u16,
        val: i16,
    } = 5,
    AnalogJerk {
        index: u16,
        val: i16,
    } = 6,
    Layer {
        state: trigger::LayerState,
        layer: u8,
        /// Scanning loops since the last state change (can be 0 if the state just changed)
        last_state: u32,
    } = 7,
    Animation {
        state: trigger::Dro,
        index: u16,
        /// Scanning loops since the last state change (can be 0 if the state just changed)
        last_state: u32,
    } = 8,
    Sleep {
        state: trigger::Aodo,
        /// Scanning loops since the last state change (can be 0 if the state just changed)
        last_state: u32,
    } = 9,
    Resume {
        state: trigger::Aodo,
        /// Scanning loops since the last state change (can be 0 if the state just changed)
        last_state: u32,
    } = 10,
    Inactive {
        state: trigger::Aodo,
        /// Scanning loops since the last state change (can be 0 if the state just changed)
        last_state: u32,
    } = 11,
    Active {
        state: trigger::Aodo,
        /// Scanning loops since the last state change (can be 0 if the state just changed)
        last_state: u32,
    } = 12,
    Rotation {
        index: u8,
        position: i8,
        /// Scanning loops since the last state change (can be 0 if the state just changed)
        last_state: u32,
    } = 13,
}

impl TriggerEvent {
    /// Convert enum to an array of bytes
    /// # Safety
    pub const unsafe fn bytes(&self) -> &[u8] {
        &*core::ptr::slice_from_raw_parts(
            (self as *const Self) as *const u8,
            core::mem::size_of::<Self>(),
        )
    }

    /// Convert array of bytes to enum
    /// # Safety
    pub const unsafe fn from_byte_array(bytes: [u8; core::mem::size_of::<Self>()]) -> Self {
        core::mem::transmute(bytes)
    }

    /// Convert slice of bytes to enum
    /// Aggressively casts the provide u8 slice to retrieve a TriggerEvent
    /// # Safety
    pub const unsafe fn from_bytes(bytes: &[u8]) -> Self {
        core::ptr::read(bytes.as_ptr() as *const &[u8] as *const Self)
    }

    /// Attempts to determine the index value of the event
    /// If an index is not valid, return 0 instead (index may not have any meaning)
    pub fn index(&self) -> u16 {
        match self {
            TriggerEvent::None => 0,
            TriggerEvent::Switch { index, .. } => *index,
            TriggerEvent::HidLed { index, .. } => (*index).into(),
            TriggerEvent::AnalogDistance { index, .. } => *index,
            TriggerEvent::AnalogVelocity { index, .. } => *index,
            TriggerEvent::AnalogAcceleration { index, .. } => *index,
            TriggerEvent::AnalogJerk { index, .. } => *index,
            TriggerEvent::Layer { layer, .. } => (*layer).into(),
            TriggerEvent::Animation { index, .. } => *index,
            TriggerEvent::Sleep { .. } => 0,
            TriggerEvent::Resume { .. } => 0,
            TriggerEvent::Inactive { .. } => 0,
            TriggerEvent::Active { .. } => 0,
            TriggerEvent::Rotation { index, .. } => (*index).into(),
        }
    }
}

// Size validation for TriggerEvent
// Less important than TriggerCondition size, but to serve as a check when updating the enum fields
// NOTE: TriggerEvents are passed directly to HID-IO, changing the structure and orders will break
// apis, so be carefut with modifications.
const_assert_eq!(core::mem::size_of::<TriggerEvent>(), 8);

/// Trigger condition definitions
///
/// XXX (HaaTa): Field order is extremely important. Rust will optimize field packing
///              if done correctly. Static assertions are included to prevent bad mistakes.
///              Changing the enum size is an API breaking change (requires KLL compiler
///              updates).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum TriggerCondition {
    None = 0,
    /// 6 bytes
    Switch {
        /// Switch state
        state: trigger::Phro,
        /// Switch identification index
        index: u16,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
    } = 1,
    /// 5 bytes
    HidLed {
        /// LED state
        state: trigger::Aodo,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        /// HID LED identification (from USB HID spec, e.g. CapsLock)
        index: u8,
    } = 2,
    /// 6 bytes
    AnalogDistance {
        /// Needed focontiguous packing
        reserved: u8,
        /// Switch identification index
        index: u16,
        /// Analog distance, units depend on the keyboard, KLL compiler handles unit conversion
        val: i16,
    } = 3,
    /// 6 bytes
    AnalogVelocity {
        /// Needed for contiguous packing
        reserved: u8,
        /// Switch identification index
        index: u16,
        /// Analog velocity, units depend on the keyboard, KLL compiler handles unit conversion
        val: i16,
    } = 4,
    /// 6 bytes
    AnalogAcceleration {
        /// Needed for contiguous packing
        reserved: u8,
        /// Switch identification index
        index: u16,
        /// Analog acceleration, units depend on the keyboard, KLL compiler handles unit conversion
        val: i16,
    } = 5,
    /// 6 bytes
    AnalogJerk {
        /// Needed for contiguous packing
        reserved: u8,
        /// Switch identification index
        index: u16,
        /// Analog jerk, units depend on the keyboard, KLL compiler handles unit conversion
        val: i16,
    } = 6,
    /// 5 bytes
    Layer {
        /// Layer state
        state: trigger::LayerState,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        /// Layer index (layer 0 is the default state and does not have events)
        layer: u8,
    } = 7,
    /// 6 bytes
    Animation {
        /// Animation state
        state: trigger::Dro,
        /// Animation index
        index: u16,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
    } = 8,
    /// Sleep events are always index 0
    /// 4 bytes
    Sleep {
        /// Sleep state
        state: trigger::Aodo,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
    } = 9,
    /// Resume events are always index 0
    /// 4 bytes
    Resume {
        /// Resume state
        state: trigger::Aodo,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
    } = 10,
    /// Inactive events are always index 0
    /// 4 bytes
    Inactive {
        /// Inactive state
        state: trigger::Aodo,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
    } = 11,
    /// Active events are always index 0
    /// 4 bytes
    Active {
        /// Active state
        state: trigger::Aodo,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
    } = 12,
    /// 5 bytes
    Rotation {
        /// Rotation index
        index: u8,
        /// Scanning loop condition (number of scanning loops attached to state condition)
        /// Lookup index
        loop_condition_index: u16,
        /// Rotation direction (-1,+1)
        position: i8,
    } = 13,
}

// Size validation for TriggerCondition
// DO NOT CHANGE THIS: Will invalidate existing generated KLL layouts
const_assert_eq!(core::mem::size_of::<TriggerCondition>(), 6);

// NOTE: It's not possible to make this a trait (yet)
impl TriggerCondition {
    /// Convert enum to an array of bytes
    /// # Safety
    pub const unsafe fn bytes(&self) -> &[u8] {
        &*core::ptr::slice_from_raw_parts(
            (self as *const Self) as *const u8,
            core::mem::size_of::<Self>(),
        )
    }

    /// Convert array of bytes to enum
    /// # Safety
    pub const unsafe fn from_byte_array(bytes: [u8; core::mem::size_of::<Self>()]) -> Self {
        core::mem::transmute(bytes)
    }

    /// Convert slice of bytes to enum
    /// Aggressively casts the provide u8 slice to retrieve a TriggerCondition
    /// # Safety
    pub const unsafe fn from_bytes(bytes: &[u8]) -> Self {
        core::ptr::read(bytes.as_ptr() as *const &[u8] as *const Self)
    }

    /// Attempts to determine the index value of the condition
    /// If an index is not valid, return 0 instead (index may not have any meaning)
    pub fn index(&self) -> u16 {
        match self {
            TriggerCondition::None => 0,
            TriggerCondition::Switch { index, .. } => *index,
            TriggerCondition::HidLed { index, .. } => (*index).into(),
            TriggerCondition::AnalogDistance { index, .. } => *index,
            TriggerCondition::AnalogVelocity { index, .. } => *index,
            TriggerCondition::AnalogAcceleration { index, .. } => *index,
            TriggerCondition::AnalogJerk { index, .. } => *index,
            TriggerCondition::Layer { layer, .. } => (*layer).into(),
            TriggerCondition::Animation { index, .. } => *index,
            TriggerCondition::Sleep { .. } => 0,
            TriggerCondition::Resume { .. } => 0,
            TriggerCondition::Inactive { .. } => 0,
            TriggerCondition::Active { .. } => 0,
            TriggerCondition::Rotation { index, .. } => (*index).into(),
        }
    }

    /// Compare TriggerEvent to TriggerCondition
    /// NOTE: This is not a direct equivalent comparison each type and state can influence
    ///       how the loop_condition_index is evaluated.
    ///       In a way, this is similar to the voting scheme of the older C KLL implementation.
    pub fn evaluate(&self, event: TriggerEvent, loop_condition_lookup: &[u32]) -> Vote {
        // Make sure the Id's match
        if u8::from(*self) != u8::from(event) {
            return Vote::Insufficient;
        }

        // Make sure the indices match
        if self.index() != event.index() {
            return Vote::Insufficient;
        }

        // We only need to compare like events as they must match
        match self {
            TriggerCondition::None => Vote::Positive,
            TriggerCondition::Switch {
                state,
                loop_condition_index,
                ..
            } => {
                if let TriggerEvent::Switch {
                    state: e_state,
                    last_state,
                    ..
                } = event
                {
                    state.compare(
                        loop_condition_lookup[*loop_condition_index as usize],
                        e_state,
                        last_state,
                    )
                } else {
                    Vote::Insufficient
                }
            }
            _ => {
                panic!("Unknown condition! Please fix.");
            }
        }
    }
}

/// CapabilityState
/// After voting with the indicated TriggerConditions, the CapabilityState is used by the Result
/// capabilities to evaluate a generic decision.
/// This mirrors CapabilityEvent, except that the Passthrough event is not stored as it is not
/// known at compile time.
/// If passthrough has been specified the final element of the last combo will be sent instead
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CapabilityState {
    /// Invalid, ignore this event
    None = 0,
    /// Initial state (e.g. press)
    Initial = 1,
    /// Last state (e.g. release)
    Last = 2,
    /// Any activation (Initial+Last)
    Any = 3,
    /// Event passthrough
    Passthrough = 4,
}

impl CapabilityState {
    /// Using a CapabilityState and TriggerEvent, generate a CapabilityEvent
    pub fn event(&self, event: TriggerEvent) -> CapabilityEvent {
        match self {
            CapabilityState::None => CapabilityEvent::None,
            CapabilityState::Initial => CapabilityEvent::Initial,
            CapabilityState::Last => CapabilityEvent::Last,
            CapabilityState::Any => CapabilityEvent::Any,
            CapabilityState::Passthrough => CapabilityEvent::Passthrough(event),
        }
    }
}

/// CapabilityEvent
/// After voting with the indicated TriggerConditions, the CapabilityEvent is used by the Result
/// capabilities to evaluate a generic decision.
/// Mirrors CapabilityState, except that Passthrough contains the TriggerEvent to pass through
/// to the corresponding Capability (see ResultGuide).
/// If passthrough has been specified the final element of the last combo will be sent instead
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CapabilityEvent {
    /// Invalid, ignore this event
    None = 0,
    /// Initial state (e.g. press)
    Initial = 1,
    /// Last state (e.g. release)
    Last = 2,
    /// Any activation (Initial+Last)
    Any = 3,
    /// TriggerEvent passthrough
    Passthrough(TriggerEvent) = 4,
}

/*
/// Position
/// Each position has 6 dimensions
/// Units are in mm
pub struct Position {
    /// x position
    x: f32,
    /// y position
    y: f32,
    /// z position
    z: f32,
    /// Rotation x direction
    rx: f32,
    /// Rotation y direction
    ry: f32,
    /// Rotation z direction
    rz: f32,
}
*/
