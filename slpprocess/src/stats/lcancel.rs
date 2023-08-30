#![allow(clippy::type_complexity)]

use crate::{
    columns::*,
    enums::{attack::Attack, general::LCancel, stage::Stage, state::ActionState},
    player::Frames,
    stats::helpers::is_fastfalling,
};
use itertools::izip;
use polars::prelude::*;

use super::helpers::{is_in_hitlag, just_input_lcancel};

pub fn find_lcancels(frames: &Frames, stage: Stage) -> DataFrame {
    let mut frame_index_col = Vec::new();
    let mut stocks_col = Vec::new();
    let mut attack_col = Vec::new();
    let mut lcancelled_col = Vec::new();
    let mut l_input_col = Vec::new();
    let mut position_col: Vec<&str> = Vec::new();
    let mut fastfall_col = Vec::new();
    let mut hitlag_col = Vec::new();
    let mut percent_col = Vec::new();

    let mut l_input_frame = None;
    let mut during_hitlag = false;

    let (lcancels, states, stocks, last_ground_ids, percents, flags, pre_buttons) =
        get_lcancel_columns(frames);

    for (i, (lcancel, state, stocks_remaining, last_ground_id, percent, flags2)) in izip!(
        lcancels.iter(),
        states.iter(),
        stocks.iter(),
        last_ground_ids.iter(),
        percents.iter(),
        flags.iter(),
    )
    .enumerate()
    {
        if just_input_lcancel(pre_buttons, i) {
            l_input_frame = Some(i as i32);
            during_hitlag = is_in_hitlag(flags2.unwrap());
        }

        if *lcancel == Some(LCancel::NOT_APPLICABLE as u8) {
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
        if *lcancel == Some(LCancel::FAILURE as u8) && l_input_frame.is_some() {
            for j in 0..6 {
                if just_input_lcancel(pre_buttons, i + j) {
                    l_input_frame = Some(j as i32)
                }
            }
        }

        let attack = match ActionState::from_repr(*state) {
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

        let temp: &'static str = attack.unwrap().into();

        frame_index_col.push(i as i32 - 123);
        stocks_col.push(*stocks_remaining);
        attack_col.push(temp);
        lcancelled_col.push(LCancel::from_repr(lcancel.unwrap()) == Some(LCancel::SUCCESS));
        l_input_col.push(l_input_frame);
        position_col.push(stage.ground_from_id(last_ground_id.unwrap()).into());
        fastfall_col.push(is_fastfalling(flags2.unwrap()));
        hitlag_col.push(during_hitlag);
        percent_col.push(*percent);
    }

    df!(LCancels::FrameIndex.into() => frame_index_col,
    LCancels::Attack.into() => attack_col,
    LCancels::StocksRemaining.into() => stocks_col,
    LCancels::Percent.into() => percent_col,
    LCancels::LCancelled.into() => lcancelled_col,
    LCancels::TriggerFrame.into() => l_input_col,
    LCancels::Position.into() => position_col,
    LCancels::Fastfall.into() => fastfall_col,
    LCancels::InputDuringHitlag.into() => hitlag_col, )
    .unwrap()
}

fn get_lcancel_columns(
    frames: &Frames,
) -> (
    &Box<[Option<u8>]>,
    &Box<[u16]>,
    &Box<[u8]>,
    &Box<[Option<u16>]>,
    &Box<[f32]>,
    &Box<[Option<u8>]>,
    &Box<[u32]>,
) {
    (
        &frames
            .post
            .l_cancel,
        &frames
            .post
            .action_state,
        &frames
            .post
            .stocks,
        &frames
            .post
            .last_ground_id,
        &frames
            .post
            .percent,
        &frames
            .post
            .flags_2,
        &frames
            .pre
            .engine_buttons,
    )
}
