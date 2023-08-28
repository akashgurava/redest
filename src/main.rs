use std::{fs::remove_file, time::Instant};

use redb::{Database, ReadableTable, TableDefinition};

const DB_PATH: &'static str = "./hello.redb";
const U64_TABLE: TableDefinition<u64, u64> = TableDefinition::new("u64");
const DATA_COUNT: u64 = 10_000_000;
const CACHE_SIZE: usize = 50 * 1024 * 1024;

fn main() {
    println!("DB Path: {}. Cache size: {}.", DB_PATH, CACHE_SIZE);
    let _ = remove_file(DB_PATH);

    let db_start = Instant::now();
    let mut db = Database::builder()
        .set_cache_size(CACHE_SIZE)
        .create(DB_PATH)
        .unwrap();
    db.compact().unwrap();
    println!(
        "Time taken to open DB: {}ms.",
        (Instant::now() - db_start).as_millis()
    );

    let write_start: Instant = Instant::now();
    let write_txn = db.begin_write().unwrap();
    {
        let mut table = write_txn.open_table(U64_TABLE).unwrap();
        for i in 1..DATA_COUNT {
            table.insert(i, i).unwrap();
        }
    }
    write_txn.commit().unwrap();
    println!(
        "Time taken to write data: {}ms.",
        (Instant::now() - write_start).as_millis()
    );

    let read_start = Instant::now();
    let read_txn = db.begin_read().unwrap();
    let table = read_txn.open_table(U64_TABLE).unwrap();
    for i in 1..DATA_COUNT {
        table.get(i).unwrap().unwrap();
    }
    println!(
        "Time Taken to read: {}ms.",
        (Instant::now() - read_start).as_millis(),
    );
}
