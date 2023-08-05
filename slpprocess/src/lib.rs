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

pub fn parse(path: &str) -> Vec<Game> {
    let f_path = Path::new(path);
    if f_path.is_file() {
        return vec![Game::new(f_path)];
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

        // let mut result = Vec::new();
        // let mut counter = 1;

        // for file in files {
        //     println!("{}: {:?}", counter, file);
        //     result.push(Game::new(file.as_path()));
        //     counter += 1;
        // }
        let result: Vec<Game> = files
            .par_iter()
            .map(|path| Game::new(path.as_path()))
            .collect();

        return result;
    }
    panic!()
}
