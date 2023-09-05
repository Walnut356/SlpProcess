use std::time::Instant;
use std::hint::black_box;
use std::ops::Add;

use slpprocess::{parse, enums::buttons::{StickRegion, EngineInput}};

pub fn main() {
    let input = EngineInput::DPAD_LEFT;
    let val = EngineInput::Raw(0b1111);

    println!("{} {}", input, val);



    // let now = Instant::now();
    // // let replay = r"G:/temp";
    // let replay = r"./Game_20230526T020459.slp";
    // let mut games = parse(replay);

    // let game = games.pop().unwrap();

    // let player = game.player_by_code("NUT#356").unwrap();
    // println!("{:?}", player.stats.items);

    // let dur = now.elapsed();

    // println!("{:?}", dur);
}
