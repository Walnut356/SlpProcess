#![allow(non_camel_case_types)]

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Copy, Clone, TryFromPrimitive, IntoPrimitive, PartialEq)]
#[repr(u16)]
pub enum Stage {
    // Tournament
    FOUNTAIN_OF_DREAMS = 2,
    POKEMON_STADIUM = 3,
    YOSHIS_STORY = 8,
    DREAM_LAND_N64 = 28,
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
