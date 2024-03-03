// I'm not super pleased with how these impls turned out. Literally every part of this file is
// ugly. Eventually I want to make this a macro, but that's a huge can of worms that i'm not
// opening yet. Grouping them all here will let me dumpster them easier when the time comes though.

use std::ops::Deref;
use std::{
    fmt::{Debug, Display},
    ops::BitOr,
};

use num_traits::{PrimInt, Zero};

use crate::enums::{
    buttons::{ControllerInput, EngineInput},
    general::Flags,
};

pub trait BitFlags: Into<Self::Other> + Copy {
    type Other: PrimInt;

    /// returns true if `other` is entirely represented in `self`
    #[inline]
    fn contains(self, other: Self::Other) -> bool {
        Into::<Self::Other>::into(self) & other == other
    }

    #[inline]
    fn contained_by(self, other: Self::Other) -> bool {
        let temp = Into::<Self::Other>::into(self);
        temp & other == temp
    }

    /// Returns true if `self` and `other` share any bits
    #[inline]
    fn intersects(self, other: Self::Other) -> bool {
        Into::<Self::Other>::into(self) & other != Self::Other::zero()
    }

    /// Returns the total number of `1` bits
    #[inline]
    fn count_ones(self) -> u32 {
        Into::<Self::Other>::into(self).count_ones()
    }

    /// Returns the total number of `0` bits
    #[inline]
    fn count_zeroes(self) -> u32 {
        Into::<Self::Other>::into(self).count_zeros()
    }
}

/// Marker trait to differentiate Input bitfields from all other bitfields
pub trait Buttons {}

// ----------------------------------------- EngineInput ---------------------------------------- //

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
    #[inline]
    fn from(val: EngineInput) -> u32 {
        use EngineInput::*;
        match val {
            Raw(x) => x,
            // safe due to repr(u32) guarantees
            _ => unsafe { *(&val as *const EngineInput as *const u32) }
        }
    }
}

impl Buttons for EngineInput {}

impl BitFlags for EngineInput {
    type Other = u32;
}

impl BitOr for EngineInput {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        EngineInput::Raw(*self | *rhs)
    }
}

impl Deref for EngineInput {
    type Target = u32;

    #[inline]
    fn deref(&self) -> &u32 {
        use EngineInput as EI;
        // The commented out code below shows an initial attempt that had the compiler
        // generating (i think) a binary search algorithm. Reinterpreting the pointer is
        // maybe 2-4x faster. That's probably not worth using unsafe for since this won't be
        // used a ton, but the experience with pointer manipulation (and any bugs that arise)
        // are good learning moments.

        // The match is unfortunately necessary. The enum is represented by 4 bytes - in Raw the
        // desired u16 value is stored in the upper 2, for non-Raw the data is stored in the lower 2
        match self {
            EI::Raw(x) => x,
            // Safety: when using #[repr] the layout is predictable
            _ => unsafe { &(*(self as *const EngineInput as *const u32)) },
        }
        // match self {
        //     EI::Raw(x) => x,
        //     EI::None => &0,
        //     EI::DPAD_LEFT => &(1 << 0),
        //     EI::DPAD_RIGHT => &(1 << 1),
        //     EI::DPAD_DOWN => &(1 << 2),
        //     EI::DPAD_UP => &(1 << 3),
        //     EI::Z => &(1 << 4),
        //     EI::R => &(1 << 5),
        //     EI::L => &(1 << 6),
        //     EI::A => &(1 << 8),
        //     EI::B => &(1 << 9),
        //     EI::X => &(1 << 10),
        //     EI::Y => &(1 << 11),
        //     EI::START => &(1 << 12),
        //     EI::JOYSTICK_UP => &(1 << 16),
        //     EI::JOYSTICK_DOWN => &(1 << 17),
        //     EI::JOYSTICK_LEFT => &(1 << 18),
        //     EI::JOYSTICK_RIGHT => &(1 << 19),
        //     EI::CSTICK_UP => &(1 << 20),
        //     EI::CSTICK_DOWN => &(1 << 21),
        //     EI::CSTICK_LEFT => &(1 << 22),
        //     EI::CSTICK_RIGHT => &(1 << 23),
        //     EI::ANY_TRIGGER => &(1 << 31),
        // }
    }
}

impl EngineInput {
    // might need it at some point
    #[inline]
    fn _to_raw(&mut self) {
        *self = Self::Raw(u32::from(*self));
    }

    fn pretty_print(&self) -> String {
        if **self == 0 {
            return "None".to_string();
        }

        let mut result: Vec<&str> = Vec::new();

        if self.contains(*Self::DPAD_LEFT) {
            result.push("DPAD_LEFT");
        }
        if self.contains(*Self::DPAD_RIGHT) {
            result.push("DPAD_RIGHT");
        }
        if self.contains(*Self::DPAD_DOWN) {
            result.push("DPAD_DOWN");
        }
        if self.contains(*Self::DPAD_UP) {
            result.push("DPAD_UP");
        }
        if self.contains(*Self::Z) {
            result.push("Z");
        }
        if self.contains(*Self::R) {
            result.push("R");
        }
        if self.contains(*Self::L) {
            result.push("L");
        }
        if self.contains(*Self::A) {
            result.push("A");
        }
        if self.contains(*Self::B) {
            result.push("B");
        }
        if self.contains(*Self::X) {
            result.push("X");
        }
        if self.contains(*Self::Y) {
            result.push("Y");
        }
        if self.contains(*Self::START) {
            result.push("START");
        }
        if self.contains(*Self::JOYSTICK_UP) {
            result.push("JOYSTICK_UP");
        }
        if self.contains(*Self::JOYSTICK_DOWN) {
            result.push("JOYSTICK_DOWN");
        }
        if self.contains(*Self::JOYSTICK_LEFT) {
            result.push("JOYSTICK_LEFT");
        }
        if self.contains(*Self::JOYSTICK_RIGHT) {
            result.push("JOYSTICK_RIGHT");
        }
        if self.contains(*Self::CSTICK_UP) {
            result.push("CSTICK_UP");
        }
        if self.contains(*Self::CSTICK_DOWN) {
            result.push("CSTICK_DOWN");
        }
        if self.contains(*Self::CSTICK_LEFT) {
            result.push("CSTICK_LEFT");
        }
        if self.contains(*Self::CSTICK_RIGHT) {
            result.push("CSTICK_RIGHT");
        }
        if self.contains(*Self::ANY_TRIGGER) {
            result.push("ANY_TRIGGER");
        }
        result.join("|")
    }
}

// impl<T: Debug + Into<u32> + Copy> BitOr<T> for EngineInput {
//     type Output = Self;

//     fn bitor(self, rhs: T) -> Self::Output {
//         assert_eq!(
//             rhs.into() & 0x7F00_E080,
//             0,
//             "Invalid bits, cannot construct EngineBtn value from {rhs:?}"
//         );
//         Self::Raw(u32::from(self) | rhs.into())
//     }
// }

// impl<T: Debug + Into<u32> + Copy> BitAnd<T> for EngineInput {
//     type Output = Self;

//     fn bitand(self, rhs: T) -> Self::Output {
//         assert_eq!(
//             rhs.into() & 0x7F00_E080,
//             0,
//             "Invalid bits, cannot construct EngineBtn value from {rhs:?}"
//         );
//         Self::Raw(u32::from(self) & rhs.into())
//     }
// }

// impl<T: Debug + Into<u32> + Copy> BitXor<T> for EngineInput {
//     type Output = Self;

//     fn bitxor(self, rhs: T) -> Self::Output {
//         assert_eq!(
//             rhs.into() & 0x7F00_E080,
//             0,
//             "Invalid bits, cannot construct EngineBtn value from {rhs:?}"
//         );
//         Self::Raw(u32::from(self) ^ rhs.into())
//     }
// }

// impl PartialEq<u32> for EngineInput {
//     fn eq(&self, other: &u32) -> bool {
//         u32::from(*self) == *other
//     }
// }

// impl PartialOrd<u32> for EngineInput {
//     fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
//         u32::from(*self).partial_cmp(other)
//     }
// }

// --------------------------------------- ControllerInput -------------------------------------- //

impl From<u16> for ControllerInput {
    #[inline]
    fn from(val: u16) -> Self {
        Self::Raw(val)
    }
}

impl From<ControllerInput> for u16 {
    #[inline]
    fn from(val: ControllerInput) -> Self {
        use ControllerInput as CI;
        match val {
            CI::Raw(x) => x,
            // safe due to repr(u16) guarantees
            _ => unsafe { *(&val as *const ControllerInput as *const u16) },
        }
    }
}

impl Buttons for ControllerInput {}

impl BitFlags for ControllerInput {
    type Other = u16;
}

impl BitOr for ControllerInput {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        ControllerInput::Raw(*self | *rhs)
    }
}
impl Deref for ControllerInput {
    type Target = u16;

    #[inline]
    fn deref(&self) -> &u16 {
        use ControllerInput as CI;
        // The commented out code below shows an initial attempt that had the compiler
        // generating (i think) a binary search algorithm. Reinterpreting the pointer is
        // maybe 2-4x faster. That's probably not worth using unsafe for since this won't be
        // used a ton, but the experience with pointer manipulation (and any bugs that arise)
        // are good learning moments.

        // The match is unfortunately necessary. The enum is represented by 4 bytes - in Raw the
        // desired u16 value is stored in the upper 2, for non-Raw the data is stored in the lower 2
        match self {
            CI::Raw(x) => x,

            // Safety: when using #[repr] the layout is predictable
            _ => unsafe { &(*(self as *const ControllerInput as *const u16)) },
            // CI::None => &(0),
            // CI::DPAD_LEFT => &(1 << 0),
            // CI::DPAD_RIGHT => &(1 << 1),
            // CI::DPAD_DOWN => &(1 << 2),
            // CI::DPAD_UP => &(1 << 3),
            // CI::Z => &(1 << 4),
            // CI::R => &(1 << 5),
            // CI::L => &(1 << 6),
            // CI::A => &(1 << 8),
            // CI::B => &(1 << 9),
            // CI::X => &(1 << 10),
            // CI::Y => &(1 << 11),
            // CI::START => &(1 << 12),
        }
    }
}

impl ControllerInput {
    fn pretty_print(&self) -> String {
        if **self == 0 {
            return "None".to_string();
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

        result.join("|")
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

// impl<T: Debug + Into<u16> + Copy> BitOr<T> for ControllerInput {
//     type Output = Self;

//     fn bitor(self, rhs: T) -> Self::Output {
//         assert_eq!(
//             rhs.into() & 0xE080,
//             0,
//             "Invalid bits, cannot construct EngineBtn value from {rhs:?}"
//         );
//         Self::Raw(u16::from(self) | rhs.into())
//     }
// }

// impl<T: Debug + Into<u16> + Copy> BitAnd<T> for ControllerInput {
//     type Output = Self;

//     fn bitand(self, rhs: T) -> Self::Output {
//         assert_eq!(
//             rhs.into() & 0xE080,
//             0,
//             "Invalid bits, cannot construct EngineBtn value from {rhs:?}"
//         );
//         Self::Raw(u16::from(self) & rhs.into())
//     }
// }

// impl<T: Debug + Into<u16> + Copy> BitXor<T> for ControllerInput {
//     type Output = Self;

//     fn bitxor(self, rhs: T) -> Self::Output {
//         assert_eq!(
//             rhs.into() & 0xE080,
//             0,
//             "Invalid bits, cannot construct EngineBtn value from {rhs:?}"
//         );
//         Self::Raw(u16::from(self) ^ rhs.into())
//     }
// }

// impl PartialEq<u16> for ControllerInput {
//     fn eq(&self, other: &u16) -> bool {
//         u16::from(*self) == *other
//     }
// }

// impl PartialOrd<u16> for ControllerInput {
//     fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
//         u16::from(*self).partial_cmp(other)
//     }
// }

// ------------------------------------------- Flags ------------------------------------------- //

impl From<u64> for Flags {
    #[inline]
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
    #[inline]
    fn from(val: Flags) -> Self {
        use Flags as F;
        match val {
            Flags::Raw(x) => x,
            // safe due to repr(u64) guarantees
            _ => unsafe { *(&val as *const Flags as *const u64) },
        }
    }
}

impl BitFlags for Flags {
    type Other = u64;
}

impl BitOr for Flags {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Flags::Raw(*self | *rhs)
    }
}

impl Deref for Flags {
    type Target = u64;

    #[inline]
    fn deref(&self) -> &u64 {
        match self {
            Flags::Raw(x) => x,
            _ => unsafe { &(*(self as *const Flags as *const u64)) },
            // Flags::None => &0,
            // Flags::BIT_1_1 => &(1 << 0),
            // Flags::ABSORB_BUBBLE => &(1 << 1),
            // Flags::BIT_1_3 => &(1 << 2),
            // Flags::REFLECT_NO_STEAL => &(1 << 3),
            // Flags::REFLECT_BUBBLE => &(1 << 4),
            // Flags::BIT_1_6 => &(1 << 5),
            // Flags::BIT_1_7 => &(1 << 6),
            // Flags::BIT_1_8 => &(1 << 7),
            // Flags::BIT_2_1 => &(1 << 8),
            // Flags::BIT_2_2 => &(1 << 9),
            // Flags::SUBACTION_INVULN => &(1 << 10),
            // Flags::FASTFALL => &(1 << 11),
            // Flags::DEFENDER_HITLAG => &(1 << 12),
            // Flags::HITLAG => &(1 << 13),
            // Flags::BIT_2_7 => &(1 << 14),
            // Flags::BIT_2_8 => &(1 << 15),
            // Flags::BIT_3_1 => &(1 << 16),
            // Flags::BIT_3_2 => &(1 << 17),
            // Flags::GRAB_HOLD => &(1 << 18),
            // Flags::BIT_3_4 => &(1 << 19),
            // Flags::BIT_3_5 => &(1 << 20),
            // Flags::BIT_3_6 => &(1 << 21),
            // Flags::BIT_3_7 => &(1 << 22),
            // Flags::SHIELDING => &(1 << 23),
            // Flags::BIT_4_1 => &(1 << 24),
            // Flags::HITSTUN => &(1 << 25),
            // Flags::HITBOX_TOUCHING_SHIELD => &(1 << 26),
            // Flags::BIT_4_4 => &(1 << 27),
            // Flags::BIT_4_5 => &(1 << 28),
            // Flags::POWERSHIELD_BUBBLE => &(1 << 29),
            // Flags::BIT_4_7 => &(1 << 30),
            // Flags::BIT_4_8 => &(1 << 31),
            // Flags::BIT_5_1 => &(1 << 32),
            // Flags::CLOAKING_DEVICE => &(1 << 33),
            // Flags::BIT_5_3 => &(1 << 34),
            // Flags::FOLLOWER => &(1 << 35),
            // Flags::INACTIVE => &(1 << 36),
            // Flags::BIT_5_6 => &(1 << 37),
            // Flags::DEAD => &(1 << 38),
            // Flags::OFFSCREEN => &(1 << 39),
        }
    }
}

impl Flags {
    fn pretty_print(&self) -> String {
        if **self == 0 {
            return "None".to_string();
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
        if self.contains(Self::ALLOW_INTERRUPT.into()) {
            result.push("ALLOW_INTERRUPT")
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
        if self.contains(Self::GUARD_BUBBLE.into()) {
            result.push("GUARD_BUBBLE");
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
            result.push("POWERSHIELD_BUBBLE");
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

        result.join("|")
    }
}

impl Debug for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ["Flags{", &self.pretty_print(), "}"].join(""))
    }
}

impl Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty_print())
    }
}

// impl<T: Debug + Into<u64>> BitOr<T> for Flags {
//     type Output = Self;

//     fn bitor(self, rhs: T) -> Self::Output {
//         Self::Raw(u64::from(self) | rhs.into())
//     }
// }

// impl<T: Debug + Into<u64>> BitAnd<T> for Flags {
//     type Output = Self;

//     fn bitand(self, rhs: T) -> Self::Output {
//         Self::Raw(u64::from(self) & rhs.into())
//     }
// }

// impl<T: Debug + Into<u64>> BitXor<T> for Flags {
//     type Output = Self;

//     fn bitxor(self, rhs: T) -> Self::Output {
//         Self::Raw(u64::from(self) ^ rhs.into())
//     }
// }

// impl PartialEq<u64> for Flags {
//     fn eq(&self, other: &u64) -> bool {
//         u64::from(*self) == *other
//     }
// }

// impl PartialOrd<u64> for Flags {
//     fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
//         u64::from(*self).partial_cmp(other)
//     }
// }
