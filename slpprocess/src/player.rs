use polars::prelude::DataFrame;

use crate::{enums::character::Character, events::game_start::ControllerFix, Port};

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Default, Clone)]
pub struct Frames {
    pub pre: DataFrame,
    pub post: DataFrame,
}

impl Frames {
    pub fn len(&self) -> usize {
        self.pre.shape().0
    }

    pub fn is_empty(&self) -> bool {
        self.len() > 0
    }
}

#[derive(Debug, Default, Clone)]
pub struct Stats {
    pub l_cancel: DataFrame,
    pub actions: DataFrame,
}