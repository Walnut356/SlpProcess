//! # SSBM Utils
//!
//! A crate for interacting with data from Super Smash Bros. Melee. Contains enums, helper types,
//! and various functions to replicate in-game calculations
//!
//!

pub mod trackers;
pub mod types;
pub mod calc {
    pub mod on_hit;
    pub use on_hit::*;
    mod general;
    pub use general::*;
    mod knockback;
    pub use knockback::*;
}
pub mod enums {
    pub mod attack;
    pub use attack::Attack;

    mod bitflag_impl;
    pub use bitflag_impl::BitFlags;
    pub use bitflag_impl::Buttons;
    pub mod buttons;
    pub use buttons::{ControllerInput, EngineInput, StickRegion};

    pub mod character;
    pub use character::{Character, Costume};

    mod general;
    pub use general::*;

    mod item;
    pub use item::*;

    pub mod stage;
    pub use stage::StageID;

    mod state;
    pub use state::*;
}
pub mod checks;
pub mod constants {
    use std::f32::consts::PI;

    /// Damage is scaled by .7 when applied
    pub const SHIELD_HEALTH_MAX: f32 = 60.0;

    /// Maximum accepted analog trigger value
    pub const TRIGGER_MAX: f32 = 1.0;
    /// Minimum accepted analog trigger value
    pub const TRIGGER_MIN: f32 = 43.0 / 140.0;
    /// Analog value when holding Z
    pub const Z_ANALOG: f32 = 49.0 / 140.0;

    /// Shield regen rate **per frame**
    pub const SHIELD_REGEN_RATE: f32 = 0.07;

    /// The rate at which knockback velocity decays per frame. Can be split into X and Y components
    /// with a velocity and trig functions.
    ///
    /// See also: `ssbm_utils::calc::attack::get_horizontal_decay()` and
    /// `ssbm_utils::calc::attack::get_vertical_decay()`
    pub const KB_DECAY: f32 = 0.051;

    /// The minimum value at which knockback will tumble/knock down
    pub const TUMBLE_THRESHOLD: f32 = 80.0;

    pub const FIRST_FRAME_INDEX: i32 = -123;

    /// The max number of frames that will be present in a tournament match that ends in a time out
    pub const MAX_TOURNAMENT_GAME_LENGTH: usize = 28924;

    /// The max amount that DI can change a knockback angle by, in degrees
    pub const DI_MAX_DEGREES: f32 = 18.0;

    // Some of these get a little dumb because floating point functions aren't const =)

    /// The max amount that DI can change a knockback angle by, in radians
    pub const DI_MAX_RADS: f32 = 18.0 * (PI / 180.0);

    /// The magnitude of the change in position per ASDI
    pub const ASDI_DIST: f32 = 3.0;
    /// The magintude of the change in position per SDI input
    pub const SDI_DIST: f32 = 6.0;
}

/// Converts a melee frame index (i32 starting at -123) to a real-time frame index (usize starting at 0)
#[macro_export]
macro_rules! mf {
    ($frames:expr) => {
        TryInto::<usize>::try_into($frames + 123).unwrap()
    };
}

pub mod prelude {
    pub use crate::calc::*;
    pub use crate::constants::*;
    pub use crate::enums::*;
    pub use crate::mf;
    pub use crate::types::*;
    pub use crate::pos;
    pub use crate::vel;
    pub use crate::stick_pos;
}
