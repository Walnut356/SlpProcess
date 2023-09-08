use crate::calc::attack::point_to_angle;

pub trait Coordinate: Sized {
    fn new(x: f32, y: f32) -> Self;

    fn as_angle(&self) -> f32;

    fn from_angle(val: f32) -> Self;

    fn distance_from(&self, other: Self) -> f32;
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
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
pub struct Joystick {
    pub x: f32,
    pub y: f32,
}

impl Joystick {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn with_deadzone(x: f32, y: f32) -> Self {
        use crate::enums::StickRegion as SR;
        match SR::from_coordinates(x, y) {
            SR::DEAD_ZONE => Joystick::new(0.0, 0.0),
            SR::UP | SR::DOWN => Joystick::new(0.0, y),
            SR::LEFT | SR::RIGHT => Joystick::new(x, 0.0),
            _ => Joystick::new(x, y),
        }
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

    pub fn as_angle(&self) -> f32 {
        point_to_angle(self.x, self.y)
    }
}