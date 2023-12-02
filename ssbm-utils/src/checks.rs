use num_traits::PrimInt;

use crate::enums::ActionRange as AR;
use crate::enums::*;

/// Returns true if the current state is different from the previous state
///
/// Minimum Slippi Version: 0.1.0
#[inline]
pub fn just_changed_state(curr_state: u16, prev_state: u16) -> bool {
    curr_state != prev_state
}

/// Returns true if the current state is the target state and the previous state isn't the target state
///
/// Minimum Slippi Version: 0.1.0
#[inline]
pub fn just_entered_state(target: ActionState, curr_state: u16, prev_state: u16) -> bool {
    curr_state == target && prev_state != target
}

/// Returns true if the current state isn't the target state and the previous state is the target state
///
/// Minimum Slippi Version: 0.1.0
#[inline]
pub fn just_exited_state(target: ActionState, curr_state: u16, prev_state: u16) -> bool {
    curr_state != target && prev_state == target
}

/// Returns true if the character input anything that would trigger an L cancel (L Digital, R Digital, Z, or analog
/// trigger)
///
/// Minimum Slippi Version: 0.1.0
#[inline]
pub fn just_input_lcancel(current: u32, prev: u32) -> bool {
    EngineInput::ANY_TRIGGER.contained_by(current) && !EngineInput::ANY_TRIGGER.contained_by(prev)
}

/// Returns true if the current percent is higher than the previous percent
///
/// Minimum Slippi Version: 0.1.0
#[inline]
pub fn just_took_damage(curr_percent: f32, prev_percent: f32) -> bool {
    curr_percent > prev_percent
}

/// Returns the difference between current percent and previous percent, clamped to 0 to prevent respawn from returning
/// negative values
///
/// Minimum Slippi Version: 0.1.0
#[inline]
pub fn get_damage_taken(curr_percent: f32, prev_percent: f32) -> f32 {
    (curr_percent - prev_percent).max(0.0)
}

/// Returns true if the character took exactly 1% and were in the magnifying glass for the 60 previous frames
///
/// Minimum Slippi Version: 2.0.0 - Post-frame Bitflags
pub fn is_magnifying_damage(damage_taken: f32, flags: &[u64], index: usize) -> bool {
    if damage_taken != 1.0 {
        return false;
    }

    let min = index.saturating_sub(60);

    for flagset in &flags[min..=index] {
        if !Flags::OFFSCREEN.intersects(*flagset) {
            return false;
        }
    }
    true
}

/// Returns true if the character has the hitlag bitflag active
///
/// Minimum Slippi Version: 2.0.0 - Post-frame Bitflags
#[inline]
pub fn is_in_hitlag(flags: u64) -> bool {
    Flags::HITLAG.intersects(flags)
}

/// Returns true if the character has the hitstun bitflag active
///
/// Minimum Slippi Version: 2.0.0 - Post-frame Bitflags
#[inline]
pub fn is_in_hitstun(flags: u64) -> bool {
    Flags::HITSTUN.intersects(flags)
}

/// Returns true if the character has the defender-hitlag bitflag active
///
/// Minimum Slippi Version: 2.0.0 - Post-frame Bitflags
#[inline]
pub fn is_in_defender_hitlag(flags: u64) -> bool {
    Flags::DEFENDER_HITLAG.intersects(flags)
}

/// Returns true if the character has the magnifying glass bitflag active
///
/// Minimum Slippi Version: 2.0.0 - Post-frame Bitflags
#[inline]
pub fn is_in_magnifying_glass(flags: u64) -> bool {
    Flags::OFFSCREEN.intersects(flags)
}

/// Returns true if the character has the shielding bitflag active
///
/// Minimum Slippi Version: 2.0.0 - Post-frame Bitflags
#[inline]
pub fn is_shielding_flag(flags: u64) -> bool {
    Flags::SHIELDING.intersects(flags)
}

/// Returns true if the character has the fastfall bitflag active
///
/// Minimum Slippi Version: 2.0.0 - Post-frame Bitflags
#[inline]
pub fn is_fastfalling(flags: u64) -> bool {
    Flags::FASTFALL.intersects(flags)
}

/// Returns true if the character is in any tumble or reeling animation, or if they are in the jab reset animation
///
/// Minimum Slippi Version: 0.1.0
#[inline]
pub fn is_damaged(state: u16) -> bool {
    (AR::DAMAGE_START..=AR::DAMAGE_END).contains(&state)
        || ActionState::DAMAGE_FALL == state
        || ActionState::DOWN_DAMAGE_D == state
        || ActionState::DOWN_DAMAGE_U == state
}

/// Returns true if the character is in any Capture animations. See also `is_cmd_grabbed`
///
/// Minimum Slippi Version: 0.1.0
#[inline]
pub fn is_grabbed(state: u16) -> bool {
    (AR::CAPTURE_START..=AR::CAPTURE_END).contains(&state)
}

/// Returns true if the character is in any command grab state
///
/// Minimum Slippi Version: 0.1.0
#[inline]
pub fn is_cmd_grabbed(state: u16) -> bool {
    ActionState::BARREL_WAIT != state
        && ((AR::COMMAND_GRAB_RANGE1_START..=AR::COMMAND_GRAB_RANGE1_END).contains(&state)
            || (AR::COMMAND_GRAB_RANGE2_START..=AR::COMMAND_GRAB_RANGE2_END).contains(&state))
}

/// Returns true if the character is in any teching state. Does not included downed states.
/// For Downed states, see `is_downed()`
///
/// Minimum Slippi Version: 0.1.0
#[inline]
pub fn is_teching(state: u16) -> bool {
    (AR::TECH_START..=AR::TECH_END).contains(&state)
        // || (AR::DOWN_START..=AR::DOWN_END).contains(&state)
        || ActionState::FLY_REFLECT_CEIL == state
        || ActionState::FLY_REFLECT_WALL == state
}

/// Returns true if the character is in any owned state
///
/// Minimum Slippi Version: 0.1.0
#[inline]
pub fn is_downed(state: u16) -> bool {
    (AR::DOWN_START..=AR::DOWN_END).contains(&state)
}

/// Returns true if the character is currently being thrown
///
/// Minimum Slippi Version: 0.1.0
#[inline]
pub fn is_thrown(state: u16) -> bool {
    (AR::THROWN_START..=AR::THROWN_END).contains(&state)
}

/// Returns true if the character is currently in a dying state (blast zone explosion, star KO, etc)
///
/// Minimum Slippi Version: 0.1.0
#[inline]
pub fn is_dying(state: u16) -> bool {
    (AR::DYING_START..=AR::DYING_END).contains(&state)
}

/// Returns true if the character is currently rolling or spot dodging
///
/// Minimum Slippi Version: 0.1.0
#[inline]
pub fn is_dodging(state: u16) -> bool {
    // intionally not `..=` due to leaving out airdodging
    (AR::DODGE_START..AR::DODGE_END).contains(&state)
}

#[inline]
pub fn is_shielding(state: u16) -> bool {
    (AR::GUARD_START..=AR::GUARD_END).contains(&state)
}

#[inline]
pub fn is_shield_broken(state: u16) -> bool {
    (AR::GUARD_BREAK_START..=AR::GUARD_BREAK_END).contains(&state)
}

/// Returns trie if the character is currently hanging from the ledge or performing any ledge action
///
/// Minimum Slippi Version: 0.1.0
#[inline]
pub fn is_ledge_action(state: u16) -> bool {
    (AR::LEDGE_ACTION_START..=AR::LEDGE_ACTION_END).contains(&state)
}

#[inline]
pub fn is_special_fall(state: u16) -> bool {
    (AR::FALL_SPECIAL_START..=AR::FALL_SPECIAL_END).contains(&state)
}

#[inline]
pub fn is_upb_lag(state: u16, prev_state: u16) -> bool {
    // TODO verify this more
    state == ActionState::LAND_FALL_SPECIAL
        && prev_state != ActionState::LAND_FALL_SPECIAL
        && prev_state != ActionState::KNEE_BEND
        && prev_state != ActionState::ESCAPE_AIR
}

#[inline]
pub fn lost_stock(current: u8, prev: u8) -> bool {
    current < prev
}

/// Returns true if **any** of the target bits appear in the current frame, but **do not** appear in
/// the previous frame. See alse: `just_pressed_all`
#[inline]
pub fn just_pressed_any<T: PrimInt>(
    target: impl BitFlags<Other = T> + Buttons,
    current: T,
    prev: T,
) -> bool {
    target.intersects(current) && !target.intersects(prev)
}

/// Returns true if **all** of the target bits appear in the current frame, but **do not** appear in
/// the previous frame. See alse: `just_pressed_any`
#[inline]
pub fn just_pressed_all<T: PrimInt>(
    target: impl BitFlags<Other = T> + Buttons,
    current: T,
    prev: T,
) -> bool {
    target.contained_by(current) && !target.contained_by(prev)
}

/// Returns true if the character's attack has the `electric` property (affects hitlag duration).
/// Does not include grabs (e.g. falco dthrow, pikachu bthrow)
pub fn is_electric_attack(attack: Attack, character: &Character) -> bool {
    match character {
        Character::CaptainFalcon => attack == Attack::FAIR,
        Character::DrMario => attack == Attack::F_SMASH,
        Character::Falco | Character::Fox => attack == Attack::DOWN_SPECIAL,
        Character::Ganondorf => [Attack::JAB_1, Attack::DAIR, Attack::UP_SPECIAL].contains(&attack),
        Character::Mewtwo => [Attack::NAIR, Attack::PUMMEL].contains(&attack),
        Character::Ness => [Attack::BAIR, Attack::DASH_ATTACK, Attack::FAIR].contains(&attack),
        Character::Pichu => {
            [Attack::DAIR, Attack::F_SMASH, Attack::PUMMEL, Attack::FAIR].contains(&attack)
        }
        Character::Pikachu => [
            Attack::DAIR,
            Attack::D_SMASH,
            Attack::FAIR,
            Attack::F_SMASH,
            Attack::PUMMEL,
        ]
        .contains(&attack),
        // samus Zair is countaed as NonStaling, nothing else she does aside from banned items are
        // NonStaling, so this shouldn't have false positives
        Character::Samus => [
            Attack::NON_STALING,
            Attack::NEUTRAL_SPECIAL,
            Attack::UP_SPECIAL,
        ]
        .contains(&attack),
        Character::Zelda => [
            Attack::BAIR,
            Attack::DASH_ATTACK,
            Attack::FAIR,
            Attack::F_SMASH,
            Attack::JAB_1,
            Attack::PUMMEL,
            Attack::U_SMASH,
            Attack::U_TILT,
        ]
        .contains(&attack),
        _ => false,
    }
}

/// Returns true if the player is in a state that allows v-cancelling. See [this post](https://old.reddit.com/r/SSBM/comments/3n1kgf/new_global_technique_discovery_vcanceling_reduce/)
/// for more details
pub fn is_vcancel_state(state: u16) -> bool {
    (ActionState::JUMP_F..=ActionState::DAMAGE_FALL).contains(&state)
        || state == ActionState::ESCAPE_AIR
}

// TODO get_randall_position() https://github.com/altf4/libmelee/blob/c98c26b776a0ad5024efa81487ae6a0ce27b6ab5/melee/stages.py#L160
