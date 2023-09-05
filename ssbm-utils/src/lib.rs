pub mod calc {}
pub mod enums {}
pub mod checks {}
pub mod constants {
    /// Damage is scaled by .7 when applied
    pub const SHIELD_HEALTH_MAX: f32 = 60.0;

    /// Maximum accepted analog trigger value
    pub const TRIGGER_MAX: f32 = 1.0;
    /// Minimum accepted analog trigger value
    pub const TRIGGER_MIN: f32 = 43.0 / 140.0;
    /// Analog value when holding Z
    pub const Z_TRIGGER: f32 = 49.0 / 140.0;

    /// Shield regen rate **per frame**
    pub const SHIELD_REGEN_RATE: f32 = 0.07;
}

pub mod attack;
pub mod character;
pub mod general;

/// Blast zones in clockwise order [top, right, bottom, left]
pub const BATTLEFIELD_BLASTZONES: [f32; 4] = [200.0, 224.0, -108.8, -224.0];
/// Blast zones in clockwise order [top, right, bottom, left]
pub const DREAMLAND_BLASTZONES: [f32; 4] = [250.0, 255.0, -123.0, -255.0];
/// Blast zones in clockwise order [top, right, bottom, left]
pub const FD_BLASTZONES: [f32; 4] = [188.0, 246.0, -140.0, -246.0];
/// Blast zones in clockwise order [top, right, bottom, left]
pub const FOUNTAIN_BLASTZONES: [f32; 4] = [202.5, 198.75, -146.25, -198.75];
/// Blast zones in clockwise order [top, right, bottom, left]
pub const STADIUM_BLASTZONES: [f32; 4] = [180.0, 230.0, -111.0, -230.0];
/// Blast zones in clockwise order [top, right, bottom, left]
pub const YOSHIS_BLASTZONES: [f32; 4] = [168.0, 173.6, -91.0, -175.7];

#[derive(Debug, Clone, Copy)]
pub enum BlastZone {
    Top,
    Right,
    Bottom,
    Left,
}
