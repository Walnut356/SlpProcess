#[allow(unused_imports)]

use std::hint::black_box;
use std::io::Read;
use std::path::PathBuf;
use std::time::Instant;
use std::{
    fs::File,
    sync::{RwLock, RwLockReadGuard},
};
use std::iter::zip;

use bytes::{Bytes, Buf};
use polars::datatypes::DataType::Struct;
use polars::prelude::*;
use rayon::prelude::*;
use serde_json;
use slpprocess::events::game_start::{GameStart, MatchType};
use slpprocess::game::GameStub;
use slpprocess::{parse_iter, parse_stubs};
use slpprocess::player::{Player, PlayerStub};
use slpprocess::stats::Stats;
use slpprocess::{get_combos, parse, stats::StatType, to_dolphin_queue, Game};
use ssbm_utils::prelude::*;
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
    // rayon::ThreadPoolBuilder::default()
    //     .stack_size(1048576 * 5)
    //     .build_global()
    //     .unwrap();
    std::env::set_var("POLARS_FMT_TABLE_CELL_LIST_LEN", "-1");

    // let replay = r"G:/temp";

    // let replay = r"E:\Slippi Replays\Netplay\Game_20231222T004632.slp";
    let replay = r"E:\Slippi Replays\Netplay\";
    // crashes on yoshi action state id 341
    // let replay = r"E:\Slippi Replays\Netplay\Game_20231213T003213.slp";
    // let replay = r"./test_replays/netplay_sample.slp";
    // let mut f = File::open(replay).unwrap();
    //     let file_length = f.metadata().unwrap().len() as usize;
    //     dbg!(file_length);
    //     let mut file_data = vec![0; file_length];
    //     f.read_exact(&mut file_data).unwrap();

    //     dbg!(&file_data[56268..56268 + 4]);
    //     let mut b = Bytes::from(file_data);
    //     let mut d = b.slice(..);

    //     d.advance(56268);

    //     dbg!(b.len() - d.len());
    //     dbg!(d.get_i32());
    let mut game = parse(replay, true).into_iter().filter_map(|x| x.metadata.is_netplay.is_some_and(|y| y).then_some(x)).collect::<Vec<_>>();
    let totals = game.iter().map(|x| x.total_frames as usize);
    let rollbacks = game.iter().map(|x| x.frames_rollbacked);

    // dbg!(rollbacks.sum::<usize>() / game.len());
    let avg_rlbk = zip(totals, rollbacks).map(|x| x.1 as f32/ (x.0 as f32+ x.1 as f32)).collect::<Vec<f32>>();
    dbg!(avg_rlbk.iter().sum::<f32>() / game.len() as f32);
    let mut min = 100000.0;
    let mut max = 0.0;
    for val in avg_rlbk {
        if val < min {
            min = val;
        }
        if val > max {
            max = val;
        }
    }

    dbg!(max);
    dbg!(min);


    // TODO this replay has an item of ID 0x62
    // let replay = r"G:/temp/Game_20230713T212214.slp";
    // let replay = r"./Game_20230526T020459.slp";

    // let replay = r"E:\Slippi Replays\Netplay\Game_20230607T011346.slp";

    // print_summary(replay)

    // loop {
    //     timeit!(
    //         "Parse Games: "
    //         let games = parse_stubs(replay, false)
    //     );
    //     dbg!(games[0].duration);
    // }
}

fn print_summary(replay: &str) {
    let now = Instant::now();
    let games = parse(replay, false);
    let dur = now.elapsed();

    println!(
        "{}",
        games[0].players[1]
            .stats
            .get_summary(StatType::Tech)
            .unwrap()
    );
}

fn print_stat(replay: &str) {
    use slpprocess::columns::DefenseStats as clm;
    timeit!("create par_iter" let mut games = parse(replay, false));

    timeit!(
        "parse, filter, collect"
    let stats: Vec<LazyFrame> = games.into_iter()
        .filter_map(|x| {
            x.player_by_code("nut#356")
                .map(|y| y.stats.clone().get_summary(StatType::Defense).map(|x| x.lazy()))
                .ok()
                .flatten()
        })
        .collect()
    );

    timeit!(
        "vstack"
        // let lf = stats.into_iter().reduce(|a, b| a.vstack(&b).unwrap()).unwrap().lazy()
        let lf = concat(stats, UnionArgs::default()).unwrap()
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

    println!("{:?}", result);
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
