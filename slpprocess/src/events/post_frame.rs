#![allow(clippy::uninit_vec)]

use bytes::{Buf, Bytes};
use nohash_hasher::IntMap;
use polars::prelude::*;
use ssbm_utils::types::{Position, Velocity};

use crate::{columns::Post, events::game_start::Version, Port};

#[derive(Debug, Default, Clone)]
pub struct PostFrames {
    len: usize,
    version: Version,
    pub frame_index: Box<[i32]>,
    pub character: Box<[u8]>,
    pub action_state: Box<[u16]>,
    pub position: Box<[Position]>,
    pub orientation: Box<[f32]>,
    pub percent: Box<[f32]>,
    pub shield_health: Box<[f32]>,
    pub last_attack_landed: Box<[u8]>,
    pub combo_count: Box<[u8]>,
    pub last_hit_by: Box<[u8]>,
    pub stocks: Box<[u8]>,
    pub state_frame: Option<Box<[f32]>>,
    pub flags: Option<Box<[u64]>>,
    pub misc_as: Option<Box<[f32]>>,
    pub is_grounded: Option<Box<[bool]>>,
    pub last_ground_id: Option<Box<[u16]>>,
    pub jumps_remaining: Option<Box<[u8]>>,
    pub l_cancel: Option<Box<[u8]>>,
    pub hurtbox_state: Option<Box<[u8]>>,
    pub air_velocity: Option<Box<[Velocity]>>,
    pub knockback: Option<Box<[Velocity]>>,
    pub ground_velocity: Option<Box<[Velocity]>>,
    pub hitlag_remaining: Option<Box<[f32]>>,
    pub animation_index: Option<Box<[u32]>>,
}

impl PostFrames {
    fn new(duration: usize, version: Version) -> Self {
        PostFrames {
            len: duration,
            version,
            frame_index: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            character: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            action_state: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            position: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            orientation: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            percent: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            shield_health: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            last_attack_landed: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            combo_count: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            last_hit_by: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            stocks: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            state_frame: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                Some(temp.into_boxed_slice())
            },
            flags: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                Some(temp.into_boxed_slice())
            },
            misc_as: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                Some(temp.into_boxed_slice())
            },
            is_grounded: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                Some(temp.into_boxed_slice())
            },
            last_ground_id: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                Some(temp.into_boxed_slice())
            },
            jumps_remaining: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                Some(temp.into_boxed_slice())
            },
            l_cancel: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                Some(temp.into_boxed_slice())
            },
            hurtbox_state: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                Some(temp.into_boxed_slice())
            },
            air_velocity: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                Some(temp.into_boxed_slice())
            },
            knockback: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                Some(temp.into_boxed_slice())
            },
            ground_velocity: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                Some(temp.into_boxed_slice())
            },
            hitlag_remaining: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                Some(temp.into_boxed_slice())
            },
            animation_index: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                Some(temp.into_boxed_slice())
            },
        }
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.len
    }

    /// When nana is dead, she is considered `inactive`, which is the variable checked by slippi to
    /// determine what characters to record frames for. As a result, we cannot rely on the same
    /// invariants as `new()` (that provide extra optimization room). Because nana can have less
    /// frames than popo, we can't get away with skipping 0-initialization, since we don't know if
    /// the whole array will be overwritten. There's also a few awkward points such as needing to
    /// initialize the frame counter to the correct values since we won't have an event with which
    /// to populate them for any frames where nana is dead.
    ///
    /// a (possibly) nice result of this is that, unlike other parsers, we can guarantee that nana
    /// frames (if they exist) will always be the same length as leader frames, even if some of the
    /// data is filled with dummy "null" values.
    fn ics(duration: usize, version: Version) -> Self {
        let len = (duration - 123) as i32;
        PostFrames {
            len: duration,
            version,
            frame_index: ((-123)..len).collect::<Vec<i32>>().into_boxed_slice(),
            // Initialize character to nana, since she's the only one who can have "skipped" frames
            character: vec![33; duration].into_boxed_slice(),
            // Initialize to ActionState::Sleep, since that's what nana will be in when frames are
            // skipped
            action_state: vec![11; duration].into_boxed_slice(),
            // probably fine, though alternatively i could do something VERY obviously wrong like
            // NAN or negative infinity.
            position: vec![Position::default(); duration].into_boxed_slice(),
            // -1 is left, 1 is right. 0 is only used for warp star animation, but is hijacked for
            // `DOWN` for stats purposes. It should be appropriate here
            orientation: vec![0.0; duration].into_boxed_slice(),
            percent: vec![-1.0; duration].into_boxed_slice(),
            // may as well assume it's full when she's dead
            shield_health: vec![60.0; duration].into_boxed_slice(),
            // 0 is actually none for this
            last_attack_landed: vec![0; duration].into_boxed_slice(),
            combo_count: vec![0; duration].into_boxed_slice(),
            // IIRC 6 is the value it resets to after a short while anyway, so this should be fine
            last_hit_by: vec![6; duration].into_boxed_slice(),
            // ugh there's no good default value to put here i think
            stocks: vec![0; duration].into_boxed_slice(),

            // finally, freedom
            state_frame: Some(vec![0.0; duration].into_boxed_slice()),
            flags: Some(vec![0; duration].into_boxed_slice()),
            misc_as: Some(vec![0.0; duration].into_boxed_slice()),
            is_grounded: Some(vec![true; duration].into_boxed_slice()),
            last_ground_id: Some(vec![0; duration].into_boxed_slice()),
            jumps_remaining: Some(vec![0; duration].into_boxed_slice()),
            l_cancel: Some(vec![0; duration].into_boxed_slice()),
            hurtbox_state: Some(vec![0; duration].into_boxed_slice()),
            air_velocity: Some(vec![Velocity::default(); duration].into_boxed_slice()),
            knockback: Some(vec![Velocity::default(); duration].into_boxed_slice()),
            ground_velocity: Some(vec![Velocity::default(); duration].into_boxed_slice()),
            hitlag_remaining: Some(vec![0.0; duration].into_boxed_slice()),
            animation_index: Some(vec![0; duration].into_boxed_slice()),
        }
    }
}

impl From<PostFrames> for DataFrame {
    fn from(val: PostFrames) -> Self {
        let len = val.len();

        use Post as col;
        let mut vec_series = vec![
            Series::new(col::FrameIndex.into(), val.frame_index),
            Series::new(col::Character.into(), val.character),
            Series::new(col::ActionState.into(), val.action_state),
            StructChunked::new(
                col::Position.into(),
                &[
                    Series::new("x", val.position.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.position.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),
            Series::new(col::Orientation.into(), val.orientation),
            Series::new(col::Percent.into(), val.percent),
            Series::new(col::ShieldHealth.into(), val.shield_health),
            Series::new(col::LastAttackLanded.into(), val.last_attack_landed),
            Series::new(col::ComboCount.into(), val.combo_count),
            Series::new(col::LastHitBy.into(), val.last_hit_by),
            Series::new(col::Stocks.into(), val.stocks),
        ];

        if val.version.at_least(2, 0, 0) {
            vec_series.push(Series::new(
                col::StateFrame.into(),
                val.state_frame.unwrap(),
            ));
            vec_series.push(Series::new(col::Flags.into(), val.flags.unwrap()));
            vec_series.push(Series::new(col::MiscAS.into(), val.misc_as.unwrap()));
            vec_series.push(Series::new(
                col::IsGrounded.into(),
                val.is_grounded.unwrap(),
            ));
            vec_series.push(Series::new(
                col::LastGroundID.into(),
                val.last_ground_id.unwrap(),
            ));
            vec_series.push(Series::new(
                col::JumpsRemaining.into(),
                val.jumps_remaining.unwrap(),
            ));
            vec_series.push(Series::new(col::LCancel.into(), val.l_cancel.unwrap()));
        } else {
            vec_series.push(Series::new_null(col::StateFrame.into(), len));
            vec_series.push(Series::new_null(col::Flags.into(), len));
            vec_series.push(Series::new_null(col::MiscAS.into(), len));
            vec_series.push(Series::new_null(col::IsGrounded.into(), len));
            vec_series.push(Series::new_null(col::LastGroundID.into(), len));
            vec_series.push(Series::new_null(col::JumpsRemaining.into(), len));
            vec_series.push(Series::new_null(col::LCancel.into(), len));
        }

        if val.version.at_least(2, 1, 0) {
            vec_series.push(Series::new(
                col::HurtboxState.into(),
                val.hurtbox_state.unwrap(),
            ));
        } else {
            vec_series.push(Series::new_null(col::HurtboxState.into(), len));
        }

        if val.version.at_least(3, 5, 0) {
            vec_series.push(
                StructChunked::new(
                    col::AirVel.into(),
                    &[
                        Series::new(
                            "x",
                            val.air_velocity
                                .as_ref()
                                .unwrap()
                                .iter()
                                .map(|p| p.x)
                                .collect::<Vec<_>>(),
                        ),
                        Series::new(
                            "y",
                            val.air_velocity
                                .as_ref()
                                .unwrap()
                                .iter()
                                .map(|p| p.y)
                                .collect::<Vec<_>>(),
                        ),
                    ],
                )
                .unwrap()
                .into_series(),
            );
            vec_series.push(
                StructChunked::new(
                    col::Knockback.into(),
                    &[
                        Series::new(
                            "x",
                            val.knockback
                                .as_ref()
                                .unwrap()
                                .iter()
                                .map(|p| p.x)
                                .collect::<Vec<_>>(),
                        ),
                        Series::new(
                            "y",
                            val.knockback
                                .as_ref()
                                .unwrap()
                                .iter()
                                .map(|p| p.y)
                                .collect::<Vec<_>>(),
                        ),
                    ],
                )
                .unwrap()
                .into_series(),
            );
            vec_series.push(
                StructChunked::new(
                    col::GroundVel.into(),
                    &[
                        Series::new(
                            "x",
                            val.ground_velocity
                                .as_ref()
                                .unwrap()
                                .iter()
                                .map(|p| p.x)
                                .collect::<Vec<_>>(),
                        ),
                        Series::new(
                            "y",
                            val.ground_velocity
                                .as_ref()
                                .unwrap()
                                .iter()
                                .map(|p| p.y)
                                .collect::<Vec<_>>(),
                        ),
                    ],
                )
                .unwrap()
                .into_series(),
            );
        } else {
            vec_series.push(Series::new_null(col::AirVel.into(), len));
            vec_series.push(Series::new_null(col::Knockback.into(), len));
            vec_series.push(Series::new_null(col::GroundVel.into(), len));
        }

        if val.version.at_least(3, 8, 0) {
            vec_series.push(Series::new(
                col::HitlagRemaining.into(),
                val.hitlag_remaining.unwrap(),
            ));
        } else {
            vec_series.push(Series::new_null(col::HitlagRemaining.into(), len));
        }

        if val.version.at_least(3, 11, 0) {
            vec_series.push(Series::new(
                col::AnimationIndex.into(),
                val.animation_index.unwrap(),
            ));
        } else {
            vec_series.push(Series::new_null(col::AnimationIndex.into(), len));
        }

        DataFrame::new(vec_series).unwrap()
    }
}

pub fn parse_postframes(
    version: Version,
    frames: &mut [Bytes],
    duration: u64,
    ports: [Port; 2],
    ics: [bool; 2],
) -> IntMap<u8, (PostFrames, Option<PostFrames>)> {
    /* splitting these out saves us a small amount of time in conditional logic, and allows for
    exact iterator chunk sizes. */
    if !ics[0] && !ics[1] {
        unpack_frames(frames, ports, version)
    } else {
        unpack_frames_ics(frames, duration, ports, ics, version)
    }
}

/// Slightly more optimized version of the typical parsing code, due to invariants regarding frame
/// event ordering and counts
pub fn unpack_frames(
    frames: &mut [Bytes],
    ports: [Port; 2],
    version: Version,
) -> IntMap<u8, (PostFrames, Option<PostFrames>)> {
    let frames_iter = frames.chunks_exact_mut(2).enumerate();
    let len = frames_iter.len();

    let mut p_frames: IntMap<u8, (PostFrames, Option<PostFrames>)> = IntMap::default();
    p_frames.insert(ports[0] as u8, (PostFrames::new(len, version), None));
    p_frames.insert(ports[1] as u8, (PostFrames::new(len, version), None));

    for (i, frames_raw) in frames_iter {
        for frame in frames_raw {
            let frame_number = frame.get_i32();
            let port = frame.get_u8();

            frame.advance(1); // skip nana byte

            let (working, _) = p_frames.get_mut(&port).unwrap();

            unsafe {
                *working.frame_index.get_unchecked_mut(i) = frame_number;
                *working.character.get_unchecked_mut(i) = frame.get_u8();
                *working.action_state.get_unchecked_mut(i) = frame.get_u16();
                *working.position.get_unchecked_mut(i) =
                    Position::new(frame.get_f32(), frame.get_f32());
                *working.orientation.get_unchecked_mut(i) = frame.get_f32();
                *working.percent.get_unchecked_mut(i) = frame.get_f32();
                *working.shield_health.get_unchecked_mut(i) = frame.get_f32();
                *working.last_attack_landed.get_unchecked_mut(i) = frame.get_u8();
                *working.combo_count.get_unchecked_mut(i) = frame.get_u8();
                *working.last_hit_by.get_unchecked_mut(i) = frame.get_u8();
                *working.stocks.get_unchecked_mut(i) = frame.get_u8();

                // version 2.0.0
                if !frame.has_remaining() {
                    continue;
                } else {
                    *working.state_frame.as_mut().unwrap().get_unchecked_mut(i) = frame.get_f32();
                    let flags_1 = frame.get_u8() as u64;
                    let flags_2 = frame.get_u8() as u64;
                    let flags_3 = frame.get_u8() as u64;
                    let flags_4 = frame.get_u8() as u64;
                    let flags_5 = frame.get_u8() as u64;
                    let flags: u64 = flags_1
                        | (flags_2 << 8)
                        | (flags_3 << 16)
                        | (flags_4 << 24)
                        | (flags_5 << 32);
                    *working.flags.as_mut().unwrap().get_unchecked_mut(i) = flags;
                    *working.misc_as.as_mut().unwrap().get_unchecked_mut(i) = frame.get_f32();
                    *working.is_grounded.as_mut().unwrap().get_unchecked_mut(i) =
                        frame.get_u8() != 0;
                    *working
                        .last_ground_id
                        .as_mut()
                        .unwrap()
                        .get_unchecked_mut(i) = frame.get_u16();
                    *working
                        .jumps_remaining
                        .as_mut()
                        .unwrap()
                        .get_unchecked_mut(i) = frame.get_u8();
                    *working.l_cancel.as_mut().unwrap().get_unchecked_mut(i) = frame.get_u8();
                }

                // version 2.1.0
                if !frame.has_remaining() {
                    continue;
                } else {
                    *working.hurtbox_state.as_mut().unwrap().get_unchecked_mut(i) = frame.get_u8();
                }

                // version 3.5.0
                if !frame.has_remaining() {
                    continue;
                } else {
                    let air_vel_x = frame.get_f32();
                    let vel_y = frame.get_f32();
                    *working.air_velocity.as_mut().unwrap().get_unchecked_mut(i) =
                        Velocity::new(air_vel_x, vel_y);
                    *working.knockback.as_mut().unwrap().get_unchecked_mut(i) =
                        Velocity::new(frame.get_f32(), frame.get_f32());
                    *working
                        .ground_velocity
                        .as_mut()
                        .unwrap()
                        .get_unchecked_mut(i) = Velocity::new(frame.get_f32(), vel_y);
                }

                // version < 3.8.0
                if !frame.has_remaining() {
                    continue;
                } else {
                    *working
                        .hitlag_remaining
                        .as_mut()
                        .unwrap()
                        .get_unchecked_mut(i) = frame.get_f32();
                }

                // version < 3.11.0
                if !frame.has_remaining() {
                    continue;
                } else {
                    *working
                        .animation_index
                        .as_mut()
                        .unwrap()
                        .get_unchecked_mut(i) = frame.get_u32();
                }
            }
        }
    }
    p_frames
}

pub fn unpack_frames_ics(
    frames: &mut [Bytes],
    duration: u64,
    ports: [Port; 2],
    ics: [bool; 2],
    version: Version,
) -> IntMap<u8, (PostFrames, Option<PostFrames>)> {
    let len = duration as usize;

    let mut p_frames: IntMap<u8, (PostFrames, Option<PostFrames>)> = IntMap::default();
    p_frames.insert(
        ports[0] as u8,
        (
            PostFrames::new(len, version),
            ics[0].then(|| PostFrames::ics(len, version)),
        ),
    );
    p_frames.insert(
        ports[1] as u8,
        (
            PostFrames::new(len, version),
            ics[1].then(|| PostFrames::ics(len, version)),
        ),
    );

    for frame in frames.iter_mut() {
        let frame_number = frame.get_i32();
        // since we can't chunk the frames, enumeration won't work. We can still get an
        // always-in-bounds index from the frame number though.
        let i = (frame_number + 123) as usize;
        assert!(
            i < len,
            "Frame index incorrect, index ({i}) is greater than or equal to the max length of the container ({len})."
        );
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
            *working.frame_index.get_unchecked_mut(i) = frame_number;
            *working.character.get_unchecked_mut(i) = frame.get_u8();
            *working.action_state.get_unchecked_mut(i) = frame.get_u16();
            *working.position.get_unchecked_mut(i) =
                Position::new(frame.get_f32(), frame.get_f32());
            *working.orientation.get_unchecked_mut(i) = frame.get_f32();
            *working.percent.get_unchecked_mut(i) = frame.get_f32();
            *working.shield_health.get_unchecked_mut(i) = frame.get_f32();
            *working.last_attack_landed.get_unchecked_mut(i) = frame.get_u8();
            *working.combo_count.get_unchecked_mut(i) = frame.get_u8();
            *working.last_hit_by.get_unchecked_mut(i) = frame.get_u8();
            *working.stocks.get_unchecked_mut(i) = frame.get_u8();

            // version 2.0.0
            if !frame.has_remaining() {
                continue;
            } else {
                *working.state_frame.as_mut().unwrap().get_unchecked_mut(i) = frame.get_f32();
                let flags_1 = frame.get_u8() as u64;
                let flags_2 = frame.get_u8() as u64;
                let flags_3 = frame.get_u8() as u64;
                let flags_4 = frame.get_u8() as u64;
                let flags_5 = frame.get_u8() as u64;
                let flags: u64 =
                    flags_1 & (flags_2 << 8) & (flags_3 << 16) & (flags_4 << 24) & (flags_5 << 32);
                *working.flags.as_mut().unwrap().get_unchecked_mut(i) = flags;
                *working.misc_as.as_mut().unwrap().get_unchecked_mut(i) = frame.get_f32();
                *working.is_grounded.as_mut().unwrap().get_unchecked_mut(i) = frame.get_u8() == 0;
                *working
                    .last_ground_id
                    .as_mut()
                    .unwrap()
                    .get_unchecked_mut(i) = frame.get_u16();
                *working
                    .jumps_remaining
                    .as_mut()
                    .unwrap()
                    .get_unchecked_mut(i) = frame.get_u8();
                *working.l_cancel.as_mut().unwrap().get_unchecked_mut(i) = frame.get_u8();
            }

            // version 2.1.0
            if !frame.has_remaining() {
                continue;
            } else {
                *working.hurtbox_state.as_mut().unwrap().get_unchecked_mut(i) = frame.get_u8();
            }

            // version 3.5.0
            if !frame.has_remaining() {
                continue;
            } else {
                let air_vel_x = frame.get_f32();
                let vel_y = frame.get_f32();
                *working.air_velocity.as_mut().unwrap().get_unchecked_mut(i) =
                    Velocity::new(air_vel_x, vel_y);
                *working.knockback.as_mut().unwrap().get_unchecked_mut(i) =
                    Velocity::new(frame.get_f32(), frame.get_f32());
                *working
                    .ground_velocity
                    .as_mut()
                    .unwrap()
                    .get_unchecked_mut(i) = Velocity::new(frame.get_f32(), vel_y);
            }

            // version < 3.8.0
            if !frame.has_remaining() {
                continue;
            } else {
                *working
                    .hitlag_remaining
                    .as_mut()
                    .unwrap()
                    .get_unchecked_mut(i) = frame.get_f32();
            }

            // version < 3.11.0
            if !frame.has_remaining() {
                continue;
            } else {
                *working
                    .animation_index
                    .as_mut()
                    .unwrap()
                    .get_unchecked_mut(i) = frame.get_u32();
            }
        }
    }

    p_frames
}
