#![allow(non_upper_case_globals)]

use std::collections::HashMap;

use bytes::{Buf, Bytes};
use ssbm_utils::prelude::Port;
use strum_macros::FromRepr;

#[derive(Debug, Clone, PartialEq, Eq, FromRepr, Copy)]
#[repr(u8)]
pub enum EndMethod {
    Unresolved,
    Timeout,
    Stocks,
    Resolved,
    /// LRAS
    NoContest = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromRepr)]
#[repr(i8)]
pub enum Placement {
    Win,
    Loss,
}

#[derive(Debug, Clone, PartialEq, Eq,)]
pub struct GameEnd {
    pub end_method: EndMethod,
    pub lras_initiator: Option<Port>,
    pub placements: Option<HashMap<Port, Placement>>,
}

pub fn parse_gameend(mut raw: Bytes) -> GameEnd {
    let end_method = EndMethod::from_repr(raw.get_u8()).unwrap();
    let mut lras_initiator = None;
    let mut placements: Option<HashMap<Port, Placement>> = None;

    if raw.has_remaining() {
        lras_initiator = Port::try_from(raw.get_i8()).ok();
    }

    if raw.has_remaining() {
        let mut map = HashMap::new();
        let temp = [raw.get_i8(), raw.get_i8(), raw.get_i8(), raw.get_i8()];
        for (i, v) in temp.iter().enumerate() {
            if *v == -1 {
                continue;
            }

            map.insert(
                Port::from_repr(i as u8).unwrap(),
                Placement::from_repr(*v).unwrap(),
            );
        }
        placements = Some(map);
    }

    GameEnd {
        end_method,
        lras_initiator,
        placements,
    }
}
