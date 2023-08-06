#![allow(non_upper_case_globals)]

use bytes::{Buf, Bytes};
use encoding_rs::SHIFT_JIS;
use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};
use polars::prelude::*;
use std::time::Duration;

use crate::{
    enums::{character::Character, stage::Stage},
    player::{Frames, Player, UCFToggles},
    Port,
};

#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Mode {
    VS = 2,
    Online = 8,
    #[default]
    Unknown = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum MatchType {
    // ascii character values for u, r, d
    Unranked = 117,
    Ranked = 114,
    Direct = 100,
    #[default]
    Unknown = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum PlayerType {
    Human = 0,
    CPU = 1,
    Demo = 2,
    Empty = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive, Default)]
#[repr(u8)]
pub enum ControllerFix {
    Off = 0,
    #[default] // this has more or less been true since like 2018
    UCF = 1,
    Dween = 2,
}

#[derive(Debug, Clone)]
pub struct GameStart {
    pub random_seed: u32,
    pub is_teams: bool,
    pub stage: Stage,
    pub timer: Duration,
    pub damage_ratio: f32,
    pub is_pal: Option<bool>,
    pub is_frozen_stadium: Option<bool>,
    pub is_netplay: Option<bool>,
    pub match_id: Option<Box<str>>,
    pub match_type: Option<MatchType>,
    pub game_number: Option<u32>,
    pub tiebreak_number: Option<u32>,
}

impl GameStart {
    // the awkward return type here is because this will only ever be constructed internally, and because it will help
    // a LOT down the line to have the players contained in the top level Game object rather than the GameStart event.
    pub fn parse(mut raw: Bytes) -> (Self, Version, [Player; 2]) {
        let version = Version::new([raw.get_u8(), raw.get_u8(), raw.get_u8(), raw.get_u8()]);
        raw.advance(8); // skip past game bitfields 1-4 and bomb rain
        let is_teams = raw.get_u8() != 0;
        raw.advance(5); // skip item spawn rate and self destruct score value
        let stage = Stage::try_from(raw.get_u16()).unwrap();
        // timer value is given in seconds, can only be changed by full-minute increments in-game
        let timer_length = Duration::from_secs(raw.get_u32() as u64);
        raw.advance(28); // skip past item spawn bitfields
        let damage_ratio = raw.get_f32();
        raw.advance(44);
        let mut temp_players = Vec::new();
        for _ in 0..4 {
            let character = Character::try_from_css(raw.get_u8()).unwrap();
            let p_type = PlayerType::try_from(raw.get_u8()).unwrap();
            raw.advance(1);
            let costume = raw.get_u8();

            temp_players.push((character, p_type, costume));

            raw.advance(32) // skip to next port
        }

        raw.advance(72); // skip past "players" 5 and 6

        let random_seed = raw.get_u32();

        // Null out potentially uninitialized values:
        let mut temp_ucf: [Option<UCFToggles>; 4] = [None; 4];
        let mut is_pal = None;
        let mut is_frozen_stadium = None;
        let mut is_netplay = None;
        let mut display_names = [None, None, None, None];
        let mut connect_codes = [None, None, None, None];
        let mut match_id = None;
        let mut match_type = None;
        let mut game_number = None;
        let mut tiebreak_number = None;

        if !raw.has_remaining() {
            // version < 1.0.0
            let mut players: [Player; 2] = [Player::default(), Player::default()];
            let mut count = 0;

            for i in 0..4 {
                if temp_players[i].1 == PlayerType::Human {
                    players[count] = Player {
                        character: temp_players[i].0,
                        port: Port::try_from(i as u8).unwrap(),
                        connect_code: connect_codes[i].clone(),
                        display_name: display_names[i].clone(),
                        winner: None,
                        stats: (),
                        combos: (),
                        frames: Frames::default(),
                        nana_frames: None,
                        costume: 0,
                        ucf: temp_ucf[i],
                    };
                    count += 1;
                }
            }
            return (
                GameStart {
                    random_seed,
                    is_teams,
                    stage,
                    timer: timer_length,
                    is_pal,
                    is_frozen_stadium,
                    is_netplay,
                    match_id,
                    match_type,
                    game_number,
                    tiebreak_number,
                    damage_ratio,
                },
                version,
                players,
            );
        }

        for val in temp_ucf.iter_mut() {
            let dashback = ControllerFix::try_from(raw.get_u32() as u8).unwrap();
            let shield_drop = ControllerFix::try_from(raw.get_u32() as u8).unwrap();
            *val = Some(UCFToggles {
                dashback,
                shield_drop,
            });
        }

        if !raw.has_remaining() {
            // version < 1.3.0
            let mut players: [Player; 2] = [Player::default(), Player::default()];
            let mut count = 0;

            for i in 0..4 {
                if temp_players[i].1 == PlayerType::Human {
                    players[count] = Player {
                        character: temp_players[i].0,
                        port: Port::try_from(i as u8).unwrap(),
                        connect_code: connect_codes[i].clone(),
                        display_name: display_names[i].clone(),
                        winner: None,
                        stats: (),
                        combos: (),
                        frames: Frames::default(),
                        nana_frames: None,
                        costume: 0,
                        ucf: temp_ucf[i],
                    };
                    count += 1;
                }
            }
            return (
                GameStart {
                    random_seed,
                    is_teams,
                    stage,
                    timer: timer_length,
                    is_pal,
                    is_frozen_stadium,
                    is_netplay,
                    match_id,
                    match_type,
                    game_number,
                    tiebreak_number,
                    damage_ratio,
                },
                version,
                players,
            );
        }

        raw.advance(64); // skip past in-game tags

        if !raw.has_remaining() {
            // version < 1.5.0
            let mut players: [Player; 2] = [Player::default(), Player::default()];
            let mut count = 0;

            for i in 0..4 {
                if temp_players[i].1 == PlayerType::Human {
                    players[count] = Player {
                        character: temp_players[i].0,
                        port: Port::try_from(i as u8).unwrap(),
                        connect_code: connect_codes[i].clone(),
                        display_name: display_names[i].clone(),
                        winner: None,
                        stats: (),
                        combos: (),
                        frames: Frames::default(),
                        nana_frames: None,
                        costume: 0,
                        ucf: temp_ucf[i],
                    };
                    count += 1;
                }
            }
            return (
                GameStart {
                    random_seed,
                    is_teams,
                    stage,
                    timer: timer_length,
                    is_pal,
                    is_frozen_stadium,
                    is_netplay,
                    match_id,
                    match_type,
                    game_number,
                    tiebreak_number,
                    damage_ratio,
                },
                version,
                players,
            );
        }

        is_pal = Some(raw.get_u8() != 0);

        if !raw.has_remaining() {
            // version < 2.0.0
            let mut players: [Player; 2] = [Player::default(), Player::default()];
            let mut count = 0;

            for i in 0..4 {
                if temp_players[i].1 == PlayerType::Human {
                    players[count] = Player {
                        character: temp_players[i].0,
                        port: Port::try_from(i as u8).unwrap(),
                        connect_code: connect_codes[i].clone(),
                        display_name: display_names[i].clone(),
                        winner: None,
                        stats: (),
                        combos: (),
                        frames: Frames::default(),
                        nana_frames: None,
                        costume: 0,
                        ucf: temp_ucf[i],
                    };
                    count += 1;
                }
            }
            return (
                GameStart {
                    random_seed,
                    is_teams,
                    stage,
                    timer: timer_length,
                    is_pal,
                    is_frozen_stadium,
                    is_netplay,
                    match_id,
                    match_type,
                    game_number,
                    tiebreak_number,
                    damage_ratio,
                },
                version,
                players,
            );
        }

        is_frozen_stadium = Some(raw.get_u8() != 0);

        if !raw.has_remaining() {
            // version < 3.7.0
            let mut players: [Player; 2] = [Player::default(), Player::default()];
            let mut count = 0;

            for i in 0..4 {
                if temp_players[i].1 == PlayerType::Human {
                    players[count] = Player {
                        character: temp_players[i].0,
                        port: Port::try_from(i as u8).unwrap(),
                        connect_code: connect_codes[i].clone(),
                        display_name: display_names[i].clone(),
                        winner: None,
                        stats: (),
                        combos: (),
                        frames: Frames::default(),
                        nana_frames: None,
                        costume: 0,
                        ucf: temp_ucf[i],
                    };
                    count += 1;
                }
            }
            return (
                GameStart {
                    random_seed,
                    is_teams,
                    stage,
                    timer: timer_length,
                    is_pal,
                    is_frozen_stadium,
                    is_netplay,
                    match_id,
                    match_type,
                    game_number,
                    tiebreak_number,
                    damage_ratio,
                },
                version,
                players,
            );
        }

        raw.advance(1); // skip minor scene
        is_netplay = Some(raw.get_u8() == 8);

        if !raw.has_remaining() {
            // version < 3.9.0
            let mut players: [Player; 2] = [Player::default(), Player::default()];
            let mut count = 0;

            for i in 0..4 {
                if temp_players[i].1 == PlayerType::Human {
                    players[count] = Player {
                        character: temp_players[i].0,
                        port: Port::try_from(i as u8).unwrap(),
                        connect_code: connect_codes[i].clone(),
                        display_name: display_names[i].clone(),
                        winner: None,
                        stats: (),
                        combos: (),
                        frames: Frames::default(),
                        nana_frames: None,
                        costume: 0,
                        ucf: temp_ucf[i],
                    };
                    count += 1;
                }
            }
            return (
                GameStart {
                    random_seed,
                    is_teams,
                    stage,
                    timer: timer_length,
                    is_pal,
                    is_frozen_stadium,
                    is_netplay,
                    match_id,
                    match_type,
                    game_number,
                    tiebreak_number,
                    damage_ratio,
                },
                version,
                players,
            );
        }

        for val in display_names.iter_mut() {
            let mut dn_bytes = vec![0; 31];
            raw.copy_to_slice(&mut dn_bytes);
            let end = dn_bytes.iter().position(|&x| x == 0).unwrap_or(30);
            dn_bytes.truncate(end);
            let (display_name, _, _) = SHIFT_JIS.decode(&dn_bytes);
            *val = Some(display_name.to_string().into_boxed_str());
        }

        for val in connect_codes.iter_mut() {
            let mut cc_bytes = vec![0; 10];
            raw.copy_to_slice(&mut cc_bytes);
            let end = cc_bytes.iter().position(|&x| x == 0).unwrap_or(30);
            cc_bytes.truncate(end);
            let (connect_code, _, _) = SHIFT_JIS.decode(&cc_bytes);
            *val = Some(connect_code.to_string().into_boxed_str());
        }

        if !raw.has_remaining() {
            // version < 3.11.0
            let mut players: [Player; 2] = [Player::default(), Player::default()];
            let mut count = 0;

            for i in 0..4 {
                if temp_players[i].1 == PlayerType::Human {
                    players[count] = Player {
                        character: temp_players[i].0,
                        port: Port::try_from(i as u8).unwrap(),
                        connect_code: connect_codes[i].clone(),
                        display_name: display_names[i].clone(),
                        winner: None,
                        stats: (),
                        combos: (),
                        frames: Frames::default(),
                        nana_frames: None,
                        costume: 0,
                        ucf: temp_ucf[i],
                    };
                    count += 1;
                }
            }
            return (
                GameStart {
                    random_seed,
                    is_teams,
                    stage,
                    timer: timer_length,
                    is_pal,
                    is_frozen_stadium,
                    is_netplay,
                    match_id,
                    match_type,
                    game_number,
                    tiebreak_number,
                    damage_ratio,
                },
                version,
                players,
            );
        }

        raw.advance(29 * 4); // skip past slippi uid

        // let mut uids = Vec::new();
        // for _ in 0..4 {
        //     let mut uid_bytes = vec![0; 29];
        //     raw.copy_to_slice(&mut uid_bytes);
        //     let end = uid_bytes.iter().position(|&x| x == 0).unwrap_or(28);
        //     uid_bytes.truncate(end);
        //     let uid = String::from_utf8(uid_bytes).unwrap();
        //     uids.push(uid);
        // }

        if !raw.has_remaining() {
            // version < 3.12.0
            let mut players: [Player; 2] = [Player::default(), Player::default()];
            let mut count = 0;

            for i in 0..4 {
                if temp_players[i].1 == PlayerType::Human {
                    players[count] = Player {
                        character: temp_players[i].0,
                        port: Port::try_from(i as u8).unwrap(),
                        connect_code: connect_codes[i].clone(),
                        display_name: display_names[i].clone(),
                        winner: None,
                        stats: (),
                        combos: (),
                        frames: Frames::default(),
                        nana_frames: None,
                        costume: 0,
                        ucf: temp_ucf[i],
                    };
                    count += 1;
                }
            }
            return (
                GameStart {
                    random_seed,
                    is_teams,
                    stage,
                    timer: timer_length,
                    is_pal,
                    is_frozen_stadium,
                    is_netplay,
                    match_id,
                    match_type,
                    game_number,
                    tiebreak_number,
                    damage_ratio,
                },
                version,
                players,
            );
        }

        raw.advance(1); // skip language option

        if !raw.has_remaining() {
            // version < 3.14.0
            let mut players: [Player; 2] = [Player::default(), Player::default()];
            let mut count = 0;

            for i in 0..4 {
                if temp_players[i].1 == PlayerType::Human {
                    players[count] = Player {
                        character: temp_players[i].0,
                        port: Port::try_from(i as u8).unwrap(),
                        connect_code: connect_codes[i].clone(),
                        display_name: display_names[i].clone(),
                        winner: None,
                        stats: (),
                        combos: (),
                        frames: Frames::default(),
                        nana_frames: None,
                        costume: 0,
                        ucf: temp_ucf[i],
                    };
                    count += 1;
                }
            }
            return (
                GameStart {
                    random_seed,
                    is_teams,
                    stage,
                    timer: timer_length,
                    is_pal,
                    is_frozen_stadium,
                    is_netplay,
                    match_id,
                    match_type,
                    game_number,
                    tiebreak_number,
                    damage_ratio,
                },
                version,
                players,
            );
        }

        let mut match_id_bytes = vec![0; 51];
        raw.copy_to_slice(&mut match_id_bytes);
        let end = match_id_bytes.iter().position(|&x| x == 0).unwrap_or(50);
        match_id_bytes.truncate(end);
        let match_id_len = match_id_bytes.len();
        match_id = Some(String::from_utf8(match_id_bytes).unwrap().into_boxed_str());

        game_number = Some(raw.get_u32());
        tiebreak_number = Some(raw.get_u32());

        match_type = {
            if match_id_len > 5 {
                Some(MatchType::from(
                    match_id.as_ref().map(|x| x.as_bytes()[5]).unwrap(),
                ))
            } else {
                Some(MatchType::Unknown)
            }
        };

        let mut players: [Player; 2] = [Player::default(), Player::default()];
        let mut count = 0;

        for i in 0..4 {
            if temp_players[i].1 == PlayerType::Human {
                players[count] = Player {
                    character: temp_players[i].0,
                    port: Port::try_from(i as u8).unwrap(),
                    connect_code: connect_codes[i].clone(),
                    display_name: display_names[i].clone(),
                    winner: None,
                    stats: (),
                    combos: (),
                    frames: Frames::default(),
                    nana_frames: None,
                    costume: 0,
                    ucf: temp_ucf[i],
                };
                count += 1;
            }
        }

        (
            GameStart {
                random_seed,
                is_teams,
                stage,
                timer: timer_length,
                is_pal,
                is_frozen_stadium,
                is_netplay,
                match_id,
                match_type,
                game_number,
                tiebreak_number,
                damage_ratio,
            },
            version,
            players,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Version {
    major: u8,
    minor: u8,
    revision: u8,
}

impl Version {
    pub fn new(data: [u8; 4]) -> Self {
        Self {
            major: data[0],
            minor: data[1],
            revision: data[2],
        }
    }
}

/// Added in slippi 2.2.0
pub fn parse_framestart(frames: Vec<Bytes>) -> DataFrame {
    let len = frames.len();
    // I choose to record frame number because it allow for accessing frames 0-indexed (through the dataframe's rows),
    // AND through melee's -123-index (through the frame_number column). This is flexibility that i sorely missed
    // when working in Py-Slippi-Stats.
    let mut frame_number = {
        let temp: Vec<i32> = Vec::with_capacity(len);
        temp
    };

    let mut random_seed = {
        let temp: Vec<u32> = Vec::with_capacity(len);
        temp
    };

    let mut scene_frame_counter = {
        let temp: Vec<Option<u32>> = Vec::with_capacity(len);
        temp
    };

    for mut frame in frames {
        frame_number.push(frame.get_i32());
        random_seed.push(frame.get_u32());
        if frame.has_remaining() {
            scene_frame_counter.push(Some(frame.get_u32()));
        }
    }

    if scene_frame_counter.is_empty() {
        scene_frame_counter.resize(len, None)
    }

    df![
        "frame_number" => frame_number,
        "random_seed" => random_seed,
        "scene_frame_counter" => scene_frame_counter,
    ]
    .unwrap()
}

pub fn parse_frameend(frames: Vec<Bytes>) -> DataFrame {
    let frame_number = {
        let temp: Vec<i32> = Vec::with_capacity(frames.len());
        temp
    };

    // NOTE i intentionally do not handle the latest finalized frame because this parser cannot handle live replays.

    df![
        "frame_number" => frame_number,
    ]
    .unwrap()
}
