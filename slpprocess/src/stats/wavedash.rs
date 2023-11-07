use polars::prelude::*;
use ssbm_utils::{enums::{Orientation, ActionState, EngineInput as Input, BitFlags}, types::{Position, StickPos}, checks::just_pressed};

use crate::player::Frames;

#[derive(Debug, Clone, Default)]
pub struct Wavedashes {
    pub frame_index: Vec<i32>,
    pub angle: Vec<f32>,
    pub direction: Vec<Orientation>,
    pub start_position: Vec<Position>,
}

impl From<Wavedashes> for DataFrame {
    fn from(val: Wavedashes) -> Self {
        use crate::columns::WavedashStats as col;

        let vec_series = vec![
            Series::new(col::FrameIndex.into(), val.frame_index),
            Series::new(col::Angle.into(), val.angle),
            Series::new(col::Direction.into(), val.direction.into_iter().map(|x| x as i8).collect::<Vec<_>>()),
            StructChunked::new(
                col::StartPosition.into(),
                &[
                    Series::new("x", val.start_position.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.start_position.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),
        ];

        DataFrame::new(vec_series).unwrap()
    }
}

pub fn find_wavedashes(frames: &Frames) -> DataFrame {
    let pre = &frames.pre;
    let post = &frames.post;

    let button_frames = &pre.engine_buttons;
    let state_frames = &post.action_state;

    let mut wavedashes = Wavedashes::default();

    let mut most_recent_l = 0;

    // start 20 frames "late" to prevent index errors

    for i in 20..button_frames.len() {
        let state = state_frames[i];
        let prev_state = state_frames[i - 1];

        if just_pressed(Input::L | Input::R, button_frames[i], button_frames[i - 1]) {
            most_recent_l = i;
        }
        // saves time and also prevents multiple wavedash events from being created once
        // land_fall_special is entered
        if state != ActionState::LAND_FALL_SPECIAL || prev_state == ActionState::LAND_FALL_SPECIAL {
            continue;
        }

        let (angle, direction) = degrees_below_horizontal(pre.joystick[i]);
        wavedashes.frame_index.push(i as i32 - 123);
        wavedashes.angle.push(angle);
        wavedashes.direction.push(direction);
        wavedashes.start_position.push(post.position[i]);

    }

    DataFrame::default()
}

fn degrees_below_horizontal(stick: StickPos) -> (f32, Orientation) {
    let angle = stick.with_deadzone().as_angle();

    match angle {
        _ if (90.0..270.0).contains(&angle) => (angle - 180.0, Orientation::LEFT),
        _ if (270.0..360.0).contains(&angle) => (angle - 270.0, Orientation::RIGHT),
        // to avoid negative 0
        _ if angle == 0.0 => (angle, Orientation::RIGHT),
        _ if (0.0..90.0).contains(&angle) => (angle * -1.0, Orientation::RIGHT),
        _ => panic!("How did you get here")
    }
}