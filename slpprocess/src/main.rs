use std::fs::File;
use std::hint::black_box;
use std::time::Instant;

use polars::datatypes::DataType::Struct;
use polars::prelude::*;
use serde_json;
use slpprocess::parse;
use ssbm_utils::enums::ActionState;
use ssbm_utils::types::Point;

pub fn main() {
    let now = Instant::now();
    // let replay = r"G:/temp";

    // TODO this replay has an item of ID 0x62
    // let replay = r"G:/temp/Game_20230713T212214.slp";
    // let replay = r"./Game_20230526T020459.slp";
    let replay = r"E:\Slippi Replays\Netplay\Game_20231018T005550.slp";
    let mut games = parse(replay);
    let dur = now.elapsed();
    dbg!(dur);

    let game = games.pop().unwrap();
    dbg!(game.version);

    // let player = game.player_by_code("NUT#356").unwrap();
    // let df = player.stats.defense.as_ref().unwrap();
    // println!("{:?}", DataFrame::from(player.frames.pre.clone()));

    let player = game.players[0].read().unwrap();
    // let frames = &player.frames.post;
    println!("{:?}", player.combos.0.len());

    // dbg!(&player.combos.0.get(0).unwrap());

    // let mut file = File::create("output.parquet").expect("could not create file");
    // ParquetWriter::new(&mut file).with_compression(ParquetCompression::Snappy)
    // .finish(&mut df.clone()).unwrap();

}
