#![allow(clippy::type_complexity)]

use ssbm_utils::{checks::{is_fastfalling, is_in_hitlag, just_input_lcancel}, prelude::stage::GroundID};
use ssbm_utils::enums::{stage::Stage, ActionState, Attack, LCancelState};
use std::ops::Deref;

use crate::frames::Frames;

#[derive(Debug, Clone, Default)]
pub struct LCancelStats {
    pub frame_index: Vec<i32>,
    pub stocks: Vec<u8>,
    pub attack: Vec<Attack>,
    pub l_cancel: Vec<bool>,
    pub trigger_input_frame: Vec<Option<i32>>,
    pub position: Vec<GroundID>,
    pub fastfall: Vec<bool>,
    pub during_hitlag: Vec<bool>,
    pub percent: Vec<f32>,
}

pub fn find_lcancels(frames: &Frames, stage: &Stage) -> LCancelStats {
    let mut table = LCancelStats::default();

    let mut l_input_frame: Option<i32> = None;
    let mut during_hitlag: bool = false;

    let lcancels: &[u8] = frames.post.l_cancel.as_deref().unwrap();
    let states: &[u16] = frames.post.action_state.deref();
    let stocks: &[u8] = frames.post.stocks.deref();
    let last_ground_ids: &[u16] = frames.post.last_ground_id.as_deref().unwrap();
    let percents: &[f32] = frames.post.percent.deref();
    let flags: &[u64] = frames.post.flags.as_deref().unwrap();
    let pre_buttons: &[u32] = frames.pre.engine_buttons.deref();

    for i in 1..frames.len() {
        let lcancel = lcancels[i];

        if just_input_lcancel(pre_buttons[i], pre_buttons[i - 1]) {
            l_input_frame = Some(i as i32);
            during_hitlag = is_in_hitlag(flags[i]);
        }

        if lcancel == LCancelState::NOT_APPLICABLE as u8 {
            continue;
        }

        if let Some(x) = l_input_frame {
            l_input_frame = Some(x - (i as i32));
        }
        // if it's been more than 15 frames since you've hit l, or more than 25 if you hit L during hitlag,
        // disregard the l press
        if l_input_frame.is_some_and(|x| (x < -15 && !during_hitlag) || (x < -25)) {
            l_input_frame = None;
        }

        // if there's no l cancel input that was too early, and the input failed, check the next 5 frames to see if
        // there was a late l cancel
        if lcancel == LCancelState::FAILURE as u8 && l_input_frame.is_some() {
            for j in 0..6 {
                let temp_index = i + j;
                if temp_index >= pre_buttons.len() {
                    break;
                }
                if just_input_lcancel(pre_buttons[i], pre_buttons[i + j]) {
                    l_input_frame = Some(j as i32)
                }
            }
        }

        let attack = match ActionState::from_repr(states[i]) {
            Some(ActionState::LANDING_AIR_N) | Some(ActionState::ATTACK_AIR_N) => {
                Some(Attack::NAIR)
            }
            Some(ActionState::LANDING_AIR_F) | Some(ActionState::ATTACK_AIR_F) => {
                Some(Attack::FAIR)
            }
            Some(ActionState::LANDING_AIR_B) | Some(ActionState::ATTACK_AIR_B) => {
                Some(Attack::BAIR)
            }
            Some(ActionState::LANDING_AIR_HI) | Some(ActionState::ATTACK_AIR_HI) => {
                Some(Attack::UAIR)
            }
            Some(ActionState::LANDING_AIR_LW) | Some(ActionState::ATTACK_AIR_LW) => {
                Some(Attack::DAIR)
            }
            _ => None,
        };

        if attack.is_none() {
            continue;
        }

        table.frame_index.push(i as i32 - 123);
        table.stocks.push(stocks[i]);
        table.attack.push(attack.unwrap());
        table.l_cancel.push(LCancelState::from_repr(lcancel) == Some(LCancelState::SUCCESS));
        table.trigger_input_frame.push(l_input_frame);
        table.position.push(stage.ground_from_id(last_ground_ids[i]));
        table.fastfall.push(is_fastfalling(flags[i - 1]));
        table.during_hitlag.push(during_hitlag);
        table.percent.push(percents[i]);
    }

    table
}
