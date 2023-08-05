use bytes::{Buf, Bytes};
use nohash_hasher::IntMap;
use polars::prelude::*;
use std::{iter::zip, sync::Mutex};
use rayon::prelude::*;
use crate::{player::Player, Port};


#[derive(Debug)]
pub struct PreFrames {
    frame_number: Vec<i32>,
    random_seed: Vec<u32>,
    action_state: Vec<u16>,
    position_x: Vec<f32>,
    position_y: Vec<f32>,
    facing: Vec<f32>,
    joystick_x: Vec<f32>,
    joystick_y: Vec<f32>,
    cstick_x: Vec<f32>,
    cstick_y: Vec<f32>,
    trigger: Vec<f32>,
    logical_buttons: Vec<u32>,
    physical_buttons: Vec<u16>,
    physical_l: Vec<f32>,
    physical_r: Vec<f32>,
    percent: Vec<Option<f32>>,
}

impl PreFrames {
    fn new(len: usize) -> Self {
        PreFrames {
            frame_number: Vec::with_capacity(len),
            random_seed: Vec::with_capacity(len),
            action_state: Vec::with_capacity(len),
            position_x: Vec::with_capacity(len),
            position_y: Vec::with_capacity(len),
            facing: Vec::with_capacity(len),
            joystick_x: Vec::with_capacity(len),
            joystick_y: Vec::with_capacity(len),
            cstick_x: Vec::with_capacity(len),
            cstick_y: Vec::with_capacity(len),
            trigger: Vec::with_capacity(len),
            logical_buttons: Vec::with_capacity(len),
            physical_buttons: Vec::with_capacity(len),
            physical_l: Vec::with_capacity(len),
            physical_r: Vec::with_capacity(len),
            percent: Vec::with_capacity(len),
        }
    }
}

// preframes objects are purely a temporary container to make the code clearer, so i impl Into rather than From (and
// implicitly Into) because I intentionally want to disallow translation back.
#[allow(clippy::from_over_into)]
impl Into<DataFrame> for PreFrames {
    fn into(mut self) -> DataFrame {
        let len = self.frame_number.len();

        // handle potentially optional values
        if self.percent.len() != len {
            self.percent.resize(len, None)
        }

        let vec_series = vec![
            Series::new("frame number", self.frame_number),
            Series::new("random seed", self.random_seed),
            Series::new("action state", self.action_state),
            Series::new("position x", self.position_x),
            Series::new("position y", self.position_y),
            Series::new("facing", self.facing),
            Series::new("joystick x", self.joystick_x),
            Series::new("joystick y", self.joystick_y),
            Series::new("cstick x", self.cstick_x),
            Series::new("cstick y", self.cstick_y),
            Series::new("trigger", self.trigger),
            Series::new("logical buttons", self.logical_buttons),
            Series::new("physical buttons", self.physical_buttons),
            Series::new("physical l", self.physical_l),
            Series::new("physical r", self.physical_r),
            Series::new("percent", self.percent),
        ];

        DataFrame::new(vec_series).unwrap()
    }
}

pub fn parse_preframes(frames: &mut [Bytes], ports: [Port; 2]) -> IntMap<u8, DataFrame> {
    let len = frames.len() / 2;

    let mut p_frames: IntMap<u8, PreFrames> = IntMap::default();

    p_frames.insert(ports[0].into(), PreFrames::new(len));
    p_frames.insert(ports[1].into(), PreFrames::new(len));

    for frame in frames {
        let frame_number = frame.get_i32();
        let port = frame.get_u8();
        let is_nana = frame.get_u8() != 0; // TODO add ic's specific logic using metadata

        if is_nana {
            continue;
        }

        let working = p_frames.get_mut(&port).unwrap();

        working.frame_number.push(frame_number);
        working.random_seed.push(frame.get_u32());
        working.action_state.push(frame.get_u16());
        working.position_x.push(frame.get_f32());
        working.position_y.push(frame.get_f32());
        working.facing.push(frame.get_f32());
        working.joystick_x.push(frame.get_f32());
        working.joystick_y.push(frame.get_f32());
        working.cstick_x.push(frame.get_f32());
        working.cstick_y.push(frame.get_f32());
        working.trigger.push(frame.get_f32());
        working.logical_buttons.push(frame.get_u32());
        working.physical_buttons.push(frame.get_u16());
        working.physical_l.push(frame.get_f32());
        working.physical_r.push(frame.get_f32());
        if !frame.has_remaining() {
            // version < 1.2.0
            continue;
        }
        frame.advance(1);
        if !frame.has_remaining() {
            // version < 1.4.0
            continue;
        }
        working.percent.push(Some(frame.get_f32_ne()));
    }

    let mut result = IntMap::default();

    for (port, frames) in p_frames {
        result.insert(port, frames.into());
    }

    result
}
