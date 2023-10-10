use strum_macros::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, Copy, Display, EnumString, IntoStaticStr)]
pub enum Pre {
    /// All versions
    ///
    /// i32
    FrameIndex,
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
    /// Position{x: f32, y: f32)
    Position,
    /// All versions
    ///
    /// f32
    Orientation,
    /// All versions
    ///
    /// f32
    JoystickPos,
    /// All versions
    ///
    /// f32
    CstickPos,
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
    FrameIndex,
    Character,
    ActionState,
    Position,
    Orientation,
    Percent,
    ShieldHealth,
    LastAttackLanded,
    ComboCount,
    LastHitBy,
    Stocks,
    StateFrame,
    Flags,
    MiscAS,
    IsGrounded,
    LastGroundID,
    JumpsRemaining,
    LCancel,
    HurtboxState,
    AirVel,
    Knockback,
    GroundVel,
    HitlagRemaining,
    AnimationIndex,
}

#[derive(Debug, Clone, Copy, Display, EnumString, IntoStaticStr)]
pub enum Items {
    FrameIndex,
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

#[derive(Debug, Clone, Copy, Display, EnumString, IntoStaticStr)]
pub enum Defense {
    FrameIndex,
    StocksRemaining,
    Percent,
    DamageTaken,
    LastHitBy,
    StateBeforeHit,
    Grounded,
    CrouchCancel,
    HitlagFrames,
    StickDuringHitlag,
    SDIInputs,
    ASDI,
    Knockback,
    KBAngle,
    DIStick,
    DIKnockback,
    DIKBAngle,
    DIEfficacy,
    HitlagStart,
    HitlagEnd,
    KillsWithDI,
    KillsNoDI,
    KillsAllDI,
}
