//! I'm not super pleased with how these impls turned out. Literally every part of this file is
//! ugly. Eventually I want to make this a macro, but that's a huge can of worms that i'm not
//! opening yet. Grouping them all here will let me dumpster them easier when the time comes though.

use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::fmt::{Debug, Display};
use std::ops::{BitAnd, BitOr, BitXor};

use crate::{
    enums::{
        buttons::{ControllerInput, EngineInput},
        general::Flags,
    },
    utils::BitFlags,
};

// ----------------------------------------- EngineInput ---------------------------------------- //
impl EngineInput {
    fn pretty_print(&self) -> String {
        if u32::from(*self) == 0 {
            "EngineInput{{None}}".to_string();
        }

        let mut result: Vec<&str> = Vec::new();

        if self.contains(Self::DPAD_LEFT.into()) {
            result.push("DPAD_LEFT");
        }
        if self.contains(Self::DPAD_RIGHT.into()) {
            result.push("DPAD_RIGHT");
        }
        if self.contains(Self::DPAD_DOWN.into()) {
            result.push("DPAD_DOWN");
        }
        if self.contains(Self::DPAD_UP.into()) {
            result.push("DPAD_UP");
        }
        if self.contains(Self::Z.into()) {
            result.push("Z");
        }
        if self.contains(Self::R.into()) {
            result.push("R");
        }
        if self.contains(Self::L.into()) {
            result.push("L");
        }
        if self.contains(Self::A.into()) {
            result.push("A");
        }
        if self.contains(Self::B.into()) {
            result.push("B");
        }
        if self.contains(Self::X.into()) {
            result.push("X");
        }
        if self.contains(Self::Y.into()) {
            result.push("Y");
        }
        if self.contains(Self::START.into()) {
            result.push("START");
        }
        if self.contains(Self::JOYSTICK_UP.into()) {
            result.push("JOYSTICK_UP");
        }
        if self.contains(Self::JOYSTICK_DOWN.into()) {
            result.push("JOYSTICK_DOWN");
        }
        if self.contains(Self::JOYSTICK_LEFT.into()) {
            result.push("JOYSTICK_LEFT");
        }
        if self.contains(Self::JOYSTICK_RIGHT.into()) {
            result.push("JOYSTICK_RIGHT");
        }
        if self.contains(Self::CSTICK_UP.into()) {
            result.push("CSTICK_UP");
        }
        if self.contains(Self::CSTICK_DOWN.into()) {
            result.push("CSTICK_DOWN");
        }
        if self.contains(Self::CSTICK_LEFT.into()) {
            result.push("CSTICK_LEFT");
        }
        if self.contains(Self::CSTICK_RIGHT.into()) {
            result.push("CSTICK_RIGHT");
        }
        if self.contains(Self::ANY_TRIGGER.into()) {
            result.push("ANY_TRIGGER");
        }
        result.join("|")
    }
}

impl Debug for EngineInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            ["EngineInput{", &self.pretty_print(), "}"].join("")
        )
    }
}

impl Display for EngineInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty_print())
    }
}

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
            // this shouldn't cause any issues? Anything that's not a valid enum member will be
            // dumped into ::Raw()
            _ => unsafe {
                let temp: u64 = std::mem::transmute(val);
                temp as u32
            },
        }
    }
}

impl BitFlags for EngineInput {
    type Other = u32;
}

impl<T: Debug + Into<u32> + Copy> BitOr<T> for EngineInput {
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

impl<T: Debug + Into<u32> + Copy> BitAnd<T> for EngineInput {
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

impl<T: Debug + Into<u32> + Copy> BitXor<T> for EngineInput {
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

impl PartialEq<u32> for EngineInput {
    fn eq(&self, other: &u32) -> bool {
        u32::from(*self) == *other
    }
}

impl PartialOrd<u32> for EngineInput {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        u32::from(*self).partial_cmp(other)
    }
}

// --------------------------------------- ControllerInput -------------------------------------- //

impl ControllerInput {
    fn pretty_print(&self) -> String {
        if u16::from(*self) == 0 {
            "ControllerInput{{None}}".to_string();
        }
        let mut result: Vec<&str> = Vec::new();

        if self.contains(Self::DPAD_LEFT.into()) {
            result.push("DPAD_LEFT");
        }
        if self.contains(Self::DPAD_RIGHT.into()) {
            result.push("DPAD_RIGHT");
        }
        if self.contains(Self::DPAD_DOWN.into()) {
            result.push("DPAD_DOWN");
        }
        if self.contains(Self::DPAD_UP.into()) {
            result.push("DPAD_UP");
        }
        if self.contains(Self::Z.into()) {
            result.push("Z");
        }
        if self.contains(Self::R.into()) {
            result.push("R");
        }
        if self.contains(Self::L.into()) {
            result.push("L");
        }
        if self.contains(Self::A.into()) {
            result.push("A");
        }
        if self.contains(Self::B.into()) {
            result.push("B");
        }
        if self.contains(Self::X.into()) {
            result.push("X");
        }
        if self.contains(Self::Y.into()) {
            result.push("Y");
        }
        if self.contains(Self::START.into()) {
            result.push("START");
        }

        result.join("")
    }
}

impl Debug for ControllerInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            ["ControllerInput{", &self.pretty_print(), "}"].join("")
        )
    }
}

impl Display for ControllerInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty_print())
    }
}

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

impl BitFlags for ControllerInput {
    type Other = u16;
}

impl<T: Debug + Into<u16> + Copy> BitOr<T> for ControllerInput {
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

impl<T: Debug + Into<u16> + Copy> BitAnd<T> for ControllerInput {
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

impl<T: Debug + Into<u16> + Copy> BitXor<T> for ControllerInput {
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

impl PartialEq<u16> for ControllerInput {
    fn eq(&self, other: &u16) -> bool {
        u16::from(*self) == *other
    }
}

impl PartialOrd<u16> for ControllerInput {
    fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
        u16::from(*self).partial_cmp(other)
    }
}

// ------------------------------------------- Flags ------------------------------------------- //

impl Flags {
    fn pretty_print(&self, debug: bool) -> String {
        if u64::from(*self) == 0 {
            "Flags{{None}}".to_string();
        }

        let mut result: Vec<&str> = Vec::new();

        if self.contains(Self::ABSORB_BUBBLE.into()) {
            result.push("ABSORB_BUBBLE");
        }
        if self.contains(Self::REFLECT_BUBBLE.into()) {
            result.push("REFLECT_BUBBLE");
        }
        if self.contains(Self::REFLECT_NO_STEAL.into()) {
            result.push("REFLECT_NO_STEAL");
        }
        if self.contains(Self::SUBACTION_INVULN.into()) {
            result.push("SUBACTION_INVULN");
        }
        if self.contains(Self::FASTFALL.into()) {
            result.push("FASTFALL");
        }
        if self.contains(Self::DEFENDER_HITLAG.into()) {
            result.push("DEFENDER_HITLAG");
        }
        if self.contains(Self::HITLAG.into()) {
            result.push("HITLAG");
        }
        if self.contains(Self::GRAB_HOLD.into()) {
            result.push("GRAB_HOLD");
        }
        if self.contains(Self::SHIELDING.into()) {
            result.push("SHIELDING");
        }
        if self.contains(Self::HITSTUN.into()) {
            result.push("HITSTUN");
        }
        if self.contains(Self::HITBOX_TOUCHING_SHIELD.into()) {
            result.push("HITBOX_TOUCHING_SHIELD");
        }
        if self.contains(Self::POWERSHIELD_BUBBLE.into()) {
            result.push("PWERSHIELD_BUBBLE");
        }
        if self.contains(Self::CLOAKING_DEVICE.into()) {
            result.push("CLOAKING_DEVICE");
        }
        if self.contains(Self::FOLLOWER.into()) {
            result.push("FOLLOWER");
        }
        if self.contains(Self::INACTIVE.into()) {
            result.push("INACTIVE");
        }
        if self.contains(Self::DEAD.into()) {
            result.push("DEAD");
        }
        if self.contains(Self::OFFSCREEN.into()) {
            result.push("OFFSCREEN");
        }

        if debug {
            if self.contains(Self::BIT_1_1.into()) {
                result.push("BIT_1_1");
            }
            if self.contains(Self::BIT_1_3.into()) {
                result.push("BIT_1_3");
            }
            if self.contains(Self::BIT_1_6.into()) {
                result.push("BIT_1_6");
            }
            if self.contains(Self::BIT_1_7.into()) {
                result.push("BIT_1_7");
            }
            if self.contains(Self::BIT_1_8.into()) {
                result.push("BIT_1_8");
            }
            if self.contains(Self::BIT_2_1.into()) {
                result.push("BIT_2_1");
            }
            if self.contains(Self::BIT_2_2.into()) {
                result.push("BIT_2_2");
            }
            if self.contains(Self::BIT_2_7.into()) {
                result.push("BIT_2_7");
            }
            if self.contains(Self::BIT_2_8.into()) {
                result.push("BIT_2_8");
            }
            if self.contains(Self::BIT_3_1.into()) {
                result.push("BIT_3_1");
            }
            if self.contains(Self::BIT_3_2.into()) {
                result.push("BIT_3_2");
            }
            if self.contains(Self::BIT_3_4.into()) {
                result.push("BIT_3_4");
            }
            if self.contains(Self::BIT_3_5.into()) {
                result.push("BIT_3_5");
            }
            if self.contains(Self::BIT_3_6.into()) {
                result.push("BIT_3_6");
            }
            if self.contains(Self::BIT_3_7.into()) {
                result.push("BIT_3_7");
            }
            if self.contains(Self::BIT_4_1.into()) {
                result.push("BIT_4_1");
            }
            if self.contains(Self::BIT_4_4.into()) {
                result.push("BIT_4_4");
            }
            if self.contains(Self::BIT_4_5.into()) {
                result.push("BIT_4_5");
            }
            if self.contains(Self::BIT_4_7.into()) {
                result.push("BIT_4_7");
            }
            if self.contains(Self::BIT_4_8.into()) {
                result.push("BIT_4_8");
            }
            if self.contains(Self::BIT_5_1.into()) {
                result.push("BIT_5_1");
            }
            if self.contains(Self::BIT_5_3.into()) {
                result.push("BIT_5_3");
            }
            if self.contains(Self::BIT_5_6.into()) {
                result.push("BIT_5_6");
            }
        }

        result.join("")
    }
}

impl Debug for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            ["Flags{", &self.pretty_print(true), "}"].join("")
        )
    }
}

impl Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty_print(false))
    }
}

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
            POWERSHIELD_BUBBLE => 1 << 29,
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

impl BitFlags for Flags {
    type Other = u64;
}

impl<T: Debug + Into<u64>> BitOr<T> for Flags {
    type Output = Self;

    fn bitor(self, rhs: T) -> Self::Output {
        Self::Raw(u64::from(self) | rhs.into())
    }
}

impl<T: Debug + Into<u64>> BitAnd<T> for Flags {
    type Output = Self;

    fn bitand(self, rhs: T) -> Self::Output {
        Self::Raw(u64::from(self) & rhs.into())
    }
}

impl<T: Debug + Into<u64>> BitXor<T> for Flags {
    type Output = Self;

    fn bitxor(self, rhs: T) -> Self::Output {
        Self::Raw(u64::from(self) ^ rhs.into())
    }
}

impl PartialEq<u64> for Flags {
    fn eq(&self, other: &u64) -> bool {
        u64::from(*self) == *other
    }
}

impl PartialOrd<u64> for Flags {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        u64::from(*self).partial_cmp(other)
    }
}
