#![allow(clippy::too_many_arguments)]

use anyhow::Result;
use polars::prelude::*;

use ssbm_utils::{
    calc::attack::apply_di,
    checks::{
        get_damage_taken, is_in_defender_hitlag, is_in_hitlag, is_magnifying_damage,
        is_shielding_flag, is_thrown, just_took_damage,
    },
    enums::StickRegion,
    utils::{Joystick, Velocity},
};

use crate::player::Frames;

#[derive(Debug, Default)]
struct DefenseStats {
    frame_index: Vec<usize>,
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
    kb_x: Vec<f32>,
    kb_y: Vec<f32>,
    di_stick_x: Vec<f32>,
    di_stick_y: Vec<f32>,
    di_kb_x: Vec<f32>,
    di_kb_y: Vec<f32>,
    start_x: Vec<f32>,
    start_y: Vec<f32>,
    end_x: Vec<f32>,
    end_y: Vec<f32>,
    died: Vec<bool>,
    kills_with_di: Vec<bool>,
    kills_no_di: Vec<bool>,
}

impl DefenseStats {
    fn insert(&mut self, stat: DefenseRow) {
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
        self.di_stick_x.push(stat.di_stick_x);
        self.di_stick_y.push(stat.di_stick_y);
        self.start_x.push(stat.start_x);
        self.start_y.push(stat.start_y);
        self.end_x.push(stat.end_x);
        self.end_y.push(stat.end_y);
    }
}

#[derive(Debug, Default)]
struct DefenseRow {
    frame_index: usize,
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
    kb_x: f32,
    kb_y: f32,
    di_stick_x: f32,
    di_stick_y: f32,
    di_kb_x: f32,
    di_kb_y: f32,
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
    died: bool,
    kills_with_di: bool,
    kills_no_di: bool,
}

impl DefenseRow {
    pub fn new(
        frame_index: usize,
        stocks_remaining: u8,
        percent: f32,
        damage_taken: f32,
        last_hit_by: u8,
        state_before_hit: u16,
        grounded: bool,
        start_x: f32,
        start_y: f32,
    ) -> Self {
        Self {
            frame_index,
            stocks_remaining,
            percent,
            damage_taken,
            last_hit_by,
            state_before_hit,
            grounded,
            start_x,
            start_y,
            ..Default::default()
        }
    }
}

pub fn find_defense(plyr_frames: &Frames, opnt_frames: &Frames) -> DataFrame {
    let pre = &plyr_frames.pre;
    let post = &plyr_frames.post;
    let attacks = &opnt_frames.post.last_attack_landed;

    let flags: &[u64] = post.flags.as_ref().unwrap();

    let mut active_event = false;
    let mut hitlag_stick: Vec<StickRegion> = Vec::default();
    let mut event = None;

    for i in 1..pre.frame_index.len() {
        // check for grab states

        // just_in_hitlag, filtering out shield hits
        let in_hitlag = is_in_hitlag(flags[i]);
        let was_in_hitlag = is_in_hitlag(flags[i - 1]);

        let shielding = is_shielding_flag(flags[i]);
        let grabbed_check = false;

        let took_damage = just_took_damage(post.percent[i], post.percent[i - 1]);
        let damage_taken = get_damage_taken(post.percent[i], post.percent[i - 1]);

        // ----------------------------------- event detection ---------------------------------- //

        if (!was_in_hitlag && took_damage)
            || (!in_hitlag && took_damage && is_thrown(post.action_state[i]))
        // && !is_magnifying_damage(damage_taken, flags, i)
        {
            event = Some(DefenseRow::new(
                i,
                post.stocks[i],
                post.percent[i],
                damage_taken,
                attacks[i],
                post.action_state[i - 1],
                post.is_grounded.as_ref().unwrap()[i],
                post.position_x[i],
                post.position_y[i],
            ));

            let row = event.as_mut().unwrap();
            row.kb_x = post.knockback_x.as_ref().unwrap()[i];
            row.kb_y = post.knockback_y.as_ref().unwrap()[i];
        }

        // ----------------------------------- mid-event data ----------------------------------- //
        if event.is_some() && in_hitlag {
            let row = event.as_mut().unwrap();
            row.hitlag_frames += 1;
            row.stick_during_hitlag
                .push(StickRegion::from_coordinates(pre.joystick_x[i], pre.joystick_y[i]) as i8);
        }

        // ----------------------------------- finalize event ----------------------------------- //
        if !in_hitlag && event.is_some() {
            let row = event.as_mut().unwrap();

            let effective_stick = Joystick::with_deadzone(pre.joystick_x[i], pre.joystick_y[i]);

            row.di_stick_x = effective_stick.x;
            row.di_stick_y = effective_stick.y;

            if row.kb_x != 0.0 && row.kb_y != 0.0 {
                let di_angle = apply_di(
                    Velocity::new(row.kb_x, row.kb_y).as_angle(),
                    effective_stick.x,
                    effective_stick.y,
                );

                let di_efficacacy = 
            }
        }
    }

    DataFrame::default()
}
