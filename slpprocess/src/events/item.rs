#![allow(clippy::uninit_vec)]

use bytes::{Buf, Bytes};
use polars::prelude::*;

pub struct Items {
    frame_number: Box<[i32]>,
    item_id: Box<[u16]>,
    state: Box<[u8]>,
    orientation: Box<[f32]>,
    velocity_x: Box<[f32]>,
    velocity_y: Box<[f32]>,
    position_x: Box<[f32]>,
    position_y: Box<[f32]>,
    damage_taken: Box<[u16]>,
    expiration_timer: Box<[f32]>,
    spawn_id: Box<[u32]>,
    missile_type: Box<[Option<u8>]>,
    turnip_type: Box<[Option<u8>]>,
    is_launched: Box<[Option<bool>]>,
    charge_power: Box<[Option<u8>]>,
    owner: Box<[Option<i8>]>,
}

impl Items {
    pub fn new(len: usize) -> Self {
        Items {
            frame_number: unsafe {
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
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            turnip_type: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            is_launched: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            charge_power: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            owner: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<DataFrame> for Items {
    fn into(self) -> DataFrame {
        let vec_series = vec![
            Series::new("frame number", self.frame_number),
            Series::new("item id", self.item_id),
            Series::new("state", self.state),
            Series::new("facing", self.orientation),
            Series::new("velocity x", self.velocity_x),
            Series::new("velocity y", self.velocity_y),
            Series::new("position x", self.position_x),
            Series::new("position y", self.position_y),
            Series::new("damage taken", self.damage_taken),
            Series::new("expiration timer", self.expiration_timer),
            Series::new("spawn id", self.spawn_id),
            Series::new("missile type", self.missile_type),
            Series::new("turnip type", self.turnip_type),
            Series::new("is launched", self.is_launched),
            Series::new("charge power", self.charge_power),
            Series::new("owner", self.owner),
        ];

        DataFrame::new(vec_series).unwrap()
    }
}

pub fn parse_itemframes(frames: &mut [Bytes]) -> DataFrame {
    let mut working = Items::new(frames.len());

    for (i, frame) in frames.iter_mut().enumerate() {
        unsafe {
            *working.frame_number.get_unchecked_mut(i) = frame.get_i32();
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
            *working.missile_type.get_unchecked_mut(i) = Some(frame.get_u8());
            *working.turnip_type.get_unchecked_mut(i) = Some(frame.get_u8());
            *working.is_launched.get_unchecked_mut(i) = Some(frame.get_u8() != 0);
            *working.charge_power.get_unchecked_mut(i) = Some(frame.get_u8());

            if !frame.has_remaining() {
                // version < 3.6.0
                continue;
            }
            *working.owner.get_unchecked_mut(i) = Some(frame.get_i8());
        }
    }

    working.into()
}
