#![allow(clippy::uninit_vec)]

use std::sync::Arc;

use anyhow::{anyhow, Result};
use bytes::{Buf, Bytes};
use nohash_hasher::IntMap;
use ssbm_utils::{
    enums::{Character, LCancelState, State},
    prelude::{Flags, Hurtbox, Orientation},
    types::{Position, Velocity},
};

use crate::{events::game_start::Version, game::Metadata, Port};

/// Contains all post-frame data for a single character. Stored in columnar format, thus row-wise
/// access via `.get_frame(index)` will be very slow. If possible, only iterate through the columns
/// you need.
#[derive(Debug, Default, Clone)]
pub struct PostFrames {
    pub metadata: Arc<Metadata>,
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
    pub instance_hit_by: Option<Box<[u16]>>,
    pub instance_id: Option<Box<[u16]>>,
}

impl PostFrames {
    fn new(metadata: Arc<Metadata>) -> Self {
        let duration = metadata.total_frames;
        let version = metadata.version;
        PostFrames {
            metadata,
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
            state_frame: if version.at_least(0, 2, 0) {
                unsafe {
                    let mut temp = Vec::with_capacity(duration);
                    temp.set_len(duration);
                    Some(temp.into_boxed_slice())
                }
            } else {
                None
            },
            flags: if version.at_least(2, 0, 0) {
                unsafe {
                    let mut temp = Vec::with_capacity(duration);
                    temp.set_len(duration);
                    Some(temp.into_boxed_slice())
                }
            } else {
                None
            },
            misc_as: if version.at_least(2, 0, 0) {
                unsafe {
                    let mut temp = Vec::with_capacity(duration);
                    temp.set_len(duration);
                    Some(temp.into_boxed_slice())
                }
            } else {
                None
            },
            is_grounded: if version.at_least(2, 0, 0) {
                unsafe {
                    let mut temp = Vec::with_capacity(duration);
                    temp.set_len(duration);
                    Some(temp.into_boxed_slice())
                }
            } else {
                None
            },
            last_ground_id: if version.at_least(2, 0, 0) {
                unsafe {
                    let mut temp = Vec::with_capacity(duration);
                    temp.set_len(duration);
                    Some(temp.into_boxed_slice())
                }
            } else {
                None
            },
            jumps_remaining: if version.at_least(2, 0, 0) {
                unsafe {
                    let mut temp = Vec::with_capacity(duration);
                    temp.set_len(duration);
                    Some(temp.into_boxed_slice())
                }
            } else {
                None
            },
            l_cancel: if version.at_least(2, 0, 0) {
                unsafe {
                    let mut temp = Vec::with_capacity(duration);
                    temp.set_len(duration);
                    Some(temp.into_boxed_slice())
                }
            } else {
                None
            },
            hurtbox_state: if version.at_least(2, 1, 0) {
                unsafe {
                    let mut temp = Vec::with_capacity(duration);
                    temp.set_len(duration);
                    Some(temp.into_boxed_slice())
                }
            } else {
                None
            },
            air_velocity: if version.at_least(3, 5, 0) {
                unsafe {
                    let mut temp = Vec::with_capacity(duration);
                    temp.set_len(duration);
                    Some(temp.into_boxed_slice())
                }
            } else {
                None
            },
            knockback: if version.at_least(3, 5, 0) {
                unsafe {
                    let mut temp = Vec::with_capacity(duration);
                    temp.set_len(duration);
                    Some(temp.into_boxed_slice())
                }
            } else {
                None
            },
            ground_velocity: if version.at_least(3, 5, 0) {
                unsafe {
                    let mut temp = Vec::with_capacity(duration);
                    temp.set_len(duration);
                    Some(temp.into_boxed_slice())
                }
            } else {
                None
            },
            hitlag_remaining: if version.at_least(3, 8, 0) {
                unsafe {
                    let mut temp = Vec::with_capacity(duration);
                    temp.set_len(duration);
                    Some(temp.into_boxed_slice())
                }
            } else {
                None
            },
            animation_index: if version.at_least(3, 11, 0) {
                unsafe {
                    let mut temp = Vec::with_capacity(duration);
                    temp.set_len(duration);
                    Some(temp.into_boxed_slice())
                }
            } else {
                None
            },
            instance_hit_by: if version.at_least(3, 16, 0) {
                unsafe {
                    let mut temp = Vec::with_capacity(duration);
                    temp.set_len(duration);
                    Some(temp.into_boxed_slice())
                }
            } else {
                None
            },
            instance_id: if version.at_least(3, 16, 0) {
                unsafe {
                    let mut temp = Vec::with_capacity(duration);
                    temp.set_len(duration);
                    Some(temp.into_boxed_slice())
                }
            } else {
                None
            },
        }
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.metadata.total_frames
    }

    /// Gets the full post-frame data for a given frame index (0-indexed). This is very
    /// slow compared to iterating through only the columns you need.
    pub fn get_frame(&self, index: usize) -> PostRow {
        PostRow {
            frame_index: self.frame_index[index],
            character: self.character[index],
            action_state: self.action_state[index],
            position: self.position[index],
            orientation: self.orientation[index],
            percent: self.percent[index],
            shield_health: self.shield_health[index],
            last_attack_landed: self.last_attack_landed[index],
            combo_count: self.combo_count[index],
            last_hit_by: self.last_hit_by[index],
            stocks: self.stocks[index],
            state_frame: self.state_frame.as_ref().map(|x| x[index]),
            flags: self.flags.as_ref().map(|x| x[index]),
            misc_as: self.misc_as.as_ref().map(|x| x[index]),
            is_grounded: self.is_grounded.as_ref().map(|x| x[index]),
            last_ground_id: self.last_ground_id.as_ref().map(|x| x[index]),
            jumps_remaining: self.jumps_remaining.as_ref().map(|x| x[index]),
            l_cancel: self.l_cancel.as_ref().map(|x| x[index]),
            hurtbox_state: self.hurtbox_state.as_ref().map(|x| x[index]),
            air_velocity: self.air_velocity.as_ref().map(|x| x[index]),
            knockback: self.knockback.as_ref().map(|x| x[index]),
            ground_velocity: self.ground_velocity.as_ref().map(|x| x[index]),
            hitlag_remaining: self.hitlag_remaining.as_ref().map(|x| x[index]),
            animation_index: self.animation_index.as_ref().map(|x| x[index]),
            instance_hit_by: self.instance_hit_by.as_ref().map(|x| x[index]),
            instance_id: self.instance_id.as_ref().map(|x| x[index]),
        }
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
    fn ics(metadata: Arc<Metadata>) -> Self {
        let duration = metadata.total_frames;
        let version = metadata.version;
        PostFrames {
            metadata,
            frame_index: ((-123)..(duration as i32 - 123))
                .collect::<Vec<i32>>()
                .into_boxed_slice(),
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
            state_frame: if version.at_least(0, 2, 0) {
                Some(vec![0.0; duration].into_boxed_slice())
            } else {
                None
            },
            flags: if version.at_least(2, 0, 0) {
                Some(vec![0; duration].into_boxed_slice())
            } else {
                None
            },
            misc_as: if version.at_least(2, 0, 0) {
                Some(vec![0.0; duration].into_boxed_slice())
            } else {
                None
            },
            is_grounded: if version.at_least(2, 0, 0) {
                Some(vec![true; duration].into_boxed_slice())
            } else {
                None
            },
            last_ground_id: if version.at_least(2, 0, 0) {
                Some(vec![0; duration].into_boxed_slice())
            } else {
                None
            },
            jumps_remaining: if version.at_least(2, 0, 0) {
                Some(vec![0; duration].into_boxed_slice())
            } else {
                None
            },
            l_cancel: if version.at_least(2, 0, 0) {
                Some(vec![0; duration].into_boxed_slice())
            } else {
                None
            },
            hurtbox_state: if version.at_least(2, 1, 0) {
                Some(vec![0; duration].into_boxed_slice())
            } else {
                None
            },
            air_velocity: if version.at_least(3, 5, 0) {
                Some(vec![Velocity::default(); duration].into_boxed_slice())
            } else {
                None
            },
            knockback: if version.at_least(3, 5, 0) {
                Some(vec![Velocity::default(); duration].into_boxed_slice())
            } else {
                None
            },
            ground_velocity: if version.at_least(3, 5, 0) {
                Some(vec![Velocity::default(); duration].into_boxed_slice())
            } else {
                None
            },
            hitlag_remaining: if version.at_least(3, 8, 0) {
                Some(vec![0.0; duration].into_boxed_slice())
            } else {
                None
            },
            animation_index: if version.at_least(3, 11, 0) {
                Some(vec![0; duration].into_boxed_slice())
            } else {
                None
            },
            instance_hit_by: if version.at_least(3, 16, 0) {
                Some(vec![0; duration].into_boxed_slice())
            } else {
                None
            },
            instance_id: if version.at_least(3, 16, 0) {
                Some(vec![0; duration].into_boxed_slice())
            } else {
                None
            },
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PostRow {
    pub frame_index: i32,
    pub character: u8,
    pub action_state: u16,
    pub position: Position,
    pub orientation: f32,
    pub percent: f32,
    pub shield_health: f32,
    pub last_attack_landed: u8,
    pub combo_count: u8,
    pub last_hit_by: u8,
    pub stocks: u8,
    pub state_frame: Option<f32>,
    pub flags: Option<u64>,
    pub misc_as: Option<f32>,
    pub is_grounded: Option<bool>,
    pub last_ground_id: Option<u16>,
    pub jumps_remaining: Option<u8>,
    pub l_cancel: Option<u8>,
    pub hurtbox_state: Option<u8>,
    pub air_velocity: Option<Velocity>,
    pub knockback: Option<Velocity>,
    pub ground_velocity: Option<Velocity>,
    pub hitlag_remaining: Option<f32>,
    pub animation_index: Option<u32>,
    pub instance_hit_by: Option<u16>,
    pub instance_id: Option<u16>,
}

impl std::fmt::Display for PostRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let character = Character::try_from_internal(self.character).unwrap();
        write!(
            f,
            r"PostFrame {{
    frame_index: {},
    character: {},
    action_state: {},
    position: {},
    orientation: {},
    percent: {},
    shield_health: {},
    last_attack_landed: {},
    combo_count: {},
    last_hit_by: {},
    stocks: {},
    state_frame: {:?},
    flags: {:?},
    misc_as: {:?},
    is_grounded: {:?},
    last_ground_id: {:?},
    jumps_remaining: {:?},
    l_cancel: {:?},
    hurtbox_state: {:?},
    air_velocity: {:?},
    knockback: {:?},
    ground_velocity: {:?},
    hitlag_remaining: {:?},
    animation_index: {:?},
    instance_hit_by: {:?},
    instance_id: {:?},
}}",
            self.frame_index,
            character,
            State::from_state_and_char(self.action_state, Some(character)),
            self.position,
            Orientation::from_repr(self.orientation as i8).unwrap(),
            self.percent,
            self.shield_health,
            self.last_attack_landed,
            self.combo_count,
            self.last_hit_by,
            self.stocks,
            self.state_frame,
            self.flags.map(Flags::Raw),
            self.misc_as,
            self.is_grounded,
            self.last_ground_id,
            self.jumps_remaining,
            self.l_cancel.and_then(LCancelState::from_repr),
            self.hurtbox_state.and_then(Hurtbox::from_repr),
            self.air_velocity,
            self.knockback,
            self.ground_velocity,
            self.hitlag_remaining,
            self.animation_index,
            self.instance_hit_by,
            self.instance_id,
        )
    }
}

pub fn parse_postframes(
    file_data: Bytes,
    metadata: Arc<Metadata>,
    frames: &[usize],
    ports: [Port; 2],
    ics: [bool; 2],
) -> Result<IntMap<u8, (PostFrames, Option<PostFrames>)>> {
    /* splitting these out saves us a small amount of time in conditional logic, and allows for
    exact iterator chunk sizes. */
    if !ics[0] && !ics[1] {
        unpack_frames(file_data, frames, metadata, ports)
    } else {
        unpack_frames_ics(file_data, frames, metadata, ports, ics)
    }
}

/// Slightly more optimized version of the typical parsing code, due to invariants regarding frame
/// event ordering and counts
pub fn unpack_frames(
    mut stream: Bytes,
    frames: &[usize],
    metadata: Arc<Metadata>,
    ports: [Port; 2],
) -> Result<IntMap<u8, (PostFrames, Option<PostFrames>)>> {
    let offsets_iter = frames.chunks_exact(2).enumerate();

    let mut p_frames: IntMap<u8, (PostFrames, Option<PostFrames>)> = IntMap::default();
    p_frames.insert(ports[0] as u8, (PostFrames::new(metadata.clone()), None));
    p_frames.insert(ports[1] as u8, (PostFrames::new(metadata.clone()), None));

    let file_length = stream.len();
    let duration = metadata.total_frames;
    let version = metadata.version;

    for (_, offsets) in offsets_iter {
        for &offset in offsets {
            // frames should always be in the same order as they appeared in the file, thus we can
            // always just move forward.
            stream.advance(offset - (file_length - stream.len()));

            let frame_number = stream.get_i32();
            let i = (frame_number + 123) as usize;
            if i == duration || i == (duration + 1) {
                #[cfg(debug_assertions)]
                println!("Skipping frame {i} due to game-end rollback");

                continue;
            }
            let port = stream.get_u8();

            stream.advance(1); // skip nana byte

            let (working, _) = p_frames.get_mut(&port).unwrap();

            unsafe {
                // this one won't be unchecked just to make sure i don't accidentally overflow =)
                *working.frame_index.get_mut(i).ok_or(anyhow!("Too many frames. Attempted to access frame at index {i}, max frame number is {duration}"))? = frame_number;
                *working.character.get_unchecked_mut(i) = stream.get_u8();
                *working.action_state.get_unchecked_mut(i) = stream.get_u16();
                *working.position.get_unchecked_mut(i) =
                    Position::new(stream.get_f32(), stream.get_f32());
                *working.orientation.get_unchecked_mut(i) = stream.get_f32();
                *working.percent.get_unchecked_mut(i) = stream.get_f32();
                *working.shield_health.get_unchecked_mut(i) = stream.get_f32();
                *working.last_attack_landed.get_unchecked_mut(i) = stream.get_u8();
                *working.combo_count.get_unchecked_mut(i) = stream.get_u8();
                *working.last_hit_by.get_unchecked_mut(i) = stream.get_u8();
                *working.stocks.get_unchecked_mut(i) = stream.get_u8();

                if !version.at_least(2, 0, 0) {
                    continue;
                }
                *working.state_frame.as_mut().unwrap().get_unchecked_mut(i) = stream.get_f32();
                let flags_1 = stream.get_u8() as u64;
                let flags_2 = stream.get_u8() as u64;
                let flags_3 = stream.get_u8() as u64;
                let flags_4 = stream.get_u8() as u64;
                let flags_5 = stream.get_u8() as u64;
                let flags: u64 =
                    flags_1 | (flags_2 << 8) | (flags_3 << 16) | (flags_4 << 24) | (flags_5 << 32);
                *working.flags.as_mut().unwrap().get_unchecked_mut(i) = flags;
                *working.misc_as.as_mut().unwrap().get_unchecked_mut(i) = stream.get_f32();
                *working.is_grounded.as_mut().unwrap().get_unchecked_mut(i) = stream.get_u8() == 0;
                *working
                    .last_ground_id
                    .as_mut()
                    .unwrap()
                    .get_unchecked_mut(i) = stream.get_u16();
                *working
                    .jumps_remaining
                    .as_mut()
                    .unwrap()
                    .get_unchecked_mut(i) = stream.get_u8();
                *working.l_cancel.as_mut().unwrap().get_unchecked_mut(i) = stream.get_u8();

                if !version.at_least(2, 1, 0) {
                    continue;
                }
                *working.hurtbox_state.as_mut().unwrap().get_unchecked_mut(i) = stream.get_u8();

                if !version.at_least(3, 5, 0) {
                    continue;
                }
                let air_vel_x = stream.get_f32();
                let vel_y = stream.get_f32();
                *working.air_velocity.as_mut().unwrap().get_unchecked_mut(i) =
                    Velocity::new(air_vel_x, vel_y);
                *working.knockback.as_mut().unwrap().get_unchecked_mut(i) =
                    Velocity::new(stream.get_f32(), stream.get_f32());
                *working
                    .ground_velocity
                    .as_mut()
                    .unwrap()
                    .get_unchecked_mut(i) = Velocity::new(stream.get_f32(), vel_y);

                if !version.at_least(3, 8, 0) {
                    continue;
                }
                *working
                    .hitlag_remaining
                    .as_mut()
                    .unwrap()
                    .get_unchecked_mut(i) = stream.get_f32();

                if !version.at_least(3, 11, 0) {
                    continue;
                }
                *working
                    .animation_index
                    .as_mut()
                    .unwrap()
                    .get_unchecked_mut(i) = stream.get_u32();

                if !version.at_least(3, 16, 0) {
                    continue;
                }
                *working
                    .instance_hit_by
                    .as_mut()
                    .unwrap()
                    .get_unchecked_mut(i) = stream.get_u16();
                *working.instance_id.as_mut().unwrap().get_unchecked_mut(i) = stream.get_u16();
            }
        }
    }
    Ok(p_frames)
}

pub fn unpack_frames_ics(
    mut stream: Bytes,
    offsets: &[usize],
    metadata: Arc<Metadata>,
    ports: [Port; 2],
    ics: [bool; 2],
) -> Result<IntMap<u8, (PostFrames, Option<PostFrames>)>> {
    let len = metadata.total_frames;
    let version = metadata.version;

    let mut p_frames: IntMap<u8, (PostFrames, Option<PostFrames>)> = IntMap::default();
    p_frames.insert(
        ports[0] as u8,
        (
            PostFrames::new(metadata.clone()),
            ics[0].then(|| PostFrames::ics(metadata.clone())),
        ),
    );
    p_frames.insert(
        ports[1] as u8,
        (
            PostFrames::new(metadata.clone()),
            ics[1].then(|| PostFrames::ics(metadata.clone())),
        ),
    );

    let file_length = stream.len();

    for &offset in offsets.iter() {
        // frames should always be in the same order as they appeared in the file, thus we can
        // always just move forward.
        stream.advance(offset - (file_length - stream.len()));
        let frame_number = stream.get_i32();
        // since we can't chunk the frames, enumeration won't work. We can still get an
        // always-in-bounds index from the frame number though.
        let i = (frame_number + 123) as usize;
        assert!(
            i < len,
            "Frame index incorrect, index ({i}) is greater than or equal to the max length of the container ({len})."
        );
        let port = stream.get_u8();
        let nana = stream.get_u8() != 0;

        let working = {
            let temp = p_frames.get_mut(&port).unwrap();
            if nana {
                temp.1.as_mut().unwrap()
            } else {
                &mut temp.0
            }
        };

        unsafe {
            *working.frame_index.get_mut(i).ok_or(anyhow!("Too many frames. Attempted to access frame at index {i}, max frame number is {len}"))? = frame_number;
            *working.character.get_unchecked_mut(i) = stream.get_u8();
            *working.action_state.get_unchecked_mut(i) = stream.get_u16();
            *working.position.get_unchecked_mut(i) =
                Position::new(stream.get_f32(), stream.get_f32());
            *working.orientation.get_unchecked_mut(i) = stream.get_f32();
            *working.percent.get_unchecked_mut(i) = stream.get_f32();
            *working.shield_health.get_unchecked_mut(i) = stream.get_f32();
            *working.last_attack_landed.get_unchecked_mut(i) = stream.get_u8();
            *working.combo_count.get_unchecked_mut(i) = stream.get_u8();
            *working.last_hit_by.get_unchecked_mut(i) = stream.get_u8();
            *working.stocks.get_unchecked_mut(i) = stream.get_u8();

            if !version.at_least(2, 0, 0) {
                continue;
            }
            *working.state_frame.as_mut().unwrap().get_unchecked_mut(i) = stream.get_f32();
            let flags_1 = stream.get_u8() as u64;
            let flags_2 = stream.get_u8() as u64;
            let flags_3 = stream.get_u8() as u64;
            let flags_4 = stream.get_u8() as u64;
            let flags_5 = stream.get_u8() as u64;
            let flags: u64 =
                flags_1 & (flags_2 << 8) & (flags_3 << 16) & (flags_4 << 24) & (flags_5 << 32);
            *working.flags.as_mut().unwrap().get_unchecked_mut(i) = flags;
            *working.misc_as.as_mut().unwrap().get_unchecked_mut(i) = stream.get_f32();
            *working.is_grounded.as_mut().unwrap().get_unchecked_mut(i) = stream.get_u8() == 0;
            *working
                .last_ground_id
                .as_mut()
                .unwrap()
                .get_unchecked_mut(i) = stream.get_u16();
            *working
                .jumps_remaining
                .as_mut()
                .unwrap()
                .get_unchecked_mut(i) = stream.get_u8();
            *working.l_cancel.as_mut().unwrap().get_unchecked_mut(i) = stream.get_u8();

            if !version.at_least(2, 1, 0) {
                continue;
            }
            *working.hurtbox_state.as_mut().unwrap().get_unchecked_mut(i) = stream.get_u8();

            if !version.at_least(3, 5, 0) {
                continue;
            }
            let air_vel_x = stream.get_f32();
            let vel_y = stream.get_f32();
            *working.air_velocity.as_mut().unwrap().get_unchecked_mut(i) =
                Velocity::new(air_vel_x, vel_y);
            *working.knockback.as_mut().unwrap().get_unchecked_mut(i) =
                Velocity::new(stream.get_f32(), stream.get_f32());
            *working
                .ground_velocity
                .as_mut()
                .unwrap()
                .get_unchecked_mut(i) = Velocity::new(stream.get_f32(), vel_y);

            if !version.at_least(3, 8, 0) {
                continue;
            }
            *working
                .hitlag_remaining
                .as_mut()
                .unwrap()
                .get_unchecked_mut(i) = stream.get_f32();

            if !version.at_least(3, 11, 0) {
                continue;
            }
            *working
                .animation_index
                .as_mut()
                .unwrap()
                .get_unchecked_mut(i) = stream.get_u32();

            if !version.at_least(3, 16, 0) {
                continue;
            }
            *working
                .instance_hit_by
                .as_mut()
                .unwrap()
                .get_unchecked_mut(i) = stream.get_u16();
            *working.instance_id.as_mut().unwrap().get_unchecked_mut(i) = stream.get_u16();
        }
    }

    Ok(p_frames)
}
