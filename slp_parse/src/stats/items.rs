use polars::prelude::*;
use ssbm_utils::enums::{Item, MissileType, Port, TurnipFace};
use std::collections::{HashMap, HashSet};

use crate::events::item_frames::ItemFrames;

pub fn find_items(port: Port, item_frames: &ItemFrames) -> DataFrame {
    let ids = &item_frames.item_id;
    let spawn_ids = &item_frames.spawn_id;
    let missiles = item_frames.missile_type.as_ref().unwrap();
    let turnips = item_frames.turnip_type.as_ref().unwrap();
    let owners = item_frames.owner.as_ref().unwrap();

    let mut item_counter: HashMap<(u16, u8), u32> = HashMap::default();
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

        if unique.insert(spawn_ids[i]) {
            let t = match Item::from_repr(id).unwrap_or(Item::UNKNOWN) {
                Item::PEACH_TURNIP => turnips[i],
                Item::SAMUS_MISSILE => missiles[i],
                _ => 0,
            };

            if let Some(x) = item_counter.get_mut(&(id, t)) {
                *x += 1;
            } else {
                item_counter.insert((id, t), 1);
            };
        }
    }

    let mut keys: Vec<&str> = vec![];
    let mut vals: Vec<u32> = vec![];
    for ((item, itype), count) in item_counter {
        let item = Item::from_repr(item).unwrap_or(Item::UNKNOWN);
        // if temp == Item::UNKNOWN.to_string() {
        //     dbg!(key);
        // }
        let temp: &str = match item {
            Item::PEACH_TURNIP => TurnipFace::from_repr(itype).unwrap().into(),
            Item::SAMUS_MISSILE => MissileType::from_repr(itype).unwrap().into(),
            _ => item.into(),
        };
        keys.push(temp);
        vals.push(count);
    }

    DataFrame::new(vec![Series::new("Item", keys), Series::new("Count", vals)]).unwrap()
}
