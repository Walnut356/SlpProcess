use std::hint::black_box;
use std::time::Instant;
use std::{
    fs::File,
    sync::{RwLock, RwLockReadGuard},
};

use polars::datatypes::DataType::Struct;
use polars::prelude::*;
use serde_json;
use slpprocess::{get_combos, parse, stats::StatType, to_dolphin_queue, Game};
use ssbm_utils::enums::ActionState;
use ssbm_utils::types::Point;

pub fn main() {
    let replay = r"G:/temp";

    // TODO this replay has an item of ID 0x62
    // let replay = r"G:/temp/Game_20230713T212214.slp";
    // let replay = r"./Game_20230526T020459.slp";
    // let replay = r"E:\Slippi Replays\Netplay\Game_20231018T005550.slp";

    // let replay = r"E:\Slippi Replays\Netplay\Game_20230607T011346.slp";
    let now = Instant::now();
    let mut games = parse(replay);
    let dur = now.elapsed();

    let mut schema = Schema::new();
    schema.with_column("TotalDamage".into(), DataType::Float32);
    schema.with_column("MostHitBy".into(), DataType::Utf8);
    schema.with_column("SDIPerHit".into(), DataType::Float32);
    let mut df = DataFrame::from(&schema);

    dbg!(dur);
    for game in games {
        let player = game.player_by_code("NUT#356");
        if let Ok(p) = player {
            if let Some(stats) = p.stats.get_summary(StatType::Defense) {
                df.extend(&stats).unwrap();
            }
        }
    }
    println!("{}", df.column("SDIPerHit").unwrap().mean().unwrap());
    // let game = games.pop().unwrap();

    // let player = game.player_by_code("NUT#356").unwrap();
    // let df = &player.stats.wavedash;
    // println!("{}", player.stats.get_summary(StatType::LCancel).unwrap())

    // dbg!(df);
    // let mut file = File::create("output.parquet").expect("could not create file");
    // ParquetWriter::new(&mut file).with_compression(ParquetCompression::Snappy)
    // .finish(&mut df.clone()).unwrap();
}
