use duckdb::arrow::record_batch::RecordBatch;
use duckdb::arrow::util::pretty::print_batches;
use duckdb::{params, Connection, DefaultOrder, Result};
use slp_parse::GameMetadata;
use slp_parse::{prelude::*, GameStub};

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute_batch("IMPORT DATABASE './test_db';")?;

    let mut stmt = conn.prepare(
        "SELECT sum(percent) FROM post_frames;"
    )?;

    let mut val = stmt.query([])?;

    println!("{:?}", val.next().unwrap().unwrap().get::<usize, usize>(0).unwrap());

    Ok(())
}
