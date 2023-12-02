use ssbm_utils::enums::StickRegion;
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

pub(crate) fn as_vec_static_str<T: Into<&'static str>>(input: Vec<T>) -> Vec<&'static str> {
    input
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<&'static str>>()
}

pub(crate) fn as_vec_arrow(input: Vec<StickRegion>) -> Vec<&'static str> {
    input.into_iter().map(|x| x.to_utf_arrow()).collect()
}