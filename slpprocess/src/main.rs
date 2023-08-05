use bytes::*;
use minstant::Instant;
use polars::export::num::PrimInt;
use polars::prelude::*;
use rand::{thread_rng, Rng};
use slpprocess::{parse, Game};
use std::io::Cursor;
use std::path::Path;
use std::{fs, io};

fn main() {
    let path = "./hbox_llod_timeout_g8.slp";
    // let path = r"G:\temp";
    let now = Instant::now();
    let thing = parse(path);
    let dur = now.elapsed();
    println!("{:?}", dur);
    println!("{:?}", thing.len());
    println!("{:?}", thing.first().unwrap().start);
    println!("{:?}", thing.first().unwrap().end);
}
