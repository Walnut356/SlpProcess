use strum_macros::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, Copy, Display, EnumString, IntoStaticStr)]
pub enum Pre {
    /// All versions
    ///
    /// i32
    FrameNumber,
    /// All versions
    ///
    /// u32
    RandomSeed,
    /// All versions
    ///
    /// u16
    ActionState,
    /// All versions
    ///
    /// f32
    PositionX,
    /// All versions
    ///
    /// f32
    PositionY,
    /// All versions
    ///
    /// f32
    Orientation,
    /// All versions
    ///
    /// f32
    JoystickX,
    /// All versions
    ///
    /// f32
    JoystickY,
    /// All versions
    ///
    /// f32
    CstickX,
    /// All versions
    ///
    /// f32
    CstickY,
    /// All versions
    ///
    /// f32
    EngineTrigger,
    /// All versions
    ///
    /// u32
    EngineButtons,
    /// All versions
    ///
    /// u16
    ControllerButtons,
    /// All versions
    ///
    /// f32
    ControllerL,
    /// All versions
    ///
    /// f32
    ControllerR,
    /// v1.4.0 or later
    ///
    /// f32
    Percent,
}

#[derive(Debug, Clone, Copy, Display, EnumString, IntoStaticStr)]
pub enum Post {
    FrameNumber,
    Character,
    ActionState,
    PositionX,
    PositionY,
    Orientation,
    Percent,
    ShieldHealth,
    LastAttackLanded,
    ComboCount,
    LastHitBy,
    Stocks,
    StateFrame,
    Flags1,
    Flags2,
    Flags3,
    Flags4,
    Flags5,
    MiscAS,
    IsGrounded,
    LastGroundID,
    JumpsRemaining,
    LCancel,
    HurtboxState,
    AirVelX,
    VelY,
    KnockbackX,
    KnockbackY,
    GroundVelX,
    HitlagRemaining,
    AnimationIndex,
}

#[derive(Debug, Clone, Copy, Display, EnumString, IntoStaticStr)]
pub enum Items {
    FrameNumber,
    ItemID,
    State,
    Orientation,
    VelX,
    VelY,
    PositionX,
    PositionY,
    DamageTaken,
    ExpirationTimer,
    SpawnID,
    MissileType,
    TurnipType,
    IsLaunched,
    ChargePower,
    Owner,
}

#[derive(Debug, Clone, Copy, Display, EnumString, IntoStaticStr)]
pub enum LCancels {
    FrameIndex,
    Attack,
    StocksRemaining,
    Percent,
    LCancelled,
    TriggerFrame,
    Position,
    Fastfall,
    InputDuringHitlag,
}

#[derive(Debug, Clone, Copy, Display, EnumString, IntoStaticStr)]
pub enum Inputs {
    Digital,
    Joystick,
    Cstick,
    AnalogTrigger,
    APM,
    TriggerPref,
    JumpPref,
}
