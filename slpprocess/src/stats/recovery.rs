use polars::prelude::*;
use ssbm_utils::{enums::{stage::Stage, Attack, Character}, types::Position};

use crate::{player::Frames, utils::Direction};

struct RecoveryRow {
    frame_index: i32,
    stocks: u8,
    percent: f32,
    last_hit_by: Attack,
    side: Direction,
    jumps_remaining: u8,
    up_b: bool,
    hitstun_end: Position,
    dist_from_stage: f32,

    pseudo_jump: Option<bool>,
    randall_involved: Option<bool>,
}

pub fn find_recovery(plyr_frames: &Frames, plyr_char: Character, opnt_frames: &Frames, stage: &Stage) -> DataFrame {

    for i in 1..plyr_frames.len() {

    }

    DataFrame::default()
}

enum RecoveryOption {
    
}