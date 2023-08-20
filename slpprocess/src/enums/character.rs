use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Character {
    CaptainFalcon = 0,
    DonkeyKong = 1,
    Fox = 2,
    GameAndWatch = 3,
    Kirby = 4,
    Bowser = 5,
    Link = 6,
    Luigi = 7,
    Mario = 8,
    Marth = 9,
    Mewtwo = 10,
    Ness = 11,
    Peach = 12,
    Pikachu = 13,
    IceClimbers = 14,
    Jigglypuff = 15,
    Samus = 16,
    Yoshi = 17,
    Zelda = 18,
    Sheik = 19,
    Falco = 20,
    #[default] // for whatever reason, empty ports have young link as the default character
    YoungLink = 21,
    DrMario = 22,
    Roy = 23,
    Pichu = 24,
    Ganondorf = 25,
    MasterHand = 26,
    WireframeMale = 27,
    WireframeFemale = 28,
    GigaBowser = 29,
    CrazyHand = 30,
    Sandbag = 31,
    Popo = 32,
    Nana = 33,
}

impl Character {

    /// Attempts to match the given ID with a character on the Character Select Screen
    pub fn try_from_css(id: u8) -> Result<Self> {
        use Character::*;
        match id {
            0 => Ok(CaptainFalcon),
            1 => Ok(DonkeyKong),
            2 => Ok(Fox),
            3 => Ok(GameAndWatch),
            4 => Ok(Kirby),
            5 => Ok(Bowser),
            6 => Ok(Link),
            7 => Ok(Luigi),
            8 => Ok(Mario),
            9 => Ok(Marth),
            10 => Ok(Mewtwo),
            11 => Ok(Ness),
            12 => Ok(Peach),
            13 => Ok(Pikachu),
            14 => Ok(IceClimbers),
            15 => Ok(Jigglypuff),
            16 => Ok(Samus),
            17 => Ok(Yoshi),
            18 => Ok(Zelda),
            19 => Ok(Sheik),
            20 => Ok(Falco),
            21 => Ok(YoungLink),
            22 => Ok(DrMario),
            23 => Ok(Roy),
            24 => Ok(Pichu),
            25 => Ok(Ganondorf),
            26 => Ok(MasterHand),
            27 => Ok(WireframeMale),
            28 => Ok(WireframeFemale),
            29 => Ok(GigaBowser),
            30 => Ok(CrazyHand),
            31 => Ok(Sandbag),
            32 => Ok(Popo),
            x => Err(anyhow!(
                "Invalid CSS Character code: {}. Expected value 0-32 (inclusive)",
                x
            )),
        }
    }

    /// Attempts to match the given ID to a character as they are represented in memory during a match
    pub fn try_from_internal(id: u8) -> Result<Self> {
        use Character::*;
        match id {
            0 => Ok(Mario),
            1 => Ok(Fox),
            2 => Ok(CaptainFalcon),
            3 => Ok(DonkeyKong),
            4 => Ok(Kirby),
            5 => Ok(Bowser),
            6 => Ok(Link),
            7 => Ok(Sheik),
            8 => Ok(Ness),
            9 => Ok(Peach),
            10 => Ok(Popo),
            11 => Ok(Nana),
            12 => Ok(Pikachu),
            13 => Ok(Samus),
            14 => Ok(Yoshi),
            15 => Ok(Jigglypuff),
            16 => Ok(Mewtwo),
            17 => Ok(Luigi),
            18 => Ok(Marth),
            19 => Ok(Zelda),
            20 => Ok(YoungLink),
            21 => Ok(DrMario),
            22 => Ok(Falco),
            23 => Ok(Pichu),
            24 => Ok(GameAndWatch),
            25 => Ok(Ganondorf),
            26 => Ok(Roy),
            27 => Ok(MasterHand),
            28 => Ok(CrazyHand),
            29 => Ok(WireframeMale),
            30 => Ok(WireframeFemale),
            31 => Ok(GigaBowser),
            32 => Ok(Sandbag),
            x => Err(anyhow!(
                "Invalid Internal Character code: {}. Expected value 0-32 (inclusive)",
                x
            )),
        }
    }

    pub fn try_into_css(&self) -> Result<u8> {
        use Character::*;
        match self {
            CaptainFalcon => Ok(0),
            DonkeyKong => Ok(1),
            Fox => Ok(2),
            GameAndWatch => Ok(3),
            Kirby => Ok(4),
            Bowser => Ok(5),
            Link => Ok(6),
            Luigi => Ok(7),
            Mario => Ok(8),
            Marth => Ok(9),
            Mewtwo => Ok(10),
            Ness => Ok(11),
            Peach => Ok(12),
            Pikachu => Ok(13),
            IceClimbers => Ok(14),
            Jigglypuff => Ok(15),
            Samus => Ok(16),
            Yoshi => Ok(17),
            Zelda => Ok(18),
            Sheik => Ok(19),
            Falco => Ok(20),
            YoungLink => Ok(21),
            DrMario => Ok(22),
            Roy => Ok(23),
            Pichu => Ok(24),
            Ganondorf => Ok(25),
            MasterHand => Ok(26),
            WireframeMale => Ok(27),
            WireframeFemale => Ok(28),
            GigaBowser => Ok(29),
            CrazyHand => Ok(30),
            Sandbag => Ok(31),
            Popo => Ok(32),
            x => Err(anyhow!(
                "Invalid CSS Character code: {:?}",
                x
            )),
        }
    }
}
