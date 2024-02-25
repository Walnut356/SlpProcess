#![allow(non_upper_case_globals)]

use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Result};
use bytes::{Buf, Bytes};
use encoding_rs::SHIFT_JIS;
use strum_macros::{Display, FromRepr, IntoStaticStr};

use crate::{
    player::{Player, UCFToggles},
    Port,
};
use ssbm_utils::enums::{character::Character, stage::StageID};

#[derive(Debug, Clone, Copy, PartialEq, FromRepr, Default)]
#[repr(u8)]
pub enum Mode {
    VS = 2,
    Online = 8,
    #[default]
    Unknown = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, FromRepr, IntoStaticStr, Default, Display)]
#[repr(u8)]
pub enum MatchType {
    // ascii character values for u, r, d
    Unranked = 117,
    Ranked = 114,
    Direct = 100,
    #[default]
    Unknown = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, FromRepr)]
#[repr(u8)]
pub enum PlayerType {
    Human = 0,
    CPU = 1,
    Demo = 2,
    Empty = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, FromRepr, Default)]
#[repr(u8)]
pub enum ControllerFix {
    Off = 0,
    #[default] // this has more or less been true since like 2018
    UCF = 1,
    Dween = 2,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GameStart {
    /// Random seed at the start of the match
    pub random_seed: u32,
    /// True if teams mode is active, regardless of the number of players in the match
    pub teams: bool,
    /// Simple stage ID. For stage data (blast zones, ledge locations, etc.), cast into `Stage`
    pub stage: StageID,
    /// The timer setting for the match, will usually be 8 minutes (480s)
    pub timer: Duration,
    /// Damage ratio in the settings menu, should almost always be 1.0
    pub damage_ratio: f32,
    /// True if PAL
    ///
    /// added v1.5.0
    pub pal: Option<bool>,
    /// True if stadium is frozen
    ///
    /// added v2.0.0
    pub frozen_stadium: Option<bool>,
    /// True if played on slippi netplay
    ///
    /// added v3.7.0
    pub netplay: Option<bool>,
    /// Match id, usually very similar to the default file name
    ///
    /// added v3.14.0
    pub match_id: Arc<String>,
    /// Unranked, Ranked, Direct, or Unknown. Note that Doubles is not an option because this parser
    /// handles 1v1 replays only
    ///
    /// added v3.14.0
    pub match_type: MatchType,
    /// For the given match ID, this is Xth game played. Starts at 1
    ///
    /// added v3.14.0
    pub game_number: Option<u32>,
    /// For the given match ID, this is the Xth tiebreak game played. Will almost always be 0
    ///
    /// added v3.14.0
    pub tiebreak_number: Option<u32>,
}

impl GameStart {
    // the awkward return type here is because this will only ever be constructed internally, and because it will help
    // a LOT down the line to have the players contained in the top level Game object rather than the GameStart event.
    pub fn parse(mut raw: Bytes) -> Result<(Self, Version, [Player; 2])> {
        let version = Version::new(raw.get_u8(), raw.get_u8(), raw.get_u8());
        raw.advance(9); // skip past revision number, game bitfields 1-4 and bomb rain

        let is_teams = raw.get_u8() != 0;
        raw.advance(5); // skip item spawn rate and self destruct score value

        let stage = StageID::from_repr(raw.get_u16()).unwrap();

        // timer value is given in seconds, can only be changed by full-minute increments in-game
        let timer_length = Duration::from_secs(raw.get_u32() as u64);
        raw.advance(28); // skip past item spawn bitfields

        let damage_ratio = raw.get_f32();
        raw.advance(44);

        let mut temp_players = Vec::new();
        for _ in 0..4 {
            let character = Character::try_from_css(raw.get_u8())?;
            let p_type = PlayerType::from_repr(raw.get_u8())
                .ok_or_else(|| anyhow!("Invalid player type"))?;
            raw.advance(1);
            let costume = character.get_costume(raw.get_u8());

            temp_players.push((character, p_type, costume));

            raw.advance(32) // skip to next port
        }

        let mut p_count = 0;
        for (i, (_, p_type, _)) in temp_players.iter().enumerate() {
            match p_type {
                PlayerType::Human => p_count += 1,
                PlayerType::CPU => return Err(anyhow!("CPU player detected in port {i}. Parser only tolerates replays with 2 Human players")),
                PlayerType::Demo => return Err(anyhow!("Demo player detected in port {i}. How did this even happen?")),
                PlayerType::Empty => continue,
            }
        }

        if p_count > 2 {
            return Err(anyhow!(
                "Invalid player countL {p_count}. Parser only tolerates 1v1 replays"
            ));
        }

        raw.advance(72); // skip past "players" 5 and 6

        let random_seed = raw.get_u32();

        // Null out potentially uninitialized values:
        let mut temp_ucf: [Option<UCFToggles>; 4] = [None; 4];
        let is_pal = None;
        let is_frozen_stadium = None;
        let is_netplay = None;
        let mut display_names = [None, None, None, None];
        let mut connect_codes = [None, None, None, None];
        let match_id = Arc::new("".to_string());
        let match_type = MatchType::Unknown;
        let game_number = None;
        let tiebreak_number = None;

        let mut result = GameStart {
            random_seed,
            teams: is_teams,
            stage,
            timer: timer_length,
            pal: is_pal,
            frozen_stadium: is_frozen_stadium,
            netplay: is_netplay,
            match_id,
            match_type,
            game_number,
            tiebreak_number,
            damage_ratio,
        };

        if !raw.has_remaining() {
            // version < 1.0.0
            let mut players: [Player; 2] = [Player::default(), Player::default()];
            let mut count = 0;

            for i in 0..4 {
                if temp_players[i].1 == PlayerType::Human {
                    players[count] = Player {
                        character: temp_players[i].0,
                        port: Port::try_from(i as i8).unwrap(),
                        connect_code: connect_codes[i].clone(),
                        display_name: display_names[i].clone(),
                        is_winner: None,
                        stats: Default::default(),
                        combos: Default::default(),
                        frames: Default::default(),
                        nana_frames: None,
                        costume: temp_players[i].2,
                        ucf: temp_ucf[i],
                    };
                    count += 1;
                }
            }
            return Ok((result, version, players));
        }

        for val in temp_ucf.iter_mut() {
            let dashback = ControllerFix::from_repr(raw.get_u32() as u8).unwrap();
            let shield_drop = ControllerFix::from_repr(raw.get_u32() as u8).unwrap();
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
                        port: Port::try_from(i as i8)?,
                        connect_code: connect_codes[i].clone(),
                        display_name: display_names[i].clone(),
                        is_winner: None,
                        stats: Default::default(),
                        combos: Default::default(),
                        frames: Default::default(),
                        nana_frames: None,
                        costume: temp_players[i].2,
                        ucf: temp_ucf[i],
                    };
                    count += 1;
                }
            }
            return Ok((result, version, players));
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
                        port: Port::try_from(i as i8)?,
                        connect_code: connect_codes[i].clone(),
                        display_name: display_names[i].clone(),
                        is_winner: None,
                        stats: Default::default(),
                        combos: Default::default(),
                        frames: Default::default(),
                        nana_frames: None,
                        costume: temp_players[i].2,
                        ucf: temp_ucf[i],
                    };
                    count += 1;
                }
            }
            return Ok((result, version, players));
        }

        result.pal = Some(raw.get_u8() != 0);

        if !raw.has_remaining() {
            // version < 2.0.0
            let mut players: [Player; 2] = [Player::default(), Player::default()];
            let mut count = 0;

            for i in 0..4 {
                if temp_players[i].1 == PlayerType::Human {
                    players[count] = Player {
                        character: temp_players[i].0,
                        port: Port::try_from(i as i8)?,
                        connect_code: connect_codes[i].clone(),
                        display_name: display_names[i].clone(),
                        is_winner: None,
                        stats: Default::default(),
                        combos: Default::default(),
                        frames: Default::default(),
                        nana_frames: None,
                        costume: temp_players[i].2,
                        ucf: temp_ucf[i],
                    };
                    count += 1;
                }
            }
            return Ok((result, version, players));
        }

        result.frozen_stadium = Some(raw.get_u8() != 0);

        if !raw.has_remaining() {
            // version < 3.7.0
            let mut players: [Player; 2] = [Player::default(), Player::default()];
            let mut count = 0;

            for i in 0..4 {
                if temp_players[i].1 == PlayerType::Human {
                    players[count] = Player {
                        character: temp_players[i].0,
                        port: Port::try_from(i as i8)?,
                        connect_code: connect_codes[i].clone(),
                        display_name: display_names[i].clone(),
                        is_winner: None,
                        stats: Default::default(),
                        combos: Default::default(),
                        frames: Default::default(),
                        nana_frames: None,
                        costume: temp_players[i].2,
                        ucf: temp_ucf[i],
                    };
                    count += 1;
                }
            }
            return Ok((result, version, players));
        }

        raw.advance(1); // skip minor scene
        result.netplay = Some(raw.get_u8() == 8);

        if !raw.has_remaining() {
            // version < 3.9.0
            let mut players: [Player; 2] = [Player::default(), Player::default()];
            let mut count = 0;

            for i in 0..4 {
                if temp_players[i].1 == PlayerType::Human {
                    players[count] = Player {
                        character: temp_players[i].0,
                        port: Port::try_from(i as i8)?,
                        connect_code: connect_codes[i].clone(),
                        display_name: display_names[i].clone(),
                        is_winner: None,
                        stats: Default::default(),
                        combos: Default::default(),
                        frames: Default::default(),
                        nana_frames: None,
                        costume: temp_players[i].2,
                        ucf: temp_ucf[i],
                    };
                    count += 1;
                }
            }
            return Ok((result, version, players));
        }

        for val in display_names.iter_mut() {
            let mut dn_bytes = vec![0; 31];
            raw.copy_to_slice(&mut dn_bytes);
            let end = dn_bytes.iter().position(|&x| x == 0).unwrap_or(30);
            dn_bytes.truncate(end);
            let (display_name, _, _) = SHIFT_JIS.decode(&dn_bytes);
            *val = Some(display_name.to_string());
        }

        for val in connect_codes.iter_mut() {
            let mut cc_bytes = vec![0; 10];
            raw.copy_to_slice(&mut cc_bytes);
            let end = cc_bytes.iter().position(|&x| x == 0).unwrap_or(10);
            cc_bytes.truncate(end);
            let (connect_code, _, _) = SHIFT_JIS.decode(&cc_bytes);
            // replace the full width hash symbol with the ascii variant so people can actually type them
            let adjusted = connect_code.replace('＃', "#");
            *val = Some(adjusted);
        }

        if !raw.has_remaining() {
            // version < 3.11.0
            let mut players: [Player; 2] = [Player::default(), Player::default()];
            let mut count = 0;

            for i in 0..4 {
                if temp_players[i].1 == PlayerType::Human {
                    players[count] = Player {
                        character: temp_players[i].0,
                        port: Port::try_from(i as i8).unwrap(),
                        connect_code: connect_codes[i].clone(),
                        display_name: display_names[i].clone(),
                        is_winner: None,
                        stats: Default::default(),
                        combos: Default::default(),
                        frames: Default::default(),
                        nana_frames: None,
                        costume: temp_players[i].2,
                        ucf: temp_ucf[i],
                    };
                    count += 1;
                }
            }
            return Ok((result, version, players));
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
                        port: Port::try_from(i as i8).unwrap(),
                        connect_code: connect_codes[i].clone(),
                        display_name: display_names[i].clone(),
                        is_winner: None,
                        stats: Default::default(),
                        combos: Default::default(),
                        frames: Default::default(),
                        nana_frames: None,
                        costume: temp_players[i].2,
                        ucf: temp_ucf[i],
                    };
                    count += 1;
                }
            }
            return Ok((result, version, players));
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
                        port: Port::try_from(i as i8).unwrap(),
                        connect_code: connect_codes[i].clone(),
                        display_name: display_names[i].clone(),
                        is_winner: None,
                        stats: Default::default(),
                        combos: Default::default(),
                        frames: Default::default(),
                        nana_frames: None,
                        costume: temp_players[i].2,
                        ucf: temp_ucf[i],
                    };
                    count += 1;
                }
            }
            return Ok((result, version, players));
        }

        let mut match_id_bytes = vec![0; 51];
        raw.copy_to_slice(&mut match_id_bytes);
        let end = match_id_bytes.iter().position(|&x| x == 0).unwrap_or(50);
        match_id_bytes.truncate(end);
        let match_id_len = match_id_bytes.len();
        result.match_id = Arc::new(String::from_utf8(match_id_bytes).unwrap());

        result.game_number = Some(raw.get_u32());
        result.tiebreak_number = Some(raw.get_u32());

        result.match_type = {
            if match_id_len > 5 {
                MatchType::from_repr(result.match_id.as_bytes()[5]).unwrap_or_default()
            } else {
                MatchType::Unknown
            }
        };

        let mut players: [Player; 2] = [Player::default(), Player::default()];
        let mut count = 0;

        for i in 0..4 {
            if temp_players[i].1 == PlayerType::Human {
                players[count] = Player {
                    character: temp_players[i].0,
                    port: Port::try_from(i as i8).unwrap(),
                    connect_code: connect_codes[i].clone(),
                    display_name: display_names[i].clone(),
                    is_winner: None,
                    stats: Default::default(),
                    combos: Default::default(),
                    frames: Default::default(),
                    nana_frames: None,
                    costume: temp_players[i].2,
                    ucf: temp_ucf[i],
                };
                count += 1;
            }
        }

        Ok((result, version, players))
    }
}

/// Slippi replay version, dictates what information is available in the replay.
///
/// Version release dates listed below. Note that date checks can be misleading due to incorrectly
/// set dates on consoles, as well as updating slippi version late. This is purely meant as a
/// reference to know how old the replay spec is (and thus, roughly how likely it is that the
/// average replay contains a desired piece of information).
///

///
/// | Version | Released    |
/// |---------|-------------|
/// | 0.1.0   | Unknown     |
/// | 0.2.0   | Unknown     |
/// | 1.0.0   | Jul 01 2018 |
/// | 1.2.0   | Aug 08 2018 |
/// | 1.3.0   | Jan 14 2019 |
/// | 1.4.0   | Jan 15 2019 |
/// | 1.5.0   | Feb 08 2019 |
/// | 2.0.0   | Mar 19 2019 |
/// | 2.1.0   | Apr 28 2019 |
/// | 2.2.0   | Jun 24 2019 |
/// | 3.0.0   | Oct 24 2019 |
/// | 3.2.0   | Jan 31 2020 |
/// | 3.3.0   | Feb 11 2020 |
/// | 3.5.0   | Jun 13 2020 |
/// | 3.6.0   | Jun 20 2020 |
/// | 3.7.0   | Jul 08 2020 |
/// | 3.8.0   | Dec 06 2020 |
/// | 3.9.0   | Feb 06 2021 |
/// | 3.10.0  | Jan 26 2022 |
/// | 3.11.0  | Jan 30 2022 |
/// | 3.12.0  | Feb 07 2022 |
/// | 3.13.0  | Aug 30 2022 |
/// | 3.14.0  | Nov 04 2022 |
/// | 3.15.0  | May 27 2023 |
/// | 3.16.0  | Sep 20 2023 |
///
/// Some noteable dates in the Slippi ecosystem:
///
/// * The first public release was Jun 18 2018
/// * Slippi was made the main dolphin version on Anther's Ladder Sep 29 2018
/// * Rollback netplay was released Jun 22 2020
/// * Unranked MMR was released Jan 19 2021
/// * Doubles matchmaking was released May 2 2021
/// * Ranked was released Dec 12 2022
///
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub build: u8,
}

impl Version {
    #[inline]
    pub fn new(major: u8, minor: u8, build: u8) -> Self {
        Self {
            major,
            minor,
            build,
        }
    }

    #[inline]
    /// Returns true if self is at least (greater than or equal to) the given version
    pub fn at_least(&self, major: u8, minor: u8, build: u8) -> bool {
        *self
            >= Version {
                major,
                minor,
                build,
            }
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        u32::from_be_bytes([self.major, self.minor, self.build, 0])
    }
}

impl Default for Version {
    /// Returns Version{0, 1, 0}, the first slippi release version
    #[inline]
    fn default() -> Self {
        Self {
            major: 0,
            minor: 1,
            build: 0,
        }
    }
}

// /// Added in slippi 2.2.0
// #[allow(dead_code)] // allowing as I might need these later
// fn parse_framestart(frames: Vec<Bytes>) -> DataFrame {
//     let len = frames.len();
//     // I choose to record frame number because it allow for accessing frames 0-indexed (through the
//     // dataframe's rows), AND through melee's -123-index (through the frame_number column). This is
//     // flexibility that i sorely missed when debugging some other stuff
//     let mut frame_number = {
//         let temp: Vec<i32> = Vec::with_capacity(len);
//         temp
//     };

//     let mut random_seed = {
//         let temp: Vec<u32> = Vec::with_capacity(len);
//         temp
//     };

//     let mut scene_frame_counter = {
//         let temp: Vec<Option<u32>> = Vec::with_capacity(len);
//         temp
//     };

//     for mut frame in frames {
//         frame_number.push(frame.get_i32());
//         random_seed.push(frame.get_u32());
//         if frame.has_remaining() {
//             scene_frame_counter.push(Some(frame.get_u32()));
//         }
//     }

//     if scene_frame_counter.is_empty() {
//         scene_frame_counter.resize(len, None)
//     }

//     df![
//         "frame_number" => frame_number,
//         "random_seed" => random_seed,
//         "scene_frame_counter" => scene_frame_counter,
//     ]
//     .unwrap()
// }

// #[allow(dead_code)] // allowing as I might need these later
// fn parse_frameend(frames: Vec<Bytes>) -> DataFrame {
//     let frame_number = {
//         let temp: Vec<i32> = Vec::with_capacity(frames.len());
//         temp
//     };

//     // NOTE i intentionally do not handle the latest finalized frame because this parser cannot
//     // handle live replays.

//     df![
//         "frame_number" => frame_number,
//     ]
//     .unwrap()
// }
