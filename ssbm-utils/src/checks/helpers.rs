use itertools::Itertools;

use crate::enums::general::*;
use crate::enums::stage::Stage;
use crate::enums::state::ActionRange as AR;
use crate::enums::{buttons::*, general::*, state::*};
use crate::utils::BitFlags;

/// Returns true if the current state is different from the previous state
///
/// Minimum Slippi Version: 0.1.0
pub fn just_changed_state(curr_state: u16, prev_state: u16) -> bool {
    curr_state != prev_state
}

/// Returns true if the current state is the target state and the previous state isn't the target state
///
/// Minimum Slippi Version: 0.1.0
pub fn just_entered_state(target: ActionState, curr_state: u16, prev_state: u16) -> bool {
    curr_state == target as u16 && prev_state != target as u16
}

/// Returns true if the current state isn't the target state and the previous state is the target state
///
/// Minimum Slippi Version: 0.1.0
pub fn just_exited_state(target: ActionState, curr_state: u16, prev_state: u16) -> bool {
    curr_state != target as u16 && prev_state == target as u16
}

/// Returns true if the character input anything that would trigger an L cancel (L Digital, R Digital, Z, or analog
/// trigger)
///
/// Minimum Slippi Version: 0.1.0
pub fn just_input_lcancel(curr_frame: u32, prev_frame: u32) -> bool {
    let current = EngineInput::from(curr_frame);
    let previous = EngineInput::from(prev_frame);

    let mask = EngineInput::ANY_TRIGGER;

    current.contains(*mask) && !previous.contains(*mask)
}

/// Returns true if the current percent is higher than the previous percent
///
/// Minimum Slippi Version: 0.1.0
pub fn just_took_damage(curr_percent: f32, prev_percent: f32) -> bool {
    curr_percent > prev_percent
}

/// Returns the difference between current percent and previous percent, clamped to 0 to prevent respawn from returning
/// negative values
///
/// Minimum Slippi Version: 0.1.0
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
        if !Flags::from(*flagset).contains(*Flags::OFFSCREEN) {
            return false;
        }
    }
    true
}

/// Returns true if the character has the hitlag bitflag active
///
/// Minimum Slippi Version: 2.0.0 - Post-frame Bitflags
pub fn is_in_hitlag(flags: u64) -> bool {
    Flags::from(flags).contains(*Flags::HITLAG)
}

/// Returns true if the character has the hitstun bitflag active
///
/// Minimum Slippi Version: 2.0.0 - Post-frame Bitflags
pub fn is_in_hitstun(flags: u64) -> bool {
    Flags::from(flags).contains(*Flags::HITSTUN)
}

/// Returns true if the character has the defender-hitlag bitflag active
///
/// Minimum Slippi Version: 2.0.0 - Post-frame Bitflags
pub fn is_in_defender_hitlag(flags: u64) -> bool {
    Flags::from(flags).contains(*Flags::DEFENDER_HITLAG)
}

/// Returns true if the character has the magnifying glass bitflag active
///
/// Minimum Slippi Version: 2.0.0 - Post-frame Bitflags
pub fn is_in_magnifying_glass(flags: u64) -> bool {
    Flags::from(flags).contains(*Flags::OFFSCREEN)
}

/// Returns true if the character has the shielding bitflag active
///
/// Minimum Slippi Version: 2.0.0 - Post-frame Bitflags
pub fn is_shielding_flag(flags: u64) -> bool {
    Flags::from(flags).contains(*Flags::SHIELDING)
}

/// Returns true if the character has the fastfall bitflag active
///
/// Minimum Slippi Version: 2.0.0 - Post-frame Bitflags
pub fn is_fastfalling(flags: u64) -> bool {
    Flags::from(flags).contains(*Flags::FASTFALL)
}

/// Returns true if the character is in any tumble or reeling animation, or if they are in the jab reset animation
///
/// Minimum Slippi Version: 0.1.0
pub fn is_damaged(state: u16) -> bool {
    (AR::DAMAGE_START as u16..=AR::DAMAGE_END as u16).contains(&state)
        || ActionState::DOWN_DAMAGE_D == state
        || ActionState::DOWN_DAMAGE_U == state
}

/// Returns true if the character is in any Capture animations. See also `is_cmd_grabbed`
///
/// Minimum Slippi Version: 0.1.0
pub fn is_grabbed(state: u16) -> bool {
    (AR::CAPTURE_START as u16..=AR::CAPTURE_END as u16).contains(&state)
}

/// Returns true if the character is in any command grab state
///
/// Minimum Slippi Version: 0.1.0
pub fn is_cmd_grabbed(state: u16) -> bool {
    ActionState::BARREL_WAIT != state
        && ((AR::COMMAND_GRAB_RANGE1_START as u16..=AR::COMMAND_GRAB_RANGE1_END as u16)
            .contains(&state)
            || (AR::COMMAND_GRAB_RANGE2_START as u16..=AR::COMMAND_GRAB_RANGE2_END as u16)
                .contains(&state))
}

/// Returns true if the character is in any teching or downed state
///
/// Minimum Slippi Version: 0.1.0
pub fn is_teching(state: u16) -> bool {
    (AR::TECH_START as u16..=AR::TECH_END as u16).contains(&state)
        || (AR::DOWN_START as u16..=AR::DOWN_END as u16).contains(&state)
        || ActionState::FLY_REFLECT_CEIL == state
        || ActionState::FLY_REFLECT_WALL == state
}
