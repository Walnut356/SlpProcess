#![allow(non_upper_case_globals)]

pub mod enums {
    pub mod character;
    pub mod stage;
}
pub mod events {
    pub mod game_end;
    pub mod game_start;
    pub mod post_frame;
    pub mod pre_frame;
    pub mod item;
}
pub mod parse;
pub mod player;
pub(crate) mod ubjson;
pub mod utils;

pub use parse::Game;
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
    panic!()
}
