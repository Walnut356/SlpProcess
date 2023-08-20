#![allow(clippy::uninit_vec)]

use bytes::{Buf, Bytes};
use nohash_hasher::IntMap;
use polars::prelude::*;

use crate::Port;

pub struct PostFrames {
    frame_number: Box<[i32]>,
    character: Box<[u8]>,
    action_state: Box<[u16]>,
    position_x: Box<[f32]>,
    position_y: Box<[f32]>,
    facing: Box<[f32]>,
    percent: Box<[f32]>,
    shield_health: Box<[f32]>,
    last_attack_landed: Box<[u8]>,
    combo_count: Box<[u8]>,
    last_hit_by: Box<[u8]>,
    stocks: Box<[u8]>,
    state_frame: Box<[Option<f32>]>,
    flags_1: Box<[Option<u8>]>,
    flags_2: Box<[Option<u8>]>,
    flags_3: Box<[Option<u8>]>,
    flags_4: Box<[Option<u8>]>,
    flags_5: Box<[Option<u8>]>,
    misc_as: Box<[Option<f32>]>,
    is_grounded: Box<[Option<bool>]>,
    last_ground_id: Box<[Option<u16>]>,
    jumps_remaining: Box<[Option<u8>]>,
    l_cancel: Box<[Option<u8>]>,
    hurtbox_state: Box<[Option<u8>]>,
    self_air_x: Box<[Option<f32>]>,
    self_y: Box<[Option<f32>]>,
    knockback_x: Box<[Option<f32>]>,
    knockback_y: Box<[Option<f32>]>,
    self_ground_x: Box<[Option<f32>]>,
    hitlag_remaining: Box<[Option<f32>]>,
    animation_index: Box<[Option<u32>]>,
}

impl PostFrames {
    fn new(len: usize) -> Self {
        PostFrames {
            frame_number: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            character: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            action_state: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            position_x: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            position_y: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            facing: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            percent: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            shield_health: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            last_attack_landed: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            combo_count: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            last_hit_by: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            stocks: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            state_frame: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            flags_1: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            flags_2: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            flags_3: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            flags_4: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            flags_5: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            misc_as: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            is_grounded: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            last_ground_id: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            jumps_remaining: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            l_cancel: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            hurtbox_state: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            self_air_x: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            self_y: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            knockback_x: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            knockback_y: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            self_ground_x: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            hitlag_remaining: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            animation_index: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
        }
    }
}

// postframe objects are purely a temporary container to make the code clearer, so I impl `Into` rather than `From` (and
// implicitly `Into`) because I intentionally want to disallow translation back.
#[allow(clippy::from_over_into)]
impl Into<DataFrame> for PostFrames {
    fn into(self) -> DataFrame {
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

pub fn parse_postframes(
    frames: &mut [Bytes],
    ports: [Port; 2],
    ics: [bool; 2],
) -> IntMap<u8, (DataFrame, Option<DataFrame>)> {
    let p_frames = {
        /* splitting these out saves us a small amount of time in conditional logic, and allows for
        exact iterator chunk sizes. */
        if !ics[0] && !ics[1] {
            no_ics(frames, ports)
        } else if ics[0] ^ ics[1] {
            one_ics(frames, ports, ics)
        } else {
            two_ics(frames, ports)
        }
    };

    let mut result = IntMap::default();

    for (port, (player_frames, nana_frames)) in p_frames {
        result.insert(port, (player_frames.into(), nana_frames.map(|x| x.into())));
    }

    result
}

pub fn no_ics(
    frames: &mut [Bytes],
    ports: [Port; 2],
) -> IntMap<u8, (PostFrames, Option<PostFrames>)> {
    let frames_iter = frames.chunks_exact_mut(2).enumerate();
    let len = frames_iter.len();

    let mut p_frames: IntMap<u8, (PostFrames, Option<PostFrames>)> = IntMap::default();
    p_frames.insert(ports[0].into(), (PostFrames::new(len), None));
    p_frames.insert(ports[1].into(), (PostFrames::new(len), None));

    for (i, frames_raw) in frames_iter {
        for frame in frames_raw {
            let frame_number = frame.get_i32();
            let port = frame.get_u8();

            frame.advance(1); // skip nana byte

            let (working, _) = p_frames.get_mut(&port).unwrap();

            unsafe {
                *working.frame_number.get_unchecked_mut(i) = frame_number;
                *working.character.get_unchecked_mut(i) = frame.get_u8();
                *working.action_state.get_unchecked_mut(i) = frame.get_u16();
                *working.position_x.get_unchecked_mut(i) = frame.get_f32();
                *working.position_y.get_unchecked_mut(i) = frame.get_f32();
                *working.facing.get_unchecked_mut(i) = frame.get_f32();
                *working.percent.get_unchecked_mut(i) = frame.get_f32();
                *working.shield_health.get_unchecked_mut(i) = frame.get_f32();
                *working.last_attack_landed.get_unchecked_mut(i) = frame.get_u8();
                *working.combo_count.get_unchecked_mut(i) = frame.get_u8();
                *working.last_hit_by.get_unchecked_mut(i) = frame.get_u8();
                *working.stocks.get_unchecked_mut(i) = frame.get_u8();

                // version 2.0.0
                if !frame.has_remaining() {
                    *working.state_frame.get_unchecked_mut(i) = None;
                    *working.flags_1.get_unchecked_mut(i) = None;
                    *working.flags_2.get_unchecked_mut(i) = None;
                    *working.flags_3.get_unchecked_mut(i) = None;
                    *working.flags_4.get_unchecked_mut(i) = None;
                    *working.flags_5.get_unchecked_mut(i) = None;
                    *working.misc_as.get_unchecked_mut(i) = None;
                    *working.is_grounded.get_unchecked_mut(i) = None;
                    *working.last_ground_id.get_unchecked_mut(i) = None;
                    *working.jumps_remaining.get_unchecked_mut(i) = None;
                    *working.l_cancel.get_unchecked_mut(i) = None;
                } else {
                    *working.state_frame.get_unchecked_mut(i) = Some(frame.get_f32());
                    *working.flags_1.get_unchecked_mut(i) = Some(frame.get_u8());
                    *working.flags_2.get_unchecked_mut(i) = Some(frame.get_u8());
                    *working.flags_3.get_unchecked_mut(i) = Some(frame.get_u8());
                    *working.flags_4.get_unchecked_mut(i) = Some(frame.get_u8());
                    *working.flags_5.get_unchecked_mut(i) = Some(frame.get_u8());
                    *working.misc_as.get_unchecked_mut(i) = Some(frame.get_f32());
                    *working.is_grounded.get_unchecked_mut(i) = Some(frame.get_u8() != 0);
                    *working.last_ground_id.get_unchecked_mut(i) = Some(frame.get_u16());
                    *working.jumps_remaining.get_unchecked_mut(i) = Some(frame.get_u8());
                    *working.l_cancel.get_unchecked_mut(i) = Some(frame.get_u8());
                }

                // version 2.1.0
                if !frame.has_remaining() {
                    *working.hurtbox_state.get_unchecked_mut(i) = None;
                } else {
                    *working.hurtbox_state.get_unchecked_mut(i) = Some(frame.get_u8());
                }

                // version 3.5.0
                if !frame.has_remaining() {
                    *working.self_air_x.get_unchecked_mut(i) = None;
                    *working.self_y.get_unchecked_mut(i) = None;
                    *working.knockback_x.get_unchecked_mut(i) = None;
                    *working.knockback_y.get_unchecked_mut(i) = None;
                    *working.self_ground_x.get_unchecked_mut(i) = None;
                } else {
                    *working.self_air_x.get_unchecked_mut(i) = Some(frame.get_f32());
                    *working.self_y.get_unchecked_mut(i) = Some(frame.get_f32());
                    *working.knockback_x.get_unchecked_mut(i) = Some(frame.get_f32());
                    *working.knockback_y.get_unchecked_mut(i) = Some(frame.get_f32());
                    *working.self_ground_x.get_unchecked_mut(i) = Some(frame.get_f32());
                }

                // version < 3.8.0
                if !frame.has_remaining() {
                    *working.hitlag_remaining.get_unchecked_mut(i) = None;
                } else {
                    *working.hitlag_remaining.get_unchecked_mut(i) = Some(frame.get_f32());
                }

                // version < 3.11.0
                if !frame.has_remaining() {
                    *working.animation_index.get_unchecked_mut(i) = None;
                } else {
                    *working.animation_index.get_unchecked_mut(i) = Some(frame.get_u32());
                }
            }
        }
    }
    p_frames
}

pub fn one_ics(
    frames: &mut [Bytes],
    ports: [Port; 2],
    ics: [bool; 2],
) -> IntMap<u8, (PostFrames, Option<PostFrames>)> {
    let frames_iter = frames.chunks_exact_mut(4).enumerate();
    let len = frames_iter.len();

    let mut p_frames: IntMap<u8, (PostFrames, Option<PostFrames>)> = IntMap::default();
    p_frames.insert(
        ports[0].into(),
        (PostFrames::new(len), ics[0].then(|| PostFrames::new(len))),
    );
    p_frames.insert(
        ports[1].into(),
        (PostFrames::new(len), ics[1].then(|| PostFrames::new(len))),
    );

    for (i, frames_raw) in frames_iter {
        for frame in frames_raw {
            let frame_number = frame.get_i32();
            let port = frame.get_u8();
            let nana = frame.get_u8() != 0;

            let working = {
                let temp = p_frames.get_mut(&port).unwrap();
                if nana {
                    temp.1.as_mut().unwrap()
                } else {
                    &mut temp.0
                }
            };

            unsafe {
                *working.frame_number.get_unchecked_mut(i) = frame_number;
                *working.character.get_unchecked_mut(i) = frame.get_u8();
                *working.action_state.get_unchecked_mut(i) = frame.get_u16();
                *working.position_x.get_unchecked_mut(i) = frame.get_f32();
                *working.position_y.get_unchecked_mut(i) = frame.get_f32();
                *working.facing.get_unchecked_mut(i) = frame.get_f32();
                *working.percent.get_unchecked_mut(i) = frame.get_f32();
                *working.shield_health.get_unchecked_mut(i) = frame.get_f32();
                *working.last_attack_landed.get_unchecked_mut(i) = frame.get_u8();
                *working.combo_count.get_unchecked_mut(i) = frame.get_u8();
                *working.last_hit_by.get_unchecked_mut(i) = frame.get_u8();
                *working.stocks.get_unchecked_mut(i) = frame.get_u8();

                // version 2.0.0
                if !frame.has_remaining() {
                    *working.state_frame.get_unchecked_mut(i) = None;
                    *working.flags_1.get_unchecked_mut(i) = None;
                    *working.flags_2.get_unchecked_mut(i) = None;
                    *working.flags_3.get_unchecked_mut(i) = None;
                    *working.flags_4.get_unchecked_mut(i) = None;
                    *working.flags_5.get_unchecked_mut(i) = None;
                    *working.misc_as.get_unchecked_mut(i) = None;
                    *working.is_grounded.get_unchecked_mut(i) = None;
                    *working.last_ground_id.get_unchecked_mut(i) = None;
                    *working.jumps_remaining.get_unchecked_mut(i) = None;
                    *working.l_cancel.get_unchecked_mut(i) = None;
                } else {
                    *working.state_frame.get_unchecked_mut(i) = Some(frame.get_f32());
                    *working.flags_1.get_unchecked_mut(i) = Some(frame.get_u8());
                    *working.flags_2.get_unchecked_mut(i) = Some(frame.get_u8());
                    *working.flags_3.get_unchecked_mut(i) = Some(frame.get_u8());
                    *working.flags_4.get_unchecked_mut(i) = Some(frame.get_u8());
                    *working.flags_5.get_unchecked_mut(i) = Some(frame.get_u8());
                    *working.misc_as.get_unchecked_mut(i) = Some(frame.get_f32());
                    *working.is_grounded.get_unchecked_mut(i) = Some(frame.get_u8() != 0);
                    *working.last_ground_id.get_unchecked_mut(i) = Some(frame.get_u16());
                    *working.jumps_remaining.get_unchecked_mut(i) = Some(frame.get_u8());
                    *working.l_cancel.get_unchecked_mut(i) = Some(frame.get_u8());
                }

                // version 2.1.0
                if !frame.has_remaining() {
                    *working.hurtbox_state.get_unchecked_mut(i) = None;
                } else {
                    *working.hurtbox_state.get_unchecked_mut(i) = Some(frame.get_u8());
                }

                // version 3.5.0
                if !frame.has_remaining() {
                    *working.self_air_x.get_unchecked_mut(i) = None;
                    *working.self_y.get_unchecked_mut(i) = None;
                    *working.knockback_x.get_unchecked_mut(i) = None;
                    *working.knockback_y.get_unchecked_mut(i) = None;
                    *working.self_ground_x.get_unchecked_mut(i) = None;
                } else {
                    *working.self_air_x.get_unchecked_mut(i) = Some(frame.get_f32());
                    *working.self_y.get_unchecked_mut(i) = Some(frame.get_f32());
                    *working.knockback_x.get_unchecked_mut(i) = Some(frame.get_f32());
                    *working.knockback_y.get_unchecked_mut(i) = Some(frame.get_f32());
                    *working.self_ground_x.get_unchecked_mut(i) = Some(frame.get_f32());
                }

                // version < 3.8.0
                if !frame.has_remaining() {
                    *working.hitlag_remaining.get_unchecked_mut(i) = None;
                } else {
                    *working.hitlag_remaining.get_unchecked_mut(i) = Some(frame.get_f32());
                }

                // version < 3.11.0
                if !frame.has_remaining() {
                    *working.animation_index.get_unchecked_mut(i) = None;
                } else {
                    *working.animation_index.get_unchecked_mut(i) = Some(frame.get_u32());
                }
            }
        }
    }

    p_frames
}

/// Frame ordering on a completed replay is guaranteed to be in port order, with that port's nana
/// frames directly following the leader frames
///
/// e.g.
/// * port 1: leader
/// * port 1: nana
/// * port 2: leader
/// * port 2: nana
pub fn two_ics(
    frames: &mut [Bytes],
    ports: [Port; 2],
) -> IntMap<u8, (PostFrames, Option<PostFrames>)> {
    let frames_iter = frames.chunks_exact_mut(4).enumerate();
    let len = frames_iter.len();

    let mut p_frames: IntMap<u8, (PostFrames, Option<PostFrames>)> = IntMap::default();
    p_frames.insert(
        ports[0].into(),
        (PostFrames::new(len), Some(PostFrames::new(len))),
    );
    p_frames.insert(
        ports[1].into(),
        (PostFrames::new(len), Some(PostFrames::new(len))),
    );

    for (i, frames_raw) in frames_iter {
        for frame in frames_raw {
            let frame_number = frame.get_i32();
            let port = frame.get_u8();
            let nana = frame.get_u8() != 0;

            let working = {
                let temp = p_frames.get_mut(&port).unwrap();
                if nana {
                    temp.1.as_mut().unwrap()
                } else {
                    &mut temp.0
                }
            };

            unsafe {
                *working.frame_number.get_unchecked_mut(i) = frame_number;
                *working.character.get_unchecked_mut(i) = frame.get_u8();
                *working.action_state.get_unchecked_mut(i) = frame.get_u16();
                *working.position_x.get_unchecked_mut(i) = frame.get_f32();
                *working.position_y.get_unchecked_mut(i) = frame.get_f32();
                *working.facing.get_unchecked_mut(i) = frame.get_f32();
                *working.percent.get_unchecked_mut(i) = frame.get_f32();
                *working.shield_health.get_unchecked_mut(i) = frame.get_f32();
                *working.last_attack_landed.get_unchecked_mut(i) = frame.get_u8();
                *working.combo_count.get_unchecked_mut(i) = frame.get_u8();
                *working.last_hit_by.get_unchecked_mut(i) = frame.get_u8();
                *working.stocks.get_unchecked_mut(i) = frame.get_u8();

                // version 2.0.0
                if !frame.has_remaining() {
                    *working.state_frame.get_unchecked_mut(i) = None;
                    *working.flags_1.get_unchecked_mut(i) = None;
                    *working.flags_2.get_unchecked_mut(i) = None;
                    *working.flags_3.get_unchecked_mut(i) = None;
                    *working.flags_4.get_unchecked_mut(i) = None;
                    *working.flags_5.get_unchecked_mut(i) = None;
                    *working.misc_as.get_unchecked_mut(i) = None;
                    *working.is_grounded.get_unchecked_mut(i) = None;
                    *working.last_ground_id.get_unchecked_mut(i) = None;
                    *working.jumps_remaining.get_unchecked_mut(i) = None;
                    *working.l_cancel.get_unchecked_mut(i) = None;
                } else {
                    *working.state_frame.get_unchecked_mut(i) = Some(frame.get_f32());
                    *working.flags_1.get_unchecked_mut(i) = Some(frame.get_u8());
                    *working.flags_2.get_unchecked_mut(i) = Some(frame.get_u8());
                    *working.flags_3.get_unchecked_mut(i) = Some(frame.get_u8());
                    *working.flags_4.get_unchecked_mut(i) = Some(frame.get_u8());
                    *working.flags_5.get_unchecked_mut(i) = Some(frame.get_u8());
                    *working.misc_as.get_unchecked_mut(i) = Some(frame.get_f32());
                    *working.is_grounded.get_unchecked_mut(i) = Some(frame.get_u8() != 0);
                    *working.last_ground_id.get_unchecked_mut(i) = Some(frame.get_u16());
                    *working.jumps_remaining.get_unchecked_mut(i) = Some(frame.get_u8());
                    *working.l_cancel.get_unchecked_mut(i) = Some(frame.get_u8());
                }

                // version 2.1.0
                if !frame.has_remaining() {
                    *working.hurtbox_state.get_unchecked_mut(i) = None;
                } else {
                    *working.hurtbox_state.get_unchecked_mut(i) = Some(frame.get_u8());
                }

                // version 3.5.0
                if !frame.has_remaining() {
                    *working.self_air_x.get_unchecked_mut(i) = None;
                    *working.self_y.get_unchecked_mut(i) = None;
                    *working.knockback_x.get_unchecked_mut(i) = None;
                    *working.knockback_y.get_unchecked_mut(i) = None;
                    *working.self_ground_x.get_unchecked_mut(i) = None;
                } else {
                    *working.self_air_x.get_unchecked_mut(i) = Some(frame.get_f32());
                    *working.self_y.get_unchecked_mut(i) = Some(frame.get_f32());
                    *working.knockback_x.get_unchecked_mut(i) = Some(frame.get_f32());
                    *working.knockback_y.get_unchecked_mut(i) = Some(frame.get_f32());
                    *working.self_ground_x.get_unchecked_mut(i) = Some(frame.get_f32());
                }

                // version < 3.8.0
                if !frame.has_remaining() {
                    *working.hitlag_remaining.get_unchecked_mut(i) = None;
                } else {
                    *working.hitlag_remaining.get_unchecked_mut(i) = Some(frame.get_f32());
                }

                // version < 3.11.0
                if !frame.has_remaining() {
                    *working.animation_index.get_unchecked_mut(i) = None;
                } else {
                    *working.animation_index.get_unchecked_mut(i) = Some(frame.get_u32());
                }
            }
        }
    }

    p_frames
}
