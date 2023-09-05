pub mod utils;
pub mod calc {
    pub mod attack;
    pub mod general;
}
pub mod enums {
    pub mod attack;
    pub use attack::Attack;

    mod bitflag_impl;
    pub use bitflag_impl::BitFlags;
    pub mod buttons;
    pub use buttons::{EngineInput, ControllerInput, StickRegion};

    pub mod character;
    pub use character::Character;

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
}
