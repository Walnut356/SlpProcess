use minstant::Instant;
use slpprocess::parse;


fn main() {
    let path = "./Game_20230526T020459.slp";
    // let path = r"G:\temp";
    let now = Instant::now();
    let thing = parse(path);
    let dur = now.elapsed();
    println!("{:?}", dur);
    println!("{:?}", thing.len());
    println!("{:?}", thing.first().unwrap().start);
    println!("{:?}", thing.first().unwrap().end);
}
