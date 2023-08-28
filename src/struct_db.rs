use std::time::Instant;

use serde::{Deserialize, Serialize};
use struct_db::{struct_db, Db, ReadableTable};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[struct_db(
   fn_primary_key(p_key),  // required
)]
struct Transaction {
    timestamp: u64,
}

impl Transaction {
    // Returns primary key as big-endian bytes for consistent lexicographical ordering.
    pub fn p_key(&self) -> Vec<u8> {
        self.timestamp.to_be_bytes().to_vec()
    }
}

fn main() {
    let mut db = Db::create("my_db_example.redb").unwrap();

    // Initialize the schema
    db.define::<Transaction>();

    let write_start = Instant::now();
    let txn = db.transaction().unwrap();
    {
        let mut tables = txn.tables();
        for i in 1..10_000_000 {
            tables.insert(&txn, Transaction { timestamp: i }).unwrap();
        }
    }
    txn.commit().unwrap();
    println!(
        "Time taken to write data: {}ms.",
        (Instant::now() - write_start).as_millis()
    );

    let read_start = Instant::now();
    let txn_read = db.read_transaction().unwrap();
    let mut tables = txn_read.tables();
    for i in 1..10_000_000u64 {
        tables
            .primary_get::<Transaction>(&txn_read, &i.to_be_bytes())
            .unwrap()
            .unwrap();
    }
    println!(
        "Time Taken to read: {}ms.",
        (Instant::now() - read_start).as_millis(),
    );
}
