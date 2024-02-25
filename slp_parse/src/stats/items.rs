use ssbm_utils::enums::{Item, Port};
use std::collections::{HashMap, HashSet};

use crate::events::item_frames::ItemFrames;

#[derive(Debug, Clone)]
pub struct ItemStats {
    pub items: Vec<Item>,
    pub counts: Vec<u32>,
    // todo accuracy: Box<[f32]>,
}

pub fn find_items(port: Port, item_frames: &ItemFrames) -> ItemStats {
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

    let mut keys: Vec<Item> = vec![];
    let mut vals: Vec<u32> = vec![];
    for ((item, itype), count) in item_counter {
        let item = Item::from_repr(item)
            .unwrap_or(Item::UNKNOWN)
            .resolve_subitem(itype);

        keys.push(item);
        vals.push(count);
    }

    ItemStats {
        items: keys,
        counts: vals,
    }
}
