use std::time::Instant;
use std::hint::black_box;


use slpprocess::parse;

pub fn main() {
    let now = Instant::now();
    // let replay = r"G:/temp";
    let replay = r"./Game_20230526T020459.slp";
    let mut games = parse(replay);

    let game = games.pop().unwrap();

    let player = game.player_by_code("NUT#356").unwrap();
    println!("{:?}", player.stats.items);

    let dur = now.elapsed();

    println!("{:?}", dur);
}
