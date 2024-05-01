#[allow(unused)]
#[allow(dead_code)]
use std::hint::black_box;

use itertools::izip;
use ssbm_utils::prelude::{Character, Flags};
use std::{collections::HashSet, time::Instant};

use slp_parse::{prelude::*, stats::{rate_falco_combos, StatType}};
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


    // dbg!(&files[0..10]);

    // TODO old replay stubs end up misaligned when reading metadata somehow
    // let replay = r"E:\Slippi Replays\Netplay\Game_20240208T014130.slp";
    //
    // let replay = r"E:\Slippi Replays\Netplay\";
    // crashes on yoshi action state id 341 - fixed but circumstance still weird
    // let replay = r"E:\Slippi Replays\Netplay\Game_20231213T003213.slp";

    let replay = r"E:\Slippi Replays\Netplay\";
    let replay = r"G:\Coding\My Projects\Slippi Stats\SlpProcess\test_replays\netplay_sample.slp";

    // print_summary(replay)
    let games = parse(replay, true, true).pop().unwrap();
    for i in 0..games.players[0].frames.len() {
        if i == 0 {
            dbg!(&games.players[0].frames.post.last_ground_id.as_slice()[0]);
        }
        if games.players[0].frames.post.stocks[i] == 3 {
            dbg!(games.players[0].frames.get_frame(i).1);
        }
    }
    // let mut rated = rate_falco_combos("NUT#356", &games);
    // rated.sort_by(|a, b| b.1.cmp(&a.1));
    // // for a in &rated {
    // //     println!("start: {}, end: {}, rating: {}", a.0.start_frame, a.0.end_frame, a.1);
    // // }
    // let combos = rated.iter().filter_map(|x| if x.1 >= 0 {Some(&x.0) } else {None}).collect::<Vec<_>>();
    //     dbg!(combos.len());
    // to_dolphin_queue("./test_combos.json".into(), combos.get(0..100).unwrap());
    // for r in 0usize..100 {
    //     println!("{}", rated[r].1)
    // }
}

// fn _print_summary(replay: &str) {
//     let now = Instant::now();
//     let games = parse(replay, false);
//     let dur = now.elapsed();

//     println!(
//         "{}",
//         games[0].players[1]
//             .stats
//             .get_summary(StatType::Tech)
//             .unwrap()
//     );
//     dbg!(dur);
// }

// fn _print_stat(replay: &str) {
//     use slp_parse::columns::DefenseStats as clm;
//     timeit!("create par_iter" let games = parse(replay, false));

//     timeit!(
//         "parse, filter, collect"
//     let stats: Vec<LazyFrame> = games.into_iter()
//         .filter_map(|x| {
//             x.player_by_code("nut#356")
//                 .map(|y| y.stats.clone().get_summary(StatType::Defense).map(|x| x.lazy()))
//                 .ok()
//                 .flatten()
//         })
//         .collect()
//     );

//     timeit!(
//         "vstack"
//         // let lf = stats.into_iter().reduce(|a, b| a.vstack(&b).unwrap()).unwrap().lazy()
//         let lf = concat(stats, UnionArgs::default()).unwrap()
//     );

//     timeit!(
//         "aggregate"
//         let result = lf.select(&[
//             col("DamageTaken").count().alias("TotalGames"),
//             col("DamageTaken").sum(),
//             col("HitsTaken").sum(),
//             col("DamageTaken").mean().alias("AvgDamage"),
//             col("HitsTaken").mean().alias("AvgHits"),
//             col("DIEfficacy").mean(),
//             col("SDIPerHit").mean(),
//             col("LivableHitsLived").mean(),
//             // col("LivableHits").sum(),
//             col("FramesInHitlag").sum(),
//         ])
//         .collect().unwrap()
//     );

//     println!("{:?}", result);
//     // for game in games {
//     //     println!(
//     //         "{:?}",
//     //         game.player_by_code("nut#356")
//     //             .unwrap()
//     //             .stats
//     //             .get_summary(StatType::Defense)
//     //     )
//     // }
// }
