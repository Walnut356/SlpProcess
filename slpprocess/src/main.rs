use std::{io, time::Duration};
use bytes::{Bytes, Buf};

use minstant::Instant;
use slpprocess::{parse, Game};

fn main() {
    let path = "./Game_20230526T020459.slp";
    // let path = r"G:\temp";

    let mut times = Vec::with_capacity(1000);

    let mut game = 0.0;
    for _i in 0..1000 {
        let now = Instant::now();

        let thing = parse(path);

        let dur = now.elapsed();
        times.push(dur.as_nanos());

        game = thing[0].start.damage_ratio;
    }

    let total_dur: u128 = times.iter().sum();
    let avg = total_dur / times.len() as u128;
    let avg_dur = Duration::from_nanos(avg as u64);

    println!("{:?}", avg_dur);
    println!("{game}");

}
