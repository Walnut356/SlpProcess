use nohash_hasher::IntMap;
use polars::prelude::*;
use std::collections::HashSet;
use ssbm_utils::enums::{Item, Port};

use crate::{
    columns::{Items, Post},
    events::item_frames::ItemFrames,
    player::Frames,
};

pub fn find_items(frames: &Frames, port: Port, item_frames: &ItemFrames) -> DataFrame {
    let ids = &item_frames.item_id;
    let spawn_ids = &item_frames.spawn_id;
    let missiles = item_frames.missile_type.as_ref().unwrap();
    let turnips = item_frames.turnip_type.as_ref().unwrap();
    let owners = item_frames.owner.as_ref().unwrap();

    let mut item_counter: IntMap<u16, u32> = IntMap::default();
    let mut unique: HashSet<u32> = HashSet::default();

    // compiler pls no bounds check
    assert_eq!(ids.len(), spawn_ids.len());
    assert_eq!(spawn_ids.len(), missiles.len());
    assert_eq!(missiles.len(), turnips.len());
    assert_eq!(turnips.len(), owners.len());

    for i in 0..ids.len() {
        if owners[i] != std::convert::TryInto::<i8>::try_into(port as u8).unwrap() {
            continue;
        }

        let id = ids[i];
        // if id == 0x62 {
        //     dbg!(item_frames.frame_index[i]);
        //     dbg!(item_frames.position_x[i]);
        //     dbg!(item_frames.position_y[i]);
        // }

        if unique.insert(spawn_ids[i]) {
            if let Some(x) = item_counter.get_mut(&id) {
                *x += 1;
            } else {
                item_counter.insert(id, 0);
            };
        }
    }

    let mut keys: Vec<&str> = vec![];
    let mut vals: Vec<u32> = vec![];
    for (key, val) in item_counter {
        let temp = Item::from_repr(key).unwrap_or(Item::UNKNOWN).into();
        // if temp == Item::UNKNOWN.to_string() {
        //     dbg!(key);
        // }
        keys.push(temp);
        vals.push(val);
    }

    DataFrame::new(vec![Series::new("Item", keys), Series::new("Count", vals)]).unwrap()
}
