#![allow(non_upper_case_globals)]

use bytes::{Buf, Bytes};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum EndMethod {
    Unresolved,
    Timeout,
    Stocks,
    Resolved,
    NoContest = 7,
}

#[derive(Debug, Clone)]
pub struct GameEnd {
    pub end_method: EndMethod,
    pub lras_initiator: Option<i8>,
    pub placements: Option<[i8; 4]>,
}

pub fn parse_gameend(mut raw: Bytes) -> GameEnd {
    let end_method = EndMethod::try_from(raw.get_u8()).unwrap();
    let mut lras_initiator = None;
    let mut placements = None;

    if raw.has_remaining() {
        let temp = raw.get_i8();
        if temp != -1 {
            lras_initiator = Some(temp);
        }
    }

    if raw.has_remaining() {
        placements = Some([raw.get_i8(), raw.get_i8(), raw.get_i8(), raw.get_i8()]);
    }

    GameEnd {
        end_method,
        lras_initiator,
        placements,
    }
}
