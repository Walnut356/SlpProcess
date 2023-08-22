#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]

use enumflags2::bitflags;
use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};
use strum_macros::EnumString;

/// The current direction the character is facing, can be LEFT, RIGHT, or DOWN*
///
/// *Down is technically only used for warpstar item animation, but it's useful to give it a
/// default value of 0 for stats
#[derive(
    Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive, PartialEq, PartialOrd, EnumString,
)]
#[repr(i8)]
enum Orientation {
    LEFT = -1,
    DOWN = 0,
    RIGHT = 1,
}

/// L cancel status, active for 1 frame upon landing during an aerial attack, which indicates
/// either SUCCESS or FAILURE. Value is NOT_APPLICABLE at all other times
#[derive(Debug, Clone, Copy, FromPrimitive, IntoPrimitive, PartialEq, PartialOrd, EnumString)]
#[repr(u8)]
enum LCancel {
    #[default]
    NOT_APPLICABLE = 0,
    SUCCESS = 1,
    FAILURE = 2,
}

/// Hurtbox state. Can be VULNERABLE, INVULNERABLE, or INTANGIBLE
#[derive(
    Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive, PartialEq, PartialOrd, EnumString,
)]
#[repr(u8)]
enum Hurtbox {
    VULNERABLE,
    /// Attacks collide with hurtboxes, incurring hitlag but dealing no damage
    INVULNERABLE,
    /// Attacks pass through hurtboxes, incurring no hitlag and dealing no damage
    INTANGIBLE,
}

/// Post-frame bitfield 1
///
/// Known Bits:
/// * Bit 2 - ABSORBER_BUBBLE
/// * Bit 4 - REFLECT_NO_STEAL
/// * Bit 5 - REFLECT_BUBBLE
#[bitflags]
#[derive(
    Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive, PartialEq, PartialOrd, EnumString,
)]
#[repr(u8)]
enum Flags1 {
    BIT_1 = 0b0000_0001,
    /// Active when any absorber hitbox is active (ness down b)
    ABSORBER_BUBBLE = 0b0000_0010,
    BIT_3 = 0b0000_0100,
    /// Active when REFLECT_BUBBLE is active, but the reflected projectile does not change ownership
    ///  (e.g. Mewtwo side b)
    REFLECT_NO_STEAL = 0b0000_1000,
    /// Active when any projectile reflect bubble is active
    REFLECT_BUBBLE = 0b0001_0000,
    BIT_6 = 0b0010_0000,
    BIT_7 = 0b0100_0000,
    BIT_8 = 0b1000_0000,
}

/// Post-frame bitfield 2
///
/// Known Bits:
/// * Bit 3 - SUBACTION_INVULN
/// * BIT 4 - FASTFALL
/// * BIT 5 - DEFENDER_HITLAG
/// * BIT 6 - HITLAG
#[bitflags]
#[derive(
    Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive, PartialEq, PartialOrd, EnumString,
)]
#[repr(u8)]
enum Flags2 {
    BIT_1 = 0b0000_0001,
    BIT_2 = 0b0000_0010,
    /// "Active when a character recieves intangibility or invulnerability due to a subaction, that
    /// is removed when the subaction ends" - per UnclePunch. Little else is known besides this
    /// description.
    SUBACTION_INVULN = 0b0000_0100,
    /// Active when the character is fastfalling
    FASTFALL = 0b0000_1000,
    /// Active when the character is in hitlag, and is the one being hit. Can be thought of as
    /// `CAN_SDI`
    DEFENDER_HITLAG = 0b0001_0000,
    /// Active when the character is in hitlag
    HITLAG = 0b0010_0000,
    BIT_7 = 0b0100_0000,
    BIT_8 = 0b1000_0000,
}

/// Post-frame bitfield 3
///
/// Known Bits:
/// * Bit 3 - GRAB_HOLD
/// * Bit 8 - SHIELDING
#[bitflags]
#[derive(
    Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive, PartialEq, PartialOrd, EnumString,
)]
#[repr(u8)]
enum Flags3 {
    BIT_1 = 0b0000_0001,
    BIT_2 = 0b0000_0010,
    /// Active when the character has grabbed another character and is holding them
    GRAB_HOLD = 0b0000_0100,
    BIT_4 = 0b0000_1000,
    BIT_5 = 0b0001_0000,
    BIT_6 = 0b0010_0000,
    BIT_7 = 0b0100_0000,
    /// Active when the character is shielding
    SHIELDING = 0b1000_0000,
}

/// Post-frame bitfield 4
///
/// Known Bits:
/// * Bit 2 - HITSTUN
/// * Bit 3 - HITBOX_TOUCHING_SHIELD
/// * Bit 6 - POWERSHIELD_BUBBLE
#[bitflags]
#[derive(
    Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive, PartialEq, PartialOrd, EnumString,
)]
#[repr(u8)]
enum Flags4 {
    BIT_1 = 0b0000_0001,
    /// Active when character is in hitstun
    HITSTUN = 0b0000_0010,
    /// Dubious meaning, likely related to subframe events (per UnclePunch). Very little is known
    /// besides offhand remarks
    HITBOX_TOUCHING_SHIELD = 0b0000_0100,
    BIT_4 = 0b0000_1000,
    BIT_5 = 0b0001_0000,
    /// Active when character's physical OR projectile Powershield bubble is active
    PWERSHIELD_BUBBLE = 0b0010_0000,
    BIT_7 = 0b0100_0000,
    BIT_8 = 0b1000_0000,
}

/// Post-frame bitfield 5
///
/// Known Bits:
/// * Bit 2 - CLOAKING_DEVICE
/// * Bit 4 - FOLLOWER
/// * Bit 5 - INACTIVE
/// * Bit 7 - DEAD
/// * Bit 8 - OFFSCREEN
#[bitflags]
#[derive(
    Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive, PartialEq, PartialOrd, EnumString,
)]
#[repr(u8)]
enum Flags5 {
    BIT_1 = 0b0000_0001,
    /// Active when character is invisible due to cloaking device item/special mode toggle
    CLOAKING_DEVICE = 0b0000_0010,
    BIT_3 = 0b0000_0100,
    /// Active when character is follower-type (e.g. Nana)
    FOLLOWER = 0b0000_1000,
    /// Character is not processed. Corresponds to Action State `Sleep` (not to be confused with
    /// `FURA_SLEEP` and `DAMAGE_SLEEP`)
    ///
    /// This is typically only relevant for shiek/zelda, and in doubles. When shiek is active, zelda
    /// will have this flag active (and vice versa). When a doubles teammate has 0 stocks, this flag
    /// is active as well.
    ///
    /// IMPORTANT: If this flag is active in a replay, something has gone horribly wrong. This is
    /// the bit checked to determine whether or not slippi records a frame event for the character
    INACTIVE = 0b0001_0000,
    BIT_6 = 0b0010_0000,
    /// Active when character is dead
    DEAD = 0b0100_0000,
    /// Active when character is in the magnifying glass
    OFFSCREEN = 0b1000_0000,
}