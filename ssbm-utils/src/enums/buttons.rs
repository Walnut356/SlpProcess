#![allow(non_camel_case_types)]

use num_traits::Signed;
use num_enum::FromPrimitive;
use strum_macros::{Display, EnumString, FromRepr, IntoStaticStr};

use crate::utils::BitFlags;

/// Maximum accepted analog trigger value
pub const TRIGGER_MAX: f32 = 1.0;
/// Minimum accepted analog trigger value
pub const TRIGGER_MIN: f32 = 43.0 / 140.0;
/// Analog value when holding Z
pub const Z_TRIGGER: f32 = 49.0 / 140.0;


pub const JOYSTICK_MASK: u32 = 0xf0000;
pub const CSTICK_MASK: u32 = 0xf00000;
pub const ANYTRIGGER_MASK: u32 = 0x8000_0000;
pub const DIGITAL_TRIGGER_MASK: u32 = 0x60;

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
#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
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
#[derive(Copy, Clone, PartialEq)]
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

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, EnumString, IntoStaticStr, Display, FromRepr,
)]
#[repr(i8)]
pub enum StickRegion {
    DEAD_ZONE = -1,
    /// (-0.2875 < stick_x < 0.2875) and stick_y >= 0.2875
    UP = 0,
    /// stick_x >= 0.2875 and stick_y >= 0.2875
    UP_RIGHT = 1,
    /// stick_x >= 0.2875 and (-0.2875 < stick_y < 0.2875)
    RIGHT = 2,
    /// stick_x >= 0.2875 and stick_y <= -0.2875
    DOWN_RIGHT = 3,
    /// (-0.2875 < stick_x < 0.2875) and stick_y <= -0.2875
    DOWN = 4,
    /// stick_x <= -0.2875 and stick_y <= -0.2875
    DOWN_LEFT = 5,
    /// stick_x <= -0.2875 and (-0.2875 < stick_y < 0.2875)
    LEFT = 6,
    /// stick_x <= -0.2875 and stick_y >= 0.2875
    UP_LEFT = 7,
}

impl StickRegion {
    pub fn from_coordinates(x: f32, y: f32) -> Self {
        use StickRegion as R;

        // is this idiomatic? It's less ugly and more compact than elif chains
        match () {
            // this one goes first because i get the feeling it's most common by a lot.
            // also, since we know it's non-deadzone past the first entry, we can just do pos/neg
            // check instead of checking against exact values which reads a little easier
            _ if (-0.2875 < x && x < 0.2875) && (-0.2875 < y && y < -0.2875) => R::DEAD_ZONE,
            _ if x.is_positive() && y.is_positive() => R::UP_RIGHT,
            _ if x.is_positive() && y.is_negative() => R::DOWN_RIGHT,
            _ if x.is_negative() && y.is_negative() => R::DOWN_LEFT,
            _ if x.is_negative() && y.is_positive() => R::UP_LEFT,
            _ if y.is_positive() => R::UP,
            _ if x.is_positive() => R::RIGHT,
            _ if y.is_negative() => R::DOWN,
            _ if x.is_negative() => R::LEFT,
            _ => R::DEAD_ZONE,
        }
    }

    pub fn from_engine_bits(bits: u32) -> Self {
        use StickRegion as R;
        let masked = bits & JOYSTICK_MASK;

        if masked == 0u32 {
            return R::DEAD_ZONE;
        }

        let js_bits = EngineInput::from(masked);

        let up = js_bits.contains(EngineInput::JOYSTICK_UP.into());
        let down = js_bits.contains(EngineInput::JOYSTICK_DOWN.into());
        let left = js_bits.contains(EngineInput::JOYSTICK_LEFT.into());
        let right = js_bits.contains(EngineInput::JOYSTICK_RIGHT.into());

        match () {
            _ if up && left => R::UP_LEFT,
            _ if down && right => R::DOWN_RIGHT,
            _ if down && left => R::DOWN_LEFT,
            _ if up && left => R::UP_LEFT,
            _ if up => R::UP,
            _ if right => R::RIGHT,
            _ if down => R::DOWN,
            _ if left => R::LEFT,
            _ => panic!("Somehow failed all conditions")
        }
    }
}