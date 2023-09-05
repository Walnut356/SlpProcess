#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}


impl Point {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}