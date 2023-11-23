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
    std::env::set_var("POLARS_FMT_TABLE_CELL_LIST_LEN", "-1");

    // let replay = r"G:/temp";

    let replay = r"G:/temp/Game_20230622T053447.slp";
        // let replay = r"E:\Slippi Replays\Netplay\Game_20231018T005550.slp";

    let game = parse(replay).pop().unwrap();

    // dbg!(game.players[0].frames.post.get_frame(7348));
    dbg!(game.players[0].frames.post.len());
    let mut j = -123;
    for i in game.players[0].frames.post.frame_index.iter() {
        // dbg!(&frame.frame_index, j);
        assert_eq!(*i, j);
        j += 1;
    }

    // dbg!(game.players[1].frames.post.get_frame(79));
    // TODO this replay has an item of ID 0x62
    // let replay = r"G:/temp/Game_20230713T212214.slp";
    // let replay = r"./Game_20230526T020459.slp";


    // let replay = r"E:\Slippi Replays\Netplay\Game_20230607T011346.slp";
    // print_stat(replay);
    // // let game = games.pop().unwrap();

    // // let player = game.player_by_code("NUT#356").unwrap();
    // let df = &player.stats.wavedash;
    // println!("{}", player.stats.get_summary(StatType::LCancel).unwrap())

    // dbg!(df);
    // let mut file = File::create("output.parquet").expect("could not create file");
    // ParquetWriter::new(&mut file).with_compression(ParquetCompression::Snappy)
    // .finish(&mut df.clone()).unwrap();
}

fn print_summary(replay: &str) {
    let now = Instant::now();
    let games = parse(replay);
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
}

fn print_stat(replay: &str) {
    use slpprocess::columns::DefenseStats as col;
    let now = Instant::now();
    let mut games = parse(replay);
    let dur = now.elapsed();

    dbg!(dur);

    let game = games.pop().unwrap();

    println!(
        "{}",
        game.player_by_code("nut#356")
            .unwrap()
            .stats
            .defense
            .clone()
            .unwrap()
            .select([col::StickDuringHitlag.to_string()])
            .unwrap()
    )
}
