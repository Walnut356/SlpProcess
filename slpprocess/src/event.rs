#[derive(Debug, Clone, Copy, PartialEq)]
struct GameStart {
    version: Version,
    players: [u8; 4],
    random_seed: u32,
    is_teams: bool,
    stage: u16,
    is_pal: bool,
    is_frozen_stadium: bool,
    match_id: String,
    match_type: MatchType,
    game_number: u8,
    tiebreak_number: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Version {
    major: u8,
    minor: u8,
    revision: u8,
}

impl Version {
    fn new(data: [u8; 4]) -> Self {
        Self {
            major: data[0],
            minor: data[1],
            revision: data[2],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MatchType {}
