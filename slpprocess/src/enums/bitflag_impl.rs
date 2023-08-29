//! I'm not super pleased with how these impls turned out, eventually I want to make this a macro,
//! but that's a huge can of worms that i'm not opening yet. Grouping them all here will let me
//! dumpster them easier when the time comes though.

use std::ops::{BitAnd, BitOr, BitXor};

use crate::{
    enums::{
        buttons::{ControllerInput, EngineInput},
        general::{Flags1, Flags2, Flags3, Flags4, Flags5},
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

impl From<u8> for Flags1 {
    fn from(val: u8) -> Self {
        Self::Raw(val)
    }
}

impl From<Flags1> for u8 {
    fn from(val: Flags1) -> Self {
        use Flags1::*;
        match val {
            Flags1::Raw(x) => x,
            None => 0,
            BIT_1 => 1 << 0,
            ABSORB_BUBBLE => 1 << 1,
            BIT_3 => 1 << 2,
            REFLECT_NO_STEAL => 1 << 3,
            REFLECT_BUBBLE => 1 << 4,
            BIT_6 => 1 << 5,
            BIT_7 => 1 << 6,
            BIT_8 => 1 << 7,
        }
    }
}

impl BitFlags<u8> for Flags1 {}

impl<T: std::fmt::Debug + Into<u8>> BitOr<T> for Flags1 {
    type Output = Self;

    fn bitor(self, rhs: T) -> Self::Output {
        Self::Raw(u8::from(self) | rhs.into())
    }
}

impl<T: std::fmt::Debug + Into<u8>> BitAnd<T> for Flags1 {
    type Output = Self;

    fn bitand(self, rhs: T) -> Self::Output {
        Self::Raw(u8::from(self) & rhs.into())
    }
}

impl<T: std::fmt::Debug + Into<u8>> BitXor<T> for Flags1 {
    type Output = Self;

    fn bitxor(self, rhs: T) -> Self::Output {
        Self::Raw(u8::from(self) ^ rhs.into())
    }
}

// ------------------------------------------- Flags2 ------------------------------------------- //

impl From<u8> for Flags2 {
    fn from(val: u8) -> Self {
        Self::Raw(val)
    }
}

impl From<Flags2> for u8 {
    fn from(val: Flags2) -> Self {
        use Flags2::*;
        match val {
            Flags2::Raw(x) => x,
            None => 0,
            BIT_1 => 1 << 0,
            BIT_2 => 1 << 1,
            SUBACTION_INVULN => 1 << 2,
            FASTFALL => 1 << 3,
            DEFENDER_HITLAG => 1 << 4,
            HITLAG => 1 << 5,
            BIT_7 => 1 << 6,
            BIT_8 => 1 << 7,
        }
    }
}

impl BitFlags<u8> for Flags2 {}

impl<T: std::fmt::Debug + Into<u8>> BitOr<T> for Flags2 {
    type Output = Self;

    fn bitor(self, rhs: T) -> Self::Output {
        Self::Raw(u8::from(self) | rhs.into())
    }
}

impl<T: std::fmt::Debug + Into<u8>> BitAnd<T> for Flags2 {
    type Output = Self;

    fn bitand(self, rhs: T) -> Self::Output {
        Self::Raw(u8::from(self) & rhs.into())
    }
}

impl<T: std::fmt::Debug + Into<u8>> BitXor<T> for Flags2 {
    type Output = Self;

    fn bitxor(self, rhs: T) -> Self::Output {
        Self::Raw(u8::from(self) ^ rhs.into())
    }
}

// ------------------------------------------- Flags3 ------------------------------------------- //

impl From<u8> for Flags3 {
    fn from(val: u8) -> Self {
        Self::Raw(val)
    }
}

impl From<Flags3> for u8 {
    fn from(val: Flags3) -> Self {
        use Flags3::*;
        match val {
            Flags3::Raw(x) => x,
            None => 0,
            BIT_1 => 0b0000_0001,
            BIT_2 => 0b0000_0010,
            GRAB_HOLD => 0b0000_0100,
            BIT_4 => 0b0000_1000,
            BIT_5 => 0b0001_0000,
            BIT_6 => 0b0010_0000,
            BIT_7 => 0b0100_0000,
            SHIELDING => 0b1000_0000,
        }
    }
}

impl BitFlags<u8> for Flags3 {}

impl<T: std::fmt::Debug + Into<u8>> BitOr<T> for Flags3 {
    type Output = Self;

    fn bitor(self, rhs: T) -> Self::Output {
        Self::Raw(u8::from(self) | rhs.into())
    }
}

impl<T: std::fmt::Debug + Into<u8>> BitAnd<T> for Flags3 {
    type Output = Self;

    fn bitand(self, rhs: T) -> Self::Output {
        Self::Raw(u8::from(self) & rhs.into())
    }
}

impl<T: std::fmt::Debug + Into<u8>> BitXor<T> for Flags3 {
    type Output = Self;

    fn bitxor(self, rhs: T) -> Self::Output {
        Self::Raw(u8::from(self) ^ rhs.into())
    }
}

// ------------------------------------------- Flags4 ------------------------------------------- //

impl From<u8> for Flags4 {
    fn from(val: u8) -> Self {
        Self::Raw(val)
    }
}

impl From<Flags4> for u8 {
    fn from(val: Flags4) -> Self {
        use Flags4::*;
        match val {
            Flags4::Raw(x) => x,
            None => 0,
            BIT_1 => 0b0000_0001,
            HITSTUN => 0b0000_0010,
            HITBOX_TOUCHING_SHIELD => 0b0000_0100,
            BIT_4 => 0b0000_1000,
            BIT_5 => 0b0001_0000,
            PWERSHIELD_BUBBLE => 0b0010_0000,
            BIT_7 => 0b0100_0000,
            BIT_8 => 0b1000_0000,
        }
    }
}

impl BitFlags<u8> for Flags4 {}

impl<T: std::fmt::Debug + Into<u8>> BitOr<T> for Flags4 {
    type Output = Self;

    fn bitor(self, rhs: T) -> Self::Output {
        Self::Raw(u8::from(self) | rhs.into())
    }
}

impl<T: std::fmt::Debug + Into<u8>> BitAnd<T> for Flags4 {
    type Output = Self;

    fn bitand(self, rhs: T) -> Self::Output {
        Self::Raw(u8::from(self) & rhs.into())
    }
}

impl<T: std::fmt::Debug + Into<u8>> BitXor<T> for Flags4 {
    type Output = Self;

    fn bitxor(self, rhs: T) -> Self::Output {
        Self::Raw(u8::from(self) ^ rhs.into())
    }
}

// ------------------------------------------- Flags5 ------------------------------------------- //

impl From<u8> for Flags5 {
    fn from(val: u8) -> Self {
        Self::Raw(val)
    }
}

impl From<Flags5> for u8 {
    fn from(val: Flags5) -> Self {
        use Flags5::*;
        match val {
            Flags5::Raw(x) => x,
            None => 0,
            BIT_1 => 0b0000_0001,
            CLOAKING_DEVICE => 0b0000_0010,
            BIT_3 => 0b0000_0100,
            FOLLOWER => 0b0000_1000,
            INACTIVE => 0b0001_0000,
            BIT_6 => 0b0010_0000,
            DEAD => 0b0100_0000,
            OFFSCREEN => 0b1000_0000,
        }
    }
}

impl BitFlags<u8> for Flags5 {}

impl<T: std::fmt::Debug + Into<u8>> BitOr<T> for Flags5 {
    type Output = Self;

    fn bitor(self, rhs: T) -> Self::Output {
        Self::Raw(u8::from(self) | rhs.into())
    }
}

impl<T: std::fmt::Debug + Into<u8>> BitAnd<T> for Flags5 {
    type Output = Self;

    fn bitand(self, rhs: T) -> Self::Output {
        Self::Raw(u8::from(self) & rhs.into())
    }
}

impl<T: std::fmt::Debug + Into<u8>> BitXor<T> for Flags5 {
    type Output = Self;

    fn bitxor(self, rhs: T) -> Self::Output {
        Self::Raw(u8::from(self) ^ rhs.into())
    }
}
