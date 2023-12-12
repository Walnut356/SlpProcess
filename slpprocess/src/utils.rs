use std::io::Cursor;

use bytes::{Bytes, Buf};
use ssbm_utils::enums::StickRegion;
use strum_macros::{IntoStaticStr, EnumString};
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

#[derive(Debug, Copy, Clone, EnumString, IntoStaticStr)]
pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

pub struct Stream(pub Cursor<Bytes>);

impl Buf for Stream {
    fn remaining(&self) -> usize {
        let len = self.0.get_ref().len();
        let pos = self.0.position();

        if pos >= len as u64 {
            return 0;
        }

        len - pos as usize
    }

    fn chunk(&self) -> &[u8] {
        self.0.get_ref().chunk()
    }

    fn advance(&mut self, cnt: usize) {
        let pos = (self.0.position() as usize)
            .checked_add(cnt)
            .expect("overflow");

        assert!(pos <= self.0.get_ref().len());
        self.0.set_position(pos as u64);
    }

    #[inline]
    fn get_u8(&mut self) -> u8 {
        self.0.get_mut().get_u8()
    }

    #[inline]
    fn get_i8(&mut self) -> i8 {
        self.0.get_mut().get_i8()
    }

    #[inline]
    fn get_u16(&mut self) -> u16 {
        self.0.get_mut().get_u16()
    }

    #[inline]
    fn get_u16_le(&mut self) -> u16 {
        self.0.get_mut().get_u16_le()
    }

    #[inline]
    fn get_u16_ne(&mut self) -> u16 {
        self.0.get_mut().get_u16_ne()
    }

    #[inline]
    fn get_i16(&mut self) -> i16 {
        self.0.get_mut().get_i16()
    }

    #[inline]
    fn get_i16_le(&mut self) -> i16 {
        self.0.get_mut().get_i16_le()
    }

    #[inline]
    fn get_i16_ne(&mut self) -> i16 {
        self.0.get_mut().get_i16_ne()
    }

    #[inline]
    fn get_u32(&mut self) -> u32 {
        self.0.get_mut().get_u32()
    }

    #[inline]
    fn get_u32_le(&mut self) -> u32 {
        self.0.get_mut().get_u32_le()
    }

    #[inline]
    fn get_u32_ne(&mut self) -> u32 {
        self.0.get_mut().get_u32_ne()
    }

    #[inline]
    fn get_i32(&mut self) -> i32 {
        self.0.get_mut().get_i32()
    }

    #[inline]
    fn get_i32_le(&mut self) -> i32 {
        self.0.get_mut().get_i32_le()
    }

    #[inline]
    fn get_i32_ne(&mut self) -> i32 {
        self.0.get_mut().get_i32_ne()
    }

    #[inline]
    fn get_u64(&mut self) -> u64 {
        self.0.get_mut().get_u64()
    }

    #[inline]
    fn get_u64_le(&mut self) -> u64 {
        self.0.get_mut().get_u64_le()
    }

    #[inline]
    fn get_u64_ne(&mut self) -> u64 {
        self.0.get_mut().get_u64_ne()
    }

    #[inline]
    fn get_i64(&mut self) -> i64 {
        self.0.get_mut().get_i64()
    }

    #[inline]
    fn get_i64_le(&mut self) -> i64 {
        self.0.get_mut().get_i64_le()
    }

    #[inline]
    fn get_i64_ne(&mut self) -> i64 {
        self.0.get_mut().get_i64_ne()
    }
}