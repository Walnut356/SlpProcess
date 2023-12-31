#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_upper_case_globals)]

use num_enum::{FromPrimitive, IntoPrimitive};
use strum_macros::{Display, EnumString, IntoStaticStr};

/// Attack values as they appear in the stale move queue, and the player's last_attack_landed field.
#[derive(
    Debug,
    Clone,
    Copy,
    FromPrimitive,
    IntoPrimitive,
    PartialEq,
    PartialOrd,
    EnumString,
    Display,
    IntoStaticStr,
)]
#[repr(u8)]
pub enum Attack {
    #[default]
    NONE = 0,
    /// Any attack that does not register as a move in the stale move queue
    NON_STALING = 1,
    JAB_1 = 2,
    JAB_2 = 3,
    JAB_3 = 4,
    RAPID_JAB = 5,
    DASH_ATTACK = 6,
    F_TILT = 7,
    U_TILT = 8,
    D_TILT = 9,
    F_SMASH = 10,
    U_SMASH = 11,
    D_SMASH = 12,
    NAIR = 13,
    FAIR = 14,
    BAIR = 15,
    UAIR = 16,
    DAIR = 17,
    NEUTRAL_SPECIAL = 18,
    SIDE_SPECIAL = 19,
    UP_SPECIAL = 20,
    DOWN_SPECIAL = 21,
    KIRBY_HAT_MARIO = 22,
    KIRBY_HAT_FOX = 23,
    KIRBY_HAT_CFALCON = 24,
    KIRBY_HAT_DKNEUTRAL_SPECIAL = 25,
    KIRBY_HAT_BOWSER = 26,
    KIRBY_HAT_LINK = 27,
    KIRBY_HAT_SHEIK = 28,
    KIRBY_HAT_NESS = 29,
    KIRBY_HAT_PEACH = 30,
    KIRBY_HAT_ICE_CLIMBER = 31,
    KIRBY_HAT_PIKACHU = 32,
    KIRBY_HAT_SAMUS = 33,
    KIRBY_HAT_YOSHI = 34,
    KIRBY_HAT_JIGGLYPUFF = 35,
    KIRBY_HAT_MEWTWO = 36,
    KIRBY_HAT_LUIGI = 37,
    KIRBY_HAT_MARTH = 38,
    KIRBY_HAT_ZELDA = 39,
    KIRBY_HAT_YOUNG_LINK = 40,
    KIRBY_HAT_DOC = 41,
    KIRBY_HAT_FALCO = 42,
    KIRBY_HAT_PICHU = 43,
    KIRBY_HAT_GAME_AND_WATCH = 44,
    KIRBY_HAT_GANON = 45,
    KIRBY_HAT_ROY = 46,
    /// Getup attack when lying belly up
    GET_UP_ATTACK_BACK = 50,
    /// Getup attack when lying belly down
    GET_UP_ATTACK_FRONT = 51,
    PUMMEL = 52,
    FORWARD_THROW = 53,
    BACK_THROW = 54,
    UP_THROW = 55,
    DOWN_THROW = 56,
    CARGO_FORWARD_THROW = 57,
    CARGO_BACK_THROW = 58,
    CARGO_UP_THROW = 59,
    CARGO_DOWN_THROW = 60,
    /// Getup attack from ledge when percent >= 100
    LEDGE_ATTACK_SLOW = 61,
    /// Getup attack from ledge when percent < 100
    LEDGE_ATTACK_FAST = 62,
    BEAM_SWORD_JAB = 63,
    BEAM_SWORD_TILT_SWING = 64,
    BEAM_SWORD_SMASH_SWING = 65,
    BEAM_SWORD_DASH_SWING = 66,
    HOME_RUN_BAT_JAB = 67,
    HOME_RUN_BAT_TILT_SWING = 68,
    HOME_RUN_BAT_SMASH_SWING = 69,
    HOME_RUN_BAT_DASH_SWING = 70,
    PARASOL_JAB = 71,
    PARASOL_TILT_SWING = 72,
    PARASOL_SMASH_SWING = 73,
    PARASOL_DASH_SWING = 74,
    FAN_JAB = 75,
    FAN_TILT_SWING = 76,
    FAN_SMASH_SWING = 77,
    FAN_DASH_SWING = 78,
    STAR_ROD_JAB = 79,
    STAR_ROD_TILT_SWING = 80,
    STAR_ROD_SMASH_SWING = 81,
    STAR_ROD_DASH_SWING = 82,
    LIPS_STICK_JAB = 83,
    LIPS_STICK_TILT_SWING = 84,
    LIPS_STICK_SMASH_SWING = 85,
    LIPS_STICK_DASH_SWING = 86,
    OPEN_PARASOL = 87,
    RAY_GUN_SHOOT = 88,
    FIRE_FLOWER_SHOOT = 89,
    SCREW_ATTACK = 90,
    SUPER_SCOPE_RAPID = 91,
    SUPER_SCOPE_CHARGED = 92,
    HAMMER = 93,
}

impl Default for Attack {
    #[inline]
    fn default() -> Self {
        Self::NONE
    }
}
