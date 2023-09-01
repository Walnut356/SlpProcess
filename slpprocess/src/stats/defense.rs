use polars::prelude::*;

use crate::player::Frames;

pub fn find_defense(plyr_frames: &Frames, opnt_frames: &Frames) -> DataFrame {
    let plyr_pre = &plyr_frames.pre;
    let plyr_post = &plyr_frames.post;
    let attacks = &opnt_frames.post.last_attack_landed;

    let mut active_event = false;

    for i in 1..plyr_pre.frame_number.len() {
        // check for grab states

        // just_in_hitlag, filtering out shield hits

        if not in hitlag {
            if throw check {}
            else if active_event {}
            else continue
        }
    }

    DataFrame::default()
}
