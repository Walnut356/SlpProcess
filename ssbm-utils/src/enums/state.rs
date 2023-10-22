#![allow(non_camel_case_types)]

use strum_macros::{Display, EnumString, FromRepr, IntoStaticStr};

/// Individual Action State IDs. See ActionRange for state ranges.
///
/// ID's match debug mode names, see docstrings for additional context
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, EnumString, IntoStaticStr, Display, FromRepr,
)]
#[repr(u16)]
pub enum ActionState {
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
    KINOKO_GIANT_START = 314,
    /// super mushroom
    KINOKO_GIANT_START_AIR = 315,
    /// super mushroom
    KINOKO_GIANT_END = 316,
    /// super mushroom
    KINOKO_GIANT_END_AIR = 317,
    /// poison mushroom
    KINOKO_SMALL_START = 318,
    /// poison mushroom
    KINOKO_SMALL_START_AIR = 319,
    /// poison mushroom
    KINOKO_SMALL_END = 320,
    /// poison mushroom
    KINOKO_SMALL_END_AIR = 321,

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

/// Used to simplify checks for clusters of action states
///
/// ranges are inclusive, comparisons should be GT/Eq or LT/Eq:
/// ```
/// ActionRange::AERIAL_ATTACK_START <= x <= ActionRange::AERIAL_ATTACK_END;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, FromRepr)]
#[repr(u16)]
pub enum ActionRange {
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


// TODO character-specific action states