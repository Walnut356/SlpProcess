use polars::prelude::DataFrame;

use crate::{
    enums::character::Character,
    events::{game_start::ControllerFix, post_frame::PostFrames, pre_frame::PreFrames},
    Port,
};

#[derive(Debug, Default)]
pub struct Player {
    pub character: Character,
    pub costume: u8,
    pub port: Port,
    pub connect_code: Option<String>,
    pub display_name: Option<String>,
    pub is_winner: Option<bool>,
    pub ucf: Option<UCFToggles>,
    pub stats: Stats,
    pub combos: (),
    pub frames: Frames,
    pub nana_frames: Option<Frames>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct UCFToggles {
    pub dashback: ControllerFix,
    pub shield_drop: ControllerFix,
}

#[derive(Debug, Default)]
pub struct Frames {
    pub pre: PreFrames,
    pub post: PostFrames,
}

impl Frames {
    pub fn len(&self) -> usize {
        self.pre.frame_number.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() > 0
    }
}

#[derive(Debug, Default, Clone)]
pub struct Stats {
    pub l_cancel: DataFrame,
    pub actions: DataFrame,
    pub items: DataFrame,
    pub defense: DataFrame,
}
