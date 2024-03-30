#![allow(clippy::uninit_vec)]

use std::sync::Arc;

use bytes::{Buf, Bytes};
use ssbm_utils::types::{Position, Velocity};

use crate::game::Metadata;

#[derive(Debug)]
pub struct ItemFrames {
    pub metadata: Arc<Metadata>,
    pub frame_index: Box<[i32]>,
    /// The ID corresponding to the type of item that this frame data is about.
    pub item_id: Box<[u16]>,
    pub state: Box<[u8]>,
    pub orientation: Box<[f32]>,
    pub velocity: Box<[Velocity]>,
    pub position: Box<[Position]>,
    pub damage_taken: Box<[u16]>,
    pub expiration_timer: Box<[f32]>,
    /// A unique ID artificially given to each projectile to help differentiate it from other items spawned
    /// during the same game.
    pub spawn_id: Box<[u32]>,
    pub missile_type: Option<Box<[u8]>>,
    pub turnip_type: Option<Box<[u8]>>,
    pub launched: Option<Box<[bool]>>,
    pub charge_power: Option<Box<[u8]>>,
    pub owner: Option<Box<[i8]>>,
    pub instance_id: Option<Box<[u16]>>,
}

impl ItemFrames {
    pub fn new(len: usize, metadata: Arc<Metadata>) -> Self {
        let version = metadata.version;
        ItemFrames {
            metadata,
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
            velocity: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            position: unsafe {
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
            launched: unsafe {
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
            instance_id: unsafe {
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

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.frame_index.len()
    }
}

pub fn parse_itemframes(
    mut stream: Bytes,
    metadata: Arc<Metadata>,
    offsets: &[usize],
) -> ItemFrames {
    let version = metadata.version;
    let mut working = ItemFrames::new(offsets.len(), metadata);

    let file_length = stream.len();

    for (i, &offset) in offsets.iter().enumerate() {
        stream.advance(offset - (file_length - stream.len()));
        unsafe {
            *working.frame_index.get_unchecked_mut(i) = stream.get_i32();
            *working.item_id.get_unchecked_mut(i) = stream.get_u16();
            *working.state.get_unchecked_mut(i) = stream.get_u8();
            *working.orientation.get_unchecked_mut(i) = stream.get_f32();
            *working.velocity.get_unchecked_mut(i) =
                Velocity::new(stream.get_f32(), stream.get_f32());
            *working.position.get_unchecked_mut(i) =
                Position::new(stream.get_f32(), stream.get_f32());
            *working.damage_taken.get_unchecked_mut(i) = stream.get_u16();
            *working.expiration_timer.get_unchecked_mut(i) = stream.get_f32();
            *working.spawn_id.get_unchecked_mut(i) = stream.get_u32();

            if !version.at_least(3, 2, 0) {
                continue;
            }
            *working.missile_type.as_mut().unwrap().get_unchecked_mut(i) = stream.get_u8();
            *working.turnip_type.as_mut().unwrap().get_unchecked_mut(i) = stream.get_u8();
            *working.launched.as_mut().unwrap().get_unchecked_mut(i) = stream.get_u8() != 0;
            *working.charge_power.as_mut().unwrap().get_unchecked_mut(i) = stream.get_u8();

            if !version.at_least(3, 6, 0) {
                continue;
            }
            *working.owner.as_mut().unwrap().get_unchecked_mut(i) = stream.get_i8();

            if !version.at_least(3, 16, 0) {
                continue;
            }
            *working.instance_id.as_mut().unwrap().get_unchecked_mut(i) = stream.get_u16();
        }
    }

    working
}
