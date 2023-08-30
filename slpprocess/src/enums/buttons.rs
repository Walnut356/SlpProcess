#![allow(non_camel_case_types)]

/// Maximum accepted analog trigger value
pub const TRIGGER_MAX: f32 = 1.0;
/// Minimum accepted analog trigger value
pub const TRIGGER_MIN: f32 = 43.0 / 140.0;
/// Analog value when holding Z
pub const Z_TRIGGER: f32 = 49.0 / 140.0;


/// The buttons as interpreted by the game engine. See `buttons::Controller` for buttons as seen by
/// the console's controller polls directly.
///
/// Can be casted trivially to and from u32
/// ```
/// # use slpprocess::enums::buttons::Engine;
/// let flags = Engine::X | Engine::A | Engine::L;
/// let val: u32 = flags.bits();
/// let back = BitFlags
///
/// Notably, the engine considers Z presses to be analog trigger 0.35 + Z + A, while the controller
/// sees Z as just Z.
///
/// `ANY_TRIGGER` is active when either L or R is active, and/or when there is an analog value
/// >= 0.30
///
#[rustfmt::skip]
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum EngineInput {
    None = 0,
    DPAD_LEFT      = 1 << 0,
    DPAD_RIGHT     = 1 << 1,
    DPAD_DOWN      = 1 << 2,
    DPAD_UP        = 1 << 3,
    /// When active, will always also be accompanied by `A` and `ANY_TRIGGER`
    /// If you're looking for Z in isolation, use `buttons::Controller`
    Z              = 1 << 4,
    /// Digital press
    R              = 1 << 5,
    /// Digital press
    L              = 1 << 6,
    // unused:       1 << 7,
    A              = 1 << 8,
    B              = 1 << 9,
    X              = 1 << 10,
    Y              = 1 << 11,
    START          = 1 << 12,
    // unused:       1 << 13,
    // unused:       1 << 14,
    // unused:       1 << 15,
    JOYSTICK_UP    = 1 << 16,
    JOYSTICK_DOWN  = 1 << 17,
    JOYSTICK_LEFT  = 1 << 18,
    JOYSTICK_RIGHT = 1 << 19,
    CSTICK_UP      = 1 << 20,
    CSTICK_DOWN    = 1 << 21,
    CSTICK_LEFT    = 1 << 22,
    CSTICK_RIGHT   = 1 << 23,
    // unused:       1 << 24,
    // unused:       1 << 25,
    // unused:       1 << 26,
    // unused:       1 << 27,
    // unused:       1 << 28,
    // unused:       1 << 29,
    // unused:       1 << 30,
    /// active when either L or R is active, and/or when there is an analog value
    /// >= 0.30
    ANY_TRIGGER    = 1 << 31,
    Raw(u32)
}





/// The buttons as seen by the console's controller poll. See `buttons::Engine` for buttons as
/// interpreted by the game engine.
///
/// Notably, the engine considers Z presses to be analog trigger 0.35 + Z + A, while the controller
/// sees Z as just Z.
///
/// `ANY_TRIGGER` is active when either L or R is active, and/or when there is an analog value
/// >= 0.30
#[rustfmt::skip]
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u16)]
pub enum ControllerInput {
    None = 0,
    DPAD_LEFT      = 1 << 0,
    DPAD_RIGHT     = 1 << 1,
    DPAD_DOWN      = 1 << 2,
    DPAD_UP        = 1 << 3,
    /// In contrast to `Engine::Z`, `Controller::Z` does not forcably trigger any other values.
    Z              = 1 << 4,
    /// Digital press
    R              = 1 << 5,
    /// Digital press
    L              = 1 << 6,
    // unused:       1 << 7,
    A              = 1 << 8,
    B              = 1 << 9,
    X              = 1 << 10,
    Y              = 1 << 11,
    START          = 1 << 12,
    // unused:       1 << 13,
    // unused:       1 << 14,
    // unused:       1 << 15,
    Raw(u16)
}
