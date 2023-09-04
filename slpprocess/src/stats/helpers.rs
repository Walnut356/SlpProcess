use itertools::Itertools;

use crate::enums::general::*;
use crate::enums::stage::Stage;
use crate::enums::state::ActionRange as AR;
use crate::enums::{buttons::*, general::*, state::*};
use crate::utils::BitFlags;

/// takes `PostFrames.action_state` and a frame index, returns true if
pub fn just_changed_state(states: &[u16], index: usize) -> bool {
    states[index] != states[index - 1]
}

pub fn just_entered_state(target: ActionState, states: &[u16], index: usize) -> bool {
    states[index] == target as u16 && states[index - 1] != target as u16
}

pub fn just_exited_state(target: ActionState, states: &[u16], index: usize) -> bool {
    states[index] != target as u16 && states[index - 1] == target as u16
}

pub fn just_input_lcancel(frames: &[u32], index: usize) -> bool {
    let current = EngineInput::from(frames[index]);
    let previous = EngineInput::from(frames[index.saturating_sub(1)]);

    let mask: u32 = EngineInput::ANY_TRIGGER.into();

    current.contains(mask) && !previous.contains(mask)
}

/// Returns true if the percent value changed between frame[index - 1] and frame
pub fn just_took_damage(percent_frames: &[f32], index: usize) -> bool {
    if index == 0 {
        return false;
    }
    percent_frames[index] > percent_frames[index - 1]
}

pub fn get_damage_taken(percent_frames: &[f32], index: usize) -> f32 {
    (percent_frames[index] - percent_frames[index - 1]).max(0.0)
}

pub fn is_magnifying_damage(damage_taken: f32, flags: &[u64], index: usize) -> bool {
    if damage_taken != 1.0 {
        return false;
    }

    let min = index.saturating_sub(60);

    for flagset in &flags[min..=index] {
        if !Flags::from(*flagset).contains(Flags::OFFSCREEN.into()) {
            return false;
        }
    }
    return true;
}

pub fn is_in_hitlag(flags: u64) -> bool {
    Flags::from(flags).contains(Flags::HITLAG.into())
}

pub fn is_in_hitstun(flags: u64) -> bool {
    Flags::from(flags).contains(Flags::HITSTUN.into())
}

pub fn is_in_magnifying_glass(flags: u64) -> bool {
    Flags::from(flags).contains(Flags::OFFSCREEN.into())
}

pub fn is_shielding(flags: u64) -> bool {
    Flags::from(flags).contains(Flags::SHIELDING.into())
}

pub fn is_fastfalling(flags: u64) -> bool {
    Flags::from(flags).contains(Flags::FASTFALL.into())
}

/// Returns true if the player is in any tumble or reeling animation, or if they are in the jab reset animation
pub fn is_damaged(state: u16) -> bool {
    (AR::DAMAGE_START as u16..=AR::DAMAGE_END as u16).contains(&state)
        || ActionState::DOWN_DAMAGE_D == state
        || ActionState::DOWN_DAMAGE_U == state
}

pub fn is_grabbed(state: u16) -> bool {
    (AR::CAPTURE_START as u16..=AR::CAPTURE_END as u16).contains(&state)
}

pub fn is_cmd_grabbed(state: u16) -> bool {
    ActionState::BARREL_WAIT != state
        && ((AR::COMMAND_GRAB_RANGE1_START as u16..=AR::COMMAND_GRAB_RANGE1_END as u16)
            .contains(&state)
            || (AR::COMMAND_GRAB_RANGE2_START as u16..=AR::COMMAND_GRAB_RANGE2_END as u16)
                .contains(&state))
}

pub fn is_teching(state: u16) -> bool {
    (AR::TECH_START as u16..=AR::TECH_END as u16).contains(&state)
        || (AR::DOWN_START as u16..=AR::DOWN_END as u16).contains(&state)
        || ActionState::FLY_REFLECT_CEIL == state
        || ActionState::FLY_REFLECT_WALL == state
}
