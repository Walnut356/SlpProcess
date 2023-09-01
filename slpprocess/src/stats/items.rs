use nohash_hasher::IntMap;
use polars::prelude::*;
use std::collections::HashSet;

use crate::{
    columns::{Items, Post},
    enums::item::Item,
    events::item_frames::ItemFrames,
    player::Frames,
    Port,
};

pub fn find_items(frames: &Frames, port: Port, item_frames: &ItemFrames) -> DataFrame {
    let ids = &item_frames.item_id;
    let spawn_ids = &item_frames.spawn_id;
    let missiles = &item_frames.missile_type;
    let turnips = &item_frames.turnip_type;
    let owners = &item_frames.owner;

    let mut item_counter: IntMap<u16, u32> = IntMap::default();
    let mut unique: HashSet<u32> = HashSet::default();

    // compiler pls no bounds check
    assert_eq!(ids.len(), spawn_ids.len());
    assert_eq!(spawn_ids.len(), missiles.len());
    assert_eq!(missiles.len(), turnips.len());
    assert_eq!(turnips.len(), owners.len());

    for i in 0..ids.len() {
        if owners[i] != Some((port as u8).try_into().unwrap()) {
            continue;
        }

        let id = ids[i];

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
        keys.push(Item::from_repr(key).unwrap().into());
        vals.push(val);
    }

    DataFrame::new(vec![Series::new("Item", keys), Series::new("Count", vals)]).unwrap()
}