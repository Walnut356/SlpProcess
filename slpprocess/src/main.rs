use std::fs::File;
use std::time::Instant;
use std::hint::black_box;


use polars::prelude::*;
use slpprocess::parse;

pub fn main() {
    let now = Instant::now();
    // let replay = r"G:/temp";
    let replay = r"./Game_20230526T020459.slp";
    let mut games = parse(replay);

    let game = games.pop().unwrap();

    let player = game.player_by_code("NUT#356").unwrap();
    let df = player.stats.defense.as_ref().unwrap();
    println!("{:?}", df);

    let dur = now.elapsed();

    println!("{:?}", dur);

    let mut file = File::create("output.csv").expect("could not create file");
    CsvWriter::new(&mut file)
    .has_header(true)
    .with_delimiter(b',')
    .finish(&mut df.clone()).unwrap();
}
