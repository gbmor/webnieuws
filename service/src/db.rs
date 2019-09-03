//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use log;
use rusqlite;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::time;

#[derive(Debug, Clone)]
struct Entry {
    id: u32,
    author: String,
    title: String,
    body: String,
    date: String,
    tags: Vec<String>,
}

impl Serialize for Entry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Entry", 6)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("author", &self.author)?;
        s.serialize_field("title", &self.title)?;
        s.serialize_field("body", &self.body)?;
        s.serialize_field("date", &self.date)?;
        s.serialize_field("tags", &self.tags)?;
        s.end()
    }
}

pub struct Conn {
    pub conn: rusqlite::Connection,
}

pub const PATH: &str = "webnieuws.db";

impl Conn {
    pub fn open(path: &str) -> Self {
        let start = time::Instant::now();
        log::info!("Connecting to database");
        let conn = rusqlite::Connection::open_with_flags(
            path,
            rusqlite::OpenFlags::SQLITE_OPEN_FULL_MUTEX
                | rusqlite::OpenFlags::SQLITE_OPEN_CREATE
                | rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE,
        )
        .expect("Could not connect to DB");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY NOT NULL,
            author TEXT NOT NULL,
            title TEXT NOT NULL,
            body TEXT NOT NULL,
            date TEXT NOT NULL,
            tags TEXT NOT NULL
        )",
            rusqlite::NO_PARAMS,
        )
        .expect("Could not initialize DB");

        log::info!(
            "Database connection established in {}ms",
            start.elapsed().as_millis()
        );

        Conn { conn }
    }
}
