#![allow(clippy::uninit_vec)]

use crate::Port;
use bytes::{Buf, Bytes};
use nohash_hasher::IntMap;
use polars::prelude::*;

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
            frame_number: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            random_seed: unsafe {
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
            joystick_x: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            joystick_y: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            cstick_x: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            cstick_y: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            trigger: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            logical_buttons: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            physical_buttons: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            physical_l: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            physical_r: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp
            },
            percent: unsafe {
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
        self.random_seed
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
        self.joystick_x
            .iter_mut()
            .for_each(|x| *x = f32::from_be_bytes(x.to_ne_bytes()));
        self.joystick_y
            .iter_mut()
            .for_each(|x| *x = f32::from_be_bytes(x.to_ne_bytes()));
        self.cstick_x
            .iter_mut()
            .for_each(|x| *x = f32::from_be_bytes(x.to_ne_bytes()));
        self.cstick_y
            .iter_mut()
            .for_each(|x| *x = f32::from_be_bytes(x.to_ne_bytes()));
        self.trigger
            .iter_mut()
            .for_each(|x| *x = f32::from_be_bytes(x.to_ne_bytes()));
        self.logical_buttons
            .iter_mut()
            .for_each(|x| *x = x.swap_bytes());
        self.physical_buttons
            .iter_mut()
            .for_each(|x| *x = x.swap_bytes());
        self.physical_l
            .iter_mut()
            .for_each(|x| *x = f32::from_be_bytes(x.to_ne_bytes()));
        self.physical_r
            .iter_mut()
            .for_each(|x| *x = f32::from_be_bytes(x.to_ne_bytes()));
        if self.percent[0].is_some() {
            self.percent
                .iter_mut()
                .for_each(|x| *x = x.map(|x| f32::from_be_bytes(x.to_ne_bytes())));
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

pub fn parse_preframes(frames: &mut [Bytes], ports: [Port; 2], ics: [bool; 2]) -> [DataFrame; 2] {
    let len = frames.len() / 2;

    let mut p_frames: [PreFrames; 2] = [PreFrames::new(len), PreFrames::new(len)];

    if !ics[0] && !ics[1] {
        no_ics(frames, &mut p_frames);
    } else if ics[0] ^ ics[1] {
        one_ics(frames, &mut p_frames);
    } else {
        two_ics(frames, &mut p_frames);
    }

    for mut frames in p_frames.iter_mut() {
        frames.byte_swap();
    }

    p_frames
        .into_iter()
        .map(|x| std::convert::Into::<DataFrame>::into(x))
        .collect::<Vec<DataFrame>>()
        .try_into()
        .unwrap()
}

fn no_ics(frames: &mut [Bytes], p_frames: &mut [PreFrames; 2]) {
    for (i, frame) in frames.chunks_exact_mut(2).enumerate() {
        let frame_number = frame[0].get_i32_ne();
        let port = frame[0].get_u8();
        let is_nana = frame[0].get_u8() != 0;

        let working = p_frames.get_mut(0).unwrap();

        working.frame_number[i] = frame_number;
        working.random_seed[i] = frame[0].get_u32_ne();
        working.action_state[i] = frame[0].get_u16_ne();
        working.position_x[i] = frame[0].get_f32_ne();
        working.position_y[i] = frame[0].get_f32_ne();
        working.facing[i] = frame[0].get_f32_ne();
        working.joystick_x[i] = frame[0].get_f32_ne();
        working.joystick_y[i] = frame[0].get_f32_ne();
        working.cstick_x[i] = frame[0].get_f32_ne();
        working.cstick_y[i] = frame[0].get_f32_ne();
        working.trigger[i] = frame[0].get_f32_ne();
        working.logical_buttons[i] = frame[0].get_u32_ne();
        working.physical_buttons[i] = frame[0].get_u16_ne();
        working.physical_l[i] = frame[0].get_f32_ne();
        working.physical_r[i] = frame[0].get_f32_ne();
        if !frame[0].has_remaining() {
            // version < 1.2.0
            continue;
        }
        frame[0].advance(1);
        if !frame[0].has_remaining() {
            // version < 1.4.0
            continue;
        }
        working.percent[i] = Some(frame[0].get_f32_ne());

        // -------------------------------------- player 2 -------------------------------------- //

        let frame_number = frame[1].get_i32_ne();
        let port = frame[1].get_u8();
        let is_nana = frame[1].get_u8() != 0;

        let working = p_frames.get_mut(1).unwrap();

        working.frame_number[i] = frame_number;
        working.random_seed[i] = frame[1].get_u32_ne();
        working.action_state[i] = frame[1].get_u16_ne();
        working.position_x[i] = frame[1].get_f32_ne();
        working.position_y[i] = frame[1].get_f32_ne();
        working.facing[i] = frame[1].get_f32_ne();
        working.joystick_x[i] = frame[1].get_f32_ne();
        working.joystick_y[i] = frame[1].get_f32_ne();
        working.cstick_x[i] = frame[1].get_f32_ne();
        working.cstick_y[i] = frame[1].get_f32_ne();
        working.trigger[i] = frame[1].get_f32_ne();
        working.logical_buttons[i] = frame[1].get_u32_ne();
        working.physical_buttons[i] = frame[1].get_u16_ne();
        working.physical_l[i] = frame[1].get_f32_ne();
        working.physical_r[i] = frame[1].get_f32_ne();
        if !frame[1].has_remaining() {
            // version < 1.2.0
            continue;
        }
        frame[1].advance(1);
        if !frame[1].has_remaining() {
            // version < 1.4.0
            continue;
        }
        working.percent[i] = Some(frame[1].get_f32_ne());
    }
}

fn one_ics(frames: &mut [Bytes], p_frames: &mut [PreFrames; 2]) {}

fn two_ics(frames: &mut [Bytes], p_frames: &mut [PreFrames; 2]) {}
