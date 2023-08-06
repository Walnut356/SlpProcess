use bytes::{Buf, Bytes};
use polars::prelude::*;

pub struct Items {
    frame_number: Vec<i32>,
    item_id: Vec<u16>,
    state: Vec<u8>,
    facing: Vec<f32>,
    velocity_x: Vec<f32>,
    velocity_y: Vec<f32>,
    position_x: Vec<f32>,
    position_y: Vec<f32>,
    damage_taken: Vec<u16>,
    expiration_timer: Vec<f32>,
    spawn_id: Vec<u32>,
    missile_type: Vec<Option<u8>>,
    turnip_type: Vec<Option<u8>>,
    is_launched: Vec<Option<bool>>,
    charge_power: Vec<Option<u8>>,
    owner: Vec<Option<i8>>,
}

impl Items {
    pub fn new(len: usize) -> Self {
        Items {
            frame_number: Vec::with_capacity(len),
            item_id: Vec::with_capacity(len),
            state: Vec::with_capacity(len),
            facing: Vec::with_capacity(len),
            velocity_x: Vec::with_capacity(len),
            velocity_y: Vec::with_capacity(len),
            position_x: Vec::with_capacity(len),
            position_y: Vec::with_capacity(len),
            damage_taken: Vec::with_capacity(len),
            expiration_timer: Vec::with_capacity(len),
            spawn_id: Vec::with_capacity(len),
            missile_type: Vec::with_capacity(len),
            turnip_type: Vec::with_capacity(len),
            is_launched: Vec::with_capacity(len),
            charge_power: Vec::with_capacity(len),
            owner: Vec::with_capacity(len),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<DataFrame> for Items {
    fn into(mut self) -> DataFrame {
        let len = self.frame_number.len();

        // handle potentially optional values
        if self.missile_type.len() != len {
            self.missile_type.resize(len, None);
            self.turnip_type.resize(len, None);
            self.is_launched.resize(len, None);
            self.charge_power.resize(len, None);
        }

        let vec_series = vec![
            Series::new("frame number", self.frame_number),
            Series::new("item id", self.item_id),
            Series::new("state", self.state),
            Series::new("facing", self.facing),
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

    for frame in frames {
        working.frame_number.push(frame.get_i32());
        working.item_id.push(frame.get_u16());
        working.state.push(frame.get_u8());
        working.facing.push(frame.get_f32());
        working.velocity_x.push(frame.get_f32());
        working.velocity_y.push(frame.get_f32());
        working.position_x.push(frame.get_f32());
        working.position_y.push(frame.get_f32());
        working.damage_taken.push(frame.get_u16());
        working.expiration_timer.push(frame.get_f32());
        working.spawn_id.push(frame.get_u32());

        if !frame.has_remaining() {
            // version < 3.2.0
            continue;
        }
        working.missile_type.push(Some(frame.get_u8()));
        working.turnip_type.push(Some(frame.get_u8()));
        working.is_launched.push(Some(frame.get_u8() != 0));
        working.charge_power.push(Some(frame.get_u8()));

        if !frame.has_remaining() {
            // version < 3.6.0
            continue;
        }
        working.owner.push(Some(frame.get_i8()));
    }

    working.into()
}
