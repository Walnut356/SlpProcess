use std::{os::windows::fs::MetadataExt, time::{Duration, Instant}};

use duckdb::{params, Connection, Result};
use slp_db::{create_stubs, export};
use slp_parse::{parse_stubs, prelude::*};

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    // conn.execute_batch("IMPORT DATABASE './test_db';")?;

    let stubs = parse_stubs(r"E:\Slippi Replays\Netplay\", true);
    dbg!(stubs.len());

    let mut total = 0u128;
    let mut dur = 0;
    for stub in stubs {
        total += stub.path().metadata().unwrap().file_size() as u128;
        dur += stub.duration().as_millis();
    }

    dbg!(dur);



    dbg!(total);

    let now = Instant::now();
    create_stubs(&conn, &parse_stubs(r"E:\Slippi Replays\Netplay\", true))?;
    export(&conn, "test_db")?;
    let mut stmt = conn.prepare(
        "SELECT sum(duration) FROM stub;"
    )?;

    let mut val = stmt.query([])?;

    let dur = now.elapsed();
    dbg!(dur);

    println!("{:?}", val.next().unwrap().unwrap().get::<usize, usize>(0).unwrap());

    Ok(())
}
