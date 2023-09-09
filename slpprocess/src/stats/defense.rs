#![allow(clippy::too_many_arguments)]

use anyhow::Result;
use polars::{chunked_array::builder::get_list_builder, prelude::*};

use ssbm_utils::{
    calc::knockback::{
        apply_di, initial_x_velocity, initial_y_velocity, kb_from_initial, should_kill,
    },
    checks::{
        get_damage_taken, is_in_defender_hitlag, is_in_hitlag, is_magnifying_damage,
        is_shielding_flag, is_thrown, just_took_damage,
    },
    enums::{Character, StickRegion},
    utils::{Joystick, Velocity},
};

use crate::player::Frames;

#[derive(Debug, Default)]
struct DefenseStats {
    frame_index: Vec<u32>,
    stocks_remaining: Vec<u8>,
    percent: Vec<f32>,
    damage_taken: Vec<f32>,
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
    di_kb_x: Vec<Option<f32>>,
    di_kb_y: Vec<Option<f32>>,
    di_efficacy: Vec<Option<f32>>,
    hitlag_start_x: Vec<f32>,
    hitlag_start_y: Vec<f32>,
    hitlag_end_x: Vec<f32>,
    hitlag_end_y: Vec<f32>,
    kills_with_di: Vec<bool>,
    kills_no_di: Vec<bool>,
    kills_any_di: Vec<bool>,
}

impl DefenseStats {
    fn append(&mut self, stat: &DefenseRow) {
        self.frame_index.push(stat.frame_index);
        self.stocks_remaining.push(stat.stocks_remaining);
        self.percent.push(stat.percent);
        self.damage_taken.push(stat.damage_taken);
        self.last_hit_by.push(stat.last_hit_by);
        self.state_before_hit.push(stat.state_before_hit);
        self.grounded.push(stat.grounded);
        self.crouch_cancel.push(stat.crouch_cancel);
        self.hitlag_frames.push(stat.hitlag_frames);
        self.stick_during_hitlag
            .push(stat.stick_during_hitlag.clone());
        self.sdi_inputs.push(stat.sdi_inputs.clone());
        self.asdi.push(stat.asdi);
        self.kb_x.push(stat.kb_x);
        self.kb_y.push(stat.kb_y);
        self.di_stick_x.push(stat.di_stick_x);
        self.di_stick_y.push(stat.di_stick_y);
        self.di_kb_x.push(stat.di_kb_x);
        self.di_kb_y.push(stat.di_kb_y);
        self.di_efficacy.push(stat.di_efficacy);
        self.hitlag_start_x.push(stat.hitlag_start_x);
        self.hitlag_start_y.push(stat.hitlag_start_y);
        self.hitlag_end_x.push(stat.hitlag_end_x);
        self.hitlag_end_y.push(stat.hitlag_end_y);
        self.kills_with_di.push(stat.kills_with_di);
        self.kills_no_di.push(stat.kills_no_di);
        self.kills_any_di.push(stat.kills_any_di);
    }
}

impl From<DefenseStats> for DataFrame {
    fn from(value: DefenseStats) -> Self {
        use crate::columns::Defense as D;

        let vec_series = vec![
            Series::new(D::FrameIndex.into(), value.frame_index),
            Series::new(D::StocksRemaining.into(), value.stocks_remaining),
            Series::new(D::Percent.into(), value.percent),
            Series::new(D::DamageTaken.into(), value.damage_taken),
            Series::new(D::LastHitBy.into(), value.last_hit_by),
            Series::new(D::StateBeforeHit.into(), value.state_before_hit),
            Series::new(D::Grounded.into(), value.grounded),
            Series::new(D::CrouchCancel.into(), value.crouch_cancel),
            Series::new(D::HitlagFrames.into(), value.hitlag_frames),
            Series::new(
                D::StickDuringHitlag.into(),
                vec![0.0; value.stick_during_hitlag.len()],
            ),
            Series::new(D::SDIInputs.into(), vec![0.0; value.sdi_inputs.len()]),
            Series::new(D::ASDI.into(), value.asdi),
            Series::new(D::KBX.into(), value.kb_x),
            Series::new(D::KBY.into(), value.kb_y),
            Series::new(D::DIStickX.into(), value.di_stick_x),
            Series::new(D::DIStickY.into(), value.di_stick_y),
            Series::new(D::DIKBX.into(), value.di_kb_x),
            Series::new(D::DIKBY.into(), value.di_kb_y),
            Series::new(D::DIEfficacy.into(), value.di_efficacy),
            Series::new(D::HitlagStartX.into(), value.hitlag_start_x),
            Series::new(D::HitlagStartY.into(), value.hitlag_start_y),
            Series::new(D::HitlagEndX.into(), value.hitlag_end_x),
            Series::new(D::HitlagEndY.into(), value.hitlag_end_y),
            Series::new(D::KillsWithDI.into(), value.kills_with_di),
            Series::new(D::KillsNoDI.into(), value.kills_no_di),
            Series::new(D::KillsAllDI.into(), value.kills_any_di),
        ];

        DataFrame::new(vec_series).unwrap()
    }
}

#[derive(Debug, Default, Clone)]
struct DefenseRow {
    frame_index: u32,
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
    di_kb_x: Option<f32>,
    di_kb_y: Option<f32>,
    di_efficacy: Option<f32>,
    hitlag_start_x: f32,
    hitlag_start_y: f32,
    hitlag_end_x: f32,
    hitlag_end_y: f32,
    kills_with_di: bool,
    kills_no_di: bool,
    kills_any_di: bool,
}

impl DefenseRow {
    pub fn new(
        frame_index: u32,
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
            hitlag_start_x: start_x,
            hitlag_start_y: start_y,
            ..Default::default()
        }
    }
}

pub fn find_defense(
    plyr_frames: &Frames,
    opnt_frames: &Frames,
    stage_id: u16,
    player_char: Character,
) -> DataFrame {
    let pre = &plyr_frames.pre;
    let post = &plyr_frames.post;
    let attacks = &opnt_frames.post.last_attack_landed;

    let flags: &[u64] = post.flags.as_ref().unwrap();

    let mut event = None;
    let mut stat_table = DefenseStats::default();

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
                i as u32,
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

            continue;
        }

        // ----------------------------------- finalize event ----------------------------------- //
        if !in_hitlag && was_in_hitlag && event.is_some() {
            let row = event.as_mut().unwrap();

            row.hitlag_end_x = post.position_x[i];
            row.hitlag_end_y = post.position_y[i];

            let effective_stick = Joystick::with_deadzone(pre.joystick_x[i], pre.joystick_y[i]);

            row.di_stick_x = effective_stick.x;
            row.di_stick_y = effective_stick.y;

            let kb_angle = Velocity::new(row.kb_x, row.kb_y).as_angle();
            if row.kb_x != 0.0 || row.kb_y != 0.0 {
                let with_di = apply_di(kb_angle, effective_stick.x, effective_stick.y);

                row.di_efficacy = Some((with_di - kb_angle).abs() / 18.0);

                let kb_scalar = kb_from_initial(row.kb_x, row.kb_y);

                row.di_kb_x = Some(initial_x_velocity(kb_scalar, with_di));
                row.di_kb_y = Some(initial_y_velocity(kb_scalar, with_di, row.grounded));

                let char_stats = player_char.get_stats();

                row.kills_no_di = should_kill(
                    stage_id,
                    row.kb_x,
                    row.kb_y,
                    row.hitlag_end_x,
                    row.hitlag_end_y,
                    char_stats.gravity,
                    char_stats.terminal_velocity,
                );

                if effective_stick.x != 0.0 || effective_stick.y != 0.0 {
                    row.kills_with_di = should_kill(
                        stage_id,
                        row.di_kb_x.unwrap(),
                        row.di_kb_y.unwrap(),
                        row.hitlag_end_x,
                        row.hitlag_end_y,
                        char_stats.gravity,
                        char_stats.terminal_velocity,
                    );
                } else {
                    row.kills_with_di = row.kills_no_di;
                }

                row.kills_any_di = {
                    let mut result = true;
                    for i in -90..90 {
                        let new_traj = kb_angle - (i as f32 / 5.0);
                        if !should_kill(
                            stage_id,
                            initial_x_velocity(kb_scalar, new_traj),
                            initial_y_velocity(kb_scalar, new_traj, row.grounded),
                            row.hitlag_end_x,
                            row.hitlag_end_y,
                            char_stats.gravity,
                            char_stats.terminal_velocity,
                        ) {
                            result = false;
                            break;
                        }
                    }

                    result
                }
            } else {
                // No reason to calculate when there's no knockback. Handles things like fox laser
                row.di_efficacy = None;
                row.di_kb_x = None;
                row.di_kb_y = None;
                row.kills_no_di = false;
                row.kills_with_di = false;
                row.kills_any_di = false;
            }

            stat_table.append(event.as_ref().unwrap());
            event = None;
        }
    }

    stat_table.into()
}
