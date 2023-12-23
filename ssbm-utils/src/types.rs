use crate::enums::StickRegion;
use serde::{Deserialize, Serialize};
use std::{
    f32::consts::TAU,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

/// Constructor macro for Velocity structs. Accepts 2 values that can be `as f32` casted.
#[macro_export]
macro_rules! vel {
    ($x:expr, $y:expr) => {
        Velocity {
            x: $x as f32,
            y: $y as f32,
        }
    };
}

/// Constructor macro for Position structs. Accepts 2 values that can be `as f32` casted.
#[macro_export]
macro_rules! pos {
    ($x:expr, $y:expr) => {
        Position {
            x: $x as f32,
            y: $y as f32,
        }
    };
}

/// Constructor macro for StickPosition structs. Accepts 2 values that can be `as f32` casted.
#[macro_export]
macro_rules! stick_pos {
    ($x:expr, $y:expr) => {
        StickPos {
            x: $x as f32,
            y: $y as f32,
        }
    };
}

pub type Radians = f32;
pub type Degrees = f32;

#[inline]
fn float_eq(lhs: f32, rhs: f32) -> bool {
    (lhs - rhs).abs() <= 0.000_1
}

/// Accepts a point, returns an angle in radians
#[inline]
pub fn point_to_angle(x: f32, y: f32) -> Radians {
    (f32::atan2(y, x) + TAU) % TAU
}

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point(x: {}, y: {})", self.x, self.y)
    }
}

impl PartialEq for Point {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        float_eq(self.x, other.x) && float_eq(self.y, other.y)
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct StickPos {
    pub x: f32,
    pub y: f32,
}

impl StickPos {
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn with_deadzone(self) -> Self {
        use crate::enums::StickRegion as SR;
        match self.as_stickregion() {
            SR::DEAD_ZONE => StickPos::new(0.0, 0.0),
            SR::UP | SR::DOWN => StickPos::new(0.0, self.y),
            SR::LEFT | SR::RIGHT => StickPos::new(self.x, 0.0),
            _ => StickPos::new(self.x, self.y),
        }
    }

    #[inline]
    pub fn as_stickregion(&self) -> StickRegion {
        StickRegion::from_coordinates(self.x, self.y)
    }

    #[inline]
    pub fn as_angle(&self) -> Radians {
        point_to_angle(self.x, self.y)
    }
}

impl std::fmt::Display for StickPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StickPos(x: {}, y: {})", self.x, self.y)
    }
}

impl PartialEq for StickPos {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        float_eq(self.x, other.x) && float_eq(self.y, other.y)
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn as_angle(&self) -> Radians {
        point_to_angle(self.x, self.y)
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
}

impl std::fmt::Display for Velocity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vel(x: {}, y: {})", self.x, self.y)
    }
}

impl PartialEq for Velocity {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        float_eq(self.x, other.x) && float_eq(self.y, other.y)
    }
}

impl Add<Velocity> for Velocity {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Velocity) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Velocity> for Velocity {
    #[inline]
    fn add_assign(&mut self, rhs: Velocity) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Velocity> for Velocity {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Velocity) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Velocity> for Velocity {
    #[inline]
    fn sub_assign(&mut self, rhs: Velocity) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f32> for Velocity {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<u32> for Velocity {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn distance(&self, other: Position) -> f32 {
        f32::hypot(self.x - other.x, self.y - other.y)
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pos(x: {}, y: {})", self.x, self.y)
    }
}

impl PartialEq for Position {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        float_eq(self.x, other.x) && float_eq(self.y, other.y)
    }
}

impl Add<Velocity> for Position {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Velocity) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Velocity> for Position {
    #[inline]
    fn add_assign(&mut self, rhs: Velocity) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Velocity> for Position {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Velocity) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Velocity> for Position {
    #[inline]
    fn sub_assign(&mut self, rhs: Velocity) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
