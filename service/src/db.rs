//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use rusqlite;
use serde;

#[derive(Debug, Clone, serde::Serialize)]
struct Entry {
    id: u32,
    author: String,
    title: String,
    body: String,
    date: String,
    tags: Vec<String>,
}

struct Conn {
    conn: rusqlite::Connection,
}

const PATH: &str = "webnieuws.db";

impl Conn {
    fn open(path: &str) -> Self {
        let start = time::Instant::now();
        info!("Connecting to database");
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

        info!(
            "Database connection established in {}ms",
            start.elapsed().as_millis()
        );

        Conn { conn }
    }
}
