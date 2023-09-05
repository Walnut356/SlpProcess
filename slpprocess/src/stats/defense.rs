use anyhow::Result;
use polars::prelude::*;

use crate::{
    enums::buttons::StickRegion,
    player::Frames,
    stats::helpers::{
        get_damage_taken, is_in_hitlag, is_magnifying_damage, is_shielding_flag, just_took_damage,
    },
    utils::Point,
};

#[derive(Debug, Default)]
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

impl DefenseStats {
    fn insert(&mut self, stat: DefenseTemp) {
        self.frame_index.push(stat.frame_index);
        self.stocks_remaining.push(stat.stocks_remaining);
        self.percent.push(stat.percent);
        self.last_hit_by.push(stat.last_hit_by);
        self.state_before_hit.push(stat.state_before_hit);
        self.grounded.push(stat.grounded);
        self.crouch_cancel.push(stat.crouch_cancel);
        self.hitlag_frames.push(stat.hitlag_frames);
        self.stick_during_hitlag.push(stat.stick_during_hitlag);
        self.sdi_inputs.push(stat.sdi_inputs);
        self.asdi.push(stat.asdi);
        self.di_pos_x.push(stat.di_pos_x);
        self.di_pos_y.push(stat.di_pos_y);
        self.start_pos_x.push(stat.start_pos_x);
        self.start_pos_y.push(stat.start_pos_y);
        self.end_pos_x.push(stat.end_pos_x);
        self.end_pos_y.push(stat.end_pos_y);
    }
}

#[derive(Debug, Default)]
struct DefenseTemp {
    frame_index: i32,
    stocks_remaining: u8,
    percent: f32,
    damage_taken: f32,
    last_hit_by: u8,
    state_before_hit: u16,
    grounded: bool,
    crouch_cancel: bool,
    hitlag_frames: u8,
    stick_during_hitlag: Vec<i8>,
    sdi_inputs: Vec<i8>,
    asdi: i8,
    di_pos_x: f32,
    di_pos_y: f32,
    start_pos_x: f32,
    start_pos_y: f32,
    end_pos_x: f32,
    end_pos_y: f32,
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

        let shielding = is_shielding_flag(flags[i]);

        let damage_taken = get_damage_taken(&post.percent, i);

        // ------------------------------------- event detection ------------------------------------ //
        if !was_in_hitlag && just_took_damage(&post.percent, i)
        // && !is_magnifying_damage(damage_taken, flags, i)
        {}

        // finalize an active event
        if !in_hitlag && was_in_hitlag {}
    }

    DataFrame::default()
}
