use strum_macros::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, Copy, Display, EnumString, IntoStaticStr)]
pub enum PreFrame {
    /// `i32` | All versions
    FrameIndex,
    /// `u32` | All versions
    RandomSeed,
    /// `u16` | All versions
    ActionState,
    /// `Struct(x: f32, y: f32)` | All versions
    Position,
    /// `f32` | All versions
    Orientation,
    /// `Struct(x: f32, y: f32)` | All versions
    JoystickPos,
    /// `Struct(x: f32, y: f32)` | All versions
    CstickPos,
    /// `f32` | All versions
    EngineTrigger,
    /// `u32` | All versions
    EngineButtons,
    /// `u16` | All versions
    ControllerButtons,
    /// `f32` | All versions
    ControllerL,
    /// `f32` | All versions
    ControllerR,
    /// `f32` | >= v1.4.0
    Percent,
}

#[derive(Debug, Clone, Copy, Display, EnumString, IntoStaticStr)]
pub enum PostFrame {
    /// `i32` | All Versions
    FrameIndex,
    /// `u8` | All Versions
    Character,
    /// `u16` | All Versions
    ActionState,
    /// `Struct(x: f32, y: f32)` | All Versiosn
    Position,
    /// `f32` | All Versions
    Orientation,
    /// `f32` | All Versions
    Percent,
    /// `f32` | All Versions
    ShieldHealth,
    /// `u8` | All Versions
    LastAttackLanded,
    /// `u8` | All Versions
    ComboCount,
    /// `u8` | All Versions
    LastHitBy,
    /// `u8` | All Versions
    Stocks,
    /// `f32` | >= v2.0.0
    StateFrame,
    /// `u64` | >= v2.0.0
    Flags,
    /// `f32` | >= v2.0.0
    MiscAS,
    /// `bool` | >= v2.0.0
    IsGrounded,
    /// `u16` | >= v2.0.0
    LastGroundID,
    /// `u8` | >= v2.0.0
    JumpsRemaining,
    /// `u8` | >= v2.0.0
    LCancel,
    /// `u8` | >= v2.1.0
    HurtboxState,
    /// `Struct(x: f32, y: f32)` | >= v3.5.0
    AirVel,
    /// `Struct(x:f32, y: f32)` | >= v3.5.0
    Knockback,
    /// `Struct(x: f32, y: f32)` | >= v3.5.0
    GroundVel,
    /// `f32` | >= v3.8.0
    HitlagRemaining,
    /// `u32` | >= v3.11.0
    AnimationIndex,
}

#[derive(Debug, Clone, Copy, Display, EnumString, IntoStaticStr)]
pub enum ItemFrame {
    /// `i32` | >= v3.0.0
    FrameIndex,
    /// `u16` | >= v3.0.0
    ItemID,
    /// `u8` | >= v3.0.0
    State,
    /// `f32` | >= v3.0.0
    Orientation,
    /// `Struct(x: f32, y: f32)` | >= v3.0.0
    Velocity,
    /// `Struct(x: f32, y: f32` | >= v3.0.0
    Position,
    /// `u16` | >= v3.0.0
    DamageTaken,
    /// `f32` | >= v3.0.0
    ExpirationTimer,
    /// `u32` | >= v3.0.0
    SpawnID,
    /// `u8` | >= v3.0.0
    MissileType,
    /// `u8` | >= v3.2.0
    TurnipType,
    /// `bool` | >= v3.2.0
    IsLaunched,
    /// `u8` | >= v3.2.0
    ChargePower,
    /// `i8` | >= v3.6.0
    Owner,
}

#[derive(Debug, Clone, Copy, Display, EnumString, IntoStaticStr)]
pub enum LCancelStats {
    FrameIndex,
    Attack,
    Stocks,
    Percent,
    LCancelled,
    TriggerFrame,
    Position,
    Fastfall,
    InputDuringHitlag,
}

#[derive(Debug, Clone, Copy, Display, EnumString, IntoStaticStr)]
pub enum InputStats {
    Digital,
    Joystick,
    Cstick,
    AnalogTrigger,
    APM,
    TriggerPref,
    JumpPref,
}

#[derive(Debug, Clone, Copy, Display, EnumString, IntoStaticStr)]
pub enum DefenseStats {
    FrameIndex,
    Stocks,
    Percent,
    DamageTaken,
    LastHitBy,
    StateBeforeHit,
    Grounded,
    CrouchCancel,
    VCancel,
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
    KillsSomeDI,
}

#[derive(Debug, Clone, Copy, Display, EnumString, IntoStaticStr)]
pub enum WavedashStats {
    FrameIndex,
    Waveland,
    Angle,
    Direction,
    StartPosition,
}

// #[derive(Debug, Clone, Copy, Display, EnumString, IntoStaticStr)]
// pub enum Summary {
//     MatchID,
//     GameNumber,
//     TiebreakNumber,
//     DateTime,
//     ReplayVersion,
//     Winner,
//     Player1,
//     Player2,

// }

#[derive(Debug, Clone, Copy, Display, EnumString, IntoStaticStr)]
pub enum TechStats {
    FrameIndex,
    Stocks,
    Percent,
    InputFrame,
    TechType,
    Punished,
    Position,
    Location,
    MissedTech,
    Lockout,
    TowardsCenter,
    TowardsOpnt,
    JabReset,
    LastHitBy,
    OpntDistance,
    DuringHitlag,
}
