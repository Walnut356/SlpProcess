use polars::prelude::*;
use ssbm_utils::enums::{
    buttons::{ANYTRIGGER_MASK, CSTICK_MASK, DIGITAL_TRIGGER_MASK, JOYSTICK_MASK},
    BitFlags, ControllerInput,
};

use crate::{columns::InputStats, frames::Frames};

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

pub fn find_inputs(frames: &Frames, duration: usize) -> DataFrame {
    let en_btn = &frames.pre.engine_buttons;
    let ctrl_btn = &frames.pre.controller_buttons;

    let mut digital_counts = 0;
    let mut stick_counts = 0;
    let mut cstick_counts = 0;
    let mut trigger_counts = 0;
    let mut x_count: f32 = 0.0;
    let mut y_count: f32 = 0.0;
    let mut l_count: f32 = 0.0;
    let mut r_count: f32 = 0.0;

    assert_eq!(en_btn.len(), ctrl_btn.len());

    for i in 1..en_btn.len() {
        let ctrl_curr = ctrl_btn[i];
        let ctrl_prev = ctrl_btn[i - 1];

        // this gets the bits that changed for the buttons
        let ctrl_changed = !ctrl_prev & ctrl_curr;

        digital_counts += ctrl_changed.count_ones();

        if ControllerInput::L.intersects(ctrl_changed) {
            l_count += 1.0;
        }

        if ControllerInput::R.intersects(ctrl_changed) {
            r_count += 1.0;
        }

        if ControllerInput::X.intersects(ctrl_changed) {
            x_count += 1.0;
        }

        if ControllerInput::Y.intersects(ctrl_changed) {
            y_count += 1.0;
        }

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

        // TODO fix this
        // special handling to detect analog trigger
        if en_changed & ANYTRIGGER_MASK != 0 // if anytrigger was just pressed
            && en_changed & DIGITAL_TRIGGER_MASK == 0 // and we didn't just digital press
            && !ControllerInput::Z.intersects(ctrl_changed)
        // and we didn't just start pressing Z
        {
            trigger_counts += 1;
        }
    }

    let trigger_pref: &str = {
        if l_count == 0.0 && r_count == 0.0 {
            "UNKNOWN"
        } else if l_count == 0.0 && r_count != 0.0 {
            "R"
        } else if r_count == 0.0 && l_count != 0.0 {
            "L"
        } else {
            let l_ratio = l_count / (l_count + r_count);
            let r_ratio = r_count / (l_count + r_count);
            match l_ratio - r_ratio {
                x if x >= 0.15 => "L",
                x if x <= -0.15 => "R",
                _ => "BOTH",
            }
        }
    };
    let jump_pref = {
        if x_count == 0.0 && y_count == 0.0 {
            "UNKNOWN"
        } else if x_count == 0.0 && y_count != 0.0 {
            "Y"
        } else if r_count == 0.0 && x_count != 0.0 {
            "X"
        } else {
            let x_ratio = x_count / (x_count + y_count);
            let y_ratio = y_count / (x_count + y_count);
            match x_ratio - y_ratio {
                x if x >= 0.15 => "X",
                x if x <= -0.15 => "Y",
                _ => "BOTH",
            }
        }
    };
    use InputStats::*;
    df!(Digital.into() => [digital_counts],
    Joystick.into() => [stick_counts],
    Cstick.into() => [cstick_counts],
    AnalogTrigger.into() => [trigger_counts],
    APM.into() => [(digital_counts + stick_counts + cstick_counts + trigger_counts) as f32 / (duration as f32 / 60.0 / 60.0)],
    TriggerPref.into() => [trigger_pref],
    JumpPref.into() => [jump_pref],).unwrap()
}
