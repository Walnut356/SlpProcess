#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]

use anyhow::{anyhow, Result};
use strum_macros::{Display, EnumString, FromRepr, IntoStaticStr};

use super::ActionState as AS;

/// Ports P1-P4. Can be converted to the 0-indexed u8 value via `as u8`
#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    FromRepr,
    Default,
    EnumString,
    IntoStaticStr,
    Display,
    Hash,
)]
#[repr(u8)]
pub enum Port {
    #[default]
    P1,
    P2,
    P3,
    P4,
}

impl TryFrom<i8> for Port {
    fn try_from(val: i8) -> Result<Self> {
        match val {
            0 => Ok(Port::P1),
            1 => Ok(Port::P2),
            2 => Ok(Port::P3),
            3 => Ok(Port::P4),
            _ => Err(anyhow!(
                "Unable to convert i8 {val} into Port, expected value 0-3"
            )),
        }
    }

    type Error = anyhow::Error;
}

/// The current direction the character is facing, can be LEFT, RIGHT, or DOWN*. Impl's TryFrom<f32>
/// as melee stores this value as a float for some reason
///
/// *Down is technically only used for warpstar item animation, but it's useful to give it a
/// default value of 0 for stats
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, EnumString, Display, FromRepr, IntoStaticStr,
)]
#[repr(i8)]
pub enum Orientation {
    LEFT = -1,
    DOWN = 0,
    RIGHT = 1,
}

impl TryFrom<f32> for Orientation {
    type Error = anyhow::Error;

    fn try_from(value: f32) -> std::prelude::v1::Result<Self, Self::Error> {
        if value == -1.0 {
            Ok(Self::LEFT)
        } else if value == 0.0 {
            Ok(Self::DOWN)
        } else if value == 1.0 {
            Ok(Self::RIGHT)
        } else {
            Err(anyhow!(
                "Cannot construct orientation from value {value}. Expected -1.0, 0.0, or 1.0"
            ))
        }
    }
}

/// L cancel status, active for 1 frame upon landing during an aerial attack, which indicates
/// either SUCCESS or FAILURE. Value is NOT_APPLICABLE at all other times
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, EnumString, Display, FromRepr, IntoStaticStr,
)]
#[repr(u8)]
pub enum LCancelState {
    NOT_APPLICABLE = 0,
    SUCCESS = 1,
    FAILURE = 2,
}

/// Hurtbox state. Can be VULNERABLE, INVULNERABLE, or INTANGIBLE
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, EnumString, Display, FromRepr, IntoStaticStr,
)]
#[repr(u8)]
pub enum Hurtbox {
    VULNERABLE,
    /// Attacks collide with hurtboxes, incurring hitlag but dealing no damage
    INVULNERABLE,
    /// Attacks pass through hurtboxes, incurring no hitlag and dealing no damage
    INTANGIBLE,
}

/// Post-frame bitfield 1
///
/// Known Bits:
/// * Bit 2 - ABSORB_BUBBLE
/// * Bit 4 - REFLECT_NO_STEAL
/// * Bit 5 - REFLECT_BUBBLE
/// * Bit 7 - ALLOW_INTERRUPT
///
/// Post-frame bitfield 2
///
/// Known Bits:
/// * Bit 3 - SUBACTION_INVULN
/// * BIT 4 - FASTFALL
/// * BIT 5 - DEFENDER_HITLAG
/// * BIT 6 - HITLAG
///
/// Post-frame bitfield 3
///
/// Known Bits:
/// * Bit 3 - GRAB_HOLD
/// * Bit 8 - SHIELDING
///
/// Post-frame bitfield 4
///
/// Known Bits:
/// * Bit 2 - HITSTUN
/// * Bit 3 - HITBOX_TOUCHING_SHIELD
/// * Bit 6 - POWERSHIELD_BUBBLE
///
/// Post-frame bitfield 5
///
/// Known Bits:
/// * Bit 2 - CLOAKING_DEVICE
/// * Bit 4 - FOLLOWER
/// * Bit 5 - INACTIVE
/// * Bit 7 - DEAD
/// * Bit 8 - OFFSCREEN
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u64)]
pub enum Flags {
    None = 0,
    BIT_1_1 = 1 << 0,
    /// Active when any absorber hitbox is active (ness down b)
    ABSORB_BUBBLE = 1 << 1,
    /// "Setting this to 1 causes reflector to skip ownership change". Not sure how this differs
    /// from REFLECT_NO_STEAL
    BIT_1_3 = 1 << 2,
    /// Active when REFLECT_BUBBLE is active, but the reflected projectile does not change ownership
    /// (e.g. Mewtwo side b)
    REFLECT_NO_STEAL = 1 << 3,
    /// Active when any projectile reflect bubble is active
    REFLECT_BUBBLE = 1 << 4,
    /// Seems like the bit that determines if the character goes for jab 2 or jab 3
    BIT_1_6 = 1 << 5,
    /// Seems like the bit that determines if the character goes for jab 1 or jab2/3
    BIT_1_7 = 1 << 6,
    /// Active when the player is able to interrupt their current state (e.g. IASA, wait, fall)
    ALLOW_INTERRUPT = 1 << 7,
    /// Active when shielding, likely always accompanied by GUARD_BUBBLE
    SHIELDING = 1 << 8,
    /// Related to being curled up? Active when inside a yoshi egg via yoshi's neutral B, active for
    /// yoshi when he is shielding, and also active for samus when she is in her morph ball
    BIT_2_2 = 1 << 9,
    /// "Active when a character recieves intangibility or invulnerability due to a subaction that
    /// is removed when the subaction ends" - per UnclePunch. Little else is known besides this
    /// description.
    SUBACTION_INVULN = 1 << 10,
    /// Active when the character is fastfalling
    FASTFALL = 1 << 11,
    /// Active when the character is in hitlag, and is the one being hit. Can be thought of as
    /// `CAN_SDI`
    DEFENDER_HITLAG = 1 << 12,
    /// Active when the character is in hitlag
    HITLAG = 1 << 13,
    BIT_2_7 = 1 << 14,
    BIT_2_8 = 1 << 15,
    /// Active when attached to another character (e.g. both characters during falcon up-b hug)
    BIT_3_1 = 1 << 16,
    BIT_3_2 = 1 << 17,
    /// Active when the character has grabbed another character and is holding them
    GRAB_HOLD = 1 << 18,
    BIT_3_4 = 1 << 19,
    BIT_3_5 = 1 << 20,
    BIT_3_6 = 1 << 21,
    BIT_3_7 = 1 << 22,
    /// Active whenever a character has a guard bubble. This includes shielding, as well as
    /// marth/roy/peach counters
    GUARD_BUBBLE = 1 << 23,
    BIT_4_1 = 1 << 24,
    /// Active when character is in hitstun
    HITSTUN = 1 << 25,
    /// Dubious meaning, likely related to subframe events (per UnclePunch). This bit is relevant when
    /// peach side B hits a guarding opponent. If the guarding opponent is marth, roy, or peach and
    /// that opponent is using their counter, peach will be forced into the regular side B end animation
    /// instead of the smash side B end animation
    HITBOX_TOUCHING_SHIELD = 1 << 26,
    BIT_4_4 = 1 << 27,
    BIT_4_5 = 1 << 28,
    /// Active when character's physical OR projectile Powershield bubble is active
    POWERSHIELD_BUBBLE = 1 << 29,
    BIT_4_7 = 1 << 30,
    BIT_4_8 = 1 << 31,
    BIT_5_1 = 1 << 32,
    /// Active when character is invisible due to cloaking device item/special mode toggle
    CLOAKING_DEVICE = 1 << 33,
    BIT_5_3 = 1 << 34,
    /// Active when character is follower-type (e.g. Nana)
    FOLLOWER = 1 << 35,
    /// Character is not processed. Corresponds to Action State `Sleep` (not to be confused with
    /// `FURA_SLEEP` and `DAMAGE_SLEEP`)
    ///
    /// This is typically only relevant for shiek/zelda, and in doubles. When shiek is active, zelda
    /// will have this flag active (and vice versa). When a doubles teammate has 0 stocks, this flag
    /// is active as well.
    ///
    /// IMPORTANT: If this flag is active in a replay, something has gone horribly wrong. This is
    /// the bit checked to determine whether or not slippi records a frame event for the character
    INACTIVE = 1 << 36,
    BIT_5_6 = 1 << 37,
    /// Active when character is dead
    DEAD = 1 << 38,
    /// Active when character is in the magnifying glass
    OFFSCREEN = 1 << 39,
    Raw(u64),
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, EnumString, Display, FromRepr, IntoStaticStr,
)]
pub enum TechType {
    TECH_IN_PLACE,
    TECH_LEFT,
    TECH_RIGHT,
    GET_UP_ATTACK,
    MISSED_TECH,
    WALL_TECH,
    MISSED_WALL_TECH,
    WALL_JUMP_TECH,
    CEILING_TECH,
    MISSED_CEILING_TECH,
    JAB_RESET,
    MISSED_TECH_GET_UP,
    MISSED_TECH_ROLL_LEFT,
    MISSED_TECH_ROLL_RIGHT,
}

impl TechType {
    pub fn from_state(state: u16, direction: i8) -> Option<Self> {
        let a_state = AS::from_repr(state);

        match a_state? {
            AS::PASSIVE => Some(TechType::TECH_IN_PLACE),
            AS::DOWN_STAND_U | AS::DOWN_STAND_D => Some(TechType::MISSED_TECH_GET_UP),
            AS::PASSIVE_STAND_F => match direction > 0 {
                true => Some(TechType::TECH_RIGHT),
                false => Some(TechType::TECH_LEFT),
            },
            AS::DOWN_FOWARD_U | AS::DOWN_FOWARD_D => match direction > 0 {
                true => Some(TechType::MISSED_TECH_ROLL_RIGHT),
                false => Some(TechType::MISSED_TECH_ROLL_LEFT),
            },
            AS::PASSIVE_STAND_B => match direction > 0 {
                true => Some(TechType::TECH_LEFT),
                false => Some(TechType::TECH_RIGHT),
            },
            AS::DOWN_BACK_U | AS::DOWN_BACK_D => match direction > 0 {
                true => Some(TechType::MISSED_TECH_ROLL_LEFT),
                false => Some(TechType::MISSED_TECH_ROLL_RIGHT),
            },
            AS::DOWN_ATTACK_U | AS::DOWN_ATTACK_D => Some(TechType::GET_UP_ATTACK),
            AS::DOWN_BOUND_U
            | AS::DOWN_BOUND_D
            | AS::DOWN_WAIT_U
            | AS::DOWN_WAIT_D
            | AS::DOWN_REFLECT => Some(TechType::MISSED_TECH),
            AS::DOWN_DAMAGE_U | AS::DOWN_DAMAGE_D => Some(TechType::JAB_RESET),
            AS::PASSIVE_WALL => Some(TechType::WALL_TECH),
            AS::PASSIVE_WALL_JUMP => Some(TechType::WALL_JUMP_TECH),
            AS::PASSIVE_CEIL => Some(TechType::CEILING_TECH),
            AS::FLY_REFLECT_CEIL => Some(TechType::MISSED_CEILING_TECH),
            AS::FLY_REFLECT_WALL => Some(TechType::MISSED_WALL_TECH),
            _ => None,
        }
    }
}
