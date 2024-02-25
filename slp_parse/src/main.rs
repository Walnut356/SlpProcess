#[allow(unused)]
#[allow(dead_code)]
use std::hint::black_box;

use std::time::Instant;

use slp_parse::{prelude::*, stats::StatType};
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
    // let replay = r"E:\Slippi Replays\Netplay\";

    // let mut files: Vec<DirEntry> = fs::read_dir(replay)
    //     .unwrap()
    //     .filter_map(|file| {
    //         if let Ok(entry) = file {
    //             let path = entry.path();
    //             if path.is_file() && path.extension().unwrap() == "slp" {
    //                 Some(entry)
    //             } else {
    //                 None
    //             }
    //         } else {
    //             None
    //         }
    //     })
    //     .collect();

    // files.sort_by_key(|b| std::cmp::Reverse(b.metadata().unwrap().created().unwrap()));

    // dbg!(&files[0..10]);

    // TODO old replay stubs end up misaligned when reading metadata somehow
    // let replay = r"E:\Slippi Replays\Netplay\Game_20240208T014130.slp";
    //
    let replay = r"G:/Coding/My Projects/Slippi Stats/SlpProcess/weird_slideon.slp";
    // let replay = r"E:\Slippi Replays\Netplay\";
    // crashes on yoshi action state id 341 - fixed but circumstance still weird
    // let replay = r"E:\Slippi Replays\Netplay\Game_20231213T003213.slp";

    // let replay = r"../test_replays/netplay_sample.slp";
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
    let games = parse(replay, true);

    let frames = games[0].players[0].frames.clone();

    println!("{}", frames.pre.get_frame(5151 + 123));
    // dbg!(games.iter().map(|g| g.item_frames.as_ref().unwrap().len()).sum::<usize>());
    // for game in games {
    //     let df = game.players[0].stats.defense.clone().unwrap();
    //     let column = df.column("FrameIndex").unwrap().chunks();
    //     dbg!(column.len());
    //     // if let Some(lras) = game.end.clone().unwrap().lras_initiator {
    //     //     if lras >=0 {
    //     //         dbg!(game.end.unwrap());
    //     //         println!("{}", game.players[0].frames.get_last_frame().0);
    //     //         println!("{}", game.players[1].frames.get_last_frame().0);
    //     //     }
    //     // }
    // }

    // dbg!(game.date);

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
