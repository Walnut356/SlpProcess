use slpprocess::parse;
use std::path::Path;
use std::{fs, io, time};

fn main() {
    let now = time::Instant::now();
    let path = "../Game_20230526T020459.slp".to_string();
    let thing = parse::parse(&path);
    let dur = now.elapsed();
    println!("{}", dur.as_millis());

    let now = time::Instant::now();
    let mut buf = io::BufReader::new(fs::File::open("../Game_20230526T020459.slp").unwrap());
    let game = peppi::game(&mut buf, None, None).unwrap();
    let dur = now.elapsed();
    println!("{}", dur.as_millis());

    println!("{:?}", thing[0]);
    println!("{:?}", game.metadata);
}
