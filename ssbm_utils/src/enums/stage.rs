#![allow(non_camel_case_types)]

use anyhow::{anyhow, Error, Result};
use strum_macros::{Display, EnumString, FromRepr, IntoStaticStr};

use crate::types::{Point, Position};

#[derive(
    Debug, Copy, Clone, Default, PartialEq, Eq, EnumString, Display, FromRepr, IntoStaticStr,
)]
#[repr(u16)]
pub enum StageID {
    // Tournament
    FOUNTAIN_OF_DREAMS = 2,
    POKEMON_STADIUM = 3,
    YOSHIS_STORY = 8,
    DREAM_LAND_N64 = 28,
    #[default]
    BATTLEFIELD = 31,
    FINAL_DESTINATION = 32,

    // Casual
    PRINCESS_PEACHS_CASTLE = 4,
    KONGO_JUNGLE = 5,
    BRINSTAR = 6,
    CORNERIA = 7,
    ONETT = 9,
    MUTE_CITY = 10,
    RAINBOW_CRUISE = 11,
    JUNGLE_JAPES = 12,
    GREAT_BAY = 13,
    HYRULE_TEMPLE = 14,
    BRINSTAR_DEPTHS = 15,
    YOSHIS_ISLAND = 16,
    GREEN_GREENS = 17,
    FOURSIDE = 18,
    MUSHROOM_KINGDOM_I = 19,
    MUSHROOM_KINGDOM_II = 20,
    VENOM = 22,
    POKE_FLOATS = 23,
    BIG_BLUE = 24,
    ICICLE_MOUNTAIN = 25,
    ICETOP = 26,
    FLAT_ZONE = 27,
    YOSHIS_ISLAND_N64 = 29,
    KONGO_JUNGLE_N64 = 30,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumString, Display, FromRepr, IntoStaticStr)]
pub enum GroundID {
    UNKNOWN,
    MAIN_STAGE,
    LEFT_PLATFORM,
    TOP_PLATFORM,
    RIGHT_PLATFORM,
    LEFT_EDGE,
    RIGHT_EDGE,
    LEFT_EDGE_OUTTER,
    LEFT_EDGE_INNER,
    RIGHT_EDGE_OUTTER,
    RIGHT_EDGE_INNER,
    LEFT_SLANT,
    RIGHT_SLANT,
    RANDALL,
}

#[derive(Debug, Clone)]
pub struct Stage {
    pub id: StageID,
    pub blastzones: BlastZones,
    pub ledges: [Point; 2],
    // TODO add dimensions, properties, etc.
}

impl TryFrom<u16> for Stage {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let id = StageID::from_repr(value);
        match id {
            None => Err(anyhow!("Invalid stage value")),
            Some(StageID::YOSHIS_STORY) => Ok(Self::YOSHIS),
            Some(StageID::BATTLEFIELD) => Ok(Self::BATTLEFIELD),
            Some(StageID::FINAL_DESTINATION) => Ok(Self::FINAL_DESTINATION),
            Some(StageID::DREAM_LAND_N64) => Ok(Self::DREAMLAND),
            Some(StageID::POKEMON_STADIUM) => Ok(Self::STADIUM),
            Some(StageID::FOUNTAIN_OF_DREAMS) => Ok(Self::FOUNTAIN),
            Some(x) => Ok(Self {
                id: x,
                blastzones: BlastZones {
                    top: 999.9,
                    bottom: -999.9,
                    left: -999.9,
                    right: 999.9,
                },
                ledges: [Point::new(-999.9, 0.0), Point::new(999.9, 0.0)],
            }),
        }
    }
}

impl Stage {
    pub const YOSHIS: Stage = Stage {
        id: StageID::YOSHIS_STORY,
        blastzones: BlastZones::YOSHIS,
        ledges: [Point::new(-56.0, -3.5), Point::new(56.0, -3.5)],
    };
    pub const BATTLEFIELD: Stage = Stage {
        id: StageID::BATTLEFIELD,
        blastzones: BlastZones::BATTLEFIELD,
        ledges: [Point::new(-68.4, 0.0), Point::new(68.4, 0.0)],
    };
    pub const DREAMLAND: Stage = Stage {
        id: StageID::DREAM_LAND_N64,
        blastzones: BlastZones::DREAMLAND,
        // unlike fountain, the non-0 elevation is consistent across the entire stage
        ledges: [Point::new(-77.27, 0.01), Point::new(77.27, 0.01)],
    };
    pub const FINAL_DESTINATION: Stage = Stage {
        id: StageID::FINAL_DESTINATION,
        blastzones: BlastZones::FINAL_DESTINATION,
        ledges: [Point::new(-85.57, 0.0), Point::new(85.57, 0.0)],
    };
    pub const STADIUM: Stage = Stage {
        id: StageID::POKEMON_STADIUM,
        blastzones: BlastZones::STADIUM,
        ledges: [Point::new(-87.75, 0.0), Point::new(87.75, 0.0)],
    };
    pub const FOUNTAIN: Stage = Stage {
        id: StageID::FOUNTAIN_OF_DREAMS,
        blastzones: BlastZones::FOUNTAIN,
        // this is technically incorrect, the y value is 0.62. This value will likely be used for
        // things like `is_onstage()` which 0.0 is more useful for. The slight elevation change
        // is only present on the outter edges of the stage
        ledges: [Point::new(-63.35, 0.0), Point::new(63.35, 0.0)],
    };

    pub fn from_id(id: StageID) -> Self {
        use StageID::*;
        match id {
            YOSHIS_STORY => Self::YOSHIS,
            BATTLEFIELD => Self::BATTLEFIELD,
            FINAL_DESTINATION => Self::FINAL_DESTINATION,
            DREAM_LAND_N64 => Self::DREAMLAND,
            POKEMON_STADIUM => Self::STADIUM,
            FOUNTAIN_OF_DREAMS => Self::FOUNTAIN,
            // Maybe better to error out? Iunno
            _ => Self {
                id,
                blastzones: BlastZones {
                    top: 999.9,
                    bottom: -999.9,
                    left: -999.9,
                    right: 999.9,
                },
                ledges: [Point::new(-999.9, 0.0), Point::new(999.9, 0.0)],
            },
        }
    }

    pub fn ground_from_id(&self, id: u16) -> GroundID {
        use GroundID::*;
        use StageID::*;
        match self.id {
            YOSHIS_STORY => match id {
                0 => RANDALL,
                1 => LEFT_PLATFORM,
                2 => LEFT_SLANT,
                3 => MAIN_STAGE,
                4 => TOP_PLATFORM,
                5 => RIGHT_PLATFORM,
                6 => RIGHT_SLANT,
                _ => UNKNOWN,
            },
            BATTLEFIELD => match id {
                0 => LEFT_EDGE,
                1 => MAIN_STAGE,
                2 => LEFT_PLATFORM,
                3 => TOP_PLATFORM,
                4 => RIGHT_PLATFORM,
                5 => RIGHT_EDGE,
                _ => UNKNOWN,
            },
            FINAL_DESTINATION => match id {
                0 => LEFT_EDGE,
                1 => MAIN_STAGE,
                2 => RIGHT_EDGE,
                _ => UNKNOWN,
            },
            DREAM_LAND_N64 => match id {
                0 => LEFT_PLATFORM,
                1 => RIGHT_PLATFORM,
                2 => TOP_PLATFORM,
                3 => LEFT_EDGE,
                4 => MAIN_STAGE,
                5 => RIGHT_EDGE,
                _ => UNKNOWN,
            },
            POKEMON_STADIUM => match id {
                34 => MAIN_STAGE,
                35 => LEFT_PLATFORM,
                36 => RIGHT_PLATFORM,
                51 => LEFT_EDGE_OUTTER,
                52 => LEFT_EDGE_INNER,
                53 => RIGHT_EDGE_INNER,
                54 => RIGHT_EDGE_OUTTER,
                _ => UNKNOWN,
            },
            FOUNTAIN_OF_DREAMS => match id {
                0 => LEFT_PLATFORM,
                1 => RIGHT_PLATFORM,
                2 => TOP_PLATFORM,
                3 => LEFT_EDGE_OUTTER,
                4 => LEFT_EDGE_INNER,
                5 => MAIN_STAGE,
                6 => RIGHT_EDGE_INNER,
                7 => RIGHT_EDGE_OUTTER,
                _ => UNKNOWN,
            },
            _ => UNKNOWN,
        }
    }

    pub fn is_past_blastzone(&self, pos: Position) -> bool {
        !(pos.x < self.blastzones.right
            && pos.x > self.blastzones.left
            && pos.y < self.blastzones.top
            && pos.y > self.blastzones.bottom)
    }

    pub fn is_offstage(&self, pos: Position) -> bool {
        if pos.y < -5.0 {
            return true;
        }

        pos.x < self.ledges[0].x || pos.x > self.ledges[1].x
    }
}

#[derive(Debug, Clone)]
pub struct BlastZones {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl BlastZones {
    pub const YOSHIS: BlastZones = BlastZones {
        top: 168.0,
        bottom: -91.0,
        left: -175.7,
        right: 173.6,
    };

    pub const BATTLEFIELD: BlastZones = BlastZones {
        top: 200.0,
        bottom: -108.8,
        left: -224.0,
        right: 224.0,
    };

    pub const DREAMLAND: BlastZones = BlastZones {
        top: 250.0,
        bottom: -123.0,
        left: -255.0,
        right: 255.0,
    };

    pub const FINAL_DESTINATION: BlastZones = BlastZones {
        top: 188.0,
        bottom: -140.0,
        left: -246.0,
        right: 246.0,
    };

    pub const STADIUM: BlastZones = BlastZones {
        top: 180.0,
        bottom: -111.0,
        left: -230.0,
        right: 230.0,
    };

    pub const FOUNTAIN: BlastZones = BlastZones {
        top: 202.5,
        bottom: -146.25,
        left: -198.75,
        right: 198.75,
    };
}
