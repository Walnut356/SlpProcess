#![allow(non_camel_case_types)]

use strum_macros::{Display, EnumString, FromRepr, IntoStaticStr};
use anyhow::{anyhow, Result};

use crate::enums::Character;

/// Wrapper enum for ActionState, CharacterState, and any possibly unknown values. Mainly useful via
/// `State::from_state_and_char`
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
#[repr(u16)]
pub enum State {
    Universal(ActionState),
    Unique(CharacterState),
    Unknown(u16),
}

impl State {
    pub fn from_state_and_char(state: u16, character: Option<Character>) -> Self {
        if state < 341 {
            Self::Universal(ActionState::from_repr(state).unwrap())
        } else if let Some(c) = character {
            match CharacterState::from_char_and_state(c, state) {
                Ok(x) => Self::Unique(x),
                Err(_) => Self::Unknown(state),
            }
        } else {
            Self::Unknown(state)
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Universal(x) => write!(f, "Universal({})", x),
            State::Unique(x) => write!(f, "Unique({})", x),
            State::Unknown(x) => write!(f, "Unknown({})", x),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::Unknown(u16::MAX)
    }
}

impl From<State> for &'static str {
    fn from(value: State) -> Self {
        match value {
            State::Universal(x) => x.into(),
            State::Unique(x) => x.into(),
            State::Unknown(_) => "Unknown",
        }
    }
}

impl From<State> for u16 {
    fn from(value: State) -> Self {
        match value {
            State::Universal(x) => x as u16,
            State::Unique(x) => x as u16,
            State::Unknown(x) => x,
        }
    }
}

/// Individual Action State IDs. See ActionRange for state ranges.
///
/// ID's match debug mode names, see docstrings for additional context
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, EnumString, IntoStaticStr, Display, FromRepr,
)]
#[repr(u16)]
pub enum ActionState {
    NONE = u16::MAX,
    /// Bottom blast zone death
    DEAD_DOWN = 0,
    /// Left blast zone death
    DEAD_LEFT = 1,
    /// Right blast zone death
    DEAD_RIGHT = 2,
    /// Top blast zone death **used in 1P** "Team Kirby", etc.
    /// See `DEAD_UP_STAR` and `DEAD_UP_FALL_*` for vs mode upward kills
    DEAD_UP = 3,
    /// Standard star KO
    DEAD_UP_STAR = 4,
    /// Star KO while encased in ice
    DEAD_UP_STAR_ICE = 5,
    /// 64-esque front fall, likely unused per OG Modders
    DEAD_UP_FALL = 6,

    DEAD_UP_FALL_HIT_CAMERA = 7,
    DEAD_UP_FALL_HIT_CAMERA_FLAT = 8,
    DEAD_UP_FALL_ICE = 9,
    DEAD_UP_FALL_HIT_CAMERA_ICE = 10,
    /// "Nothing" state - used as sheik/zelda state when the other is active, when a doubles teammate is out of stocks,
    /// or for nana when she is dead.
    SLEEP = 11,
    /// Entering on halo
    REBIRTH = 12,
    /// Waiting on halo
    REBIRTH_WAIT = 13,
    /// Default standing state
    WAIT = 14,

    // ---------------------------------------------- Generic Movement ---------------------------------------------- //
    WALK_SLOW = 15,
    WALK_MIDDLE = 16,
    WALK_FAST = 17,
    TURN = 18,
    /// Slow sliding turnaround when in full run
    TURN_RUN = 19,
    DASH = 20,
    RUN = 21,
    RUN_DIRECT = 22,
    RUN_BRAKE = 23,
    /// Jumpsquat
    KNEE_BEND = 24,
    /// First jump, forward
    JUMP_F = 25,
    /// First jump, backwards
    JUMP_B = 26,
    /// Aerial jump forward
    JUMP_AERIAL_F = 27,
    /// Aerial jump backward
    JUMP_AERIAL_B = 28,
    /// Default fall with no drift
    FALL = 29,
    /// Fall with forward drift
    FALL_F = 30,
    /// Fall with backwards drift
    FALL_B = 31,
    /// Fall after second jump with no drift
    FALL_AERIAL = 32,
    /// Fall after second jump, forward drift
    FALL_AERIAL_F = 33,
    /// Fall after second jump, backward DI
    FALL_AERIAL_B = 34,
    /// Non-actionable fall used after Up B, air dodge, and some B moves
    FALL_SPECIAL = 35,
    /// Non-actionable fall, forward drift
    FALL_SPECIAL_F = 36,
    /// Non-actionable fall, backward drift
    FALL_SPECIAL_B = 37,
    /// Tumble
    DAMAGE_FALL = 38,
    /// Transition state from any standing state to `SQUAT_WAIT`
    SQUAT = 39,
    /// Full crouch
    SQUAT_WAIT = 40,
    /// Transition state from `SQUAT_WAIT` to `WAIT`
    SQUAT_RV = 41,
    /// Universal no-action landing lag, fully interruptable after 4 frames
    LAND = 42,
    /// Landing from FALL_SPECIAL(_F/B)
    LAND_FALL_SPECIAL = 43,

    // --------------------------------------------------- Attacks -------------------------------------------------- //
    /// Jab 1
    ATTACK_11 = 44,
    /// Jab 2
    ATTACK_12 = 45,
    /// Jab 3
    ATTACK_13 = 46,
    /// Rapid jab start
    ATTACK_100_START = 47,
    /// Rapid jab loop
    ATTACK_100_LOOP = 48,
    /// Rapid jab end
    ATTACK_100_END = 49,
    /// Dash attack
    ATTACK_DASH = 50,
    /// Up-angled Ftilt
    ATTACK_S_3_HI = 51,
    /// Slight up-angled F-tilt
    ATTACK_S_3_HI_S = 52,
    /// No angle Ftilt
    ATTACK_S_3_S = 53,
    /// Slight down-angled Ftilt
    ATTACK_S_3_LW_S = 54,
    /// Down-angled Ftilt
    ATTACK_S_3_LW = 55,
    /// Utilt
    ATTACK_HI_3 = 56,
    /// Dtilt
    ATTACK_LW_3 = 57,
    /// Up-angled Fsmash
    ATTACK_S_4_HI = 58,
    /// Slight up-angled Fsmash
    ATTACK_S_4_HI_S = 59,
    /// No angle Fsmash
    ATTACK_S_4_S = 60,
    /// Slight down-angled Fsmash
    ATTACK_S_4_LW_S = 61,
    /// Down-angled Fsmash
    ATTACK_S_4_LW = 62,
    /// Usmash
    ATTACK_HI_4 = 63,
    /// Dsmash
    ATTACK_LW_4 = 64,
    /// Nair
    ATTACK_AIR_N = 65,
    /// Fair
    ATTACK_AIR_F = 66,
    /// Bair
    ATTACK_AIR_B = 67,
    /// Uair
    ATTACK_AIR_HI = 68,
    /// Dair
    ATTACK_AIR_LW = 69,
    /// Nair landing animation
    LANDING_AIR_N = 70,
    /// Fair landing animation
    LANDING_AIR_F = 71,
    /// Bair landing animation
    LANDING_AIR_B = 72,
    /// Uair landing animation
    LANDING_AIR_HI = 73,
    /// Dair landing animation
    LANDING_AIR_LW = 74,

    // ----------------------------------------------- Generic Damage ----------------------------------------------- //
    DAMAGE_HI_1 = 75,
    DAMAGE_HI_2 = 76,
    DAMAGE_HI_3 = 77,
    DAMAGE_N_1 = 78,
    DAMAGE_N_2 = 79,
    DAMAGE_N_3 = 80,
    DAMAGE_LW_1 = 81,
    DAMAGE_LW_2 = 82,
    DAMAGE_LW_3 = 83,
    DAMAGE_AIR_1 = 84,
    DAMAGE_AIR_2 = 85,
    DAMAGE_AIR_3 = 86,
    DAMAGE_FLY_HI = 87,
    DAMAGE_FLY_N = 88,
    DAMAGE_FLY_LW = 89,
    DAMAGE_FLY_TOP = 90,
    DAMAGE_FLY_ROLL = 91,

    // ------------------------------------------------ Generic Item ------------------------------------------------ //
    /// Picking up most items
    LIGHT_GET = 92,
    /// Picking up heavy items (Barrel)
    HEAVY_GET = 93,
    /// Start of item throw
    LIGHT_THROW_F = 94,
    LIGHT_THROW_B = 95,
    LIGHT_THROW_HI = 96,
    LIGHT_THROW_LW = 97,
    LIGHT_THROW_DASH = 98,
    LIGHT_THROW_DROP = 99,
    LIGHT_THROW_AIR_F = 100,
    LIGHT_THROW_AIR_B = 101,
    LIGHT_THROW_AIR_HI = 102,
    LIGHT_THROW_AIR_LW = 103,
    HEAVY_THROW_F = 104,
    HEAVY_THROW_B = 105,
    HEAVY_THROW_HI = 106,
    HEAVY_THROW_LW = 107,
    LIGHT_THROW_F_4 = 108,

    // Smash throw start
    LIGHT_THROW_B_4 = 109,
    LIGHT_THROW_HI_4 = 110,
    LIGHT_THROW_LW_4 = 111,
    LIGHT_THROW_AIR_F_4 = 112,
    LIGHT_THROW_AIR_B_4 = 113,
    LIGHT_THROW_AIR_HI_4 = 114,
    LIGHT_THROW_AIR_LW_4 = 115,
    HEAVY_THROW_F_4 = 116,
    HEAVY_THROW_B_4 = 117,
    HEAVY_THROW_HI_4 = 118,
    HEAVY_THROW_LW_4 = 119,

    // End of item throw

    // ------------------------------------------------ Item-Specific ----------------------------------------------- //
    /// Beam Sword swing
    SWORD_SWING_1 = 120,
    /// Beam Sword swing
    SWORD_SWING_3 = 121,
    /// Beam Sword swing
    SWORD_SWING_4 = 122,
    /// Beam Sword swing
    SWORD_SWING_DASH = 123,
    BAT_SWING_1 = 124,
    BAT_SWING_3 = 125,
    BAT_SWING_4 = 126,
    BAT_SWING_DASH = 127,
    PARASOL_SWING_1 = 128,
    PARASOL_SWING_3 = 129,
    PARASOL_SWING_4 = 130,
    PARASOL_SWING_DASH = 131,
    HARISEN_SWING_1 = 132,
    HARISEN_SWING_3 = 133,
    HARISEN_SWING_4 = 134,
    HARISEN_SWING_DASH = 135,
    STAR_ROD_SWING_1 = 136,
    STAR_ROD_SWING_3 = 137,
    STAR_ROD_SWING_4 = 138,
    STAR_ROD_SWING_DASH = 139,
    LIP_STICK_SWING_1 = 140,
    LIP_STICK_SWING_3 = 141,
    LIP_STICK_SWING_4 = 142,
    LIP_STICK_SWING_DASH = 143,
    ITEM_PARASOL_OPEN = 144,
    ITEM_PARASOL_FALL = 145,
    ITEM_PARASOL_FALL_SPECIAL = 146,
    ITEM_PARASOL_DAMAGE_FALL = 147,
    L_GUN_SHOOT = 148,
    L_GUN_SHOOT_AIR = 149,
    L_GUN_SHOOT_EMPTY = 150,
    L_GUN_SHOOT_AIR_EMPTY = 151,
    FIRE_FLOWER_SHOOT = 152,
    FIRE_FLOWER_SHOOT_AIR = 153,
    ITEM_SCREW = 154,
    ITEM_SCREW_AIR = 155,
    DAMAGE_SCREW = 156,
    DAMAGE_SCREW_AIR = 157,
    ITEM_SCOPE_START = 158,
    ITEM_SCOPE_RAPID = 159,
    ITEM_SCOPE_FIRE = 160,
    ITEM_SCOPE_END = 161,
    ITEM_SCOPE_AIR_START = 162,
    ITEM_SCOPE_AIR_RAPID = 163,
    ITEM_SCOPE_AIR_FIRE = 164,
    ITEM_SCOPE_AIR_END = 165,
    ITEM_SCOPE_START_EMPTY = 166,
    ITEM_SCOPE_RAPID_EMPTY = 167,
    ITEM_SCOPE_FIRE_EMPTY = 168,
    ITEM_SCOPE_END_EMPTY = 169,
    ITEM_SCOPE_AIR_START_EMPTY = 170,
    ITEM_SCOPE_AIR_RAPID_EMPTY = 171,
    ITEM_SCOPE_AIR_FIRE_EMPTY = 172,
    ITEM_SCOPE_AIR_END_EMPTY = 173,

    LIFT_WAIT = 174,
    LIFT_WALK_1 = 175,
    LIFT_WALK_2 = 176,
    LIFT_TURN = 177,

    // --------------------------------------------------- Shield --------------------------------------------------- //

    // Raising shield
    GUARD_ON = 178,
    /// Holding shield
    GUARD = 179,
    /// Releasing shield, 15 frames, but can be interrupted by jumping.
    GUARD_OFF = 180,
    /// Shield stun
    GUARD_SET_OFF = 181,
    /// Powershield
    GUARD_REFLECT = 182,

    // ------------------------------------------------- Tech States ------------------------------------------------ //
    /// Missed tech bounce, facing upwards
    DOWN_BOUND_U = 183,
    /// Downed, facing up
    DOWN_WAIT_U = 184,
    /// Jab reset while laying facing up
    DOWN_DAMAGE_U = 185,
    /// Neutral getup, facing up
    DOWN_STAND_U = 186,
    /// Getup attack, facing up
    DOWN_ATTACK_U = 187,
    /// Missed tech roll forward
    DOWN_FOWARD_U = 188,
    /// Missed tech roll backward
    DOWN_BACK_U = 189,
    /// Does not appear to be used
    DOWN_SPOT_U = 190,
    /// Missed tech bounce, facing down
    DOWN_BOUND_D = 191,
    /// Downed, facing down
    DOWN_WAIT_D = 192,
    /// Hit while laying on ground, facing down
    DOWN_DAMAGE_D = 193,
    /// Neutral getup, facing down
    DOWN_STAND_D = 194,
    /// Getup attack, facing down
    DOWN_ATTACK_D = 195,
    /// Missed tech roll forward
    DOWN_FOWARD_D = 196,
    /// Missed tech roll backward
    DOWN_BACK_D = 197,
    /// Does not appear to be used
    DOWN_SPOT_D = 198,
    /// Neutral tech
    PASSIVE = 199,
    /// Forward tech
    PASSIVE_STAND_F = 200,
    /// Backward tech
    PASSIVE_STAND_B = 201,
    /// Wall tech
    PASSIVE_WALL = 202,
    /// Walljump and Walljump tech
    PASSIVE_WALL_JUMP = 203,
    /// Ceiling tech
    PASSIVE_CEIL = 204,

    // ------------------------------------------------ Shield Break ------------------------------------------------ //
    /// Initial bounce when shield is broken
    SHIELD_BREAK_FLY = 205,
    /// Fall after `SHIELD_BREAK_FLY`
    SHIELD_BREAK_FALL = 206,

    SHIELD_BREAK_DOWN_U = 207,
    SHIELD_BREAK_DOWN_D = 208,
    SHIELD_BREAK_STAND_U = 209,
    SHIELD_BREAK_STAND_D = 210,
    /// Shield break totter
    FURA_FURA = 211,

    // ---------------------------------------------------- Grab ---------------------------------------------------- //
    /// Grab
    CATCH = 212,
    /// Successful grab, pulling opponent in
    CATCH_PULL = 213,
    /// Dash grab
    CATCH_DASH = 214,
    /// Successful dash grab, pulling opponent in
    CATCH_DASH_PULL = 215,
    /// Grab hold
    CATCH_WAIT = 216,
    /// Pummel
    CATCH_ATTACK = 217,
    /// Grab release
    CATCH_CUT = 218,
    /// Fthrow
    THROW_F = 219,
    /// Bthrow
    THROW_B = 220,
    /// Uthrow
    THROW_HI = 221,
    /// Dthrow
    THROW_LW = 222,

    /// Being grabbed and pulled
    CAPTURE_PULLED_HI = 223,
    /// Grabbed and held
    CAPTURE_WAIT_HI = 224,
    /// Pummeled
    CAPTURE_DAMAGE_HI = 225,
    /// Being grabbed and pulled
    CAPTURE_PULLED_LW = 226,
    /// Grabbed and held
    CAPTURE_WAIT_LW = 227,
    /// Pummeled
    CAPTURE_DAMAGE_LW = 228,
    /// Grab release
    CAPTURE_CUT = 229,
    /// Jumping mash out
    CAPTURE_JUMP = 230,
    /// Does not appear to be used
    CAPTURE_NECK = 231,
    /// Does not appear to be used
    CAPTURE_FOOT = 232,

    // ------------------------------------------------- Dodge/Roll ------------------------------------------------- //
    /// Shield roll forward
    ESCAPE_F = 233,
    /// Shield roll backward
    ESCAPE_B = 234,
    /// Spot dodge
    ESCAPE = 235,
    /// Airdodge
    ESCAPE_AIR = 236,

    REBOUND_STOP = 237,
    REBOUND = 238,

    // --------------------------------------------------- Thrown --------------------------------------------------- //
    /// Being Fthrown
    THROWN_F = 239,
    /// Being Bthrown
    THROWN_B = 240,
    /// Being Uthrown
    THROWN_HI = 241,
    /// Being Dthrown
    THROWN_LW = 242,
    /// BeingDthrown
    THROWN_LW_WOMEN = 243,

    // ---------------------------------------------- Wall/Edge/Ceiling --------------------------------------------- //
    /// Drop through platform
    PASS = 244,
    /// Ledge teeter
    OTTOTTO = 245,
    /// Teeter loop?
    OTTOTTO_WAIT = 246,
    /// Missed wall tech
    FLY_REFLECT_WALL = 247,
    /// Missed ceiling tech
    FLY_REFLECT_CEIL = 248,
    /// Wall bonk
    STOP_WALL = 249,
    /// Ceiling bonk
    STOP_CEIL = 250,
    /// Backward shield slideoff
    MISS_FOOT = 251,

    // ---------------------------------------------------- Ledge --------------------------------------------------- //
    /// Ledge grab
    CLIFF_CATCH = 252,
    /// Ledge hang
    CLIFF_WAIT = 253,
    /// Regular getup >100%
    CLIFF_CLIMB_SLOW = 254,
    /// Regular getup <100%
    CLIFF_CLIMB_QUICK = 255,
    /// Ledge attack >100%
    CLIFF_ATTACK_SLOW = 256,
    /// Ledge attack <100%
    CLIFF_ATTACK_QUICK = 257,
    /// Ledge roll >100%
    CLIFF_ESCAPE_SLOW = 258,
    /// Ledge roll <100%
    CLIFF_ESCAPE_QUICK = 259,
    /// Ledge jump >100%
    CLIFF_JUMP_SLOW_1 = 260,
    /// Ledge jump >100%
    CLIFF_JUMP_SLOW_2 = 261,
    /// Ledge jump <100%
    CLIFF_JUMP_QUICK_1 = 262,
    /// Ledge jump <100%
    CLIFF_JUMP_QUICK_2 = 263,

    // ---------------------------------------------------- Taunt --------------------------------------------------- //
    /// Taunt facing right
    APPEAL_R = 264,
    /// Taunt facing left
    APPEAL_L = 265,

    // ------------------------------------------- Command Grabs and Misc ------------------------------------------- //
    /// DK cargo carry
    SHOULDERED_WAIT = 266,
    /// DK cargo carry
    SHOULDERED_WALK_SLOW = 267,
    /// DK cargo carry
    SHOULDERED_WALK_MIDDLE = 268,
    /// DK cargo carry
    SHOULDERED_WALK_FAST = 269,
    /// DK cargo carry
    SHOULDERED_TURN = 270,
    /// DK cargo throw
    THROWN_F_F = 271,
    /// DK cargo throw
    THROWN_F_B = 272,
    /// DK cargo throw
    THROWN_F_HI = 273,
    /// DK cargo throw
    THROWN_F_LW = 274,

    /// Falcon up B grab
    CAPTURE_CAPTAIN = 275,
    CAPTURE_YOSHI = 276, // TODO Yoshi neutral B grab victim?
    YOSHI_EGG = 277,     // TODO Yoshi neutral B grab victim in egg?
    /// Bowser side B
    CAPTURE_KOOPA = 278,
    /// Bowser side B
    CAPTURE_DAMAGE_KOOPA = 279,
    /// Bowser side B
    CAPTURE_WAIT_KOOPA = 280,
    /// Bowser side B
    THROWN_KOOPA_F = 281,
    /// Bowser side B
    THROWN_KOOPA_B = 282,
    /// Bowser side B
    CAPTURE_KOOPA_AIR = 283,
    /// Bowser side B
    CAPTURE_DAMAGE_KOOPA_AIR = 284,
    /// Bowser side B
    CAPTURE_WAIT_KOOPA_AIR = 285,
    /// Bowser side B
    THROWN_KOOPA_AIR_F = 286,
    /// Bowser side B
    THROWN_KOOPA_AIR_B = 287,
    /// Kirby succ victim
    CAPTURE_KIRBY = 288,
    /// Kirby succ victim
    CAPTURE_WAIT_KIRBY = 289,
    /// Kirby spit victim
    THROWN_KIRBY_STAR = 290,
    /// Kirby swallow victim
    THROWN_COPY_STAR = 291,
    THROWN_KIRBY = 292,
    BARREL_WAIT = 293, // I think this is used for the barrel on DK jungle 64?

    /// Stuck in ground by DK side b or similar
    BURY = 294,
    /// Stuck in ground by DK side b or similar
    BURY_WAIT = 295,
    /// Stuck in ground by DK side b or similar
    BURY_JUMP = 296,

    /// Put to sleep by Jiggs sing or similar
    DAMAGE_SONG = 297,
    /// Put to sleep by Jiggs sing or similar
    DAMAGE_SONG_WAIT = 298,
    /// Put to sleep by Jiggs sing or similar
    DAMAGE_SONG_RV = 299,

    /// Hit by Mewtwo disable
    DAMAGE_BIND = 300,
    /// Does not appear to be used
    CAPTURE_MEWTWO = 301,
    /// Does not appear to be used
    CAPTURE_MEWTWO_AIR = 302,
    /// Hit by Mewtwo confusion
    THROWN_MEWTWO = 303,
    /// Hit by Mewtwo's confusion in the air
    THROWN_MEWTWO_AIR = 304,

    // --------------------------------------------- More Item-Specific --------------------------------------------- //
    WARP_STAR_JUMP = 305,
    WARP_STAR_FALL = 306,
    HAMMER_WAIT = 307,
    HAMMER_WALK = 308,
    HAMMER_TURN = 309,
    HAMMER_KNEE_BEND = 310,
    HAMMER_FALL = 311,
    HAMMER_JUMP = 312,
    HAMMER_LANDING = 313,
    /// super mushroom
    MUSHROOM_GIANT_START = 314,
    /// super mushroom
    MUSHROOM_GIANT_START_AIR = 315,
    /// super mushroom
    MUSHROOM_GIANT_END = 316,
    /// super mushroom
    MUSHROOM_GIANT_END_AIR = 317,
    /// poison mushroom
    MUSHROOM_SMALL_START = 318,
    /// poison mushroom
    MUSHROOM_SMALL_START_AIR = 319,
    /// poison mushroom
    MUSHROOM_SMALL_END = 320,
    /// poison mushroom
    MUSHROOM_SMALL_END_AIR = 321,

    /// Beginning of the match warp in
    ENTRY = 322,
    /// Beginning of the match warp in
    ENTRY_START = 323,
    /// Beginning of the match warp in
    ENTRY_END = 324,

    DAMAGE_ICE = 325,
    DAMAGE_ICE_JUMP = 326,
    CAPTURE_MASTER_HAND = 327,
    CAPTURE_DAMAGE_MASTER_HAND = 328,
    CAPTURE_WAIT_MASTER_HAND = 329,
    THROWN_MASTER_HAND = 330,
    CAPTURE_KIRBY_YOSHI = 331,
    KIRBY_YOSHI_EGG = 332,
    CAPTURE_REDEAD = 333,
    CAPTURE_LIKE_LIKE = 334,

    /// A very rare action state where the character transitions from a DownBoundU or DownBoundD (missed tech) state
    /// into a wall bounce. This state is not techable and neither is the probable next floor hit.
    /// Most commonly encountered on Pokémon Stadium
    DOWN_REFLECT = 335,

    CAPTURE_CRAZY_HAND = 336,
    CAPTURE_DAMAGE_CRAZY_HAND = 337,
    CAPTURE_WAIT_CRAZY_HAND = 338,
    THROWN_CRAZY_HAND = 339,
    BARREL_CANNON_WAIT = 340,
}

impl PartialEq<u16> for ActionState {
    fn eq(&self, other: &u16) -> bool {
        *self as u16 == *other
    }
}

impl PartialOrd<u16> for ActionState {
    fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
        (*self as u16).partial_cmp(other)
    }
}

impl PartialEq<ActionState> for u16 {
    fn eq(&self, other: &ActionState) -> bool {
        *self == (*other) as u16
    }
}

impl PartialOrd<ActionState> for u16 {
    fn partial_cmp(&self, other: &ActionState) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&(*other as u16))
    }
}

impl AsRef<u16> for ActionState {
    fn as_ref(&self) -> &u16 {
        // using the same trick as bitflags to coerce to a reference since any attempt to extract
        // the value will put it on the stack and we can't return a reference to that
        unsafe { &(*(self as *const ActionState as *const u16)) }
    }
}

impl PartialEq<ActionRange> for ActionState {
    fn eq(&self, other: &ActionRange) -> bool {
        *self as u16 == *other as u16
    }
}

impl PartialOrd<ActionRange> for ActionState {
    fn partial_cmp(&self, other: &ActionRange) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&(*other as u16))
    }
}

/// Used to simplify checks for clusters of action states
///
/// ranges are inclusive, comparisons should be GT/Eq or LT/Eq:
/// ```
/// use ssbm_utils::enums::{ActionRange as AR, ActionState};
/// let x = ActionState::ATTACK_AIR_HI;
/// assert!((AR::AERIAL_ATTACK_START..=AR::AERIAL_ATTACK_END).contains(&x));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, FromRepr)]
#[repr(u16)]
pub enum ActionRange {
    NONE = u16::MAX,
    DAMAGE_START = 75,
    DAMAGE_END = 91,
    CAPTURE_START = 223,
    CAPTURE_END = 232,
    GUARD_START = 178,
    GUARD_END = 182,
    GUARD_BREAK_START = 205,
    GUARD_BREAK_END = 211,
    GROUNDED_CONTROL_START = 14,
    GROUNDED_CONTROL_END = 24,
    LEDGE_ACTION_START = 252,
    LEDGE_ACTION_END = 263,
    SQUAT_START = 39,
    SQUAT_END = 41,
    DOWN_START = 183,
    DOWN_END = 198,
    TECH_START = 199,
    TECH_END = 204,
    DODGE_START = 233,
    DODGE_END = 236,
    DYING_START = 0,
    DYING_END = 10,
    GROUND_ATTACK_START = 44,
    GROUND_ATTACK_END = 64,
    AERIAL_ATTACK_START = 65,
    AERIAL_ATTACK_END = 69,
    FALL_SPECIAL_START = 35,
    FALL_SPECIAL_END = 37,
    AERIAL_LAND_LAG_START = 70,
    AERIAL_LAND_LAG_END = 74,
    ACTIONABLE_AIR_START = 25,
    ACTIONABLE_AIR_END = 34,
    THROWN_START = 239,
    THROWN_END = 243,
    COMMAND_GRAB_RANGE1_START = 266,
    COMMAND_GRAB_RANGE1_END = 304,
    COMMAND_GRAB_RANGE2_START = 327,
    COMMAND_GRAB_RANGE2_END = 338,
}

impl PartialEq<u16> for ActionRange {
    fn eq(&self, other: &u16) -> bool {
        *self as u16 == *other
    }
}

impl PartialOrd<u16> for ActionRange {
    fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
        (*self as u16).partial_cmp(other)
    }
}

impl PartialEq<ActionRange> for u16 {
    fn eq(&self, other: &ActionRange) -> bool {
        *self == (*other) as u16
    }
}

impl PartialOrd<ActionRange> for u16 {
    fn partial_cmp(&self, other: &ActionRange) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&(*other as u16))
    }
}

impl PartialEq<ActionState> for ActionRange {
    fn eq(&self, other: &ActionState) -> bool {
        *self as u16 == *other as u16
    }
}

impl PartialOrd<ActionState> for ActionRange {
    fn partial_cmp(&self, other: &ActionState) -> Option<std::cmp::Ordering> {
        self.partial_cmp(AsRef::<u16>::as_ref(other))
    }
}

// TODO character-specific action states

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, EnumString, IntoStaticStr, Display,
)]
#[repr(u16)]
pub enum CharacterState {
    // Bowser:
    FIRE_BREATH_GROUND_STARTUP,
    FIRE_BREATH_GROUND_LOOP,
    FIRE_BREATH_GROUND_END,
    FIRE_BREATH_AIR_STARTUP,
    FIRE_BREATH_AIR_LOOP,
    FIRE_BREATH_AIR_END,
    KOOPA_KLAW_GROUND,
    KOOPA_KLAW_GROUND_GRAB,
    KOOPA_KLAW_GROUND_PUMMEL,
    KOOPA_KLAW_GROUND_WAIT,
    KOOPA_KLAW_GROUND_THROW_F,
    KOOPA_KLAW_GROUND_THROW_B,
    KOOPA_KLAW_AIR,
    KOOPA_KLAW_AIR_GRAB,
    KOOPA_KLAW_AIR_PUMMEL,
    KOOPA_KLAW_AIR_WAIT,
    KOOPA_KLAW_AIR_THROW_F,
    KOOPA_KLAW_AIR_THROW_B,
    WHIRLING_FORTRESS_GROUND,
    WHIRLING_FORTRESS_AIR,
    BOMB_GROUND_BEGIN,
    BOMB_AIR,
    BOMB_LAND,

    // CaptainFalcon
    FALCON_PUNCH_GROUND,
    FALCON_PUNCH_AIR,
    RAPTOR_BOOST_GROUND,
    RAPTOR_BOOST_GROUND_HIT,
    RAPTOR_BOOST_AIR,
    RAPTOR_BOOST_AIR_HIT,
    /// Up B
    FALCON_DIVE_GROUND,
    /// Up B
    FALCON_DIVE_AIR,
    // Up B
    FALCON_DIVE_CATCH,
    // Up B
    FALCON_DIVE_ENDING,
    FALCON_KICK_GROUND,
    FALCON_KICK_GROUND_ENDING_ON_GROUND,
    FALCON_KICK_AIR,
    FALCON_KICK_AIR_ENDING_ON_GROUND,
    FALCON_KICK_AIR_ENDING_IN_AIR,
    FALCON_KICK_GROUND_ENDING_IN_AIR,
    FALCON_KICK_HIT_WALL,

    // DonkeyKong
    KONG_KARRY_WAIT,
    KONG_KARRY_WALK_SLOW,
    KONG_KARRY_WALK_MIDDLE,
    KONG_KARRY_WALK_FAST,
    KONG_KARRY_TURN,
    KONG_KARRY_KNEE_BEND,
    KONG_KARRY_FALL,
    KONG_KARRY_JUMP,
    KONG_KARRY_LANDING,
    KONG_KARRY_GROUND_THROW_FORWARD,
    KONG_KARRY_GROUND_THROW_BACKWARD,
    KONG_KARRY_GROUND_THROW_UP,
    KONG_KARRY_GROUND_THROW_DOWN,
    KONG_KARRY_AIR_THROW_FORWARD,
    KONG_KARRY_AIR_THROW_BACKWARD,
    KONG_KARRY_AIR_THROW_UP,
    KONG_KARRY_AIR_THROW_DOWN,
    GIANT_PUNCH_GROUND_CHARGE_STARTUP,
    GIANT_PUNCH_GROUND_CHARGE_LOOP,
    GIANT_PUNCH_GROUND_CHARGE_STOP,
    GIANT_PUNCH_GROUND_EARLY_PUNCH,
    GIANT_PUNCH_GROUND_FULL_CHARGE_PUNCH,
    GIANT_PUNCH_AIR_CHARGE_STARTUP,
    GIANT_PUNCH_AIR_CHARGE_LOOP,
    GIANT_PUNCH_AIR_CHARGE_STOP,
    GIANT_PUNCH_AIR_EARLY_PUNCH,
    GIANT_PUNCH_AIR_FULL_CHARGE_PUNCH,
    HEADBUTT_GROUND,
    HEADBUTT_AIR,
    SPINNING_KONG_GROUND,
    SPINNING_KONG_AIR,
    HAND_SLAP_STARTUP,
    HAND_SLAP_LOOP,
    HAND_SLAP_END,

    // DrMario
    TAUNT_R,
    PILL_GROUND,
    PILL_AIR,
    SUPER_SHEET_GROUND,
    SUPER_SHEET_AIR,
    SUPER_JUMP_PUNCH_GROUND,
    SUPER_JUMP_PUNCH_AIR,
    TORNADO_GROUND,
    TORNADO_AIR,

    // Falco
    BLASTER_GROUND_STARTUP,
    BLASTER_GROUND_LOOP,
    BLASTER_GROUND_END,
    BLASTER_AIR_STARTUP,
    BLASTER_AIR_LOOP,
    BLASTER_AIR_END,
    PHANTASM_GROUND_STARTUP,
    PHANTASM_GROUND,
    PHANTASM_GROUND_END,
    PHANTASM_STARTUP_AIR,
    PHANTASM_AIR,
    PHANTASM_AIR_END,
    FIRE_BIRD_GROUND_STARTUP,
    FIRE_BIRD_AIR_STARTUP,
    FIRE_BIRD_GROUND,
    FIRE_BIRD_AIR,
    FIRE_BIRD_GROUND_END,
    FIRE_BIRD_AIR_END,
    FIRE_BIRD_BOUNCE_END,
    REFLECTOR_GROUND_STARTUP,
    REFLECTOR_GROUND_LOOP,
    REFLECTOR_GROUND_REFLECT,
    REFLECTOR_GROUND_END,
    REFLECTOR_GROUND_TURN,
    REFLECTOR_AIR_STARTUP,
    REFLECTOR_AIR_LOOP,
    REFLECTOR_AIR_REFLECT,
    REFLECTOR_AIR_END,
    REFLECTOR_AIR_TURN,
    SMASH_TAUNT_RIGHT_STARTUP,
    SMASH_TAUNT_LEFT_STARTUP,
    SMASH_TAUNT_RIGHT_RISE,
    SMASH_TAUNT_LEFT_RISE,
    SMASH_TAUNT_RIGHT_FINISH,
    SMASH_TAUNT_LEFT_FINISH,

    // Fox
    // BLASTER_GROUND_STARTUP,
    // BLASTER_GROUND_LOOP,
    // BLASTER_GROUND_END,
    // BLASTER_AIR_STARTUP,
    // BLASTER_AIR_LOOP,
    // BLASTER_AIR_END,
    ILLUSION_GROUND_STARTUP,
    ILLUSION_GROUND,
    ILLUSION_GROUND_END,
    ILLUSION_STARTUP_AIR,
    ILLUSION_AIR,
    ILLUSION_AIR_END,
    FIRE_FOX_GROUND_STARTUP,
    FIRE_FOX_AIR_STARTUP,
    FIRE_FOX_GROUND,
    FIRE_FOX_AIR,
    FIRE_FOX_GROUND_LANDING,
    FIRE_FOX_AIR_FALL,
    FIRE_FOX_GROUND_BOUNCE,
    // REFLECTOR_GROUND_STARTUP,
    // REFLECTOR_GROUND_LOOP,
    // REFLECTOR_GROUND_REFLECT,
    // REFLECTOR_GROUND_END,
    // REFLECTOR_GROUND_CHANGE_DIRECTION,
    // REFLECTOR_AIR_STARTUP,
    // REFLECTOR_AIR_LOOP,
    // REFLECTOR_AIR_REFLECT,
    // REFLECTOR_AIR_END,
    // REFLECTOR_AIR_CHANGE_DIRECTION,
    // SMASH_TAUNT_RIGHT_STARTUP,
    // SMASH_TAUNT_LEFT_STARTUP,
    // SMASH_TAUNT_RIGHT_RISE,
    // SMASH_TAUNT_LEFT_RISE,
    // SMASH_TAUNT_RIGHT_FINISH,
    // SMASH_TAUNT_LEFT_FINISH,

    // GameAndWatch
    JAB,
    RAPID_JAB_START,
    RAPID_JAB_LOOP,
    RAPID_JAB_END,
    DOWN_TILT,
    SIDE_SMASH,
    NAIR,
    BAIR,
    UAIR,
    NAIR_LANDING,
    BAIR_LANDING,
    UAIR_LANDING,
    CHEF_GROUND,
    CHEF_AIR,
    JUDGMENT_1_GROUND,
    JUDGMENT_2_GROUND,
    JUDGMENT_3_GROUND,
    JUDGMENT_4_GROUND,
    JUDGMENT_5_GROUND,
    JUDGMENT_6_GROUND,
    JUDGMENT_7_GROUND,
    JUDGMENT_8_GROUND,
    JUDGMENT_9_GROUND,
    JUDGMENT_1_AIR,
    JUDGMENT_2_AIR,
    JUDGMENT_3_AIR,
    JUDGMENT_4_AIR,
    JUDGMENT_5_AIR,
    JUDGMENT_6_AIR,
    JUDGMENT_7_AIR,
    JUDGMENT_8_AIR,
    JUDGMENT_9_AIR,
    /// Up B
    FIRE_GROUND,
    /// Up B
    FIRE_AIR,
    OIL_PANIC_GROUND,
    OIL_PANIC_GROUND_ABSORB,
    OIL_PANIC_GROUND_SPILL,
    OIL_PANIC_AIR,
    OIL_PANIC_AIR_ABSORB,
    OIL_PANIC_AIR_SPILL,

    // Ganondorf
    WARLOCK_PUNCH_GROUND,
    WARLOCK_PUNCH_AIR,
    GERUDO_DRAGON_GROUND,
    GERUDO_DRAGON_GROUND_HIT,
    GERUDO_DRAGON_AIR,
    GERUDO_DRAGON_AIR_HIT,
    DARK_DIVE_GROUND,
    DARK_DIVE_AIR,
    DARK_DIVE_CATCH,
    DARK_DIVE_ENDING,
    WIZARDS_FOOT_GROUND,
    WIZARDS_FOOT_GROUND_ENDING_ON_GROUND,
    WIZARDS_FOOT_AIR,
    WIZARDS_FOOT_AIR_ENDING_ON_GROUND,
    WIZARDS_FOOT_AIR_ENDING_IN_AIR,
    WIZARDS_FOOT_GROUND_ENDING_IN_AIR,
    WIZARDS_FOOT_HIT_WALL,

    // Jigglypuff
    JUMP_2,
    JUMP_3,
    JUMP_4,
    JUMP_5,
    JUMP_6,
    ROLLOUT_GROUND_START_CHARGE_RIGHT,
    ROLLOUT_GROUND_START_CHARGE_LEFT,
    ROLLOUT_GROUND_CHARGE_LOOP,
    ROLLOUT_GROUND_FULLY_CHARGED,
    ROLLOUT_GROUND_CHARGE_RELEASE,
    ROLLOUT_GROUND_START_TURN,
    ROLLOUT_GROUND_END_RIGHT,
    ROLLOUT_GROUND_END_LEFT,
    ROLLOUT_AIR_START_CHARGE_RIGHT,
    ROLLOUT_AIR_START_CHARGE_LEFT,
    ROLLOUT_AIR_CHARGE_LOOP,
    ROLLOUT_AIR_FULLY_CHARGED,
    ROLLOUT_AIR_CHARGE_RELEASE,
    ROLLOUT_AIR_END_RIGHT,
    ROLLOUT_AIR_END_LEFT,
    ROLLOUT_HIT,
    POUND_GROUND,
    POUND_AIR,
    SING_GROUND_LEFT,
    SING_AIR_LEFT,
    SING_GROUND_RIGHT,
    SING_AIR_RIGHT,
    REST_GROUND_LEFT,
    REST_AIR_LEFT,
    REST_GROUND_RIGHT,
    REST_AIR_RIGHT,

    // Kirby
    // JUMP_2,
    // JUMP_3,
    // JUMP_4,
    // JUMP_5,
    // JUMP_6,
    JUMP_2_WITH_HAT,
    JUMP_3_WITH_HAT,
    JUMP_4_WITH_HAT,
    JUMP_5_WITH_HAT,
    JUMP_6_WITH_HAT,
    DASH_ATTACK_GROUND,
    DASH_ATTACK_AIR,
    SWALLOW_GROUND_STARTUP,
    SWALLOW_GROUND_LOOP,
    SWALLOW_GROUND_END,
    SWALLOW_GROUND_CAPTURE,
    SWALLOW_GROUND_HOLD,
    SWALLOW_GROUND_HOLD_WAIT,
    SWALLOW_HOLD_WALK_SLOW,
    SWALLOW_HOLD_WALK_MIDDLE,
    SWALLOW_HOLD_WALK_FAST,
    SWALLOW_HOLD_CAPTURE_TURN,
    SWALLOW_HOLD_KNEE_BEND,
    SWALLOW_HOLD_JUMP,
    SWALLOW_HOLD_LANDING,
    SWALLOW_GROUND_DIGEST,
    SWALLOW_GROUND_SPIT,
    SWALLOW_AIR_STARTUP,
    SWALLOW_AIR_LOOP,
    SWALLOW_AIR_END,
    SWALLOW_AIR_CAPTURE,
    SWALLOW_AIR_HOLD,
    SWALLOW_AIR_HOLD_WAIT,
    SWALLOW_AIR_DIGEST,
    SWALLOW_AIR_SPIT,
    SWALLOW_AIR_HOLD_TURN,
    HAMMER_GROUND,
    HAMMER_AIR,
    FINAL_CUTTER_GROUND_STARTUP,
    FINAL_CUTTER_GROUND_END,
    FINAL_CUTTER_AIR_STARTUP,
    FINAL_CUTTER_AIR_APEX,
    FINAL_CUTTER_SWORD_DESCENT,
    FINAL_CUTTER_AIR_END,
    STONE_GROUND_STARTUP,
    STONE_GROUND,
    STONE_GROUND_END,
    STONE_AIR_STARTUP,
    STONE_AIR,
    STONE_AIR_END,
    MARIO_FIREBALL_GROUND,
    MARIO_FIREBALL_AIR,
    LINK_BOW_GROUND_CHARGE,
    LINK_BOW_GROUND_FULLY_CHARGED,
    LINK_BOW_GROUND_SHOOT,
    LINK_BOW_AIR_CHARGE,
    LINK_BOW_AIR_FULLY_CHARGED,
    LINK_BOW_AIR_SHOOT,
    SAMUS_CHARGE_SHOT_GROUND_START,
    SAMUS_CHARGE_SHOT_GROUND_LOOP,
    SAMUS_CHARGE_SHOT_GROUND_END,
    SAMUS_CHARGE_SHOT_GROUND_SHOOT,
    SAMUS_CHARGE_SHOT_AIR_START,
    SAMUS_CHARGE_SHOT_AIR_SHOOT,
    YOSHI_EGG_LAY_GROUND,
    YOSHI_EGG_LAY_GROUND_CAPTURE_START,
    YOSHI_EGG_LAY_GROUND_CAPTURE,
    YOSHI_EGG_LAY_AIR,
    YOSHI_EGG_LAY_AIR_CAPTURE_START,
    YOSHI_EGG_LAY_AIR_CAPTURE,
    FOX_BLASTER_GROUND_STARTUP,
    FOX_BLASTER_GROUND_LOOP,
    FOX_BLASTER_GROUND_END,
    FOX_BLASTER_AIR_STARTUP,
    FOX_BLASTER_AIR_LOOP,
    FOX_BLASTER_AIR_END,
    PIKACHU_TJOLT_GROUND,
    PIKACHU_TJOLT_AIR,
    LUIGI_FIREBALL_GROUND,
    LUIGI_FIREBALL_AIR,
    NESS_PK_FLASH_GROUND_STARTUP,
    NESS_PK_FLASH_GROUND_CHARGE,
    NESS_PK_FLASH_GROUND_EXPLODE,
    NESS_PK_FLASH_GROUND_END,
    NESS_PK_FLASH_AIR_STARTUP,
    NESS_PK_FLASH_AIR_CHARGE,
    NESS_PK_FLASH_AIR_EXPLODE,
    NESS_PK_FLASH_AIR_END,
    BOWSER_FIRE_BREATH_GROUND_START,
    BOWSER_FIRE_BREATH_GROUND_LOOP,
    BOWSER_FIRE_BREATH_GROUND_END,
    BOWSER_FIRE_BREATH_AIR_START,
    BOWSER_FIRE_BREATH_AIR_LOOP,
    BOWSER_FIRE_BREATH_AIR_END,
    PEACH_TOAD_GROUND,
    PEACH_TOAD_GROUND_ATTACK,
    PEACH_TOAD_AIR,
    PEACH_TOAD_AIR_ATTACK,
    ICE_CLIMBERS_ICE_SHOT_GROUND,
    ICE_CLIMBERS_ICE_SHOT_AIR,
    DK_GIANT_PUNCH_GROUND_CHARGE_STARTUP,
    DK_GIANT_PUNCH_GROUND_CHARGE_LOOP,
    DK_GIANT_PUNCH_GROUND_CHARGE_STOP,
    DK_GIANT_PUNCH_GROUND_EARLY_PUNCH,
    DK_GIANT_PUNCH_GROUND_FULL_CHARGE_PUNCH,
    DK_GIANT_PUNCH_AIR_CHARGE_STARTUP,
    DK_GIANT_PUNCH_AIR_CHARGE_LOOP,
    DK_GIANT_PUNCH_AIR_CHARGE_STOP,
    DK_GIANT_PUNCH_AIR_EARLY_PUNCH,
    DK_GIANT_PUNCH_AIR_FULL_CHARGE_PUNCH,
    ZELDA_NAYRUS_LOVE_GROUND,
    ZELDA_NAYRUS_LOVE_AIR,
    SHEIK_NEEDLE_GROUND_START_CHARGE,
    SHEIK_NEEDLE_GROUND_CHARGE_LOOP,
    SHEIK_NEEDLE_GROUND_END_CHARGE,
    SHEIK_NEEDLE_GROUND_SHOOT,
    SHEIK_NEEDLE_AIR_START_CHARGE,
    SHEIK_NEEDLE_AIR_CHARGE_LOOP,
    SHEIK_NEEDLE_AIR_END_CHARGE,
    SHEIK_NEEDLE_AIR_SHOOT,
    JIGGLYPUFF_ROLLOUT_GROUND_START_CHARGE_RIGHT,
    JIGGLYPUFF_ROLLOUT_GROUND_START_CHARGE_LEFT,
    JIGGLYPUFF_ROLLOUT_GROUND_CHARGE_LOOP,
    JIGGLYPUFF_ROLLOUT_GROUND_FULLY_CHARGED,
    JIGGLYPUFF_ROLLOUT_GROUND_CHARGE_RELEASE,
    JIGGLYPUFF_ROLLOUT_GROUND_START_TURN,
    JIGGLYPUFF_ROLLOUT_GROUND_END_RIGHT,
    JIGGLYPUFF_ROLLOUT_GROUND_END_LEFT,
    JIGGLYPUFF_ROLLOUT_AIR_START_CHARGE_RIGHT,
    JIGGLYPUFF_ROLLOUT_AIR_START_CHARGE_LEFT,
    JIGGLYPUFF_ROLLOUT_AIR_CHARGE_LOOP,
    JIGGLYPUFF_ROLLOUT_AIR_FULLY_CHARGED,
    JIGGLYPUFF_ROLLOUT_AIR_CHARGE_RELEASE,
    JIGGLYPUFF_ROLLOUT_AIR_END_RIGHT,
    JIGGLYPUFF_ROLLOUT_AIR_END_LEFT,
    JIGGLYPUFF_ROLLOUT_HIT,
    MARTH_SHIELD_BREAKER_GROUND_START_CHARGE,
    MARTH_SHIELD_BREAKER_GROUND_CHARGE_LOOP,
    MARTH_SHIELD_BREAKER_GROUND_EARLY_RELEASE,
    MARTH_SHIELD_BREAKER_GROUND_FULLY_CHARGED,
    MARTH_SHIELD_BREAKER_AIR_START_CHARGE,
    MARTH_SHIELD_BREAKER_AIR_CHARGE_LOOP,
    MARTH_SHIELD_BREAKER_AIR_EARLY_RELEASE,
    MARTH_SHIELD_BREAKER_AIR_FULLY_CHARGED,
    MEWTWO_SHADOW_BALL_GROUND_START_CHARGE,
    MEWTWO_SHADOW_BALL_GROUND_CHARGE_LOOP,
    MEWTWO_SHADOW_BALL_GROUND_FULLY_CHARGED,
    MEWTWO_SHADOW_BALL_GROUND_END_CHARGE,
    MEWTWO_SHADOW_BALL_GROUND_SHOOT,
    MEWTWO_SHADOW_BALL_AIR_START_CHARGE,
    MEWTWO_SHADOW_BALL_AIR_CHARGE_LOOP,
    MEWTWO_SHADOW_BALL_AIR_FULLY_CHARGED,
    MEWTWO_SHADOW_BALL_AIR_END_CHARGE,
    MEWTWO_SHADOW_BALL_AIR_SHOOT,
    GAMEAND_WATCH_OIL_PANIC_GROUND,
    GAMEAND_WATCH_OIL_PANIC_AIR,
    DOC_PILL_GROUND,
    DOC_PILL_AIR,
    YOUNG_LINK_BOW_GROUND_CHARGE,
    YOUNG_LINK_BOW_GROUND_FULLY_CHARGED,
    YOUNG_LINK_BOW_GROUND_SHOOT,
    YOUNG_LINK_BOW_AIR_CHARGE,
    YOUNG_LINK_BOW_AIR_FULLY_CHARGED,
    YOUNG_LINK_BOW_AIR_SHOOT,
    FALCO_BLASTER_GROUND_STARTUP,
    FALCO_BLASTER_GROUND_LOOP,
    FALCO_BLASTER_GROUND_END,
    FALCO_BLASTER_AIR_STARTUP,
    FALCO_BLASTER_AIR_LOOP,
    FALCO_BLASTER_AIR_END,
    PICHU_TJOLT_GROUND,
    PICHU_TJOLT_AIR,
    GANON_WARLOCK_PUNCH_GROUND,
    GANON_WARLOCK_PUNCH_AIR,
    ROY_FLARE_BLADE_GROUND_START_CHARGE,
    ROY_FLARE_BLADE_GROUND_CHARGE_LOOP,
    ROY_FLARE_BLADE_GROUND_EARLY_RELEASE,
    ROY_FLARE_BLADE_GROUND_FULLY_CHARGED,
    ROY_FLARE_BLADE_AIR_START_CHARGE,
    ROY_FLARE_BLADE_AIR_CHARGE_LOOP,
    ROY_FLARE_BLADE_AIR_EARLY_RELEASE,
    ROY_FLARE_BLADE_AIR_FULLY_CHARGED,

    // Link
    SIDE_SMASH_2,
    BOW_GROUND_CHARGE,
    BOW_GROUND_FULLY_CHARGED,
    BOW_GROUND_SHOOT,
    BOW_AIR_CHARGE,
    BOW_AIR_FULLY_CHARGED,
    BOW_AIR_SHOOT,
    BOOMERANG_GROUND_THROW,
    BOOMERANG_GROUND_CATCH,
    BOOMERANG_GROUND_THROW_EMPTY,
    BOOMERANG_AIR_THROW,
    BOOMERANG_AIR_CATCH,
    BOOMERANG_AIR_THROW_EMPTY,
    SPIN_ATTACK_GROUND,
    SPIN_ATTACK_AIR,
    BOMB_GROUND,
    // BOMB_AIR,
    ZAIR,
    ZAIR_CATCH,

    // Luigi
    FIREBALL_GROUND,
    FIREBALL_AIR,
    GREEN_MISSILE_GROUND_STARTUP,
    GREEN_MISSILE_GROUND_CHARGE,
    GREEN_MISSILE_GROUND_LANDING,
    GREEN_MISSILE_GROUND_TAKEOFF,
    GREEN_MISSILE_GROUND_TAKEOFF_MISFIRE,
    GREEN_MISSILE_AIR_STARTUP,
    GREEN_MISSILE_AIR_CHARGE,
    GREEN_MISSILE_AIR,
    GREEN_MISSILE_AIR_END,
    GREEN_MISSILE_AIR_TAKEOFF,
    GREEN_MISSILE_AIR_TAKEOFF_MISFIRE,
    // SUPER_JUMP_PUNCH_GROUND,
    // SUPER_JUMP_PUNCH_AIR,
    CYCLONE_GROUND,
    CYCLONE_AIR,

    // Mario
    // FIREBALL_GROUND,
    // FIREBALL_AIR,
    CAPE_GROUND,
    CAPE_AIR,
    // SUPER_JUMP_PUNCH_GROUND,
    // SUPER_JUMP_PUNCH_AIR,
    // TORNADO_GROUND,
    // TORNADO_AIR,

    // Marth
    SHIELD_BREAKER_GROUND_START_CHARGE,
    SHIELD_BREAKER_GROUND_CHARGE_LOOP,
    SHIELD_BREAKER_GROUND_EARLY_RELEASE,
    SHIELD_BREAKER_GROUND_FULLY_CHARGED,
    SHIELD_BREAKER_AIR_START_CHARGE,
    SHIELD_BREAKER_AIR_CHARGE_LOOP,
    SHIELD_BREAKER_AIR_EARLY_RELEASE,
    SHIELD_BREAKER_AIR_FULLY_CHARGED,
    DANCING_BLADE_1_GROUND,
    DANCING_BLADE_2_UP_GROUND,
    DANCING_BLADE_2_SIDE_GROUND,
    DANCING_BLADE_3_UP_GROUND,
    DANCING_BLADE_3_SIDE_GROUND,
    DANCING_BLADE_3_DOWN_GROUND,
    DANCING_BLADE_4_UP_GROUND,
    DANCING_BLADE_4_SIDE_GROUND,
    DANCING_BLADE_4_DOWN_GROUND,
    DANCING_BLADE_1_AIR,
    DANCING_BLADE_2_UP_AIR,
    DANCING_BLADE_2_SIDE_AIR,
    DANCING_BLADE_3_UP_AIR,
    DANCING_BLADE_3_SIDE_AIR,
    DANCING_BLADE_3_DOWN_AIR,
    DANCING_BLADE_4_UP_AIR,
    DANCING_BLADE_4_SIDE_AIR,
    DANCING_BLADE_4_DOWN_AIR,
    DOLPHIN_SLASH_GROUND,
    DOLPHIN_SLASH_AIR,
    COUNTER_GROUND,
    COUNTER_GROUND_HIT,
    COUNTER_AIR,
    COUNTER_AIR_HIT,

    // Mewtwo
    SHADOW_BALL_GROUND_START_CHARGE,
    SHADOW_BALL_GROUND_CHARGE_LOOP,
    SHADOW_BALL_GROUND_FULLY_CHARGED,
    SHADOW_BALL_GROUND_END_CHARGE,
    SHADOW_BALL_GROUND_SHOOT,
    SHADOW_BALL_AIR_START_CHARGE,
    SHADOW_BALL_AIR_CHARGE_LOOP,
    SHADOW_BALL_AIR_FULLY_CHARGED,
    SHADOW_BALL_AIR_END_CHARGE,
    SHADOW_BALL_AIR_SHOOT,
    CONFUSION_GROUND,
    CONFUSION_AIR,
    TELEPORT_GROUND_STARTUP,
    TELEPORT_GROUND_DISAPPEAR,
    TELEPORT_GROUND_REAPPEAR,
    TELEPORT_AIR_STARTUP,
    TELEPORT_AIR_DISAPPEAR,
    TELEPORT_AIR_REAPPEAR,
    DISABLE_GROUND,
    DISABLE_AIR,

    // Nana
    ICE_SHOT_GROUND,
    ICE_SHOT_AIR,
    BLIZZARD_GROUND,
    BLIZZARD_AIR,
    SQUALL_HAMMER_GROUND_TOGETHER,
    SQUALL_HAMMER_AIR_TOGETHER,
    BELAY_CATAPULT_STARTUP,
    BELAY_GROUND_CATAPULT_END,
    BELAY_CATAPULTING,

    // Ness
    // SIDE_SMASH,
    UP_SMASH,
    UP_SMASH_CHARGE,
    UP_SMASH_CHARGED,
    DOWN_SMASH,
    DOWN_SMASH_CHARGE,
    DOWN_SMASH_CHARGED,
    PK_FLASH_GROUND_STARTUP,
    PK_FLASH_GROUND_CHARGE,
    PK_FLASH_GROUND_EXPLODE,
    PK_FLASH_GROUND_END,
    PK_FLASH_AIR_STARTUP,
    PK_FLASH_AIR_CHARGE,
    PK_FLASH_AIR_EXPLODE,
    PK_FLASH_AIR_END,
    PK_FIRE_GROUND,
    PK_FIRE_AIR,
    PK_THUNDER_GROUND_STARTUP,
    PK_THUNDER_GROUND,
    PK_THUNDER_GROUND_END,
    PK_THUNDER_GROUND_HIT,
    PK_THUNDER_AIR_STARTUP,
    PK_THUNDER_AIR,
    PK_THUNDER_AIR_END,
    PK_THUNDER_AIR_HIT,
    PK_THUNDER_AIR_WALL_BOUNCE,
    PSI_MAGNET_GROUND_STARTUP,
    PSI_MAGNET_GROUND_LOOP,
    PSI_MAGNET_GROUND_ABSORB,
    PSI_MAGNET_GROUND_END,
    PSI_MAGNET_AIR_STARTUP,
    PSI_MAGNET_AIR_LOOP,
    PSI_MAGNET_AIR_ABSORB,
    PSI_MAGNET_AIR_END,

    // Peach
    FLOAT,
    FLOAT_FALL_FORWARD,
    FLOAT_FALL_BACKWARD,
    FLOAT_NAIR,
    FLOAT_FAIR,
    FLOAT_BAIR,
    FLOAT_UAIR,
    FLOAT_DAIR,
    SIDE_SMASH_GOLF_CLUB,
    SIDE_SMASH_FRYING_PAN,
    SIDE_SMASH_TENNIS_RACKET,
    VEGETABLE_GROUND,
    VEGETABLE_AIR,
    BOMBER_GROUND_STARTUP,
    BOMBER_GROUND_END,
    BOMBER_AIR_STARTUP,
    BOMBER_AIR_END,
    BOMBER_AIR_HIT,
    BOMBER_AIR,
    PARASOL_GROUND_START,
    PARASOL_AIR_START,
    TOAD_GROUND,
    TOAD_GROUND_ATTACK,
    TOAD_AIR,
    TOAD_AIR_ATTACK,
    PARASOL_OPEN,
    PARASOL_FALL,

    // Pichu
    TJOLT_GROUND,
    TJOLT_AIR,
    SKULL_BASH_GROUND_STARTUP,
    SKULL_BASH_GROUND_CHARGE,
    SKULL_BASH_GROUND_LANDING,
    SKULL_BASH_GROUND_TAKEOFF,
    SKULL_BASH_AIR_STARTUP,
    SKULL_BASH_AIR_CHARGE,
    SKULL_BASH_AIR,
    SKULL_BASH_AIR_END,
    SKULL_BASH_AIR_TAKEOFF,
    AGILITY_GROUND_STARTUP,
    AGILITY_GROUND,
    AGILITY_GROUND_END,
    AGILITY_AIR_STARTUP,
    AGILITY_AIR,
    AGILITY_AIR_END,
    THUNDER_GROUND_STARTUP,
    THUNDER_GROUND,
    THUNDER_GROUND_HIT,
    THUNDER_GROUND_END,
    THUNDER_AIR_STARTUP,
    THUNDER_AIR,
    THUNDER_AIR_HIT,
    THUNDER_AIR_END,

    // Pikachu
    // THUNDER_JOLT_GROUND,
    // THUNDER_JOLT_AIR,
    // SKULL_BASH_GROUND_STARTUP,
    // SKULL_BASH_GROUND_CHARGE,
    // SKULL_BASH_GROUND_LANDING,
    // SKULL_BASH_GROUND_TAKEOFF,
    // SKULL_BASH_AIR_STARTUP,
    // SKULL_BASH_AIR_CHARGE,
    // SKULL_BASH_AIR,
    // SKULL_BASH_AIR_END,
    // SKULL_BASH_AIR_TAKEOFF,
    QUICK_ATTACK_GROUND_STARTUP,
    QUICK_ATTACK_GROUND,
    QUICK_ATTACK_GROUND_END,
    QUICK_ATTACK_AIR_STARTUP,
    QUICK_ATTACK_AIR,
    QUICK_ATTACK_AIR_END,
    // THUNDER_GROUND_STARTUP,
    // THUNDER_GROUND,
    // THUNDER_GROUND_HIT,
    // THUNDER_GROUND_END,
    // THUNDER_AIR_STARTUP,
    // THUNDER_AIR,
    // THUNDER_AIR_HIT,
    // THUNDER_AIR_END,

    // Popo
    // ICE_SHOT_GROUND,
    // ICE_SHOT_AIR,
    SQUALL_HAMMER_GROUND_SOLO,
    // SQUALL_HAMMER_GROUND_TOGETHER,
    SQUALL_HAMMER_AIR_SOLO,
    // SQUALL_HAMMER_AIR_TOGETHER,
    BELAY_GROUND_STARTUP,
    BELAY_GROUND_CATAPULTING_NANA,
    BELAY_GROUND_FAILED_CATAPULTING,
    BELAY_GROUND_FAILED_CATAPULTING_END,
    BELAY_AIR_STARTUP,
    BELAY_AIR_CATAPULTING_NANA,
    // BELAY_CATAPULTING,
    BELAY_AIR_FAILED_CATAPULTING,
    BELAY_AIR_FAILED_CATAPULTING_END,
    // BLIZZARD_GROUND,
    // BLIZZARD_AIR,

    // Roy
    FLARE_BLADE_GROUND_START_CHARGE,
    FLARE_BLADE_GROUND_CHARGE_LOOP,
    FLARE_BLADE_GROUND_EARLY_RELEASE,
    FLARE_BLADE_GROUND_FULLY_CHARGED,
    FLARE_BLADE_AIR_START_CHARGE,
    FLARE_BLADE_AIR_CHARGE_LOOP,
    FLARE_BLADE_AIR_EARLY_RELEASE,
    FLARE_BLADE_AIR_FULLY_CHARGED,
    DOUBLE_EDGE_DANCE_1_GROUND,
    DOUBLE_EDGE_DANCE_2_UP_GROUND,
    DOUBLE_EDGE_DANCE_2_SIDE_GROUND,
    DOUBLE_EDGE_DANCE_3_UP_GROUND,
    DOUBLE_EDGE_DANCE_3_SIDE_GROUND,
    DOUBLE_EDGE_DANCE_3_DOWN_GROUND,
    DOUBLE_EDGE_DANCE_4_UP_GROUND,
    DOUBLE_EDGE_DANCE_4_SIDE_GROUND,
    DOUBLE_EDGE_DANCE_4_DOWN_GROUND,
    DOUBLE_EDGE_DANCE_1_AIR,
    DOUBLE_EDGE_DANCE_2_UP_AIR,
    DOUBLE_EDGE_DANCE_2_SIDE_AIR,
    DOUBLE_EDGE_DANCE_3_UP_AIR,
    DOUBLE_EDGE_DANCE_3_SIDE_AIR,
    DOUBLE_EDGE_DANCE_3_DOWN_AIR,
    DOUBLE_EDGE_DANCE_4_UP_AIR,
    DOUBLE_EDGE_DANCE_4_SIDE_AIR,
    DOUBLE_EDGE_DANCE_4_DOWN_AIR,
    BLAZER_GROUND,
    BLAZER_AIR,
    // COUNTER_GROUND,
    // COUNTER_GROUND_HIT,
    // COUNTER_AIR,
    // COUNTER_AIR_HIT,

    // Samus
    BOMB_JUMP_GROUND,
    BOMB_JUMP_AIR,
    CHARGE_SHOT_GROUND_START,
    CHARGE_SHOT_GROUND_LOOP,
    CHARGE_SHOT_GROUND_END,
    CHARGE_SHOT_GROUND_SHOOT,
    CHARGE_SHOT_AIR_START,
    CHARGE_SHOT_AIR_SHOOT,
    MISSILE_GROUND,
    MISSILE_SMASH_GROUND,
    MISSILE_AIR,
    MISSILE_SMASH_AIR,
    SCREW_ATTACK_GROUND,
    SCREW_ATTACK_AIR,
    BOMB_END_GROUND,
    // BOMB_AIR,
    // ZAIR,
    // ZAIR_CATCH,

    // Sheik
    NEEDLE_GROUND_START_CHARGE,
    NEEDLE_GROUND_CHARGE_LOOP,
    NEEDLE_GROUND_END_CHARGE,
    NEEDLE_GROUND_FIRE,
    NEEDLE_AIR_START_CHARGE,
    NEEDLE_AIR_CHARGE_LOOP,
    NEEDLE_AIR_END_CHARGE,
    NEEDLE_AIR_FIRE,
    CHAIN_GROUND_STARTUP,
    CHAIN_GROUND_LOOP,
    CHAIN_GROUND_END,
    CHAIN_AIR_STARTUP,
    CHAIN_AIR_LOOP,
    CHAIN_AIR_END,
    VANISH_GROUND_STARTUP,
    VANISH_GROUND_DISAPPEAR,
    VANISH_GROUND_REAPPEAR,
    VANISH_AIR_STARTUP,
    VANISH_AIR_DISAPPEAR,
    VANISH_AIR_REAPPEAR,
    TRANSFORM_GROUND,
    TRANSFORM_GROUND_ENDING,
    TRANSFORM_AIR,
    TRANSFORM_AIR_ENDING,

    // Yoshi
    GUARD_ON,
    GUARD,
    GUARD_OFF,
    GUARD_DAMAGE,
    GUARD_ON_2,
    EGG_LAY_GROUND,
    EGG_LAY_GROUND_CAPTURE_START,
    EGG_LAY_GROUND_CAPTURE,
    EGG_LAY_AIR,
    EGG_LAY_AIR_CAPTURE_START,
    EGG_LAY_AIR_CAPTURE,
    EGG_ROLL_GROUND_STARTUP,
    EGG_ROLL_GROUND,
    EGG_ROLL_GROUND_CHANGE_DIRECTION,
    EGG_ROLL_GROUND_END,
    EGG_ROLL_AIR_START,
    EGG_ROLL_AIR,
    EGG_ROLL_BOUNCE,
    EGG_ROLL_AIR_END,
    EGG_THROW_GROUND,
    EGG_THROW_AIR,
    // BOMB_GROUND,
    // BOMB_LAND,
    // BOMB_AIR,

    // YoungLink
    // SIDE_SMASH_2,
    // TAUNT_R,
    TAUNT_L,
    // FIRE_BOW_GROUND_CHARGE,
    // FIRE_BOW_GROUND_FULLY_CHARGED,
    // FIRE_BOW_GROUND_FIRE,
    // FIRE_BOW_AIR_CHARGE,
    // FIRE_BOW_AIR_FULLY_CHARGED,
    // FIRE_BOW_AIR_FIRE,
    // BOOMERANG_GROUND_THROW,
    // BOOMERANG_GROUND_CATCH,
    // BOOMERANG_GROUND_THROW_EMPTY,
    // BOOMERANG_AIR_THROW,
    // BOOMERANG_AIR_CATCH,
    // BOOMERANG_AIR_THROW_EMPTY,
    // SPIN_ATTACK_GROUND,
    // SPIN_ATTACK_AIR,
    // BOMB_GROUND,
    // BOMB_AIR,
    // ZAIR,
    // ZAIR_CATCH,

    // Zelda
    NAYRUS_LOVE_GROUND,
    NAYRUS_LOVE_AIR,
    DINS_FIRE_GROUND_STARTUP,
    DINS_FIRE_GROUND_TRAVEL,
    DINS_FIRE_GROUND_EXPLODE,
    DINS_FIRE_AIR_STARTUP,
    DINS_FIRE_AIR_TRAVEL,
    DINS_FIRE_AIR_EXPLODE,
    FARORES_WIND_GROUND,
    FARORES_WIND_GROUND_DISAPPEAR,
    FARORES_WIND_GROUND_REAPPEAR,
    FARORES_WIND_AIR,
    FARORES_WIND_AIR_DISAPPEAR,
    FARORES_WIND_AIR_REAPPEAR,
    // TRANSFORM_GROUND,
    // TRANSFORM_GROUND_ENDING,
    // TRANSFORM_AIR,
    // TRANSFORM_AIR_ENDING,

    // ----------------------------- These are taken from the decomp ---------------------------- //
    SWORD_SWING_4,
    BAT_SWING_4,
    PARASOL_SWING_4,
    FAN_SWING_4,
    STAR_ROD_SWING_4,
    LIPSTICK_SWING_4,
    HEAVY_WAIT,
    HEAVY_WALK_SLOW,
    HEAVY_WALK_MIDDLE,
    HEAVY_WALK_FAST,
    HEAVY_TURN,
    HEAVY_KNEE_BEND,
    HEAVY_FALL,
    HEAVY_JUMP,
    HEAVY_LANDING,
    HEAVY_WAIT_2,
    KONG_KARRY_WAIT_2,
    SWALLOW_GROUND_CAPTURE_2,

    SWALLOW_GROUND_DIGEST_2,
    SWALLOW_GROUND_SPIT_2,
    SWALLOW_AIR_CAPTURE_2,
    SWALLOW_AIR_DIGEST_2,
    SWALLOW_AIR_SPIT_2,
    FINAL_CUTTER_GROUND_APEX,
    FINAL_CUTTER_GROUND_DESCENT,
    YOSHI_EGG_LAY_GROUND_CAPTURE_START_2,
    YOSHI_EGG_LAY_GROUND_CAPTURE_2,
    YOSHI_EGG_LAY_AIR_CAPTURE_START_2,
    YOSHI_EGG_LAY_AIR_CAPTURE_2,
    GIGA_BOWSER_FIRE_BREATH_GROUND_START,
    GIGA_BOWSER_FIRE_BREATH_GROUND_LOOP,
    GIGA_BOWSER_FIRE_BREATH_GROUND_END,
    GIGA_BOWSER_FIRE_BREATH_AIR_START,
    GIGA_BOWSER_FIRE_BREATH_AIR_LOOP,
    GIGA_BOWSER_FIRE_BREATH_AIR_END,

    PSI_MAGNET_GROUND_TURN,
    PSI_MAGNET_AIR_TURN,
    BOMBER_GROUND,
    PARASOL_GROUND_END,
    PARASOL_AIR_END,
    EGG_LAY_GROUND_CAPTURE_START_2,
    EGG_LAY_GROUND_CAPTURE_2,
    EGG_LAY_AIR_CAPTURE_START_2,
    EGG_LAY_AIR_CAPTURE_2,
    SKULL_BASH_GROUND,
    BELAY_GROUND_CATAPULTING,
    BELAY_2,
    BELAY_3,
    BELAY_5,
}

impl CharacterState {
    pub fn from_char_and_state(character: Character, state: u16) -> Result<Self> {
        use CharacterState::*;
        assert!(state >= 341);
        // pain
        Ok(match character {
            Character::CaptainFalcon => match state {
                341 => SWORD_SWING_4,
                342 => BAT_SWING_4,
                343 => PARASOL_SWING_4,
                344 => FAN_SWING_4,
                345 => STAR_ROD_SWING_4,
                346 => LIPSTICK_SWING_4,
                347 => FALCON_PUNCH_GROUND,
                348 => FALCON_PUNCH_AIR,
                349 => RAPTOR_BOOST_GROUND,
                350 => RAPTOR_BOOST_GROUND_HIT,
                351 => RAPTOR_BOOST_AIR,
                352 => RAPTOR_BOOST_AIR_HIT,
                353 => FALCON_DIVE_GROUND,
                354 => FALCON_DIVE_AIR,
                355 => FALCON_DIVE_CATCH,
                356 => FALCON_DIVE_ENDING,
                357 => FALCON_KICK_GROUND,
                358 => FALCON_KICK_GROUND_ENDING_ON_GROUND,
                359 => FALCON_KICK_AIR,
                360 => FALCON_KICK_AIR_ENDING_ON_GROUND,
                361 => FALCON_KICK_AIR_ENDING_IN_AIR,
                362 => FALCON_KICK_GROUND_ENDING_IN_AIR,
                363 => FALCON_KICK_HIT_WALL,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 347-363 inclusive. Got: {state}"))
            },
            Character::DonkeyKong => match state {
                341 => HEAVY_WAIT,
                342 => HEAVY_WALK_SLOW,
                343 => HEAVY_WALK_MIDDLE,
                344 => HEAVY_WALK_FAST,
                345 => HEAVY_TURN,
                346 => HEAVY_KNEE_BEND,
                347 => HEAVY_FALL,
                348 => HEAVY_JUMP,
                349 => HEAVY_LANDING,
                350 => HEAVY_WAIT_2,
                351 => KONG_KARRY_WAIT,
                352 => KONG_KARRY_WALK_SLOW,
                353 => KONG_KARRY_WALK_MIDDLE,
                354 => KONG_KARRY_WALK_FAST,
                355 => KONG_KARRY_TURN,
                356 => KONG_KARRY_KNEE_BEND,
                357 => KONG_KARRY_FALL,
                358 => KONG_KARRY_JUMP,
                359 => KONG_KARRY_LANDING,
                360 => KONG_KARRY_WAIT_2,
                361 => KONG_KARRY_GROUND_THROW_FORWARD,
                362 => KONG_KARRY_GROUND_THROW_BACKWARD,
                363 => KONG_KARRY_GROUND_THROW_UP,
                364 => KONG_KARRY_GROUND_THROW_DOWN,
                365 => KONG_KARRY_AIR_THROW_FORWARD,
                366 => KONG_KARRY_AIR_THROW_BACKWARD,
                367 => KONG_KARRY_AIR_THROW_UP,
                368 => KONG_KARRY_AIR_THROW_DOWN,
                369 => GIANT_PUNCH_GROUND_CHARGE_STARTUP,
                370 => GIANT_PUNCH_GROUND_CHARGE_LOOP,
                371 => GIANT_PUNCH_GROUND_CHARGE_STOP,
                372 => GIANT_PUNCH_GROUND_EARLY_PUNCH,
                373 => GIANT_PUNCH_GROUND_FULL_CHARGE_PUNCH,
                374 => GIANT_PUNCH_AIR_CHARGE_STARTUP,
                375 => GIANT_PUNCH_AIR_CHARGE_LOOP,
                376 => GIANT_PUNCH_AIR_CHARGE_STOP,
                377 => GIANT_PUNCH_AIR_EARLY_PUNCH,
                378 => GIANT_PUNCH_AIR_FULL_CHARGE_PUNCH,
                379 => HEADBUTT_GROUND,
                380 => HEADBUTT_AIR,
                381 => SPINNING_KONG_GROUND,
                382 => SPINNING_KONG_AIR,
                383 => HAND_SLAP_STARTUP,
                384 => HAND_SLAP_LOOP,
                385 => HAND_SLAP_END,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 351-385 inclusive. Got: {state}"))
            },
            Character::Fox => match state {
                341 => BLASTER_GROUND_STARTUP,
                342 => BLASTER_GROUND_LOOP,
                343 => BLASTER_GROUND_END,
                344 => BLASTER_AIR_STARTUP,
                345 => BLASTER_AIR_LOOP,
                346 => BLASTER_AIR_END,
                347 => ILLUSION_GROUND_STARTUP,
                348 => ILLUSION_GROUND,
                349 => ILLUSION_GROUND_END,
                350 => ILLUSION_STARTUP_AIR,
                351 => ILLUSION_AIR,
                352 => ILLUSION_AIR_END,
                353 => FIRE_FOX_GROUND_STARTUP,
                354 => FIRE_FOX_AIR_STARTUP,
                355 => FIRE_FOX_GROUND,
                356 => FIRE_FOX_AIR,
                357 => FIRE_FOX_GROUND_LANDING,
                358 => FIRE_FOX_AIR_FALL,
                359 => FIRE_FOX_GROUND_BOUNCE,
                360 => REFLECTOR_GROUND_STARTUP,
                361 => REFLECTOR_GROUND_LOOP,
                362 => REFLECTOR_GROUND_REFLECT,
                363 => REFLECTOR_GROUND_END,
                364 => REFLECTOR_GROUND_TURN,
                365 => REFLECTOR_AIR_STARTUP,
                366 => REFLECTOR_AIR_LOOP,
                367 => REFLECTOR_AIR_REFLECT,
                368 => REFLECTOR_AIR_END,
                369 => REFLECTOR_AIR_TURN,
                370 => SMASH_TAUNT_RIGHT_STARTUP,
                371 => SMASH_TAUNT_LEFT_STARTUP,
                372 => SMASH_TAUNT_RIGHT_RISE,
                373 => SMASH_TAUNT_LEFT_RISE,
                374 => SMASH_TAUNT_RIGHT_FINISH,
                375 => SMASH_TAUNT_LEFT_FINISH,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 341-375 inclusive. Got: {state}"))
            },
            Character::GameAndWatch => match state {
                341 => JAB,
                342 => RAPID_JAB_START,
                343 => RAPID_JAB_LOOP,
                344 => RAPID_JAB_END,
                345 => DOWN_TILT,
                346 => SIDE_SMASH,
                347 => NAIR,
                348 => BAIR,
                349 => UAIR,
                350 => NAIR_LANDING,
                351 => BAIR_LANDING,
                352 => UAIR_LANDING,
                353 => CHEF_GROUND,
                354 => CHEF_AIR,
                355 => JUDGMENT_1_GROUND,
                356 => JUDGMENT_2_GROUND,
                357 => JUDGMENT_3_GROUND,
                358 => JUDGMENT_4_GROUND,
                359 => JUDGMENT_5_GROUND,
                360 => JUDGMENT_6_GROUND,
                361 => JUDGMENT_7_GROUND,
                362 => JUDGMENT_8_GROUND,
                363 => JUDGMENT_9_GROUND,
                364 => JUDGMENT_1_AIR,
                365 => JUDGMENT_2_AIR,
                366 => JUDGMENT_3_AIR,
                367 => JUDGMENT_4_AIR,
                368 => JUDGMENT_5_AIR,
                369 => JUDGMENT_6_AIR,
                370 => JUDGMENT_7_AIR,
                371 => JUDGMENT_8_AIR,
                372 => JUDGMENT_9_AIR,
                373 => FIRE_GROUND,
                374 => FIRE_AIR,
                375 => OIL_PANIC_GROUND,
                376 => OIL_PANIC_GROUND_ABSORB,
                377 => OIL_PANIC_GROUND_SPILL,
                378 => OIL_PANIC_AIR,
                379 => OIL_PANIC_AIR_ABSORB,
                380 => OIL_PANIC_AIR_SPILL,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 341-380 inclusive. Got: {state}"))
            },
            // lol
            Character::Kirby => match state {
                341 => JUMP_2,
                342 => JUMP_3,
                343 => JUMP_4,
                344 => JUMP_5,
                345 => JUMP_6,
                346 => JUMP_2_WITH_HAT,
                347 => JUMP_3_WITH_HAT,
                348 => JUMP_4_WITH_HAT,
                349 => JUMP_5_WITH_HAT,
                350 => JUMP_6_WITH_HAT,
                351 => DASH_ATTACK_GROUND,
                352 => DASH_ATTACK_AIR,
                353 => SWALLOW_GROUND_STARTUP,
                354 => SWALLOW_GROUND_LOOP,
                355 => SWALLOW_GROUND_END,
                356 => SWALLOW_GROUND_CAPTURE,
                357 => SWALLOW_GROUND_CAPTURE_2,
                358 => SWALLOW_GROUND_HOLD,
                359 => SWALLOW_GROUND_HOLD_WAIT,
                360 => SWALLOW_HOLD_WALK_SLOW,
                361 => SWALLOW_HOLD_WALK_MIDDLE,
                362 => SWALLOW_HOLD_WALK_FAST,
                363 => SWALLOW_HOLD_CAPTURE_TURN,
                364 => SWALLOW_HOLD_KNEE_BEND,
                365 => SWALLOW_HOLD_JUMP,
                366 => SWALLOW_HOLD_LANDING,
                367 => SWALLOW_GROUND_DIGEST,
                368 => SWALLOW_GROUND_DIGEST_2,
                369 => SWALLOW_GROUND_SPIT,
                370 => SWALLOW_GROUND_SPIT_2,
                371 => SWALLOW_AIR_STARTUP,
                372 => SWALLOW_AIR_LOOP,
                373 => SWALLOW_AIR_END,
                374 => SWALLOW_AIR_CAPTURE,
                375 => SWALLOW_AIR_CAPTURE_2,
                376 => SWALLOW_AIR_HOLD,
                377 => SWALLOW_AIR_HOLD_WAIT,
                378 => SWALLOW_AIR_DIGEST,
                379 => SWALLOW_AIR_DIGEST_2,
                380 => SWALLOW_AIR_SPIT,
                381 => SWALLOW_AIR_SPIT_2,
                382 => SWALLOW_AIR_HOLD_TURN,
                383 => HAMMER_GROUND,
                384 => HAMMER_AIR,
                385 => FINAL_CUTTER_GROUND_STARTUP,
                386 => FINAL_CUTTER_GROUND_APEX,
                387 => FINAL_CUTTER_GROUND_DESCENT,
                388 => FINAL_CUTTER_GROUND_END,
                389 => FINAL_CUTTER_AIR_STARTUP,
                390 => FINAL_CUTTER_AIR_APEX,
                391 => FINAL_CUTTER_SWORD_DESCENT,
                392 => FINAL_CUTTER_AIR_END,
                393 => STONE_GROUND_STARTUP,
                394 => STONE_GROUND,
                395 => STONE_GROUND_END,
                396 => STONE_AIR_STARTUP,
                397 => STONE_AIR,
                398 => STONE_AIR_END,
                399 => MARIO_FIREBALL_GROUND,
                400 => MARIO_FIREBALL_AIR,
                401 => LINK_BOW_GROUND_CHARGE,
                402 => LINK_BOW_GROUND_FULLY_CHARGED,
                403 => LINK_BOW_GROUND_SHOOT,
                404 => LINK_BOW_AIR_CHARGE,
                405 => LINK_BOW_AIR_FULLY_CHARGED,
                406 => LINK_BOW_AIR_SHOOT,
                407 => SAMUS_CHARGE_SHOT_GROUND_START,
                408 => SAMUS_CHARGE_SHOT_GROUND_LOOP,
                409 => SAMUS_CHARGE_SHOT_GROUND_END,
                410 => SAMUS_CHARGE_SHOT_GROUND_SHOOT,
                411 => SAMUS_CHARGE_SHOT_AIR_START,
                412 => SAMUS_CHARGE_SHOT_AIR_SHOOT,
                413 => YOSHI_EGG_LAY_GROUND,
                414 => YOSHI_EGG_LAY_GROUND_CAPTURE_START,
                415 => YOSHI_EGG_LAY_GROUND_CAPTURE_START_2,
                416 => YOSHI_EGG_LAY_GROUND_CAPTURE,
                417 => YOSHI_EGG_LAY_GROUND_CAPTURE_2,
                418 => YOSHI_EGG_LAY_AIR,
                419 => YOSHI_EGG_LAY_AIR_CAPTURE_START,
                420 => YOSHI_EGG_LAY_AIR_CAPTURE_START_2,
                421 => YOSHI_EGG_LAY_AIR_CAPTURE,
                422 => YOSHI_EGG_LAY_AIR_CAPTURE_2,
                423 => FOX_BLASTER_GROUND_STARTUP,
                424 => FOX_BLASTER_GROUND_LOOP,
                425 => FOX_BLASTER_GROUND_END,
                426 => FOX_BLASTER_AIR_STARTUP,
                427 => FOX_BLASTER_AIR_LOOP,
                428 => FOX_BLASTER_AIR_END,
                429 => PIKACHU_TJOLT_GROUND,
                430 => PIKACHU_TJOLT_AIR,
                431 => LUIGI_FIREBALL_GROUND,
                432 => LUIGI_FIREBALL_AIR,
                433 => FALCON_PUNCH_GROUND,
                434 => FALCON_PUNCH_AIR,
                435 => NESS_PK_FLASH_GROUND_STARTUP,
                436 => NESS_PK_FLASH_GROUND_CHARGE,
                437 => NESS_PK_FLASH_GROUND_EXPLODE,
                438 => NESS_PK_FLASH_GROUND_END,
                439 => NESS_PK_FLASH_AIR_STARTUP,
                440 => NESS_PK_FLASH_AIR_CHARGE,
                441 => NESS_PK_FLASH_AIR_EXPLODE,
                442 => NESS_PK_FLASH_AIR_END,
                443 => BOWSER_FIRE_BREATH_GROUND_START,
                444 => BOWSER_FIRE_BREATH_GROUND_LOOP,
                445 => BOWSER_FIRE_BREATH_GROUND_END,
                446 => BOWSER_FIRE_BREATH_AIR_START,
                447 => BOWSER_FIRE_BREATH_AIR_LOOP,
                448 => BOWSER_FIRE_BREATH_AIR_END,
                449 => PEACH_TOAD_GROUND,
                450 => PEACH_TOAD_GROUND_ATTACK,
                451 => PEACH_TOAD_AIR,
                452 => PEACH_TOAD_AIR_ATTACK,
                453 => ICE_CLIMBERS_ICE_SHOT_GROUND,
                454 => ICE_CLIMBERS_ICE_SHOT_AIR,
                455 => DK_GIANT_PUNCH_GROUND_CHARGE_STARTUP,
                456 => DK_GIANT_PUNCH_GROUND_CHARGE_LOOP,
                457 => DK_GIANT_PUNCH_GROUND_CHARGE_STOP,
                458 => DK_GIANT_PUNCH_GROUND_EARLY_PUNCH,
                459 => DK_GIANT_PUNCH_GROUND_FULL_CHARGE_PUNCH,
                460 => DK_GIANT_PUNCH_AIR_CHARGE_STARTUP,
                461 => DK_GIANT_PUNCH_AIR_CHARGE_LOOP,
                462 => DK_GIANT_PUNCH_AIR_CHARGE_STOP,
                463 => DK_GIANT_PUNCH_AIR_EARLY_PUNCH,
                464 => DK_GIANT_PUNCH_AIR_FULL_CHARGE_PUNCH,
                465 => ZELDA_NAYRUS_LOVE_GROUND,
                466 => ZELDA_NAYRUS_LOVE_AIR,
                467 => SHEIK_NEEDLE_GROUND_START_CHARGE,
                468 => SHEIK_NEEDLE_GROUND_CHARGE_LOOP,
                469 => SHEIK_NEEDLE_GROUND_END_CHARGE,
                470 => SHEIK_NEEDLE_GROUND_SHOOT,
                471 => SHEIK_NEEDLE_AIR_START_CHARGE,
                472 => SHEIK_NEEDLE_AIR_CHARGE_LOOP,
                473 => SHEIK_NEEDLE_AIR_END_CHARGE,
                474 => SHEIK_NEEDLE_AIR_SHOOT,
                475 => JIGGLYPUFF_ROLLOUT_GROUND_START_CHARGE_RIGHT,
                476 => JIGGLYPUFF_ROLLOUT_GROUND_START_CHARGE_LEFT,
                477 => JIGGLYPUFF_ROLLOUT_GROUND_CHARGE_LOOP,
                478 => JIGGLYPUFF_ROLLOUT_GROUND_FULLY_CHARGED,
                479 => JIGGLYPUFF_ROLLOUT_GROUND_CHARGE_RELEASE,
                480 => JIGGLYPUFF_ROLLOUT_GROUND_START_TURN,
                481 => JIGGLYPUFF_ROLLOUT_GROUND_END_RIGHT,
                482 => JIGGLYPUFF_ROLLOUT_GROUND_END_LEFT,
                483 => JIGGLYPUFF_ROLLOUT_AIR_START_CHARGE_RIGHT,
                484 => JIGGLYPUFF_ROLLOUT_AIR_START_CHARGE_LEFT,
                485 => JIGGLYPUFF_ROLLOUT_AIR_CHARGE_LOOP,
                486 => JIGGLYPUFF_ROLLOUT_AIR_FULLY_CHARGED,
                487 => JIGGLYPUFF_ROLLOUT_AIR_CHARGE_RELEASE,
                489 => JIGGLYPUFF_ROLLOUT_AIR_END_RIGHT,
                490 => JIGGLYPUFF_ROLLOUT_AIR_END_LEFT,
                491 => JIGGLYPUFF_ROLLOUT_HIT,
                492 => MARTH_SHIELD_BREAKER_GROUND_START_CHARGE,
                493 => MARTH_SHIELD_BREAKER_GROUND_CHARGE_LOOP,
                494 => MARTH_SHIELD_BREAKER_GROUND_EARLY_RELEASE,
                495 => MARTH_SHIELD_BREAKER_GROUND_FULLY_CHARGED,
                496 => MARTH_SHIELD_BREAKER_AIR_START_CHARGE,
                497 => MARTH_SHIELD_BREAKER_AIR_CHARGE_LOOP,
                498 => MARTH_SHIELD_BREAKER_AIR_EARLY_RELEASE,
                499 => MARTH_SHIELD_BREAKER_AIR_FULLY_CHARGED,
                500 => MEWTWO_SHADOW_BALL_GROUND_START_CHARGE,
                501 => MEWTWO_SHADOW_BALL_GROUND_CHARGE_LOOP,
                502 => MEWTWO_SHADOW_BALL_GROUND_FULLY_CHARGED,
                503 => MEWTWO_SHADOW_BALL_GROUND_END_CHARGE,
                504 => MEWTWO_SHADOW_BALL_GROUND_SHOOT,
                505 => MEWTWO_SHADOW_BALL_AIR_START_CHARGE,
                506 => MEWTWO_SHADOW_BALL_AIR_CHARGE_LOOP,
                507 => MEWTWO_SHADOW_BALL_AIR_FULLY_CHARGED,
                508 => MEWTWO_SHADOW_BALL_AIR_END_CHARGE,
                509 => MEWTWO_SHADOW_BALL_AIR_SHOOT,
                510 => GAMEAND_WATCH_OIL_PANIC_GROUND,
                511 => GAMEAND_WATCH_OIL_PANIC_AIR,
                512 => DOC_PILL_GROUND,
                513 => DOC_PILL_AIR,
                514 => YOUNG_LINK_BOW_GROUND_CHARGE,
                515 => YOUNG_LINK_BOW_GROUND_FULLY_CHARGED,
                516 => YOUNG_LINK_BOW_GROUND_SHOOT,
                517 => YOUNG_LINK_BOW_AIR_CHARGE,
                518 => YOUNG_LINK_BOW_AIR_FULLY_CHARGED,
                519 => YOUNG_LINK_BOW_AIR_SHOOT,
                520 => FALCO_BLASTER_GROUND_STARTUP,
                521 => FALCO_BLASTER_GROUND_LOOP,
                522 => FALCO_BLASTER_GROUND_END,
                523 => FALCO_BLASTER_AIR_STARTUP,
                524 => FALCO_BLASTER_AIR_LOOP,
                525 => FALCO_BLASTER_AIR_END,
                526 => PICHU_TJOLT_GROUND,
                527 => PICHU_TJOLT_AIR,
                528 => GANON_WARLOCK_PUNCH_GROUND,
                529 => GANON_WARLOCK_PUNCH_AIR,
                530 => ROY_FLARE_BLADE_GROUND_START_CHARGE,
                531 => ROY_FLARE_BLADE_GROUND_CHARGE_LOOP,
                532 => ROY_FLARE_BLADE_GROUND_EARLY_RELEASE,
                533 => ROY_FLARE_BLADE_GROUND_FULLY_CHARGED,
                534 => ROY_FLARE_BLADE_AIR_START_CHARGE,
                535 => ROY_FLARE_BLADE_AIR_CHARGE_LOOP,
                536 => ROY_FLARE_BLADE_AIR_EARLY_RELEASE,
                537 => ROY_FLARE_BLADE_AIR_FULLY_CHARGED,
                538 => GIGA_BOWSER_FIRE_BREATH_GROUND_START,
                539 => GIGA_BOWSER_FIRE_BREATH_GROUND_LOOP,
                540 => GIGA_BOWSER_FIRE_BREATH_GROUND_END,
                541 => GIGA_BOWSER_FIRE_BREATH_AIR_START,
                542 => GIGA_BOWSER_FIRE_BREATH_AIR_LOOP,
                543 => GIGA_BOWSER_FIRE_BREATH_AIR_END,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 341-537 inclusive. Got: {state}"))
            },
            Character::Bowser => match state {
                341 => FIRE_BREATH_GROUND_STARTUP,
                342 => FIRE_BREATH_GROUND_LOOP,
                343 => FIRE_BREATH_GROUND_END,
                344 => FIRE_BREATH_AIR_STARTUP,
                345 => FIRE_BREATH_AIR_LOOP,
                346 => FIRE_BREATH_AIR_END,
                347 => KOOPA_KLAW_GROUND,
                348 => KOOPA_KLAW_GROUND_GRAB,
                349 => KOOPA_KLAW_GROUND_PUMMEL,
                350 => KOOPA_KLAW_GROUND_WAIT,
                351 => KOOPA_KLAW_GROUND_THROW_F,
                352 => KOOPA_KLAW_GROUND_THROW_B,
                353 => KOOPA_KLAW_AIR,
                354 => KOOPA_KLAW_AIR_GRAB,
                355 => KOOPA_KLAW_AIR_PUMMEL,
                356 => KOOPA_KLAW_AIR_WAIT,
                357 => KOOPA_KLAW_AIR_THROW_F,
                358 => KOOPA_KLAW_AIR_THROW_B,
                359 => WHIRLING_FORTRESS_GROUND,
                360 => WHIRLING_FORTRESS_AIR,
                361 => BOMB_GROUND_BEGIN,
                362 => BOMB_AIR,
                363 => BOMB_LAND,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 341-363 inclusive. Got: {state}"))
            },
            Character::Link => match state {
                341 => SIDE_SMASH_2,
                342 => TAUNT_R,
                343 => TAUNT_L,
                344 => BOW_GROUND_CHARGE,
                345 => BOW_GROUND_FULLY_CHARGED,
                346 => BOW_GROUND_SHOOT,
                347 => BOW_AIR_CHARGE,
                348 => BOW_AIR_FULLY_CHARGED,
                349 => BOW_AIR_SHOOT,
                350 => BOOMERANG_GROUND_THROW,
                351 => BOOMERANG_GROUND_CATCH,
                352 => BOOMERANG_GROUND_THROW_EMPTY,
                353 => BOOMERANG_AIR_THROW,
                354 => BOOMERANG_AIR_CATCH,
                355 => BOOMERANG_AIR_THROW_EMPTY,
                356 => SPIN_ATTACK_GROUND,
                357 => SPIN_ATTACK_AIR,
                358 => BOMB_GROUND,
                359 => BOMB_AIR,
                360 => ZAIR,
                361 => ZAIR_CATCH,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 341-361 inclusive. Got: {state}"))
            },
            Character::Luigi => match state {
                341 => FIREBALL_GROUND,
                342 => FIREBALL_AIR,
                343 => GREEN_MISSILE_GROUND_STARTUP,
                344 => GREEN_MISSILE_GROUND_CHARGE,
                346 => GREEN_MISSILE_GROUND_LANDING,
                347 => GREEN_MISSILE_GROUND_TAKEOFF,
                348 => GREEN_MISSILE_GROUND_TAKEOFF_MISFIRE,
                349 => GREEN_MISSILE_AIR_STARTUP,
                350 => GREEN_MISSILE_AIR_CHARGE,
                351 => GREEN_MISSILE_AIR,
                352 => GREEN_MISSILE_AIR_END,
                353 => GREEN_MISSILE_AIR_TAKEOFF,
                354 => GREEN_MISSILE_AIR_TAKEOFF_MISFIRE,
                355 => SUPER_JUMP_PUNCH_GROUND,
                356 => SUPER_JUMP_PUNCH_AIR,
                357 => CYCLONE_GROUND,
                358 => CYCLONE_AIR,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 341-358 inclusive. Got: {state}"))
            },
            Character::Mario => match state {
                341 => TAUNT_R,
                342 => TAUNT_L,
                343 => FIREBALL_GROUND,
                344 => FIREBALL_AIR,
                345 => CAPE_GROUND,
                346 => CAPE_AIR,
                347 => SUPER_JUMP_PUNCH_GROUND,
                348 => SUPER_JUMP_PUNCH_AIR,
                349 => TORNADO_GROUND,
                350 => TORNADO_AIR,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 343-350 inclusive. Got: {state}"))
            },
            Character::Marth => match state {
                341 => SHIELD_BREAKER_GROUND_START_CHARGE,
                342 => SHIELD_BREAKER_GROUND_CHARGE_LOOP,
                343 => SHIELD_BREAKER_GROUND_EARLY_RELEASE,
                344 => SHIELD_BREAKER_GROUND_FULLY_CHARGED,
                345 => SHIELD_BREAKER_AIR_START_CHARGE,
                346 => SHIELD_BREAKER_AIR_CHARGE_LOOP,
                347 => SHIELD_BREAKER_AIR_EARLY_RELEASE,
                348 => SHIELD_BREAKER_AIR_FULLY_CHARGED,
                349 => DANCING_BLADE_1_GROUND,
                350 => DANCING_BLADE_2_UP_GROUND,
                351 => DANCING_BLADE_2_SIDE_GROUND,
                352 => DANCING_BLADE_3_UP_GROUND,
                353 => DANCING_BLADE_3_SIDE_GROUND,
                354 => DANCING_BLADE_3_DOWN_GROUND,
                355 => DANCING_BLADE_4_UP_GROUND,
                356 => DANCING_BLADE_4_SIDE_GROUND,
                357 => DANCING_BLADE_4_DOWN_GROUND,
                358 => DANCING_BLADE_1_AIR,
                359 => DANCING_BLADE_2_UP_AIR,
                360 => DANCING_BLADE_2_SIDE_AIR,
                361 => DANCING_BLADE_3_UP_AIR,
                362 => DANCING_BLADE_3_SIDE_AIR,
                363 => DANCING_BLADE_3_DOWN_AIR,
                364 => DANCING_BLADE_4_UP_AIR,
                365 => DANCING_BLADE_4_SIDE_AIR,
                366 => DANCING_BLADE_4_DOWN_AIR,
                367 => DOLPHIN_SLASH_GROUND,
                368 => DOLPHIN_SLASH_AIR,
                369 => COUNTER_GROUND,
                370 => COUNTER_GROUND_HIT,
                371 => COUNTER_AIR,
                372 => COUNTER_AIR_HIT,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 341-372 inclusive. Got: {state}"))
            },
            Character::Mewtwo => match state {
                341 => SHADOW_BALL_GROUND_START_CHARGE,
                342 => SHADOW_BALL_GROUND_CHARGE_LOOP,
                343 => SHADOW_BALL_GROUND_FULLY_CHARGED,
                344 => SHADOW_BALL_GROUND_END_CHARGE,
                345 => SHADOW_BALL_GROUND_SHOOT,
                346 => SHADOW_BALL_AIR_START_CHARGE,
                347 => SHADOW_BALL_AIR_CHARGE_LOOP,
                348 => SHADOW_BALL_AIR_FULLY_CHARGED,
                349 => SHADOW_BALL_AIR_END_CHARGE,
                350 => SHADOW_BALL_AIR_SHOOT,
                351 => CONFUSION_GROUND,
                352 => CONFUSION_AIR,
                353 => TELEPORT_GROUND_STARTUP,
                354 => TELEPORT_GROUND_DISAPPEAR,
                355 => TELEPORT_GROUND_REAPPEAR,
                356 => TELEPORT_AIR_STARTUP,
                357 => TELEPORT_AIR_DISAPPEAR,
                358 => TELEPORT_AIR_REAPPEAR,
                359 => DISABLE_GROUND,
                360 => DISABLE_AIR,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 341-360 inclusive. Got: {state}"))
            },
            Character::Ness => match state {
                341 => SIDE_SMASH,
                342 => UP_SMASH,
                343 => UP_SMASH_CHARGE,
                344 => UP_SMASH_CHARGED,
                345 => DOWN_SMASH,
                346 => DOWN_SMASH_CHARGE,
                347 => DOWN_SMASH_CHARGED,
                348 => PK_FLASH_GROUND_STARTUP,
                349 => PK_FLASH_GROUND_CHARGE,
                350 => PK_FLASH_GROUND_EXPLODE,
                351 => PK_FLASH_GROUND_END,
                352 => PK_FLASH_AIR_STARTUP,
                353 => PK_FLASH_AIR_CHARGE,
                354 => PK_FLASH_AIR_EXPLODE,
                355 => PK_FLASH_AIR_END,
                356 => PK_FIRE_GROUND,
                357 => PK_FIRE_AIR,
                358 => PK_THUNDER_GROUND_STARTUP,
                359 => PK_THUNDER_GROUND,
                360 => PK_THUNDER_GROUND_END,
                361 => PK_THUNDER_GROUND_HIT,
                362 => PK_THUNDER_AIR_STARTUP,
                363 => PK_THUNDER_AIR,
                364 => PK_THUNDER_AIR_END,
                365 => PK_THUNDER_AIR_HIT,
                366 => PK_THUNDER_AIR_WALL_BOUNCE,
                367 => PSI_MAGNET_GROUND_STARTUP,
                368 => PSI_MAGNET_GROUND_LOOP,
                369 => PSI_MAGNET_GROUND_ABSORB,
                370 => PSI_MAGNET_GROUND_END,
                371 => PSI_MAGNET_GROUND_TURN,
                372 => PSI_MAGNET_AIR_STARTUP,
                373 => PSI_MAGNET_AIR_LOOP,
                374 => PSI_MAGNET_AIR_ABSORB,
                375 => PSI_MAGNET_AIR_END,
                376 => PSI_MAGNET_AIR_TURN,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 341-375 inclusive. Got: {state}"))
            },
            Character::Peach => match state {
                341 => FLOAT,
                342 => FLOAT_FALL_FORWARD,
                343 => FLOAT_FALL_BACKWARD,
                344 => FLOAT_NAIR,
                345 => FLOAT_FAIR,
                346 => FLOAT_BAIR,
                347 => FLOAT_UAIR,
                348 => FLOAT_DAIR,
                349 => SIDE_SMASH_GOLF_CLUB,
                350 => SIDE_SMASH_FRYING_PAN,
                351 => SIDE_SMASH_TENNIS_RACKET,
                352 => VEGETABLE_GROUND,
                353 => VEGETABLE_AIR,
                354 => BOMBER_GROUND_STARTUP,
                355 => BOMBER_GROUND_END,
                356 => BOMBER_GROUND,
                357 => BOMBER_AIR_STARTUP,
                358 => BOMBER_AIR_END,
                359 => BOMBER_AIR_HIT,
                360 => BOMBER_AIR,
                361 => PARASOL_GROUND_START,
                362 => PARASOL_GROUND_END,
                363 => PARASOL_AIR_START,
                364 => PARASOL_AIR_END,
                365 => TOAD_GROUND,
                366 => TOAD_GROUND_ATTACK,
                367 => TOAD_AIR,
                368 => TOAD_AIR_ATTACK,
                369 => PARASOL_OPEN,
                370 => PARASOL_FALL,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 341-370 inclusive. Got: {state}"))
            },
            Character::Pikachu => match state {
                341 => TJOLT_GROUND,
                342 => TJOLT_AIR,
                343 => SKULL_BASH_GROUND_STARTUP,
                344 => SKULL_BASH_GROUND_CHARGE,
                345 => SKULL_BASH_GROUND,
                346 => SKULL_BASH_GROUND_LANDING,
                347 => SKULL_BASH_GROUND_TAKEOFF,
                348 => SKULL_BASH_AIR_STARTUP,
                349 => SKULL_BASH_AIR_CHARGE,
                350 => SKULL_BASH_AIR,
                351 => SKULL_BASH_AIR_END,
                352 => SKULL_BASH_AIR_TAKEOFF,
                353 => QUICK_ATTACK_GROUND_STARTUP,
                354 => QUICK_ATTACK_GROUND,
                355 => QUICK_ATTACK_GROUND_END,
                356 => QUICK_ATTACK_AIR_STARTUP,
                357 => QUICK_ATTACK_AIR,
                358 => QUICK_ATTACK_AIR_END,
                359 => THUNDER_GROUND_STARTUP,
                360 => THUNDER_GROUND,
                361 => THUNDER_GROUND_HIT,
                362 => THUNDER_GROUND_END,
                363 => THUNDER_AIR_STARTUP,
                364 => THUNDER_AIR,
                365 => THUNDER_AIR_HIT,
                366 => THUNDER_AIR_END,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 341-366 inclusive. Got: {state}"))
            },
            Character::Jigglypuff => match state {
                341 => JUMP_2,
                342 => JUMP_3,
                343 => JUMP_4,
                344 => JUMP_5,
                345 => JUMP_6,
                346 => ROLLOUT_GROUND_START_CHARGE_RIGHT,
                347 => ROLLOUT_GROUND_START_CHARGE_LEFT,
                348 => ROLLOUT_GROUND_CHARGE_LOOP,
                349 => ROLLOUT_GROUND_FULLY_CHARGED,
                350 => ROLLOUT_GROUND_CHARGE_RELEASE,
                351 => ROLLOUT_GROUND_START_TURN,
                352 => ROLLOUT_GROUND_END_RIGHT,
                353 => ROLLOUT_GROUND_END_LEFT,
                354 => ROLLOUT_AIR_START_CHARGE_RIGHT,
                355 => ROLLOUT_AIR_START_CHARGE_LEFT,
                356 => ROLLOUT_AIR_CHARGE_LOOP,
                357 => ROLLOUT_AIR_FULLY_CHARGED,
                358 => ROLLOUT_AIR_CHARGE_RELEASE,
                360 => ROLLOUT_AIR_END_RIGHT,
                361 => ROLLOUT_AIR_END_LEFT,
                362 => ROLLOUT_HIT,
                363 => POUND_GROUND,
                364 => POUND_AIR,
                365 => SING_GROUND_LEFT,
                366 => SING_AIR_LEFT,
                367 => SING_GROUND_RIGHT,
                368 => SING_AIR_RIGHT,
                369 => REST_GROUND_LEFT,
                370 => REST_AIR_LEFT,
                371 => REST_GROUND_RIGHT,
                372 => REST_AIR_RIGHT,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 341-372 inclusive. Got: {state}"))
            },
            Character::Samus => match state {
                341 => BOMB_JUMP_GROUND,
                342 => BOMB_JUMP_AIR,
                343 => CHARGE_SHOT_GROUND_START,
                344 => CHARGE_SHOT_GROUND_LOOP,
                345 => CHARGE_SHOT_GROUND_END,
                346 => CHARGE_SHOT_GROUND_SHOOT,
                347 => CHARGE_SHOT_AIR_START,
                348 => CHARGE_SHOT_AIR_SHOOT,
                349 => MISSILE_GROUND,
                350 => MISSILE_SMASH_GROUND,
                351 => MISSILE_AIR,
                352 => MISSILE_SMASH_AIR,
                353 => SCREW_ATTACK_GROUND,
                354 => SCREW_ATTACK_AIR,
                355 => BOMB_END_GROUND,
                356 => BOMB_AIR,
                357 => ZAIR,
                358 => ZAIR_CATCH,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 341-358 inclusive. Got: {state}"))
            },
            Character::Yoshi => match state {
                341 => GUARD_ON,
                342 => GUARD,
                343 => GUARD_OFF,
                344 => GUARD_DAMAGE,
                345 => GUARD_ON_2,
                346 => EGG_LAY_GROUND,
                347 => EGG_LAY_GROUND_CAPTURE_START,
                348 => EGG_LAY_GROUND_CAPTURE_START_2,
                349 => EGG_LAY_GROUND_CAPTURE,
                350 => EGG_LAY_GROUND_CAPTURE_2,
                351 => EGG_LAY_AIR,
                352 => EGG_LAY_AIR_CAPTURE_START,
                353 => EGG_LAY_AIR_CAPTURE_START_2,
                354 => EGG_LAY_AIR_CAPTURE,
                355 => EGG_LAY_AIR_CAPTURE_2,
                356 => EGG_ROLL_GROUND_STARTUP,
                357 => EGG_ROLL_GROUND,
                358 => EGG_ROLL_GROUND_CHANGE_DIRECTION,
                359 => EGG_ROLL_GROUND_END,
                360 => EGG_ROLL_AIR_START,
                361 => EGG_ROLL_AIR,
                362 => EGG_ROLL_BOUNCE,
                363 => EGG_ROLL_AIR_END,
                364 => EGG_THROW_GROUND,
                365 => EGG_THROW_AIR,
                366 => BOMB_GROUND,
                367 => BOMB_LAND,
                368 => BOMB_AIR,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 342-368 inclusive. Got: {state}"))
            },
            Character::Zelda => match state {
                341 => NAYRUS_LOVE_GROUND,
                342 => NAYRUS_LOVE_AIR,
                343 => DINS_FIRE_GROUND_STARTUP,
                344 => DINS_FIRE_GROUND_TRAVEL,
                345 => DINS_FIRE_GROUND_EXPLODE,
                346 => DINS_FIRE_AIR_STARTUP,
                347 => DINS_FIRE_AIR_TRAVEL,
                348 => DINS_FIRE_AIR_EXPLODE,
                349 => FARORES_WIND_GROUND,
                350 => FARORES_WIND_GROUND_DISAPPEAR,
                351 => FARORES_WIND_GROUND_REAPPEAR,
                352 => FARORES_WIND_AIR,
                353 => FARORES_WIND_AIR_DISAPPEAR,
                354 => FARORES_WIND_AIR_REAPPEAR,
                355 => TRANSFORM_GROUND,
                356 => TRANSFORM_GROUND_ENDING,
                357 => TRANSFORM_AIR,
                358 => TRANSFORM_AIR_ENDING,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 341-358 inclusive. Got: {state}"))
            },
            Character::Sheik => match state {
                341 => NEEDLE_GROUND_START_CHARGE,
                342 => NEEDLE_GROUND_CHARGE_LOOP,
                343 => NEEDLE_GROUND_END_CHARGE,
                344 => NEEDLE_GROUND_FIRE,
                345 => NEEDLE_AIR_START_CHARGE,
                346 => NEEDLE_AIR_CHARGE_LOOP,
                347 => NEEDLE_AIR_END_CHARGE,
                348 => NEEDLE_AIR_FIRE,
                349 => CHAIN_GROUND_STARTUP,
                350 => CHAIN_GROUND_LOOP,
                351 => CHAIN_GROUND_END,
                352 => CHAIN_AIR_STARTUP,
                353 => CHAIN_AIR_LOOP,
                354 => CHAIN_AIR_END,
                355 => VANISH_GROUND_STARTUP,
                356 => VANISH_GROUND_DISAPPEAR,
                357 => VANISH_GROUND_REAPPEAR,
                358 => VANISH_AIR_STARTUP,
                359 => VANISH_AIR_DISAPPEAR,
                360 => VANISH_AIR_REAPPEAR,
                361 => TRANSFORM_GROUND,
                362 => TRANSFORM_GROUND_ENDING,
                363 => TRANSFORM_AIR,
                364 => TRANSFORM_AIR_ENDING,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 341-364 inclusive. Got: {state}"))
            },
            Character::Falco => match state {
                341 => BLASTER_GROUND_STARTUP,
                342 => BLASTER_GROUND_LOOP,
                343 => BLASTER_GROUND_END,
                344 => BLASTER_AIR_STARTUP,
                345 => BLASTER_AIR_LOOP,
                346 => BLASTER_AIR_END,
                347 => PHANTASM_GROUND_STARTUP,
                348 => PHANTASM_GROUND,
                349 => PHANTASM_GROUND_END,
                350 => PHANTASM_STARTUP_AIR,
                351 => PHANTASM_AIR,
                352 => PHANTASM_AIR_END,
                353 => FIRE_BIRD_GROUND_STARTUP,
                354 => FIRE_BIRD_AIR_STARTUP,
                355 => FIRE_BIRD_GROUND,
                356 => FIRE_BIRD_AIR,
                357 => FIRE_BIRD_GROUND_END,
                358 => FIRE_BIRD_AIR_END,
                359 => FIRE_BIRD_BOUNCE_END,
                360 => REFLECTOR_GROUND_STARTUP,
                361 => REFLECTOR_GROUND_LOOP,
                362 => REFLECTOR_GROUND_REFLECT,
                363 => REFLECTOR_GROUND_END,
                364 => REFLECTOR_GROUND_TURN,
                365 => REFLECTOR_AIR_STARTUP,
                366 => REFLECTOR_AIR_LOOP,
                367 => REFLECTOR_AIR_REFLECT,
                368 => REFLECTOR_AIR_END,
                369 => REFLECTOR_AIR_TURN,
                370 => SMASH_TAUNT_RIGHT_STARTUP,
                371 => SMASH_TAUNT_LEFT_STARTUP,
                372 => SMASH_TAUNT_RIGHT_RISE,
                373 => SMASH_TAUNT_LEFT_RISE,
                374 => SMASH_TAUNT_RIGHT_FINISH,
                375 => SMASH_TAUNT_LEFT_FINISH,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 341-375 inclusive. Got: {state}"))
            },
            Character::YoungLink => match state {
                341 => SIDE_SMASH_2,
                342 => TAUNT_R,
                343 => TAUNT_L,
                344 => BOW_GROUND_CHARGE,
                345 => BOW_GROUND_FULLY_CHARGED,
                346 => BOW_GROUND_SHOOT,
                347 => BOW_AIR_CHARGE,
                348 => BOW_AIR_FULLY_CHARGED,
                349 => BOW_AIR_SHOOT,
                350 => BOOMERANG_GROUND_THROW,
                351 => BOOMERANG_GROUND_CATCH,
                352 => BOOMERANG_GROUND_THROW_EMPTY,
                353 => BOOMERANG_AIR_THROW,
                354 => BOOMERANG_AIR_CATCH,
                355 => BOOMERANG_AIR_THROW_EMPTY,
                356 => SPIN_ATTACK_GROUND,
                357 => SPIN_ATTACK_AIR,
                358 => BOMB_GROUND,
                359 => BOMB_AIR,
                360 => ZAIR,
                361 => ZAIR_CATCH,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 341-361 inclusive. Got: {state}"))
            },
            Character::DrMario => match state {
                341 => TAUNT_R,
                342 => TAUNT_L,
                343 => PILL_GROUND,
                344 => PILL_AIR,
                345 => SUPER_SHEET_GROUND,
                346 => SUPER_SHEET_AIR,
                347 => SUPER_JUMP_PUNCH_GROUND,
                348 => SUPER_JUMP_PUNCH_AIR,
                349 => TORNADO_GROUND,
                350 => TORNADO_AIR,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 341-350 inclusive. Got: {state}"))
            },
            Character::Roy => match state {
                341 => FLARE_BLADE_GROUND_START_CHARGE,
                342 => FLARE_BLADE_GROUND_CHARGE_LOOP,
                343 => FLARE_BLADE_GROUND_EARLY_RELEASE,
                344 => FLARE_BLADE_GROUND_FULLY_CHARGED,
                345 => FLARE_BLADE_AIR_START_CHARGE,
                346 => FLARE_BLADE_AIR_CHARGE_LOOP,
                347 => FLARE_BLADE_AIR_EARLY_RELEASE,
                348 => FLARE_BLADE_AIR_FULLY_CHARGED,
                349 => DOUBLE_EDGE_DANCE_1_GROUND,
                350 => DOUBLE_EDGE_DANCE_2_UP_GROUND,
                351 => DOUBLE_EDGE_DANCE_2_SIDE_GROUND,
                352 => DOUBLE_EDGE_DANCE_3_UP_GROUND,
                353 => DOUBLE_EDGE_DANCE_3_SIDE_GROUND,
                354 => DOUBLE_EDGE_DANCE_3_DOWN_GROUND,
                355 => DOUBLE_EDGE_DANCE_4_UP_GROUND,
                356 => DOUBLE_EDGE_DANCE_4_SIDE_GROUND,
                357 => DOUBLE_EDGE_DANCE_4_DOWN_GROUND,
                358 => DOUBLE_EDGE_DANCE_1_AIR,
                359 => DOUBLE_EDGE_DANCE_2_UP_AIR,
                360 => DOUBLE_EDGE_DANCE_2_SIDE_AIR,
                361 => DOUBLE_EDGE_DANCE_3_UP_AIR,
                362 => DOUBLE_EDGE_DANCE_3_SIDE_AIR,
                363 => DOUBLE_EDGE_DANCE_3_DOWN_AIR,
                364 => DOUBLE_EDGE_DANCE_4_UP_AIR,
                365 => DOUBLE_EDGE_DANCE_4_SIDE_AIR,
                366 => DOUBLE_EDGE_DANCE_4_DOWN_AIR,
                367 => BLAZER_GROUND,
                368 => BLAZER_AIR,
                369 => COUNTER_GROUND,
                370 => COUNTER_GROUND_HIT,
                371 => COUNTER_AIR,
                372 => COUNTER_AIR_HIT,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 341-372 inclusive. Got: {state}"))
            },
            Character::Pichu => match state {
                341 => TJOLT_GROUND,
                342 => TJOLT_AIR,
                343 => SKULL_BASH_GROUND_STARTUP,
                344 => SKULL_BASH_GROUND_CHARGE,
                345 => SKULL_BASH_GROUND,
                346 => SKULL_BASH_GROUND_LANDING,
                347 => SKULL_BASH_GROUND_TAKEOFF,
                348 => SKULL_BASH_AIR_STARTUP,
                349 => SKULL_BASH_AIR_CHARGE,
                350 => SKULL_BASH_AIR,
                351 => SKULL_BASH_AIR_END,
                352 => SKULL_BASH_AIR_TAKEOFF,
                353 => AGILITY_GROUND_STARTUP,
                354 => AGILITY_GROUND,
                355 => AGILITY_GROUND_END,
                356 => AGILITY_AIR_STARTUP,
                357 => AGILITY_AIR,
                358 => AGILITY_AIR_END,
                359 => THUNDER_GROUND_STARTUP,
                360 => THUNDER_GROUND,
                361 => THUNDER_GROUND_HIT,
                362 => THUNDER_GROUND_END,
                363 => THUNDER_AIR_STARTUP,
                364 => THUNDER_AIR,
                365 => THUNDER_AIR_HIT,
                366 => THUNDER_AIR_END,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 341-366 inclusive. Got: {state}"))
            },
            Character::Ganondorf => match state {
                341 => SWORD_SWING_4,
                342 => BAT_SWING_4,
                343 => PARASOL_SWING_4,
                344 => FAN_SWING_4,
                345 => STAR_ROD_SWING_4,
                346 => LIPSTICK_SWING_4,
                347 => WARLOCK_PUNCH_GROUND,
                348 => WARLOCK_PUNCH_AIR,
                349 => GERUDO_DRAGON_GROUND,
                350 => GERUDO_DRAGON_GROUND_HIT,
                351 => GERUDO_DRAGON_AIR,
                352 => GERUDO_DRAGON_AIR_HIT,
                353 => DARK_DIVE_GROUND,
                354 => DARK_DIVE_AIR,
                355 => DARK_DIVE_CATCH,
                356 => DARK_DIVE_ENDING,
                357 => WIZARDS_FOOT_GROUND,
                358 => WIZARDS_FOOT_GROUND_ENDING_ON_GROUND,
                359 => WIZARDS_FOOT_AIR,
                360 => WIZARDS_FOOT_AIR_ENDING_ON_GROUND,
                361 => WIZARDS_FOOT_AIR_ENDING_IN_AIR,
                362 => WIZARDS_FOOT_GROUND_ENDING_IN_AIR,
                363 => WIZARDS_FOOT_HIT_WALL,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 347-363 inclusive. Got: {state}"))
            },

            Character::Popo | Character::IceClimbers | Character::Nana => match state {
                341 => ICE_SHOT_GROUND,
                342 => ICE_SHOT_AIR,
                343 => SQUALL_HAMMER_GROUND_SOLO,
                344 => SQUALL_HAMMER_GROUND_TOGETHER,
                345 => SQUALL_HAMMER_AIR_SOLO,
                346 => SQUALL_HAMMER_AIR_TOGETHER,
                347 => BELAY_GROUND_STARTUP,
                348 => BELAY_GROUND_CATAPULTING_NANA,
                349 => BELAY_GROUND_CATAPULTING,
                350 => BELAY_GROUND_FAILED_CATAPULTING,
                351 => BELAY_GROUND_FAILED_CATAPULTING_END,
                352 => BELAY_AIR_STARTUP,
                353 => BELAY_AIR_CATAPULTING_NANA,
                354 => BELAY_CATAPULTING,
                355 => BELAY_AIR_FAILED_CATAPULTING,
                356 => BELAY_AIR_FAILED_CATAPULTING_END,
                357 => BLIZZARD_GROUND,
                358 => BLIZZARD_AIR,
                359 => SQUALL_HAMMER_GROUND_TOGETHER,
                360 => SQUALL_HAMMER_AIR_TOGETHER,
                361 => BELAY_CATAPULT_STARTUP,
                362 => BELAY_GROUND_CATAPULT_END,
                363 => BELAY_2,
                364 => BELAY_3,
                365 => BELAY_CATAPULTING,
                366 => BELAY_5,
                _ => return Err(anyhow!("Invalid state value for {character}. Value must be in range 341-358 inclusive. Got: {state}"))
            },
            Character::MasterHand |
            Character::WireframeMale |
            Character::WireframeFemale |
            Character::GigaBowser |
            Character::CrazyHand |
            Character::Sandbag => return Err(anyhow!("Invalid state value for {character}. No state specific information available")),
        })
    }
}
