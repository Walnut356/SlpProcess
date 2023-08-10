use std::io;

use minstant::Instant;
use slpprocess::parse;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let path = buf.as_str().trim();
    // let path = r"G:\temp";
    let mut thing = parse(path);
    let game = thing.pop().unwrap();
    let now = Instant::now();
    let df = game.players[0].read().unwrap().frames.pre.clone();
    let dur = now.elapsed();
    println!("{:?}", dur);
    println!("{:?}", df);
    println!("{:?}", game.players[0].read().unwrap().frames.pre);
}
