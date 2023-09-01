//! I'm not super pleased with how these impls turned out, eventually I want to make this a macro,
//! but that's a huge can of worms that i'm not opening yet. Grouping them all here will let me
//! dumpster them easier when the time comes though.

use std::ops::{BitAnd, BitOr, BitXor};

use crate::{
    enums::{
        buttons::{ControllerInput, EngineInput},
        general::Flags,
    },
    utils::BitFlags,
};

// ----------------------------------------- EngineInput ---------------------------------------- //
impl From<u32> for EngineInput {
    fn from(val: u32) -> Self {
        // ensure no invalid bits
        assert_eq!(
            val & 0x7F00_E080,
            0,
            "Invalid bits, cannot construct EngineBtn value from {val}"
        );
        Self::Raw(val)
    }
}

impl From<EngineInput> for u32 {
    fn from(val: EngineInput) -> u32 {
        use EngineInput::*;
        match val {
            Raw(x) => x,
            None => 0,
            DPAD_LEFT => 1 << 0,
            DPAD_RIGHT => 1 << 1,
            DPAD_DOWN => 1 << 2,
            DPAD_UP => 1 << 3,
            Z => 1 << 4,
            R => 1 << 5,
            L => 1 << 6,
            A => 1 << 8,
            B => 1 << 9,
            X => 1 << 10,
            Y => 1 << 11,
            START => 1 << 12,
            JOYSTICK_UP => 1 << 16,
            JOYSTICK_DOWN => 1 << 17,
            JOYSTICK_LEFT => 1 << 18,
            JOYSTICK_RIGHT => 1 << 19,
            CSTICK_UP => 1 << 20,
            CSTICK_DOWN => 1 << 21,
            CSTICK_LEFT => 1 << 22,
            CSTICK_RIGHT => 1 << 23,
            ANY_TRIGGER => 1 << 31,
        }
    }
}

impl BitFlags<u32> for EngineInput {}

impl<T: std::fmt::Debug + Into<u32> + Copy> BitOr<T> for EngineInput {
    type Output = Self;

    fn bitor(self, rhs: T) -> Self::Output {
        assert_eq!(
            rhs.into() & 0x7F00_E080,
            0,
            "Invalid bits, cannot construct EngineBtn value from {rhs:?}"
        );
        Self::Raw(u32::from(self) | rhs.into())
    }
}

impl<T: std::fmt::Debug + Into<u32> + Copy> BitAnd<T> for EngineInput {
    type Output = Self;

    fn bitand(self, rhs: T) -> Self::Output {
        assert_eq!(
            rhs.into() & 0x7F00_E080,
            0,
            "Invalid bits, cannot construct EngineBtn value from {rhs:?}"
        );
        Self::Raw(u32::from(self) & rhs.into())
    }
}

impl<T: std::fmt::Debug + Into<u32> + Copy> BitXor<T> for EngineInput {
    type Output = Self;

    fn bitxor(self, rhs: T) -> Self::Output {
        assert_eq!(
            rhs.into() & 0x7F00_E080,
            0,
            "Invalid bits, cannot construct EngineBtn value from {rhs:?}"
        );
        Self::Raw(u32::from(self) ^ rhs.into())
    }
}

// --------------------------------------- ControllerInput -------------------------------------- //

impl From<u16> for ControllerInput {
    fn from(val: u16) -> Self {
        Self::Raw(val)
    }
}

impl From<ControllerInput> for u16 {
    fn from(val: ControllerInput) -> Self {
        use ControllerInput::*;
        match val {
            Raw(x) => x,
            None => 0,
            DPAD_LEFT => 1 << 0,
            DPAD_RIGHT => 1 << 1,
            DPAD_DOWN => 1 << 2,
            DPAD_UP => 1 << 3,
            Z => 1 << 4,
            R => 1 << 5,
            L => 1 << 6,
            A => 1 << 8,
            B => 1 << 9,
            X => 1 << 10,
            Y => 1 << 11,
            START => 1 << 12,
        }
    }
}

impl BitFlags<u16> for ControllerInput {}

impl<T: std::fmt::Debug + Into<u16> + Copy> BitOr<T> for ControllerInput {
    type Output = Self;

    fn bitor(self, rhs: T) -> Self::Output {
        assert_eq!(
            rhs.into() & 0xE080,
            0,
            "Invalid bits, cannot construct EngineBtn value from {rhs:?}"
        );
        Self::Raw(u16::from(self) | rhs.into())
    }
}

impl<T: std::fmt::Debug + Into<u16> + Copy> BitAnd<T> for ControllerInput {
    type Output = Self;

    fn bitand(self, rhs: T) -> Self::Output {
        assert_eq!(
            rhs.into() & 0xE080,
            0,
            "Invalid bits, cannot construct EngineBtn value from {rhs:?}"
        );
        Self::Raw(u16::from(self) & rhs.into())
    }
}

impl<T: std::fmt::Debug + Into<u16> + Copy> BitXor<T> for ControllerInput {
    type Output = Self;

    fn bitxor(self, rhs: T) -> Self::Output {
        assert_eq!(
            rhs.into() & 0xE080,
            0,
            "Invalid bits, cannot construct EngineBtn value from {rhs:?}"
        );
        Self::Raw(u16::from(self) ^ rhs.into())
    }
}

// ------------------------------------------- Flags1 ------------------------------------------- //

impl From<u64> for Flags {
    fn from(val: u64) -> Self {
        assert_eq!(
            val & 0xFFFF_FF00_0000_0000,
            0,
            "Cannot construct flags, value '{val}' contains invalid bits"
        );
        Self::Raw(val)
    }
}

impl From<Flags> for u64 {
    fn from(val: Flags) -> Self {
        use Flags::*;
        match val {
            Flags::Raw(x) => x,
            None => 0,
            BIT_1_1 => 1 << 0,
            ABSORB_BUBBLE => 1 << 1,
            BIT_1_3 => 1 << 2,
            REFLECT_NO_STEAL => 1 << 3,
            REFLECT_BUBBLE => 1 << 4,
            BIT_1_6 => 1 << 5,
            BIT_1_7 => 1 << 6,
            BIT_1_8 => 1 << 7,
            BIT_2_1 => 1 << 8,
            BIT_2_2 => 1 << 9,
            SUBACTION_INVULN => 1 << 10,
            FASTFALL => 1 << 11,
            DEFENDER_HITLAG => 1 << 12,
            HITLAG => 1 << 13,
            BIT_2_7 => 1 << 14,
            BIT_2_8 => 1 << 15,
            BIT_3_1 => 1 << 16,
            BIT_3_2 => 1 << 17,
            GRAB_HOLD => 1 << 18,
            BIT_3_4 => 1 << 19,
            BIT_3_5 => 1 << 20,
            BIT_3_6 => 1 << 21,
            BIT_3_7 => 1 << 22,
            SHIELDING => 1 << 23,
            BIT_4_1 => 1 << 24,
            HITSTUN => 1 << 25,
            HITBOX_TOUCHING_SHIELD => 1 << 26,
            BIT_4_4 => 1 << 27,
            BIT_4_5 => 1 << 28,
            PWERSHIELD_BUBBLE => 1 << 29,
            BIT_4_7 => 1 << 30,
            BIT_4_8 => 1 << 31,
            BIT_5_1 => 1 << 32,
            CLOAKING_DEVICE => 1 << 33,
            BIT_5_3 => 1 << 34,
            FOLLOWER => 1 << 35,
            INACTIVE => 1 << 36,
            BIT_5_6 => 1 << 37,
            DEAD => 1 << 38,
            OFFSCREEN => 1 << 39,
        }
    }
}

impl BitFlags<u64> for Flags {}

impl<T: std::fmt::Debug + Into<u64>> BitOr<T> for Flags {
    type Output = Self;

    fn bitor(self, rhs: T) -> Self::Output {
        Self::Raw(u64::from(self) | rhs.into())
    }
}

impl<T: std::fmt::Debug + Into<u64>> BitAnd<T> for Flags {
    type Output = Self;

    fn bitand(self, rhs: T) -> Self::Output {
        Self::Raw(u64::from(self) & rhs.into())
    }
}

impl<T: std::fmt::Debug + Into<u64>> BitXor<T> for Flags {
    type Output = Self;

    fn bitxor(self, rhs: T) -> Self::Output {
        Self::Raw(u64::from(self) ^ rhs.into())
    }
}
