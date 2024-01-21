#![allow(clippy::uninit_vec)]

use bytes::{Buf, Bytes};
use polars::prelude::*;
use ssbm_utils::types::{Position, Velocity};

use crate::events::game_start::Version;

#[derive(Debug)]
pub struct ItemFrames {
    len: usize,
    version: Version,
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
        self.len
    }
}

#[allow(clippy::from_over_into)]
impl From<ItemFrames> for DataFrame {
    fn from(val: ItemFrames) -> DataFrame {
        let len = val.len();

        use crate::columns::ItemFrame as col;
        let mut vec_series = vec![
            Series::new(col::FrameIndex.into(), val.frame_index),
            Series::new(col::ItemID.into(), val.item_id),
            Series::new(col::State.into(), val.state),
            Series::new(col::Orientation.into(), val.orientation),
            StructChunked::new(
                col::Velocity.into(),
                &[
                    Series::new("x", val.velocity.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.velocity.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),
            StructChunked::new(
                col::Position.into(),
                &[
                    Series::new("x", val.position.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.position.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),
            Series::new(col::DamageTaken.into(), val.damage_taken),
            Series::new(col::ExpirationTimer.into(), val.expiration_timer),
            Series::new(col::SpawnID.into(), val.spawn_id),
        ];

        if val.version.at_least(3, 2, 0) {
            vec_series.push(Series::new(
                col::MissileType.into(),
                val.missile_type.unwrap(),
            ));
            vec_series.push(Series::new(
                col::TurnipType.into(),
                val.turnip_type.unwrap(),
            ));
            vec_series.push(Series::new(col::Launched.into(), val.launched.unwrap()));
            vec_series.push(Series::new(
                col::ChargePower.into(),
                val.charge_power.unwrap(),
            ));
        } else {
            vec_series.push(Series::new_null(col::MissileType.into(), len));
            vec_series.push(Series::new_null(col::TurnipType.into(), len));
            vec_series.push(Series::new_null(col::Launched.into(), len));
            vec_series.push(Series::new_null(col::ChargePower.into(), len));
        }

        if val.version.at_least(3, 6, 0) {
            vec_series.push(Series::new(col::Owner.into(), val.owner.unwrap()));
        } else {
            vec_series.push(Series::new_null(col::Owner.into(), len));
        }

        if val.version.at_least(3, 16, 0) {
            vec_series.push(Series::new(
                col::InstanceID.into(),
                val.instance_id.unwrap(),
            ));
        } else {
            vec_series.push(Series::new_null(col::InstanceID.into(), len));
        }

        DataFrame::new(vec_series).unwrap()
    }
}

pub fn parse_itemframes(
    mut stream: Bytes,
    version: Version,
    offsets: &[usize],
) -> ItemFrames {
    let mut working = ItemFrames::new(offsets.len(), version);

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
