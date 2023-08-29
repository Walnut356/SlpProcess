use std::time::Instant;

use polars::lazy::dsl::col;
use polars::prelude::*;
use slpprocess::columns::*;
use slpprocess::enums::buttons::ControllerInput;
use slpprocess::enums::buttons::EngineInput;
use slpprocess::parse;

pub fn main() {


    let now = Instant::now();
    // let replay = r"G:/temp";
    let replay = r"./Game_20230526T020459.slp";
    let mut games = parse(replay);

    let game = games.pop().unwrap();

    let player = game.player_by_code("NUT#356").unwrap();
    println!("{:?}", player.stats.actions);

    // // let stats = &game.players[0].read().unwrap().stats;
    // // println!("{:?}", stats.actions);

    // for game in games {
    //     let Ok(player) = game.player_by_code("NUT#356") else {
    //         println!("Could not find player");
    //         continue;
    //     };

    //     let mut exit = false;

    //     let pre = player.frames.pre.clone();
    // }

    let dur = now.elapsed();

    println!("{:?}", dur);
}
