#![allow(clippy::type_complexity)]

use ssbm_utils::checks::{is_fastfalling, is_in_hitlag, just_input_lcancel};
use ssbm_utils::enums::{stage::Stage, ActionState, Attack, LCancel};
use std::ops::Deref;

use crate::{columns::*, player::Frames};
use polars::prelude::*;

pub fn find_lcancels(frames: &Frames, stage: &Stage) -> DataFrame {
    let mut frame_index_col: Vec<i32> = Vec::new();
    let mut stocks_col: Vec<u8> = Vec::new();
    let mut attack_col: Vec<&str> = Vec::new();
    let mut lcancelled_col: Vec<bool> = Vec::new();
    let mut l_input_col: Vec<Option<i32>> = Vec::new();
    let mut position_col: Vec<&str> = Vec::new();
    let mut fastfall_col: Vec<bool> = Vec::new();
    let mut hitlag_col: Vec<bool> = Vec::new();
    let mut percent_col: Vec<f32> = Vec::new();

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

        if lcancel == LCancel::NOT_APPLICABLE as u8 {
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
        if lcancel == LCancel::FAILURE as u8 && l_input_frame.is_some() {
            for j in 0..6 {
                let temp_index = i + j;
                if temp_index > pre_buttons.len() {
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

        frame_index_col.push(i as i32 - 123);
        stocks_col.push(stocks[i]);
        attack_col.push(attack.unwrap().into());
        lcancelled_col.push(LCancel::from_repr(lcancel) == Some(LCancel::SUCCESS));
        l_input_col.push(l_input_frame);
        position_col.push(stage.ground_from_id(last_ground_ids[i]).into());
        fastfall_col.push(is_fastfalling(flags[i - 1]));
        hitlag_col.push(during_hitlag);
        percent_col.push(percents[i]);
    }

    df!(LCancelStats::FrameIndex.into() => frame_index_col,
    LCancelStats::Attack.into() => attack_col,
    LCancelStats::Stocks.into() => stocks_col,
    LCancelStats::Percent.into() => percent_col,
    LCancelStats::LCancelled.into() => lcancelled_col,
    LCancelStats::TriggerFrame.into() => l_input_col,
    LCancelStats::Position.into() => position_col,
    LCancelStats::Fastfall.into() => fastfall_col,
    LCancelStats::InputDuringHitlag.into() => hitlag_col, )
    .unwrap()
}
