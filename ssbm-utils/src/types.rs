use crate::enums::StickRegion;
use serde::{Deserialize, Serialize};
use std::{
    f32::consts::TAU,
    ops::{Add, AddAssign, Deref, Sub, SubAssign},
};

pub type Radians = f32;
pub type Degrees = f32;

/// Accepts a point, returns an angle in radians
pub fn point_to_angle(x: f32, y: f32) -> Radians {
    (f32::atan2(y, x) + TAU) % TAU
}

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct StickPos {
    pub x: f32,
    pub y: f32,
}

impl StickPos {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn with_deadzone(&self) -> Self {
        use crate::enums::StickRegion as SR;
        match self.as_stickregion() {
            SR::DEAD_ZONE => StickPos::new(0.0, 0.0),
            SR::UP | SR::DOWN => StickPos::new(0.0, self.y),
            SR::LEFT | SR::RIGHT => StickPos::new(self.x, 0.0),
            _ => StickPos::new(self.x, self.y),
        }
    }

    pub fn as_stickregion(&self) -> StickRegion {
        StickRegion::from_coordinates(self.x, self.y)
    }

    pub fn as_angle(&self) -> Radians {
        point_to_angle(self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn as_angle(&self) -> Radians {
        point_to_angle(self.x, self.y)
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Add<Velocity> for Position {
    type Output = Self;

    fn add(self, rhs: Velocity) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Velocity> for Position {
    fn add_assign(&mut self, rhs: Velocity) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Velocity> for Position {
    type Output = Self;

    fn sub(self, rhs: Velocity) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Velocity> for Position {
    fn sub_assign(&mut self, rhs: Velocity) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
