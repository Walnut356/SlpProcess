use anyhow::Result;
use polars::prelude::*;

use crate::{
    enums::buttons::StickRegion,
    player::Frames,
    stats::helpers::{is_in_hitlag, is_shielding, just_took_damage, is_magnifying_damage, get_damage_taken}, utils::Point,
};

#[derive(Debug)]
struct DefenseStats {
    frame_index: Vec<i32>,
    stocks_remaining: Vec<u8>,
    percent: Vec<f32>,
    last_hit_by: Vec<u8>,
    state_before_hit: Vec<u16>,
    grounded: Vec<bool>,
    crouch_cancel: Vec<bool>,
    hitlag_frames: Vec<u8>,
    stick_during_hitlag: Vec<Vec<i8>>,
    sdi_inputs: Vec<Vec<i8>>,
    asdi: Vec<i8>,
    di_pos_x: Vec<f32>,
    di_pos_y: Vec<f32>,
    start_pos_x: Vec<f32>,
    start_pos_y: Vec<f32>,
    end_pos_x: Vec<f32>,
    end_pos_y: Vec<f32>,
}

pub fn find_defense(plyr_frames: &Frames, opnt_frames: &Frames) -> DataFrame {
    let pre = &plyr_frames.pre;
    let post = &plyr_frames.post;
    let attacks = &opnt_frames.post.last_attack_landed;

    let flags: &[u64] = post.flags.as_ref().unwrap();

    let mut active_event = false;
    let mut hitlag_stick: Vec<StickRegion> = Vec::default();

    for i in 1..pre.frame_index.len() {
        // check for grab states

        // just_in_hitlag, filtering out shield hits
        let in_hitlag = is_in_hitlag(flags[i]);
        let was_in_hitlag = is_in_hitlag(flags[i - 1]);

        let shielding = is_shielding(flags[i]);

        let damage_taken = get_damage_taken(&post.percent, i);

        // start a new event
        if !was_in_hitlag && just_took_damage(&post.percent, i) && !is_magnifying_damage(damage_taken, flags, i) {

        }

        // finalize an active event
        if !in_hitlag && was_in_hitlag {}
    }

    DataFrame::default()
}
