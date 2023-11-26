use std::hint::black_box;
use std::path::PathBuf;
use std::time::Instant;
use std::{
    fs::File,
    sync::{RwLock, RwLockReadGuard},
};

use bytes::Bytes;
use polars::datatypes::DataType::Struct;
use polars::prelude::*;
use rayon::prelude::*;
use serde_json;
use slpprocess::parse_iter;
use slpprocess::{get_combos, parse, stats::StatType, to_dolphin_queue, Game};
use ssbm_utils::enums::ActionState;
use ssbm_utils::types::Point;

// static REPLAY: &[u8; 165123] = include_bytes!(r"G:/temp\Game_20230627T174002.slp");

macro_rules! timeit {
    ($i:literal $x:stmt) => {
        let now = Instant::now();
        $x
        let dur = now.elapsed();
        println!("{}: {:?}", $i, dur);
    }
}

pub fn main() {
    std::env::set_var("POLARS_FMT_TABLE_CELL_LIST_LEN", "-1");

    // let replay = r"G:/temp";
    let replay = r"E:\Slippi Replays\Netplay\";

    // let replay = r"./";
    // let replay = r"G:/temp\Game_20230627T174002.slp";
    // let replay = r"G:/temp/Game_20230622T053447.slp";
    // let replay = r"E:\Slippi Replays\Netplay\Game_20231018T005550.slp";

    // TODO this replay has an item of ID 0x62
    // let replay = r"G:/temp/Game_20230713T212214.slp";
    // let replay = r"./Game_20230526T020459.slp";

    // let replay = r"E:\Slippi Replays\Netplay\Game_20230607T011346.slp";
    print_stat(replay);
    // // let game = games.pop().unwrap();

    // // let player = game.player_by_code("NUT#356").unwrap();
    // let df = &player.stats.wavedash;
    // println!("{}", player.stats.get_summary(StatType::LCancel).unwrap())

    // dbg!(df);
    // let mut file = File::create("output.parquet").expect("could not create file");
    // ParquetWriter::new(&mut file).with_compression(ParquetCompression::Snappy)
    // .finish(&mut df.clone()).unwrap();
}

// fn print_summary(replay: &str) {
//     let now = Instant::now();
//     let games = parse(replay, false);
//     let dur = now.elapsed();

//     let mut schema = Schema::new();
//     schema.with_column("TotalDamage".into(), DataType::Float32);
//     schema.with_column("MostHitBy".into(), DataType::Utf8);
//     schema.with_column("SDIPerHit".into(), DataType::Float32);
//     let mut df = DataFrame::from(&schema);

//     dbg!(dur);
//     for game in games {
//         let player = game.player_by_code("NUT#356");
//         if let Ok(p) = player {
//             if let Some(stats) = p.stats.get_summary(StatType::Defense) {
//                 df.extend(&stats).unwrap();
//             }
//         }
//     }
//     println!("{}", df.column("SDIPerHit").unwrap().mean().unwrap());
// }

fn print_stat(replay: &str) {
    use slpprocess::columns::DefenseStats as clm;
    timeit!("create par_iter" let mut games = parse_iter(replay));

    timeit!(
        "parse, filter, collect"
    let stats: Vec<LazyFrame> = games
        .filter_map(|x| {
            x.player_by_code("nut#356")
                .map(|y| y.stats.clone())
                .map(|stat| stat.get_summary(StatType::Defense).map(|df| df.lazy()))
                .ok()
                .flatten()
        })
        .collect()
    );

    // let mut result: Option<DataFrame> = None;

    timeit!(
        "concat frames"
        let lf = concat(stats, UnionArgs {parallel: true, rechunk: true, to_supertypes: false}).unwrap()
    );

    timeit!(
        "aggregate"
        let result = lf.select(&[
            col("DamageTaken").count().alias("TotalGames"),
            col("DamageTaken").sum(),
            col("HitsTaken").sum(),
            col("DamageTaken").mean().alias("AvgDamage"),
            col("HitsTaken").mean().alias("AvgHits"),
            col("DIEfficacy").mean(),
            col("SDIPerHit").mean(),
            col("LivableHitsLived").mean(),
            // col("LivableHits").sum(),
            col("FramesInHitlag").sum(),
        ])
        .collect().unwrap()
    );

    println!(
        "{:?}",
        result
    );
    // for game in games {
    //     println!(
    //         "{:?}",
    //         game.player_by_code("nut#356")
    //             .unwrap()
    //             .stats
    //             .get_summary(StatType::Defense)
    //     )
    // }
}
