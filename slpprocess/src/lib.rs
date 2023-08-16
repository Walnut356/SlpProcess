#![allow(non_upper_case_globals)]

pub mod enums {
    pub mod character;
    pub mod stage;
}
pub mod events {
    pub mod game_end;
    pub mod game_start;
    pub mod item;
    pub mod post_frame;
    pub mod pre_frame;
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
    panic!()
}

pub mod columns {
    use strum_macros::{Display, EnumString};
    #[derive(Debug, Clone, Copy, Display, EnumString)]
    pub enum Post {
        #[strum(serialize = "frame number")]
        FrameNumber,
        #[strum(serialize = "character")]
        Character,
        #[strum(serialize = "action state")]
        ActionState,
        #[strum(serialize = "position x")]
        PositionX,
        #[strum(serialize = "position y")]
        PositionY,
        #[strum(serialize = "facing")]
        Facing,
        #[strum(serialize = "percent")]
        Percent,
        #[strum(serialize = "shield health")]
        ShieldHealth,
        #[strum(serialize = "last attack landed")]
        LastAttackLanded,
        #[strum(serialize = "combo count")]
        ComboCount,
        #[strum(serialize = "last hit by")]
        LastHitBy,
        #[strum(serialize = "stocks")]
        Stocks,
        #[strum(serialize = "state frame")]
        StateFrame,
        #[strum(serialize = "flags 1")]
        Flags1,
        #[strum(serialize = "flags 2")]
        Flags2,
        #[strum(serialize = "flags 3")]
        Flags3,
        #[strum(serialize = "flags 4")]
        Flags4,
        #[strum(serialize = "flags 5")]
        Flags5,
        #[strum(serialize = "misc as")]
        MiscAS,
        #[strum(serialize = "is grounded")]
        IsGrounded,
        #[strum(serialize = "last ground id")]
        LastGroundID,
        #[strum(serialize = "jumps remaining")]
        JumpsRemaining,
        #[strum(serialize = "l cancel")]
        LCancel,
        #[strum(serialize = "hurtbox state")]
        HurtboxState,
        #[strum(serialize = "self air x")]
        SelfAirX,
        #[strum(serialize = "self y")]
        SelfY,
        #[strum(serialize = "knockback x")]
        KnockbackX,
        #[strum(serialize = "knockback y")]
        KnockbackY,
        #[strum(serialize = "self ground x")]
        SelfGroundX,
        #[strum(serialize = "hitlag remaining")]
        HitlagRemaining,
        #[strum(serialize = "animation index")]
        AnimationIndex,
    }
}

use bytes::{Bytes, Buf};
pub fn temp(eef: Vec<u8>) -> i32 {

    let mut bytes = Bytes::from(eef);

    let temp = bytes.get_i32();

    temp
}