use polars::prelude::*;
use ssbm_utils::{
    checks::{is_damaged, is_downed, is_in_defender_hitlag, is_teching},
    enums::{
        stage::{GroundID, Stage},
        Attack, TechType,
    },
    trackers::LockoutTracker,
    types::Position,
};

use crate::{frames::Frames, utils::as_vec_static_str};

pub fn find_techs(plyr_frames: &Frames, opnt_frames: &Frames, stage: &Stage) -> DataFrame {
    let pre = &plyr_frames.pre;
    let post = &plyr_frames.post;
    let flags = post.flags.as_ref().unwrap();
    let last_ground = post.last_ground_id.as_ref().unwrap();
    let attacks = &opnt_frames.post.last_attack_landed;
    let opnt_pos = &opnt_frames.post.position;

    let mut event: Option<TechRow> = None;
    let mut table = TechStats::default();

    // value tracking for v cancel
    let mut lockout = LockoutTracker::default();

    for i in 1..pre.len() {
        lockout.update(pre.engine_buttons[i], flags[i]);

        let curr_teching = is_teching(post.action_state[i]) || is_downed(post.action_state[i]);
        let was_teching =
            is_teching(post.action_state[i - 1]) || is_downed(post.action_state[i - 1]);

        // If we're not teching, but we were, close out the active event and add it to the table
        if !curr_teching {
            if was_teching && event.is_some() {
                let row = event.as_mut().unwrap();
                for j in 0..=8 {
                    if i + j >= pre.len() {
                        break;
                    }
                    if is_in_defender_hitlag(flags[i + j]) {
                        row.punished = true;
                    }
                }
                table.push_row(event.as_ref().unwrap());
                event = None;
            }
            continue;
        }

        // If we weren't teching (but were previously), start a tech event with the info we know
        if !was_teching {
            let tech_type =
                TechType::from_state(post.action_state[i], post.orientation[i] as i8).unwrap();
            // this weeds out regular wall jumps, which use the same action state was walljump techs
            if tech_type == TechType::WALL_JUMP_TECH
                && !(is_damaged(post.action_state[i - 1]) || is_in_defender_hitlag(flags[i - 1]))
            {
                continue;
            }

            let most_recent_input = lockout.frames_since_input();
            event = Some(TechRow::new(
                i as i32 - 123,
                post.stocks[i],
                post.percent[i],
                tech_type,
                post.position[i],
                stage.ground_from_id(last_ground[i]),
                attacks[i].into(),
                post.position[i].distance(opnt_frames.post.position[i]),
                (-40..=0)
                    .contains(&most_recent_input)
                    .then_some(most_recent_input),
                lockout.is_locked_out(),
                lockout.input_during_hitlag(),
            ));
        }

        /*
        this allows the tech data to update exactly once on the frame where the option is chosen
        (e.g. missed tech -> tech roll right) this matters for the positional checks, as we only
        want the earliest possible position values for each "decision" or "event" that happens
        during the tech situation
        */
        if post.action_state[i] == post.action_state[i - 1] {
            continue;
        }

        let orientation = post.orientation[i] as i8;
        let tech_type = TechType::from_state(post.action_state[i], orientation).unwrap();

        let row = event.as_mut().unwrap();
        row.tech_type = tech_type;
        let opnt_position = opnt_pos[i];
        row.opnt_distance = post.position[i].distance(opnt_position);

        match tech_type {
            TechType::MISSED_TECH => {
                row.missed_tech = true;
                row.jab_reset = Some(false);
            }
            TechType::JAB_RESET => row.jab_reset = Some(true),
            TechType::TECH_LEFT | TechType::MISSED_TECH_ROLL_LEFT => {
                let rel_pos = opnt_position.x - post.position[i].x;

                row.towards_center = Some(orientation > 0);
                row.towards_opponent = Some(rel_pos > 0.0);
            }
            TechType::TECH_RIGHT | TechType::MISSED_TECH_ROLL_RIGHT => {
                let rel_pos = opnt_position.x - post.position[i].x;

                row.towards_center = Some(orientation < 0);
                row.towards_opponent = Some(rel_pos < 0.0);
            }
            _ => (), // tech in place, getup attack, wall/ceil techs
        }
    }

    table.into()
}

#[derive(Debug)]
struct TechRow {
    frame_index: i32,
    stocks_remaining: u8,
    percent: f32,
    tech_type: TechType,
    punished: bool,
    position: Position,
    location: GroundID,
    missed_tech: bool,
    towards_center: Option<bool>,
    towards_opponent: Option<bool>,
    jab_reset: Option<bool>,
    last_hit_by: Attack,
    opnt_distance: f32,
    input_frame: Option<i32>,
    lockout: bool,
    during_hitlag: bool,
}

impl TechRow {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        frame_index: i32,
        stocks_remaining: u8,
        percent: f32,
        tech_type: TechType,
        position: Position,
        ground_id: GroundID,
        last_hit_by: Attack,
        opnt_distance: f32,
        input_frame: Option<i32>,
        lockout: bool,
        during_hitlag: bool,
    ) -> Self {
        let missed_tech = matches!(
            tech_type,
            TechType::MISSED_CEILING_TECH | TechType::MISSED_TECH | TechType::MISSED_WALL_TECH
        );
        TechRow {
            frame_index,
            stocks_remaining,
            percent,
            tech_type,
            punished: false,
            position,
            location: ground_id,
            missed_tech,
            towards_center: None,
            towards_opponent: None,
            jab_reset: None,
            last_hit_by,
            opnt_distance,
            input_frame,
            lockout,
            during_hitlag,
        }
    }
}

#[derive(Debug, Default)]
struct TechStats {
    frame_index: Vec<i32>,
    stocks_remaining: Vec<u8>,
    percent: Vec<f32>,
    input_frame: Vec<Option<i32>>,
    tech_type: Vec<TechType>,
    punished: Vec<bool>,
    position: Vec<Position>,
    location: Vec<GroundID>,
    missed_tech: Vec<bool>,
    lockout: Vec<bool>,
    towards_center: Vec<Option<bool>>,
    towards_opponent: Vec<Option<bool>>,
    jab_reset: Vec<Option<bool>>,
    last_hit_by: Vec<Attack>,
    opnt_distance: Vec<f32>,
    during_hitlag: Vec<bool>,
}

impl TechStats {
    pub fn push_row(&mut self, stat: &TechRow) {
        self.frame_index.push(stat.frame_index);
        self.stocks_remaining.push(stat.stocks_remaining);
        self.percent.push(stat.percent);
        self.input_frame.push(stat.input_frame);
        self.tech_type.push(stat.tech_type);
        self.punished.push(stat.punished);
        self.position.push(stat.position);
        self.location.push(stat.location);
        self.missed_tech.push(stat.missed_tech);
        self.lockout.push(stat.lockout);
        self.towards_center.push(stat.towards_center);
        self.towards_opponent.push(stat.towards_opponent);
        self.jab_reset.push(stat.jab_reset);
        self.last_hit_by.push(stat.last_hit_by);
        self.opnt_distance.push(stat.opnt_distance);
        self.during_hitlag.push(stat.during_hitlag);
    }
}

impl From<TechStats> for DataFrame {
    fn from(value: TechStats) -> Self {
        use crate::columns::TechStats as clm;
        let v_s = vec![
            Series::new(clm::FrameIndex.into(), value.frame_index),
            Series::new(clm::Stocks.into(), value.stocks_remaining),
            Series::new(clm::Percent.into(), value.percent),
            Series::new(clm::LastHitBy.into(), as_vec_static_str(value.last_hit_by)),
            Series::new(clm::InputFrame.into(), value.input_frame),
            Series::new(clm::DuringHitlag.into(), value.during_hitlag),
            Series::new(clm::MissedTech.into(), value.missed_tech),
            Series::new(clm::Lockout.into(), value.lockout),
            Series::new(clm::TechType.into(), as_vec_static_str(value.tech_type)),
            Series::new(clm::JabReset.into(), value.jab_reset),
            Series::new(clm::Punished.into(), value.punished),
            StructChunked::new(
                clm::Position.into(),
                &[
                    Series::new("x", value.position.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", value.position.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),
            Series::new(clm::Location.into(), as_vec_static_str(value.location)),
            Series::new(clm::TowardsCenter.into(), value.towards_center),
            Series::new(clm::TowardsOpnt.into(), value.towards_opponent),
            Series::new(clm::OpntDistance.into(), value.opnt_distance),
        ];

        DataFrame::new(v_s).unwrap()
    }
}
