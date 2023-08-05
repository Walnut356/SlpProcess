use bytes::{Buf, Bytes};
use nohash_hasher::IntMap;
use polars::prelude::*;
use std::{iter::zip, sync::Mutex};

use crate::{player::Player, Port};

pub struct PostFrames {
    frame_number: Vec<i32>,
    character: Vec<u8>,
    action_state: Vec<u16>,
    position_x: Vec<f32>,
    position_y: Vec<f32>,
    facing: Vec<f32>,
    percent: Vec<f32>,
    shield_health: Vec<f32>,
    last_attack_landed: Vec<u8>,
    combo_count: Vec<u8>,
    last_hit_by: Vec<u8>,
    stocks: Vec<u8>,
    state_frame: Vec<Option<f32>>,
    flags_1: Vec<Option<u8>>,
    flags_2: Vec<Option<u8>>,
    flags_3: Vec<Option<u8>>,
    flags_4: Vec<Option<u8>>,
    flags_5: Vec<Option<u8>>,
    misc_as: Vec<Option<f32>>,
    is_grounded: Vec<Option<bool>>,
    last_ground_id: Vec<Option<u16>>,
    jumps_remaining: Vec<Option<u8>>,
    l_cancel: Vec<Option<u8>>,
    hurtbox_state: Vec<Option<u8>>,
    self_air_x: Vec<Option<f32>>,
    self_y: Vec<Option<f32>>,
    knockback_x: Vec<Option<f32>>,
    knockback_y: Vec<Option<f32>>,
    self_ground_x: Vec<Option<f32>>,
    hitlag_remaining: Vec<Option<f32>>,
    animation_index: Vec<Option<u32>>,
}

impl PostFrames {
    fn new(len: usize) -> Self {
        PostFrames {
            frame_number: Vec::with_capacity(len),
            character: Vec::with_capacity(len),
            action_state: Vec::with_capacity(len),
            position_x: Vec::with_capacity(len),
            position_y: Vec::with_capacity(len),
            facing: Vec::with_capacity(len),
            percent: Vec::with_capacity(len),
            shield_health: Vec::with_capacity(len),
            last_attack_landed: Vec::with_capacity(len),
            combo_count: Vec::with_capacity(len),
            last_hit_by: Vec::with_capacity(len),
            stocks: Vec::with_capacity(len),
            state_frame: Vec::with_capacity(len),
            flags_1: Vec::with_capacity(len),
            flags_2: Vec::with_capacity(len),
            flags_3: Vec::with_capacity(len),
            flags_4: Vec::with_capacity(len),
            flags_5: Vec::with_capacity(len),
            misc_as: Vec::with_capacity(len),
            is_grounded: Vec::with_capacity(len),
            last_ground_id: Vec::with_capacity(len),
            jumps_remaining: Vec::with_capacity(len),
            l_cancel: Vec::with_capacity(len),
            hurtbox_state: Vec::with_capacity(len),
            self_air_x: Vec::with_capacity(len),
            self_y: Vec::with_capacity(len),
            knockback_x: Vec::with_capacity(len),
            knockback_y: Vec::with_capacity(len),
            self_ground_x: Vec::with_capacity(len),
            hitlag_remaining: Vec::with_capacity(len),
            animation_index: Vec::with_capacity(len),
        }
    }
}

// postframe objects are purely a temporary container to make the code clearer, so I impl `Into` rather than `From` (and
// implicitly `Into`) because I intentionally want to disallow translation back.
#[allow(clippy::from_over_into)]
impl Into<DataFrame> for PostFrames {
    fn into(mut self) -> DataFrame {
        let len = self.frame_number.len();

        // handle potentially optional values. We only need to check against 1 per version bump
        if self.state_frame.len() != len {
            // version < 0.2.0
            self.state_frame.resize(len, None)
        }

        if self.flags_1.len() != len {
            // version < 2.0.0
            self.flags_1.resize(len, None);
            self.flags_2.resize(len, None);
            self.flags_3.resize(len, None);
            self.flags_4.resize(len, None);
            self.flags_5.resize(len, None);
            self.misc_as.resize(len, None);
            self.is_grounded.resize(len, None);
            self.last_ground_id.resize(len, None);
            self.jumps_remaining.resize(len, None);
            self.l_cancel.resize(len, None);
        }

        if self.hurtbox_state.len() != len {
            // version < 2.1.0
            self.hurtbox_state.resize(len, None);
        }

        if self.self_air_x.len() != len {
            // version < 3.5.0
            self.self_air_x.resize(len, None);
            self.self_y.resize(len, None);
            self.knockback_x.resize(len, None);
            self.knockback_y.resize(len, None);
            self.self_ground_x.resize(len, None);
        }

        if self.hitlag_remaining.len() != len {
            // version < 3.8.0
            self.hitlag_remaining.resize(len, None);
        }

        if self.animation_index.len() != len {
            // version < 3.11.0
            self.animation_index.resize(len, None);
        }

        let vec_series = vec![
            Series::new("frame number", self.frame_number),
            Series::new("character", self.character),
            Series::new("action state", self.action_state),
            Series::new("position x", self.position_x),
            Series::new("position y", self.position_y),
            Series::new("facing", self.facing),
            Series::new("percent", self.percent),
            Series::new("shield health", self.shield_health),
            Series::new("last attack landed", self.last_attack_landed),
            Series::new("combo count", self.combo_count),
            Series::new("last hit by", self.last_hit_by),
            Series::new("stocks", self.stocks),
            Series::new("state frame", self.state_frame),
            Series::new("flags 1", self.flags_1),
            Series::new("flags 2", self.flags_2),
            Series::new("flags 3", self.flags_3),
            Series::new("flags 4", self.flags_4),
            Series::new("flags 5", self.flags_5),
            Series::new("misc as", self.misc_as),
            Series::new("is grounded", self.is_grounded),
            Series::new("last ground id", self.last_ground_id),
            Series::new("jumps remaining", self.jumps_remaining),
            Series::new("l cancel", self.l_cancel),
            Series::new("hurtbox state", self.hurtbox_state),
            Series::new("self air x", self.self_air_x),
            Series::new("self y", self.self_y),
            Series::new("knockback x", self.knockback_x),
            Series::new("knockback y", self.knockback_y),
            Series::new("self ground x", self.self_ground_x),
            Series::new("hitlag remaining", self.hitlag_remaining),
            Series::new("animation index", self.animation_index),
        ];

        DataFrame::new(vec_series).unwrap()
    }
}

pub fn parse_postframes(frames: &mut [Bytes], ports: [Port; 2]) -> IntMap<u8, DataFrame> {
    let len = frames.len() / 2;

    let mut p_frames: IntMap<u8, PostFrames> = IntMap::default();
    p_frames.insert(ports[0].into(), PostFrames::new(len));
    p_frames.insert(ports[1].into(), PostFrames::new(len));

    for mut frame in frames {
        let frame_number = frame.get_i32();
        let port = frame.get_u8();
        let working = p_frames.get_mut(&port).unwrap();
        let is_nana = frame.get_u8() != 0; // TODO add ic's specific logic using metadata

        working.frame_number.push(frame_number);
        working.character.push(frame.get_u8());
        working.action_state.push(frame.get_u16());
        working.position_x.push(frame.get_f32());
        working.position_y.push(frame.get_f32());
        working.facing.push(frame.get_f32());
        working.percent.push(frame.get_f32());
        working.shield_health.push(frame.get_f32());
        working.last_attack_landed.push(frame.get_u8());
        working.combo_count.push(frame.get_u8());
        working.last_hit_by.push(frame.get_u8());
        working.stocks.push(frame.get_u8());
        if !frame.has_remaining() {
            // version < 2.0.0
            continue;
        }
        working.state_frame.push(Some(frame.get_f32()));
        working.flags_1.push(Some(frame.get_u8()));
        working.flags_2.push(Some(frame.get_u8()));
        working.flags_3.push(Some(frame.get_u8()));
        working.flags_4.push(Some(frame.get_u8()));
        working.flags_5.push(Some(frame.get_u8()));
        working.misc_as.push(Some(frame.get_f32()));
        working.is_grounded.push(Some(frame.get_u8() != 0));
        working.last_ground_id.push(Some(frame.get_u16()));
        working.jumps_remaining.push(Some(frame.get_u8()));
        working.l_cancel.push(Some(frame.get_u8()));
        if !frame.has_remaining() {
            // version < 2.1.0
            continue;
        }
        working.hurtbox_state.push(Some(frame.get_u8()));
        if !frame.has_remaining() {
            // version < 3.5.0
            continue;
        }
        working.self_air_x.push(Some(frame.get_f32()));
        working.self_y.push(Some(frame.get_f32()));
        working.knockback_x.push(Some(frame.get_f32()));
        working.knockback_y.push(Some(frame.get_f32()));
        working.self_ground_x.push(Some(frame.get_f32()));
        if !frame.has_remaining() {
            // version < 3.8.0
            continue;
        }
        working.hitlag_remaining.push(Some(frame.get_f32()));
        if !frame.has_remaining() {
            // version < 3.11.0
            continue;
        }
        working.animation_index.push(Some(frame.get_u32()));
    }

    let mut result = IntMap::default();

    for (port, frames) in p_frames {
        result.insert(port, frames.into());
    }

    result
}
