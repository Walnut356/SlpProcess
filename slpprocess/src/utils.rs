use thiserror::Error;
use polars::prelude::*;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Incorrect file type, expected '.slp', got {0}")]
    FileType(String),
    #[error("Replay must have exactly 2 human players")]
    PlayerCount,
    #[error("Expected {0}, got {1}")]
    Value(String, String),
}

pub fn downcast_u8(series: &Series) -> Result<Vec<Option<u8>>, PolarsError> {
    let chunked = series.u8()?;
    Ok(chunked.to_vec())
}