#![allow(clippy::uninit_vec)]

use bytes::{Buf, Bytes};
use polars::prelude::*;

use crate::events::game_start::Version;

pub struct ItemFrames {
    len: usize,
    pub version: Version,
    pub frame_index: Box<[i32]>,
    /// The ID corresponding to the type of item that this frame data is about.
    pub item_id: Box<[u16]>,
    pub state: Box<[u8]>,
    pub orientation: Box<[f32]>,
    pub velocity_x: Box<[f32]>,
    pub velocity_y: Box<[f32]>,
    pub position_x: Box<[f32]>,
    pub position_y: Box<[f32]>,
    pub damage_taken: Box<[u16]>,
    pub expiration_timer: Box<[f32]>,
    /// A unique ID artificially given to each projectile to help differentiate it from other items spawned
    /// during the same game.
    pub spawn_id: Box<[u32]>,
    pub missile_type: Option<Box<[u8]>>,
    pub turnip_type: Option<Box<[u8]>>,
    pub is_launched: Option<Box<[bool]>>,
    pub charge_power: Option<Box<[u8]>>,
    pub owner: Option<Box<[i8]>>,
}

impl ItemFrames {
    pub fn new(len: usize, version: Version) -> Self {
        ItemFrames {
            len,
            version,
            frame_index: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            item_id: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            state: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            orientation: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            velocity_x: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            velocity_y: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            position_x: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            position_y: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            damage_taken: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            expiration_timer: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            spawn_id: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            missile_type: unsafe {
                if version.at_least(3, 2, 0) {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                    Some(temp.into_boxed_slice())
                } else {
                    None
                }
            },
            turnip_type: unsafe {
                if version.at_least(3, 2, 0) {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                    Some(temp.into_boxed_slice())
                } else {
                    None
                }
            },
            is_launched: unsafe {
                if version.at_least(3, 2, 0) {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                    Some(temp.into_boxed_slice())
                } else {
                    None
                }
            },
            charge_power: unsafe {
                if version.at_least(3, 2, 0) {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                    Some(temp.into_boxed_slice())
                } else {
                    None
                }
            },
            owner: unsafe {
                if version.at_least(3, 6, 0) {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                    Some(temp.into_boxed_slice())
                } else {
                    None
                }
            },
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[allow(clippy::from_over_into)]
impl From<ItemFrames> for DataFrame {
    fn from(val: ItemFrames) -> DataFrame {
        let len = val.len();

        let mut vec_series = vec![
            Series::new("frame number", val.frame_index),
            Series::new("item id", val.item_id),
            Series::new("state", val.state),
            Series::new("facing", val.orientation),
            Series::new("velocity x", val.velocity_x),
            Series::new("velocity y", val.velocity_y),
            Series::new("position x", val.position_x),
            Series::new("position y", val.position_y),
            Series::new("damage taken", val.damage_taken),
            Series::new("expiration timer", val.expiration_timer),
            Series::new("spawn id", val.spawn_id),
        ];

        if val.version.at_least(3, 2, 0) {
            vec_series.push(Series::new("missile type", val.missile_type.unwrap()));
            vec_series.push(Series::new("turnip type", val.turnip_type.unwrap()));
            vec_series.push(Series::new("is launched", val.is_launched.unwrap()));
            vec_series.push(Series::new("charge power", val.charge_power.unwrap()));
        } else {
            vec_series.push(Series::new_null("missile type", len));
            vec_series.push(Series::new_null("turnip type", len));
            vec_series.push(Series::new_null("is launched", len));
            vec_series.push(Series::new_null("charge power", len));
        }

        if val.version.at_least(3, 6, 0) {
            vec_series.push(Series::new("owner", val.owner.unwrap()));
        } else {
            vec_series.push(Series::new_null("owner", len));
        }

        DataFrame::new(vec_series).unwrap()
    }
}

pub fn parse_itemframes(version: Version, frames: &mut [Bytes]) -> ItemFrames {
    let mut working = ItemFrames::new(frames.len(), version);

    for (i, frame) in frames.iter_mut().enumerate() {
        unsafe {
            *working.frame_index.get_unchecked_mut(i) = frame.get_i32();
            *working.item_id.get_unchecked_mut(i) = frame.get_u16();
            *working.state.get_unchecked_mut(i) = frame.get_u8();
            *working.orientation.get_unchecked_mut(i) = frame.get_f32();
            *working.velocity_x.get_unchecked_mut(i) = frame.get_f32();
            *working.velocity_y.get_unchecked_mut(i) = frame.get_f32();
            *working.position_x.get_unchecked_mut(i) = frame.get_f32();
            *working.position_y.get_unchecked_mut(i) = frame.get_f32();
            *working.damage_taken.get_unchecked_mut(i) = frame.get_u16();
            *working.expiration_timer.get_unchecked_mut(i) = frame.get_f32();
            *working.spawn_id.get_unchecked_mut(i) = frame.get_u32();

            if !frame.has_remaining() {
                // version < 3.2.0
                continue;
            }
            *working.missile_type.as_mut().unwrap().get_unchecked_mut(i) = frame.get_u8();
            *working.turnip_type.as_mut().unwrap().get_unchecked_mut(i) = frame.get_u8();
            *working.is_launched.as_mut().unwrap().get_unchecked_mut(i) = frame.get_u8() != 0;
            *working.charge_power.as_mut().unwrap().get_unchecked_mut(i) = frame.get_u8();

            if !frame.has_remaining() {
                // version < 3.6.0
                continue;
            }
            *working.owner.as_mut().unwrap().get_unchecked_mut(i) = frame.get_i8();
        }
    }

    working
}
