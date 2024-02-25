use ssbm_utils::{
    enums::ActionState,
    types::{Position, StickPos},
};

use crate::{frames::Frames, utils::Direction};

#[derive(Debug, Clone, Default)]
pub struct WavedashStats {
    pub frame_index: Vec<i32>,
    pub angle: Vec<f32>,
    pub direction: Vec<Direction>,
    pub start_position: Vec<Position>,
    pub waveland: Vec<bool>,
}

pub fn find_wavedashes(frames: &Frames) -> WavedashStats {
    let pre = &frames.pre;
    let post = &frames.post;

    // let button_frames = &pre.engine_buttons;
    let state_frames = &post.action_state;

    let mut wavedashes = WavedashStats::default();

    // start 20 frames "late" to prevent index errors
    for i in 20..frames.len() {
        let state = state_frames[i];
        let prev_state = state_frames[i - 1];

        // saves time and also prevents multiple wavedash events from being created once
        // land_fall_special is entered
        if state != ActionState::LAND_FALL_SPECIAL || prev_state == ActionState::LAND_FALL_SPECIAL {
            continue;
        }

        let mut waveland = true;
        for j in 1..6 {
            if state_frames[i - j] == ActionState::KNEE_BEND {
                waveland = false;
                break;
            }
        }

        let (angle, direction) = degrees_below_horizontal(pre.joystick[i]);
        wavedashes.frame_index.push(i as i32 - 123);
        wavedashes.angle.push(angle);
        wavedashes.direction.push(direction);
        wavedashes.start_position.push(post.position[i]);
        wavedashes.waveland.push(waveland);
    }

    wavedashes.into()
}

fn degrees_below_horizontal(stick: StickPos) -> (f32, Direction) {
    let angle = stick.with_deadzone().as_angle().to_degrees();

    match angle {
        _ if (90.0..270.0).contains(&angle) => (angle - 180.0, Direction::LEFT),
        _ if (270.1..360.0).contains(&angle) => (angle - 270.0, Direction::RIGHT),
        // to avoid negative 0
        _ if angle == 270.0 => (90.0, Direction::DOWN),
        _ if angle == 0.0 => (angle, Direction::RIGHT),
        _ if (0.0..90.0).contains(&angle) => (angle * -1.0, Direction::RIGHT),
        _ => panic!("How did you get here"),
    }
}
