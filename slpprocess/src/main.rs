use std::fs::File;
use std::hint::black_box;
use std::time::Instant;

use polars::datatypes::DataType::Struct;
use polars::prelude::*;
use serde_json;
use slpprocess::parse;
use ssbm_utils::types::Point;

pub fn main() {
    let now = Instant::now();
    let replay = r"G:/temp";
    // let replay = r"./Game_20230526T020459.slp";
    let mut games = parse(replay);
    let dur = now.elapsed();

    let game = games.pop().unwrap();

    dbg!(&game.duration);

    let player = game.player_by_code("NUT#356").unwrap();
    let df = player.stats.defense.as_ref().unwrap();
    println!("{:?}", DataFrame::from(player.frames.pre.clone()));



    println!("{:?}", dur);

    // let mut file = File::create("output.parquet").expect("could not create file");
    // ParquetWriter::new(&mut file).with_compression(ParquetCompression::Snappy)
    // .finish(&mut df.clone()).unwrap();

}
