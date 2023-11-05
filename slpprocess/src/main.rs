use std::hint::black_box;
use std::time::Instant;
use std::{
    fs::File,
    sync::{RwLock, RwLockReadGuard},
};

use polars::datatypes::DataType::Struct;
use polars::prelude::*;
use serde_json;
use slpprocess::{parse, Game, get_combos, to_dolphin_queue};
use ssbm_utils::enums::ActionState;
use ssbm_utils::types::Point;

pub fn main() {
    let now = Instant::now();
    // let replay = r"G:/temp";

    // TODO this replay has an item of ID 0x62
    // let replay = r"G:/temp/Game_20230713T212214.slp";
    // let replay = r"./Game_20230526T020459.slp";
    // let replay = r"E:\Slippi Replays\Netplay\Game_20231018T005550.slp";

    let replay = r"E:\Slippi Replays\Netplay\Game_20230607T011346.slp";
    let mut games = parse(replay);
    // let dur = now.elapsed();
    // dbg!(dur);
    // let game = games.pop().unwrap();
    // dbg!(game.metadata);
    // dbg!(game.total_frames);
    let combos = get_combos(&games, "NUT#356");
    to_dolphin_queue("./output.json".into(), &combos)

    // let player = game.player_by_code("NUT#356").unwrap();
    // let df = player.stats.defense.as_ref().unwrap();
    // println!("{:?}", DataFrame::from(player.frames.pre.clone()));

    // let player = &game.players[0];
    // // let frames = &player.frames.post;
    // println!("{:?}", player.load().connect_code);

    // dbg!(&player.combos.0.get(0).unwrap());

    // let mut file = File::create("output.parquet").expect("could not create file");
    // ParquetWriter::new(&mut file).with_compression(ParquetCompression::Snappy)
    // .finish(&mut df.clone()).unwrap();
}

pub struct Test {
    inner: RwLock<u32>,
}

pub fn test(thing: &Test) -> RwLockReadGuard<u32> {
    thing.inner.read().unwrap()
}
