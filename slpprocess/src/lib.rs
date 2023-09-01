#![allow(non_upper_case_globals)]

pub mod enums {
    pub mod attack;
    mod bitflag_impl;
    pub mod buttons;
    pub mod character;
    pub mod general;
    pub mod stage;
    pub mod state;
    pub mod item;
}
pub mod events {
    pub mod game_end;
    pub mod game_start;
    pub mod item_frames;
    pub mod post_frame;
    pub mod pre_frame;
}
pub mod stats {
    pub mod helpers;
    pub mod inputs;
    pub mod items;
    pub mod lcancel;
    pub mod defense;
}
pub mod columns;
pub mod game;
pub mod parse;
pub mod player;
pub(crate) mod ubjson;
pub mod utils;

pub use crate::game::Game;
use rayon::prelude::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(
    Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, TryFromPrimitive, IntoPrimitive, Default,
)]
#[repr(u8)]
pub enum Port {
    #[default]
    P1,
    P2,
    P3,
    P4,
}

impl TryFrom<i8> for Port {
    fn try_from(val: i8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Port::P1),
            1 => Ok(Port::P2),
            2 => Ok(Port::P3),
            3 => Ok(Port::P4),
            _ => Err("Port must be a number 0-3 inclusive"),
        }
    }

    type Error = &'static str;
}

/// Accepts a string file path to a single replay, or a directory containing replays. Returns a vector containing the
/// resultant game object(s).
///
/// Replays that error out during parsing for any reason are skipped.
///
/// Directory parsing is multi-threaded by default, can end up IO limited if replays aren't on an SSD
pub fn parse(path: &str) -> Vec<Game> {
    let f_path = Path::new(path);
    if f_path.is_file() {
        return vec![Game::new(f_path).unwrap()];
    }
    if f_path.is_dir() {
        let files: Vec<PathBuf> = fs::read_dir(f_path)
            .unwrap()
            .filter_map(|file| {
                if let Ok(entry) = file {
                    let path = entry.path();
                    if path.is_file() && path.extension().unwrap() == "slp" {
                        Some(path)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        let result: Vec<Game> = files
            .par_iter()
            .filter_map(|path| Game::new(path.as_path()).ok())
            .collect();

        return result;
    }
    panic!("invalid file path")
}

#[cfg(test)]
mod test {
    use crate::{parse, Port};

    #[test]
    fn test_ics() {
        let replay = r"../../py-slippi-stats/test/Bench Replays/ics_ditto.slp";
        let game = parse(replay).pop().unwrap();

        let player = game.player_by_port(Port::P1).unwrap();

        assert_eq!(
            (game.duration.as_millis() as f32 / 1000.0 * 60.0) as u64 + 124,
            16408
        );
        // asserts in parsing code itself should take care of out of bounds access
        // game.total_frames is 16408, this was also manually checked against py-slippi
        assert!(player.frames.pre.frame_number.len() == game.total_frames as usize);
        assert!(
            player.nana_frames.as_ref().unwrap().post.frame_number.len()
                == game.total_frames as usize
        );
    }
}
