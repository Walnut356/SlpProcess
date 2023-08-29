use circular_queue::CircularQueue;
use polars::prelude::*;
use std::iter::zip;

use crate::{columns::Pre, enums::buttons::*, player::Frames};

const BUTTON_MASK: u32 = 0xf0000fff;
const JOYSTICK_MASK: u32 = 0xf0000;
const CSTICK_MASK: u32 = 0xf00000;
const ANYTRIGGER_MASK: u32 = 0x8000_0000;
const DIGITAL_TRIGGER_MASK: u32 = 0x60;

// #[derive(Debug, Default)]
// struct ButtonCounts {
//     a: u32,
//     b: u32,
//     x: u32,
//     y: u32,
//     l: u32,
//     r: u32,
//     z: u32,
//     start: u32,
//     j_up: u32,
//     j_down: u32,
//     j_left: u32,
//     j_right: u32,
//     c_up: u32,
//     c_down: u32,
//     c_left: u32,
//     c_right: u32,
//     d_up: u32,
//     d_down: u32,
//     d_left: u32,
//     d_right: u32,
// }

pub fn find_actions(frames: &Frames, duration: u64) -> DataFrame {
    let en_btn = frames
        .pre
        .column(Pre::EngineButtons.into())
        .unwrap()
        .u32()
        .unwrap()
        .to_vec_null_aware()
        .expect_left("Possible malformed replay, pre-frame buttons contains null values");

    let ctrl_btn = frames
        .pre
        .column(Pre::ControllerButtons.into())
        .unwrap()
        .u16()
        .unwrap()
        .to_vec_null_aware()
        .expect_left("Possible malformed replay, pre-frame buttons contains null values");

    let mut digital_counts = 0;
    let mut stick_counts = 0;
    let mut cstick_counts = 0;
    let mut trigger_counts = 0;

    assert_eq!(en_btn.len(), ctrl_btn.len());

    for i in 1..en_btn.len() {
        let ctrl_curr = ctrl_btn[i];
        let ctrl_prev = ctrl_btn[i - 1];

        // this gets the bits that changed for the buttons
        let ctrl_changed = !ctrl_prev & ctrl_curr;

        digital_counts += ctrl_changed.count_ones();

        let en_curr = en_btn[i];
        let en_prev = en_btn[i - 1];
        let en_changed = !en_prev & en_curr;

        let curr_stick = en_curr & JOYSTICK_MASK;
        let prev_stick = en_prev & JOYSTICK_MASK;

        if curr_stick != 0 {
            let changed = prev_stick ^ curr_stick;
            stick_counts += changed.count_ones()
        }

        let curr_cstick = en_curr & CSTICK_MASK;
        let prev_cstick = en_prev & CSTICK_MASK;

        if curr_cstick != 0 {
            let changed = prev_cstick ^ curr_cstick;
            cstick_counts += changed.count_ones()
        }

        // special handling to detect analog trigger
        if en_changed & ANYTRIGGER_MASK != 0 // if anytrigger was just pressed
            && en_changed & DIGITAL_TRIGGER_MASK == 0 // and we didn't just digital press
            && ctrl_changed & u16::from(ControllerInput::Z) == 0 // and we didn't just start pressing Z
        {
            trigger_counts += 1;
        }
    }

    df!("Digital" => [digital_counts],
    "Joystick" => [stick_counts],
    "Cstick" => [cstick_counts],
    "AnalogTrigger" => [trigger_counts],
"APM" => [(digital_counts + stick_counts + cstick_counts + trigger_counts) as f32 / (duration as f32 / 60.0 / 60.0)]).unwrap()
}
