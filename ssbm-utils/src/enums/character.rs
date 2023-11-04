use anyhow::{anyhow, Result};
use strum_macros::{Display, EnumString};

/// All in-game characters, including non-playable character such as the wireframes and masterhand.
///
/// In-game and Character Select Screen **do not** use the same numbering system. When retrieving
/// a character, match your number's source to the correct `try_from`
/// ```rust
/// # use slpprocess::enums::character::Character;
/// let char_1 = Character::try_from_css(0);
/// let char_2 = Character::try_from_internal(0);
/// assert!(char_1.is_ok_and(|char| char == Character::CaptainFalcon));
/// assert!(char_2.is_ok_and(|char| char == Character::Mario));
/// ```
/// Several colloquial names for some characters are also valid, these are also case-insensitive
/// ```
/// # use slpprocess::enums::character::Character;
/// let char_3 = Character::try_from("jiggs").unwrap();
/// let char_4 = Character::try_from("puff").unwrap();
/// let char_5 = Character::try_from("jIgGlYpUfF").unwrap();
/// assert_eq!(char_3, Character::Jigglypuff);
/// assert_eq!(char_4, Character::Jigglypuff);
/// assert_eq!(char_5, Character::Jigglypuff);
/// ```
#[derive(Debug, Clone, Default, Copy, PartialEq, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum Character {
    #[strum(serialize = "falcon", serialize = "captainfalcon")]
    CaptainFalcon = 0,
    #[strum(serialize = "dk", serialize = "donkeykong")]
    DonkeyKong = 1,
    Fox = 2,
    #[strum(serialize = "gnw", serialize = "gameandwatch")]
    GameAndWatch = 3,
    Kirby = 4,
    Bowser = 5,
    Link = 6,
    Luigi = 7,
    Mario = 8,
    Marth = 9,
    #[strum(serialize = "mew2", serialize = "m2", serialize = "mewtwo")]
    Mewtwo = 10,
    Ness = 11,
    Peach = 12,
    #[strum(serialize = "pika", serialize = "pikachu")]
    Pikachu = 13,
    /// Individaul climbers can be accessed via
    /// ```
    /// # use ssbm_utils::enums::character::Character;
    /// let poo = Character::Popo;
    /// let nana = Character::Nana;
    /// ```
    /// via their in-engine codes
    /// ```
    /// # use ssbm_utils::enums::character::Character;
    /// let popo = Character::try_from_internal(10).unwrap();
    /// let nana = Character::try_from_internal(11).unwrap();
    /// ```
    /// and via their string names (not case sensitive)
    /// ```
    /// # use ssbm_utils::enums::character::Character;
    /// let popo = Character::try_from("Popo").unwrap();
    /// let nana = Character::try_from("Nana").unwrap();
    /// ```
    #[strum(serialize = "ics", serialize = "iceclimbers")]
    IceClimbers = 14,
    #[strum(serialize = "puff", serialize = "jigglypuff", serialize = "jigglypuff")]
    Jigglypuff = 15,
    Samus = 16,
    Yoshi = 17,
    Zelda = 18,
    Sheik = 19,
    Falco = 20,
    #[default] // for whatever reason, the game uses YL as the default/empty value for ports
    #[strum(serialize = "ylink", serialize = "younglink")]
    YoungLink = 21,
    #[strum(serialize = "doc", serialize = "drmario")]
    DrMario = 22,
    Roy = 23,
    Pichu = 24,
    #[strum(serialize = "ganon", serialize = "ganondorf")]
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

    /// Attempts to match the given ID to a character as they are represented in memory during a
    /// match
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

    pub fn try_as_css(&self) -> Result<u8> {
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
            x => Err(anyhow!("Invalid CSS Character code: {:?}", x)),
        }
    }

    pub fn as_internal(&self) -> u8 {
        *self as u8
    }

    pub fn get_stats(&self) -> Attributes {
        match *self {
            Character::Fox => Attributes::FOX,
            Character::Marth => Attributes::MARTH,
            Character::CaptainFalcon => Attributes::FALCON,
            Character::Falco => Attributes::FALCO,
            Character::Peach => Attributes::PEACH,
            Character::Jigglypuff => Attributes::JIGGS,
            Character::Pikachu => Attributes::PIKACHU,
            Character::Sheik => Attributes::SHEIK,
            Character::Samus => Attributes::SAMUS,
            Character::Ganondorf => Attributes::GANONDORF,
            Character::Roy => Attributes::ROY,
            Character::DrMario => Attributes::DOC,
            Character::Mario => Attributes::MARIO,
            Character::Luigi => Attributes::LUIGI,
            Character::Link => Attributes::LINK,
            Character::YoungLink => Attributes::YLINK,
            Character::DonkeyKong => Attributes::DK,
            Character::Bowser => Attributes::BOWSER,
            Character::GameAndWatch => Attributes::GNW,
            Character::Popo => Attributes::ICECLIMBERS,
            Character::Nana => Attributes::ICECLIMBERS,
            Character::Kirby => Attributes::KIRBY,
            Character::Mewtwo => Attributes::MEWTWO,
            Character::Ness => Attributes::NESS,
            Character::Pichu => Attributes::PICHU,
            Character::Yoshi => Attributes::YOSHI,
            Character::Zelda => Attributes::ZELDA,
            Character::IceClimbers => Attributes::ICECLIMBERS,
            _ => panic!("No attribute data available for non-playable characters"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DJType {
    Normal(f32),
    /// Accounts for jiggs/kirby 5 jumps
    Multiple([f32; 5]),
    /// DJC characters (and jigg's h speed on double jump) are animation mapped, meaning any in-game value for these
    /// is irrelevant. This serves as the appropriate `None` type
    AnimationMapped,
}

#[derive(Debug, Clone)]
pub struct Attributes<'a> {
    pub name: &'a str,
    pub index: u8,
    pub air_jumps: u8,
    pub friction: f32,
    pub size: u8,
    pub gravity: f32,
    pub max_fall_speed: f32,
    pub max_walk_speed: f32,
    pub fast_fall_speed: f32,
    /// Maximum horizontal velocity in the air (not counting knockback)
    pub air_speed: f32,
    /// The value subtracted from the character's current air speed each frame when the stick is in neutral
    pub air_friction: f32,
    /// The value added to the character's current air speed each frame when the stick is fully left or right
    pub air_accel: f32,
    /// Initial X velocity if holding fully left/right upon double jumping
    pub dj_x_speed: DJType,
    /// Initial Y velocity of a fullhop
    pub fh_jump_force: f32,
    /// Initial Y velocity of a short hop
    pub sh_jump_force: f32,
    /// Multiply FH jump force by this value to get effective DJ force
    ///
    /// characters with double jump cancels have velocities mapped to animations, so this metric does not apply.
    ///
    /// Kirby and jigglypuff have multiple different dj forces depending on how many double jumps they've already used
    pub dj_force_multiplier: DJType,
    pub weight: u32,
    // TODO add jumpsquat, item throw velocity/damage ratio, normal landlag, walljump velocities, dash/run speeds and jump horizontal velocities
}

impl<'a> Attributes<'a> {
    pub const BOWSER: Attributes<'a> = Attributes {
        name: "Bowser",
        index: 5,
        air_jumps: 1,
        friction: 0.06,
        size: 16,
        gravity: 0.13,
        max_fall_speed: 1.9,
        max_walk_speed: 0.65,
        fast_fall_speed: 2.4,
        air_speed: 0.8,
        air_friction: 0.01,
        air_accel: 0.05,
        dj_x_speed: DJType::Normal(0.9),
        fh_jump_force: 2.8,
        sh_jump_force: 1.6,
        dj_force_multiplier: DJType::Normal(1.0),
        weight: 117,
    };
    pub const FALCON: Attributes<'a> = Attributes {
        name: "Captain Falcon",
        index: 2,
        air_jumps: 1,
        friction: 0.08,
        size: 14,
        gravity: 0.13,
        max_fall_speed: 2.9,
        max_walk_speed: 0.85,
        fast_fall_speed: 3.5,
        air_speed: 1.12,
        air_friction: 0.01,
        air_accel: 0.06,
        dj_x_speed: DJType::Normal(0.9),
        fh_jump_force: 3.1,
        sh_jump_force: 1.9,
        dj_force_multiplier: DJType::Normal(0.9),
        weight: 104,
    };
    pub const DK: Attributes<'a> = Attributes {
        name: "Donkey Kong",
        index: 3,
        air_jumps: 1,
        friction: 0.08,
        size: 16,
        gravity: 0.1,
        max_fall_speed: 2.4,
        max_walk_speed: 1.2,
        fast_fall_speed: 2.96,
        air_speed: 1.0,
        air_friction: 0.02,
        air_accel: 0.04,
        dj_x_speed: DJType::Normal(1.0),
        fh_jump_force: 2.7,
        sh_jump_force: 1.6,
        dj_force_multiplier: DJType::Normal(0.91),
        weight: 114,
    };
    pub const DOC: Attributes<'a> = Attributes {
        name: "Dr. Mario",
        index: 21,
        air_jumps: 1,
        friction: 0.06,
        size: 10,
        gravity: 0.095,
        max_fall_speed: 1.7,
        max_walk_speed: 1.1,
        fast_fall_speed: 2.3,
        air_speed: 0.9,
        air_friction: 0.016,
        air_accel: 0.044,
        dj_x_speed: DJType::Normal(0.9),
        fh_jump_force: 2.3,
        sh_jump_force: 1.4,
        dj_force_multiplier: DJType::Normal(1.0),
        weight: 100,
    };
    pub const FALCO: Attributes<'a> = Attributes {
        name: "Falco",
        index: 22,
        air_jumps: 1,
        friction: 0.08,
        size: 11,
        gravity: 0.17,
        max_fall_speed: 3.1,
        max_walk_speed: 1.4,
        fast_fall_speed: 3.5,
        air_speed: 0.83,
        air_friction: 0.02,
        air_accel: 0.07,
        dj_x_speed: DJType::Normal(0.94),
        fh_jump_force: 4.1,
        sh_jump_force: 1.9,
        dj_force_multiplier: DJType::Normal(0.94),
        weight: 80,
    };
    pub const FOX: Attributes<'a> = Attributes {
        name: "Fox",
        index: 1,
        air_jumps: 1,
        friction: 0.08,
        size: 11,
        gravity: 0.23,
        max_fall_speed: 2.8,
        max_walk_speed: 1.6,
        fast_fall_speed: 3.4,
        air_speed: 0.83,
        air_friction: 0.02,
        air_accel: 0.08,
        dj_x_speed: DJType::Normal(0.88),
        fh_jump_force: 3.68,
        sh_jump_force: 2.1,
        dj_force_multiplier: DJType::Normal(1.2),
        weight: 75,
    };
    pub const GNW: Attributes<'a> = Attributes {
        name: "Game and Watch",
        index: 24,
        air_jumps: 1,
        friction: 0.06,
        size: 10,
        gravity: 0.095,
        max_fall_speed: 1.7,
        max_walk_speed: 1.1,
        fast_fall_speed: 2.3,
        air_speed: 1.0,
        air_friction: 0.016,
        air_accel: 0.05,
        dj_x_speed: DJType::Normal(0.9),
        fh_jump_force: 2.3,
        sh_jump_force: 1.4,
        dj_force_multiplier: DJType::Normal(1.0),
        weight: 60,
    };
    pub const GANONDORF: Attributes<'a> = Attributes {
        name: "Ganondorf",
        index: 25,
        air_jumps: 1,
        friction: 0.07,
        size: 15,
        gravity: 0.13,
        max_fall_speed: 2.0,
        max_walk_speed: 0.73,
        fast_fall_speed: 2.6,
        air_speed: 0.78,
        air_friction: 0.02,
        air_accel: 0.06,
        dj_x_speed: DJType::Normal(0.9),
        fh_jump_force: 2.6,
        sh_jump_force: 2.0,
        dj_force_multiplier: DJType::Normal(0.95),
        weight: 109,
    };
    /// Popo and nana have identical stats
    pub const ICECLIMBERS: Attributes<'a> = Attributes {
        name: "IceClimbers",
        index: 10,
        air_jumps: 1,
        friction: 0.035,
        size: 10,
        gravity: 0.1,
        max_fall_speed: 1.6,
        max_walk_speed: 0.95,
        fast_fall_speed: 2.0,
        air_speed: 0.7,
        air_friction: 0.02,
        air_accel: 0.047,
        dj_x_speed: DJType::Normal(1.0),
        fh_jump_force: 2.6,
        sh_jump_force: 1.4,
        dj_force_multiplier: DJType::Normal(1.0),
        weight: 88,
    };
    pub const JIGGS: Attributes<'a> = Attributes {
        name: "Jigglypuff",
        index: 15,
        air_jumps: 5,
        friction: 0.09,
        size: 9,
        gravity: 0.064,
        max_fall_speed: 1.3,
        max_walk_speed: 0.7,
        fast_fall_speed: 1.6,
        air_speed: 1.35,
        air_friction: 0.05,
        air_accel: 0.28,
        // this one is kinda weird. The in-game value is .9, but it looks maybe animation mapped to 0.572
        // which would explain why if you do an aerial while jumping, it doesn't override X speed at all
        dj_x_speed: DJType::AnimationMapped,
        fh_jump_force: 1.6,
        sh_jump_force: 1.05,
        dj_force_multiplier: DJType::Multiple([
            // kinda gross but gives f32 accurate values instead of something rounded
            // this is just the fullhop jump force divided by the observed Y speed (accounding for gravity)
            1.6 / 1.65,
            1.6 / 1.59,
            1.6 / 1.47,
            1.6 / 1.36,
            1.6 / 1.25,
        ]),
        weight: 60,
    };
    pub const KIRBY: Attributes<'a> = Attributes {
        name: "Kirby",
        index: 4,
        air_jumps: 5,
        friction: 0.08,
        size: 9,
        gravity: 0.08,
        max_fall_speed: 1.6,
        max_walk_speed: 0.85,
        fast_fall_speed: 2.0,
        air_speed: 0.78,
        air_friction: 0.02,
        air_accel: 0.06,
        // this one is kinda weird. The in-game value is .9, but it looks maybe animation mapped to 0.472
        // (0.532 w/ air accel) which would explain why if you do an aerial while jumping, it doesn't override X speed
        // at all
        dj_x_speed: DJType::AnimationMapped,
        fh_jump_force: 2.0,
        sh_jump_force: 1.5,
        dj_force_multiplier: DJType::Multiple(
            // kinda gross but gives f32 accurate values instead of something rounded
            // this is just the fullhop jump force divided by the observed Y speed (accounding for gravity)
            [1.0, 1.0, 2.0 / 1.73, 2.0 / 1.56, 2.0 / 1.33],
        ),
        weight: 70,
    };
    pub const LINK: Attributes<'a> = Attributes {
        name: "Link",
        index: 6,
        air_jumps: 1,
        friction: 0.1,
        size: 11,
        gravity: 0.11,
        max_fall_speed: 2.13,
        max_walk_speed: 1.2,
        fast_fall_speed: 3.0,
        air_speed: 1.0,
        air_friction: 0.005,
        air_accel: 0.06,
        dj_x_speed: DJType::Normal(1.0),
        fh_jump_force: 2.5,
        sh_jump_force: 1.5,
        dj_force_multiplier: DJType::Normal(0.88),
        weight: 104,
    };
    pub const LUIGI: Attributes<'a> = Attributes {
        name: "Luigi",
        index: 17,
        air_jumps: 1,
        friction: 0.025,
        size: 11,
        gravity: 0.069,
        max_fall_speed: 1.6,
        max_walk_speed: 1.1,
        fast_fall_speed: 2.0,
        air_speed: 0.68,
        air_friction: 0.01,
        air_accel: 0.04,
        dj_x_speed: DJType::Normal(0.9),
        fh_jump_force: 2.4,
        sh_jump_force: 1.4,
        dj_force_multiplier: DJType::Normal(0.9),
        weight: 100,
    };
    pub const MARIO: Attributes<'a> = Attributes {
        name: "Mario",
        index: 0,
        air_jumps: 1,
        friction: 0.06,
        size: 10,
        gravity: 0.095,
        max_fall_speed: 1.7,
        max_walk_speed: 1.1,
        fast_fall_speed: 2.3,
        air_speed: 0.86,
        air_friction: 0.016,
        air_accel: 0.045,
        dj_x_speed: DJType::Normal(0.9),
        fh_jump_force: 2.3,
        sh_jump_force: 1.4,
        dj_force_multiplier: DJType::Normal(1.0),
        weight: 100,
    };
    pub const MARTH: Attributes<'a> = Attributes {
        name: "Marth",
        index: 18,
        air_jumps: 1,
        friction: 0.06,
        size: 14,
        gravity: 0.085,
        max_fall_speed: 2.2,
        max_walk_speed: 1.6,
        fast_fall_speed: 2.5,
        air_speed: 0.9,
        air_friction: 0.005,
        air_accel: 0.05,
        dj_x_speed: DJType::Normal(1.0),
        fh_jump_force: 2.4,
        sh_jump_force: 1.5,
        dj_force_multiplier: DJType::Normal(0.88),
        weight: 87,
    };
    pub const MEWTWO: Attributes<'a> = Attributes {
        name: "Mewtwo",
        index: 16,
        air_jumps: 1,
        friction: 0.04,
        size: 15,
        gravity: 0.082,
        max_fall_speed: 1.5,
        max_walk_speed: 1.0,
        fast_fall_speed: 2.3,
        air_speed: 1.2,
        air_friction: 0.016,
        air_accel: 0.05,
        dj_x_speed: DJType::AnimationMapped,
        fh_jump_force: 2.3,
        sh_jump_force: 1.4,
        dj_force_multiplier: DJType::AnimationMapped,
        weight: 85,
    };
    pub const NESS: Attributes<'a> = Attributes {
        name: "Ness",
        index: 8,
        air_jumps: 1,
        friction: 0.06,
        size: 10,
        gravity: 0.09,
        max_fall_speed: 1.83,
        max_walk_speed: 0.84,
        fast_fall_speed: 2.2,
        air_speed: 0.93,
        air_friction: 0.03,
        air_accel: 0.06,
        dj_x_speed: DJType::AnimationMapped,
        fh_jump_force: 2.5,
        sh_jump_force: 1.5,
        dj_force_multiplier: DJType::AnimationMapped,
        weight: 94,
    };
    pub const PEACH: Attributes<'a> = Attributes {
        name: "Peach",
        index: 9,
        air_jumps: 1,
        friction: 0.1,
        size: 14,
        gravity: 0.08,
        max_fall_speed: 1.5,
        max_walk_speed: 0.85,
        fast_fall_speed: 2.0,
        air_speed: 1.1,
        air_friction: 0.005,
        air_accel: 0.07,
        dj_x_speed: DJType::AnimationMapped,
        fh_jump_force: 2.2,
        sh_jump_force: 1.6,
        dj_force_multiplier: DJType::AnimationMapped,
        weight: 90,
    };
    pub const PICHU: Attributes<'a> = Attributes {
        name: "Pichu",
        index: 23,
        air_jumps: 1,
        friction: 0.1,
        size: 9,
        gravity: 0.11,
        max_fall_speed: 1.9,
        max_walk_speed: 1.24,
        fast_fall_speed: 2.5,
        air_speed: 0.85,
        air_friction: 0.01,
        air_accel: 0.05,
        dj_x_speed: DJType::Normal(0.8),
        fh_jump_force: 2.6,
        sh_jump_force: 1.7,
        dj_force_multiplier: DJType::Normal(1.0),
        weight: 55,
    };
    pub const PIKACHU: Attributes<'a> = Attributes {
        name: "Pikachu",
        index: 12,
        air_jumps: 1,
        friction: 0.09,
        size: 9,
        gravity: 0.11,
        max_fall_speed: 1.9,
        max_walk_speed: 1.24,
        fast_fall_speed: 2.7,
        air_speed: 0.85,
        air_friction: 0.01,
        air_accel: 0.05,
        dj_x_speed: DJType::Normal(0.8),
        fh_jump_force: 2.6,
        sh_jump_force: 1.7,
        dj_force_multiplier: DJType::Normal(1.0),
        weight: 80,
    };
    pub const ROY: Attributes<'a> = Attributes {
        name: "Roy",
        index: 26,
        air_jumps: 1,
        friction: 0.06,
        size: 14,
        gravity: 0.114,
        max_fall_speed: 2.4,
        max_walk_speed: 1.2,
        fast_fall_speed: 2.9,
        air_speed: 0.9,
        air_friction: 0.005,
        air_accel: 0.05,
        dj_x_speed: DJType::Normal(1.0),
        fh_jump_force: 2.6,
        sh_jump_force: 1.5,
        dj_force_multiplier: DJType::Normal(0.88),
        weight: 85,
    };
    pub const SAMUS: Attributes<'a> = Attributes {
        name: "Samus",
        index: 13,
        air_jumps: 1,
        friction: 0.06,
        size: 14,
        gravity: 0.066,
        max_fall_speed: 1.4,
        max_walk_speed: 1.0,
        fast_fall_speed: 2.3,
        air_speed: 0.89,
        air_friction: 0.01,
        air_accel: 0.0325,
        dj_x_speed: DJType::Normal(0.9),
        fh_jump_force: 2.1,
        sh_jump_force: 1.7,
        dj_force_multiplier: DJType::Normal(0.9),
        weight: 110,
    };
    pub const SHEIK: Attributes<'a> = Attributes {
        name: "Sheik",
        index: 7,
        air_jumps: 1,
        friction: 0.08,
        size: 11,
        gravity: 0.12,
        max_fall_speed: 2.13,
        max_walk_speed: 1.2,
        fast_fall_speed: 3.0,
        air_speed: 0.8,
        air_friction: 0.04,
        air_accel: 0.06,
        dj_x_speed: DJType::Normal(1.0),
        fh_jump_force: 2.8,
        sh_jump_force: 2.14,
        dj_force_multiplier: DJType::Normal(1.1),
        weight: 90,
    };
    pub const YOSHI: Attributes<'a> = Attributes {
        name: "Yoshi",
        index: 14,
        air_jumps: 1,
        friction: 0.06,
        size: 11,
        gravity: 0.093,
        max_fall_speed: 1.93,
        max_walk_speed: 1.15,
        fast_fall_speed: 2.93,
        air_speed: 1.2,
        air_friction: 0.013,
        air_accel: 0.048,
        dj_x_speed: DJType::AnimationMapped,
        fh_jump_force: 2.5,
        sh_jump_force: 1.8,
        dj_force_multiplier: DJType::AnimationMapped,
        weight: 108,
    };
    pub const YLINK: Attributes<'a> = Attributes {
        name: "Young Link",
        index: 20,
        air_jumps: 1,
        friction: 0.08,
        size: 10,
        gravity: 0.11,
        max_fall_speed: 2.13,
        max_walk_speed: 1.2,
        fast_fall_speed: 2.2,
        air_speed: 1.0,
        air_friction: 0.005,
        air_accel: 0.06,
        dj_x_speed: DJType::Normal(1.0),
        fh_jump_force: 2.62,
        sh_jump_force: 1.5,
        dj_force_multiplier: DJType::Normal(0.88),
        weight: 85,
    };
    pub const ZELDA: Attributes<'a> = Attributes {
        name: "Zelda",
        index: 19,
        air_jumps: 1,
        friction: 0.1,
        size: 11,
        gravity: 0.073,
        max_fall_speed: 1.4,
        max_walk_speed: 0.7,
        fast_fall_speed: 1.85,
        air_speed: 0.95,
        air_friction: 0.005,
        air_accel: 0.048,
        dj_x_speed: DJType::Normal(0.9),
        fh_jump_force: 2.1,
        sh_jump_force: 1.6,
        dj_force_multiplier: DJType::Normal(0.86),
        weight: 90,
    };
}
