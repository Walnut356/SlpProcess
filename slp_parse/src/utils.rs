use strum_macros::{EnumString, IntoStaticStr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Incorrect file type, expected '.slp', got {0}")]
    FileType(String),
    #[error("Replay must have exactly 2 human players")]
    PlayerCount,
    #[error("Expected {0}, got {1}")]
    Value(String, String),
}

#[derive(Debug, Copy, Clone, EnumString, IntoStaticStr)]
pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}