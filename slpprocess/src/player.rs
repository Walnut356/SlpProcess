use polars::prelude::{DataFrame, LazyFrame};

use crate::{enums::character::Character, events::game_start::ControllerFix, Port};

#[derive(Debug, Clone, Default)]
pub struct Player {
    pub character: Character,
    pub costume: u8,
    pub port: Port,
    pub connect_code: Option<Box<str>>,
    pub display_name: Option<Box<str>>,
    pub winner: Option<bool>,
    pub ucf: Option<UCFToggles>,
    pub stats: (),
    pub combos: (),
    pub frames: Frames,
    pub nana_frames: Option<DataFrame>,
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
