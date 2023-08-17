#![allow(clippy::uninit_vec)]

use bytes::{Buf, Bytes};
use minstant::Instant;
use nohash_hasher::IntMap;
use polars::prelude::*;
use rayon::prelude::IntoParallelRefIterator;

use crate::{utils::BufUnchecked, Port};

impl BufUnchecked for Bytes {}

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
            frame_number: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            character: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            action_state: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            position_x: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            position_y: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            facing: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            percent: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            shield_health: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            last_attack_landed: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            combo_count: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            last_hit_by: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            stocks: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            state_frame: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            flags_1: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            flags_2: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            flags_3: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            flags_4: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            flags_5: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            misc_as: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            is_grounded: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            last_ground_id: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            jumps_remaining: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            l_cancel: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            hurtbox_state: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            self_air_x: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            self_y: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            knockback_x: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            knockback_y: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            self_ground_x: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            hitlag_remaining: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            animation_index: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
        }
    }

    fn byte_swap(&mut self) {
        self.frame_number
            .iter_mut()
            .for_each(|x| *x = x.swap_bytes());
        self.action_state
            .iter_mut()
            .for_each(|x| *x = x.swap_bytes());
        self.position_x
            .iter_mut()
            .for_each(|x| *x = f32::from_be_bytes(x.to_ne_bytes()));
        self.position_y
            .iter_mut()
            .for_each(|x| *x = f32::from_be_bytes(x.to_ne_bytes()));
        self.facing
            .iter_mut()
            .for_each(|x| *x = f32::from_be_bytes(x.to_ne_bytes()));
        self.percent
            .iter_mut()
            .for_each(|x| *x = f32::from_be_bytes(x.to_ne_bytes()));
        self.shield_health
            .iter_mut()
            .for_each(|x| *x = f32::from_be_bytes(x.to_ne_bytes()));
        self.state_frame
            .iter_mut()
            .for_each(|x| *x = x.map(|x| f32::from_be_bytes(x.to_ne_bytes())));
        self.misc_as
            .iter_mut()
            .for_each(|x| *x = x.map(|x| f32::from_be_bytes(x.to_ne_bytes())));
        self.last_ground_id
            .iter_mut()
            .for_each(|x| *x = x.map(|x| x.swap_bytes()));
        self.self_air_x
            .iter_mut()
            .for_each(|x| *x = x.map(|x| f32::from_be_bytes(x.to_ne_bytes())));
        self.self_y
            .iter_mut()
            .for_each(|x| *x = x.map(|x| f32::from_be_bytes(x.to_ne_bytes())));
        self.knockback_x
            .iter_mut()
            .for_each(|x| *x = x.map(|x| f32::from_be_bytes(x.to_ne_bytes())));
        self.knockback_y
            .iter_mut()
            .for_each(|x| *x = x.map(|x| f32::from_be_bytes(x.to_ne_bytes())));
        self.self_ground_x
            .iter_mut()
            .for_each(|x| *x = x.map(|x| f32::from_be_bytes(x.to_ne_bytes())));
        self.hitlag_remaining
            .iter_mut()
            .for_each(|x| *x = x.map(|x| f32::from_be_bytes(x.to_ne_bytes())));
        self.animation_index
            .iter_mut()
            .for_each(|x| *x = x.map(|x| x.swap_bytes()));
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

pub fn parse_postframes(frames: &mut [Bytes], ports: [Port; 2], ics: [bool; 2]) -> [DataFrame; 2] {
    let len = frames.len() / 2;

    let mut p_frames: [PostFrames; 2] = [PostFrames::new(len), PostFrames::new(len)];

    if !ics[0] && !ics[1] {
        no_ics(frames, &mut p_frames);
    } else if ics[0] ^ ics[1] {
        one_ics(frames, &mut p_frames);
    } else {
        two_ics(frames, &mut p_frames);
    }

    // for frames in p_frames.iter_mut() {
    //     frames.byte_swap();
    // }

    p_frames.map(|x| x.into())
}

pub fn no_ics(frames: &mut [Bytes], p_frames: &mut [PostFrames; 2]) {
    for (i, frame) in frames.chunks_exact_mut(2).enumerate() {
        let frame_number = frame[0].get_i32();
        let port = frame[0].get_u8();
        let working = p_frames.get_mut(0).unwrap();
        // let is_nana = frame.get_u8() != 0;
        frame[0].advance(1);

        working.frame_number[i] = frame_number;

        working.character[i] = frame[0].get_u8();
        working.action_state[i] = frame[0].get_u16();
        working.position_x[i] = frame[0].get_f32();
        working.position_y[i] = frame[0].get_f32();
        working.facing[i] = frame[0].get_f32();
        working.percent[i] = frame[0].get_f32();
        working.shield_health[i] = frame[0].get_f32();
        working.last_attack_landed[i] = frame[0].get_u8();
        working.combo_count[i] = frame[0].get_u8();
        working.last_hit_by[i] = frame[0].get_u8();
        working.stocks[i] = frame[0].get_u8();
        if !frame[0].has_remaining() {
            // version < 2.0.0
            continue;
        }
        working.state_frame[i] = Some(frame[0].get_f32());
        working.flags_1[i] = Some(frame[0].get_u8());
        working.flags_2[i] = Some(frame[0].get_u8());
        working.flags_3[i] = Some(frame[0].get_u8());
        working.flags_4[i] = Some(frame[0].get_u8());
        working.flags_5[i] = Some(frame[0].get_u8());
        working.misc_as[i] = Some(frame[0].get_f32());
        working.is_grounded[i] = Some(frame[0].get_u8() != 0);
        working.last_ground_id[i] = Some(frame[0].get_u16());
        working.jumps_remaining[i] = Some(frame[0].get_u8());
        working.l_cancel[i] = Some(frame[0].get_u8());
        if !frame[0].has_remaining() {
            // version < 2.1.0
            continue;
        }
        working.hurtbox_state[i] = Some(frame[0].get_u8());
        if !frame[0].has_remaining() {
            // version < 3.5.0
            continue;
        }
        working.self_air_x[i] = Some(frame[0].get_f32());
        working.self_y[i] = Some(frame[0].get_f32());
        working.knockback_x[i] = Some(frame[0].get_f32());
        working.knockback_y[i] = Some(frame[0].get_f32());
        working.self_ground_x[i] = Some(frame[0].get_f32());
        if !frame[0].has_remaining() {
            // version < 3.8.0
            continue;
        }
        working.hitlag_remaining[i] = Some(frame[0].get_f32());
        if !frame[0].has_remaining() {
            // version < 3.11.0
            continue;
        }
        working.animation_index[i] = Some(frame[0].get_u32());

        // -------------------------------------- player 2 -------------------------------------- //
        let frame_number = frame[1].get_i32();
        let port = frame[1].get_u8();
        let working = p_frames.get_mut(1).unwrap();
        // let is_nana = frame.get_u8() != 0;
        frame[1].advance(1);

        working.frame_number[i] = frame_number;

        working.character[i] = frame[1].get_u8();
        working.action_state[i] = frame[1].get_u16();
        working.position_x[i] = frame[1].get_f32();
        working.position_y[i] = frame[1].get_f32();
        working.facing[i] = frame[1].get_f32();
        working.percent[i] = frame[1].get_f32();
        working.shield_health[i] = frame[1].get_f32();
        working.last_attack_landed[i] = frame[1].get_u8();
        working.combo_count[i] = frame[1].get_u8();
        working.last_hit_by[i] = frame[1].get_u8();
        working.stocks[i] = frame[1].get_u8();
        if !frame[1].has_remaining() {
            // version < 2.0.0
            continue;
        }
        working.state_frame[i] = Some(frame[1].get_f32());
        working.flags_1[i] = Some(frame[1].get_u8());
        working.flags_2[i] = Some(frame[1].get_u8());
        working.flags_3[i] = Some(frame[1].get_u8());
        working.flags_4[i] = Some(frame[1].get_u8());
        working.flags_5[i] = Some(frame[1].get_u8());
        working.misc_as[i] = Some(frame[1].get_f32());
        working.is_grounded[i] = Some(frame[1].get_u8() != 0);
        working.last_ground_id[i] = Some(frame[1].get_u16_ne());
        working.jumps_remaining[i] = Some(frame[1].get_u8());
        working.l_cancel[i] = Some(frame[1].get_u8());
        if !frame[1].has_remaining() {
            // version < 2.1.0
            continue;
        }
        working.hurtbox_state[i] = Some(frame[1].get_u8());
        if !frame[1].has_remaining() {
            // version < 3.5.0
            continue;
        }
        working.self_air_x[i] = Some(frame[1].get_f32());
        working.self_y[i] = Some(frame[1].get_f32());
        working.knockback_x[i] = Some(frame[1].get_f32());
        working.knockback_y[i] = Some(frame[1].get_f32());
        working.self_ground_x[i] = Some(frame[1].get_f32());
        if !frame[1].has_remaining() {
            // version < 3.8.0
            continue;
        }
        working.hitlag_remaining[i] = Some(frame[1].get_f32());
        if !frame[1].has_remaining() {
            // version < 3.11.0
            continue;
        }
        working.animation_index[i] = Some(frame[1].get_u32());
    }
}

fn one_ics(frames: &mut [Bytes], p_frames: &mut [PostFrames; 2]) {}

fn two_ics(frames: &mut [Bytes], p_frames: &mut [PostFrames; 2]) {}
