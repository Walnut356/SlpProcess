#![allow(clippy::too_many_arguments)]

use polars::prelude::*;

use ssbm_utils::{
    calc::knockback::{
        apply_di, get_di_efficacy, initial_x_velocity, initial_y_velocity, kb_from_initial,
        should_kill,
    },
    checks::{get_damage_taken, is_in_hitlag, is_shielding_flag, is_thrown, just_took_damage},
    enums::{Character, StickRegion, Attack, State},
    types::{Degrees, Position, StickPos, Velocity},
};

use crate::player::Frames;

fn as_vec_i8(input: Vec<StickRegion>) -> Vec<i8> {
    input.into_iter().map(|s| s as i8).collect()
}

fn as_vec_static_str<T: Into<&'static str>>(input: Vec<T>) -> Vec<&'static str> {
    input.into_iter().map(|x| x.into()).collect::<Vec<&'static str>>()
}

#[derive(Debug, Default)]
struct DefenseStats {
    frame_index: Vec<u32>,
    stocks_remaining: Vec<u8>,
    percent: Vec<f32>,
    damage_taken: Vec<f32>,
    last_hit_by: Vec<Attack>,
    state_before_hit: Vec<State>,
    grounded: Vec<bool>,
    crouch_cancel: Vec<bool>,
    hitlag_frames: Vec<u8>,
    stick_during_hitlag: Vec<Vec<StickRegion>>,
    sdi_inputs: Vec<Vec<StickRegion>>,
    asdi: Vec<StickRegion>,
    kb: Vec<Velocity>,
    kb_angle: Vec<Degrees>,
    di_stick: Vec<StickPos>,
    di_kb: Vec<Velocity>,
    di_efficacy: Vec<Option<f32>>,
    di_kb_angle: Vec<Degrees>,
    hitlag_start: Vec<Position>,
    hitlag_end: Vec<Position>,
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
        self.kb.push(stat.kb);
        self.kb_angle.push(stat.kb_angle);
        self.di_stick.push(stat.di_stick);
        self.di_kb.push(stat.di_kb);
        self.di_efficacy.push(stat.di_efficacy);
        self.di_kb_angle.push(stat.di_kb_angle);
        self.hitlag_start.push(stat.hitlag_start);
        self.hitlag_end.push(stat.hitlag_end);
        self.kills_with_di.push(stat.kills_with_di);
        self.kills_no_di.push(stat.kills_no_di);
        self.kills_any_di.push(stat.kills_any_di);
    }
}

impl From<DefenseStats> for DataFrame {
    fn from(val: DefenseStats) -> Self {
        use crate::columns::DefenseStats as col;

        let vec_series = vec![
            Series::new(col::FrameIndex.into(), val.frame_index),
            Series::new(col::Stocks.into(), val.stocks_remaining),
            Series::new(col::Percent.into(), val.percent),
            Series::new(col::DamageTaken.into(), val.damage_taken),
            Series::new(col::LastHitBy.into(), as_vec_static_str(val.last_hit_by)),
            Series::new(col::StateBeforeHit.into(), as_vec_static_str(val.state_before_hit)),
            Series::new(col::Grounded.into(), val.grounded),
            Series::new(col::CrouchCancel.into(), val.crouch_cancel),
            Series::new(col::HitlagFrames.into(), val.hitlag_frames),

            Series::new(
                col::StickDuringHitlag.into(),
                val.stick_during_hitlag
                    .into_iter()
                    .map(|x| Series::new("", as_vec_i8(x)))
                    .collect::<Vec<_>>(),
            ),

            Series::new(
                col::SDIInputs.into(),
                val.sdi_inputs
                    .into_iter()
                    .map(|x| Series::new("", as_vec_static_str(x)))
                    .collect::<Vec<_>>(),
            ),

            Series::new(col::ASDI.into(), as_vec_static_str(val.asdi)),

            StructChunked::new(
                col::Knockback.into(),
                &[
                    Series::new("x", val.kb.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.kb.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),

            Series::new(col::KBAngle.into(), val.kb_angle),

            StructChunked::new(
                col::DIStick.into(),
                &[
                    Series::new("x", val.di_stick.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.di_stick.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),

            StructChunked::new(
                col::DIKnockback.into(),
                &[
                    Series::new("x", val.di_kb.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.di_kb.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),

            Series::new(col::DIKBAngle.into(), val.di_kb_angle),
            Series::new(col::DIEfficacy.into(), val.di_efficacy),

            StructChunked::new(
                col::HitlagStart.into(),
                &[
                    Series::new("x", val.hitlag_start.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.hitlag_start.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),

            StructChunked::new(
                col::HitlagEnd.into(),
                &[
                    Series::new("x", val.hitlag_end.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.hitlag_end.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),

            Series::new(col::KillsWithDI.into(), val.kills_with_di),
            Series::new(col::KillsNoDI.into(), val.kills_no_di),
            Series::new(col::KillsAllDI.into(), val.kills_any_di),
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
    last_hit_by: Attack,
    state_before_hit: State,
    grounded: bool,
    crouch_cancel: bool,
    hitlag_frames: u8,
    stick_during_hitlag: Vec<StickRegion>,
    sdi_inputs: Vec<StickRegion>,
    asdi: StickRegion,
    kb: Velocity,
    kb_angle: Degrees,
    di_stick: StickPos,
    di_kb: Velocity,
    di_kb_angle: Degrees,
    di_efficacy: Option<f32>,
    hitlag_start: Position,
    hitlag_end: Position,
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
        last_hit_by: Attack,
        state_before_hit: State,
        grounded: bool,
        start: Position,
    ) -> Self {
        Self {
            frame_index,
            stocks_remaining,
            percent,
            damage_taken,
            last_hit_by,
            state_before_hit,
            grounded,
            hitlag_start: start,
            ..Default::default()
        }
    }
}

pub(crate) fn find_defense(
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

    // start 1 frame "late" to prevent index errors
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
        // TODO CC check
        if (!was_in_hitlag && took_damage)
            || (!in_hitlag && took_damage && is_thrown(post.action_state[i]))
        // && !is_magnifying_damage(damage_taken, flags, i)
        {
            event = Some(DefenseRow::new(
                i as u32,
                post.stocks[i],
                post.percent[i],
                damage_taken,
                Attack::from(attacks[i]),
                State::from_char_and_state(player_char, post.action_state[i - 1]),
                post.is_grounded.as_ref().unwrap()[i],
                post.position[i],
            ));

            let row = event.as_mut().unwrap();
            row.kb = post.knockback.as_ref().unwrap()[i];
        }

        // ----------------------------------- mid-event data ----------------------------------- //
        if event.is_some() && in_hitlag {
            let row = event.as_mut().unwrap();
            row.hitlag_frames += 1;
            let curr_stick = pre.joystick[i].as_stickregion();
            row.stick_during_hitlag.push(curr_stick);

            if row.hitlag_frames > 1 && curr_stick.valid_sdi(pre.joystick[i - 1].as_stickregion()) {
                row.sdi_inputs.push(curr_stick)
            }

            continue;
        }

        // ----------------------------------- finalize event ----------------------------------- //
        // TODO asdi, list output,

        if !in_hitlag && was_in_hitlag && event.is_some() {
            let row = event.as_mut().unwrap();

            row.hitlag_end = post.position[i - 1];

            let effective_stick = pre.joystick[i];

            row.di_stick = effective_stick;

            let cstick = pre.cstick[i].as_stickregion();
            row.asdi = if !cstick.is_deadzone() {
                cstick
            } else {
                effective_stick.as_stickregion()
            };

            let kb_angle_rads = row.kb.as_angle();
            row.kb_angle = kb_angle_rads.to_degrees();

            if !row.kb.is_zero() {
                let with_di = apply_di(kb_angle_rads, effective_stick);

                row.di_efficacy = Some(get_di_efficacy(kb_angle_rads, with_di));
                row.di_kb_angle = with_di.to_degrees();

                let kb_scalar = kb_from_initial(row.kb);

                row.di_kb = Velocity::new(
                    initial_x_velocity(kb_scalar, with_di),
                    initial_y_velocity(kb_scalar, with_di, row.grounded),
                );

                let char_stats = player_char.get_stats();

                row.kills_no_di = should_kill(
                    stage_id,
                    row.kb,
                    row.hitlag_end,
                    char_stats.gravity,
                    char_stats.max_fall_speed,
                );

                if effective_stick.x != 0.0 || effective_stick.y != 0.0 {
                    row.kills_with_di = should_kill(
                        stage_id,
                        row.di_kb,
                        row.hitlag_end,
                        char_stats.gravity,
                        char_stats.max_fall_speed,
                    );
                } else {
                    row.kills_with_di = row.kills_no_di;
                }

                row.kills_any_di = {
                    let mut result = true;
                    for i in -90..90 {
                        let new_traj = kb_angle_rads - (i as f32 / 5.0).to_radians();
                        if !should_kill(
                            stage_id,
                            Velocity::new(
                                initial_x_velocity(kb_scalar, new_traj),
                                initial_y_velocity(kb_scalar, new_traj, row.grounded),
                            ),
                            row.hitlag_end,
                            char_stats.gravity,
                            char_stats.max_fall_speed,
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
                row.di_kb = row.kb;
                row.di_kb_angle = row.kb_angle;
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
