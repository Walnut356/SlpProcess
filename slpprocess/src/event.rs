use std::slice;

use num_enum::FromPrimitive;

#[derive(Debug, Clone, PartialEq)]
pub struct GameStart {
    pub version: Version,
    pub players: [u8; 4],
    pub random_seed: u32,
    pub is_teams: bool,
    pub stage: u16,
    pub is_pal: bool,
    pub is_frozen_stadium: bool,
    pub mode: Mode,
    pub match_id: String,
    pub match_type: MatchType,
    pub game_number: u32,
    pub tiebreak_number: u32,
}

impl GameStart {
    pub fn new(ptr: *const u8, size: usize) -> Self {
        let slice = unsafe { slice::from_raw_parts(ptr, size).clone() };

        let match_id = unsafe {
            let mut temp = slice.get(0x2BD..0x2BD + 50).unwrap_or(&[0; 50]).to_vec();
            let i = temp.binary_search(&0x00).unwrap_or(50);
            temp.truncate(i);
            String::from_utf8_unchecked(temp)
        };

        GameStart {
            version: Version::new(&slice[0x0..=0x3]),
            is_teams: slice[0x4 + 0x8] != 0,
            stage: u16::from_be_bytes([slice[0x4 + 0xE], slice[0x4 + 0xF]]),
            players: [0; 4], //TODO
            random_seed: u32::from_be_bytes(
                slice
                    .get(0x13C..=0x13F)
                    .unwrap_or(&[0, 0, 0, 0])
                    .try_into()
                    .unwrap(),
            ),
            // v1.5
            // assume not pal and frozen stadium if replay is too old
            is_pal: *(slice.get(0x1A0).unwrap_or(&0)) != 0,
            // v2.0
            is_frozen_stadium: *(slice.get(0x1A1).unwrap_or(&1)) != 0,
            // v3.7
            mode: Mode::from(*(slice.get(0x1A3).unwrap_or(&0))),
            // 3.14
            match_id,
            game_number: u32::from_be_bytes(
                slice
                    .get(0x2F0..=0x2F3)
                    .unwrap_or(&[0, 0, 0, 0])
                    .try_into()
                    .unwrap(),
            ),
            tiebreak_number: u32::from_be_bytes(
                slice
                    .get(0x2F4..=0x2F7)
                    .unwrap_or(&[0, 0, 0, 0])
                    .try_into()
                    .unwrap(),
            ),
            match_type: MatchType::from(*slice.get(0x2BD + 5).unwrap_or(&0)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Mode {
    VS = 2,
    Online = 8,
    #[default]
    Unknown = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum MatchType {
    // ascii character values for u, r, d
    Unranked = 117,
    Ranked = 114,
    Direct = 100,
    #[default]
    Unknown = 0,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Version {
    major: u8,
    minor: u8,
    revision: u8,
}

impl Version {
    pub fn new(data: &[u8]) -> Self {
        Self {
            major: data[0] as u8,
            minor: data[1] as u8,
            revision: data[2],
        }
    }
}
