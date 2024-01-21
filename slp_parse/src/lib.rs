#![allow(non_upper_case_globals)]

pub mod events {
    pub mod game_end;
    pub mod game_start;
    pub mod item_frames;
    pub mod post_frame;
    pub mod pre_frame;
}
pub mod columns;
pub mod frames;
pub mod game;
pub mod parse;
pub mod player;
pub mod stats;
pub(crate) mod ubjson;
pub mod utils;

pub use crate::game::{Game, GameStub, GameMetadata};

use crate::stats::combos::Combos;
use serde_json::json;
pub use ssbm_utils::enums::Port;
use stats::Stats;

use rayon::{iter::FilterMap, prelude::*, vec::IntoIter};
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    sync::Arc,
};

/// Accepts a string file path to a single replay, or a directory containing replays. Returns a vector containing the
/// resultant game object(s). Sorted by newest -> oldest
///
/// Replays that error out during parsing for any reason are skipped.
///
/// Directory parsing is multi-threaded by default, can end up IO limited if replays aren't on an SSD
pub fn parse(path: &str, multithreaded: bool) -> Vec<Game> {
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

        let mut result: Vec<Game> = if multithreaded {
            files
                .par_iter()
                .filter_map(|path| match Game::new(path.as_path()) {
                    Ok(game) => Some(game),
                    Err(err) => {
                        #[cfg(debug_assertions)]
                        {
                            dbg!(path);
                            dbg!(err);
                        }
                        None
                    }
                })
                .collect()
        } else {
            files
                .iter()
                .filter_map(|path| match Game::new(path.as_path()) {
                    Ok(game) => Some(game),
                    Err(err) => {
                        #[cfg(debug_assertions)]
                        {
                            dbg!(path);
                            dbg!(err);
                        }
                        None
                    }
                })
                .collect()
        };

        result.sort();
        return result;
    }
    panic!("invalid file path: {f_path:?}")
}

/// Returns a parallel iterator over all .slp files in a directory. Any files that error out during
/// processing are ignored. No ordering is guaranteed
pub fn parse_iter(path: &str) -> FilterMap<IntoIter<PathBuf>, impl Fn(PathBuf) -> Option<Game>> {
    let f_path = Path::new(path);
    if f_path.is_dir() {
        let files = fs::read_dir(f_path)
            .unwrap()
            .filter_map(|file| {
                if let Ok(entry) = file {
                    let path = entry.path();
                    if path.is_file() && path.extension().is_some_and(|x| x == "slp") {
                        Some(path)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let result =
            files
                .into_par_iter()
                .filter_map(move |path| match Game::new(path.as_path()) {
                    Ok(game) => Some(game),
                    Err(err) => {
                        #[cfg(debug_assertions)]
                        {
                            dbg!(path);
                            dbg!(err);
                        }
                        None
                    }
                });

        return result;
    }
    panic!("invalid file path")
}

pub fn parse_stubs(path: &str, multithreaded: bool) -> Vec<GameStub> {
    if path.is_empty() {
        return Vec::new();
    }

    let f_path = Path::new(path);
    if f_path.is_file() {
        return vec![Game::stub(f_path).unwrap()];
    }
    if f_path.is_dir() {
        let eef = fs::read_dir(f_path).unwrap();
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

        let mut result: Vec<GameStub> = if multithreaded {
            files
                .par_iter()
                .filter_map(|path| match Game::stub(path.as_path()) {
                    Ok(game) => Some(game),
                    Err(err) => {
                        #[cfg(debug_assertions)]
                        {
                            dbg!(path);
                            dbg!(err);
                        }
                        None
                    }
                })
                .collect()
        } else {
            files
                .iter()
                .filter_map(|path| match Game::stub(path.as_path()) {
                    Ok(game) => Some(game),
                    Err(err) => {
                        #[cfg(debug_assertions)]
                        {
                            dbg!(path);
                            dbg!(err);
                        }
                        None
                    }
                })
                .collect()
        };

        // sort newest -> oldest by date
        result.sort();

        return result;
    }
    panic!("invalid file path: {f_path:?}")
}

/// Accepts a string file path to a single replay, or a directory containing replays. Returns a HashMap containing the
/// resultant game object(s). The hashmap keys are the match id, so any replays older than 3.14.0 will be filtered out. Iterating over the keys will return them in order
///
/// Replays that error out during parsing for any reason are skipped
// pub fn parse_sets(path: &str, multithreaded: bool) -> BTreeMap<_, Game> {

// }

/// Returns a single stats object containing the stats from all individual games.
pub fn get_stats(games: &[Game], connect_code: &str) -> Stats {
    games
        .iter()
        .filter_map(|game| {
            let player = game.player_by_code(connect_code);
            match player {
                Ok(p) => Some(p.stats.clone()),
                Err(_) => None,
            }
        })
        .collect::<Vec<_>>()
        .into()
}

pub fn get_combos(games: &[Game], connect_code: &str) -> Vec<Arc<Combos>> {
    games
        .iter()
        .filter_map(|game| {
            let player = game.player_by_code(connect_code);
            match player {
                Ok(p) => Some(p.combos.clone()),
                Err(_) => None,
            }
        })
        .collect()
}

pub fn to_dolphin_queue(target_path: PathBuf, combo_list: &[Arc<Combos>]) {
    let mut playback_queue = json!({
        "mode": "queue",
        "replay": "",
        "isRealTimeMode": false,
        "outputOverlayFiles": true,
        "queue": Vec::<serde_json::Value>::new(),
    });

    let result = playback_queue["queue"].as_array_mut().unwrap();

    for combos in combo_list {
        let path = combos.path.to_str().unwrap();
        for combo in combos.iter() {
            result.push(combo.to_queue_obj(path));
        }
    }

    let f = File::create(target_path).unwrap();
    serde_json::to_writer_pretty(f, &playback_queue).unwrap();
    // f.write_all(playback_queue.to_string().as_bytes()).unwrap();
}

pub mod prelude {
    pub use crate::{game::{Game, GameStub}, player::Player, stats::Stats};
    pub use crate::{get_combos, get_stats, parse, to_dolphin_queue};

    pub use ssbm_utils::enums::BitFlags;
}

#[cfg(test)]
mod test {
    use crate::parse;
    use ssbm_utils::enums::{Character, Port};

    #[test]
    fn test_ics() {
        let replay = r"../test_replays/ics_ditto.slp";
        let game = parse(replay, true).pop().unwrap();

        let player = game.player_by_port(Port::P1).unwrap();

        assert_eq!(
            (game.duration.as_millis() as f32 / 1000.0 * 60.0) as u64 + 124,
            16408
        );
        // asserts in parsing code itself should take care of out of bounds access
        // game.total_frames is 16408, this was also manually checked against py-slippi
        assert!(player.frames.pre.frame_index.len() == game.total_frames as usize);
        assert!(
            player.nana_frames.as_ref().unwrap().post.frame_index.len()
                == game.total_frames as usize
        );

        assert_eq!(player.character, Character::IceClimbers);
    }
}
