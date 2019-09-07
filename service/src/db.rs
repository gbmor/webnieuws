//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use crossbeam_channel;
use log;
use parking_lot::{Mutex, RwLock};
use rusqlite;
use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time;

use crate::error;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Entry {
    pub id: u64,
    pub author: String,
    pub title: String,
    pub body: String,
    pub date: String,
    pub tags: Vec<String>,
}

#[derive(Debug)]
pub struct Conn {
    pub conn: rusqlite::Connection,
}

pub const PATH: &str = "webnieuws.db";

lazy_static! {
    pub static ref CONNECTION: Arc<Mutex<Conn>> = Arc::new(Mutex::new(Conn::open(PATH)));
    pub static ref CACHE: Arc<RwLock<BTreeMap<String, Vec<String>>>> =
        Arc::new(RwLock::new(BTreeMap::new()));
}

impl Conn {
    pub fn open(path: &str) -> Self {
        let start = time::Instant::now();
        log::info!("Connecting to database");
        let conn = error::helper(rusqlite::Connection::open_with_flags(
            path,
            rusqlite::OpenFlags::SQLITE_OPEN_FULL_MUTEX
                | rusqlite::OpenFlags::SQLITE_OPEN_CREATE
                | rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE,
        ));

        error::helper(conn.execute(
            "CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY NOT NULL,
            author TEXT NOT NULL,
            title TEXT NOT NULL,
            body TEXT NOT NULL,
            date TEXT NOT NULL,
            tags TEXT NOT NULL
        )",
            rusqlite::NO_PARAMS,
        ));

        error::helper(conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                pass TEXT NOT NULL
        )",
            rusqlite::NO_PARAMS,
        ));

        log::info!(
            "Database connection established in {}ms",
            start.elapsed().as_millis()
        );

        Conn { conn }
    }
}

// Periodically refreshes the post cache.
pub fn cache_ticker() {
    let ticker = crossbeam_channel::tick(time::Duration::from_millis(60000));
    loop {
        select! {
            recv(ticker) -> _ => load_cache(),
        }
    }
}

// Loads the posts from sqlite to memory.
pub fn load_cache() {
    let start = std::time::Instant::now();
    log::info!("Loading cache of posts...");

    let db = CONNECTION.lock();
    let db = &*db;

    let stmt = format!("SELECT * FROM posts");
    let mut stmt = match db.conn.prepare(&stmt) {
        Ok(val) => val,
        Err(err) => {
            log::error!("{:?}", err);
            return;
        }
    };

    let posts = error::helper(stmt.query_map(rusqlite::NO_PARAMS, |r| {
        let auth: String = r.get(1)?;
        let title: String = r.get(2)?;
        let body: String = r.get(3)?;
        let date: String = r.get(4)?;
        let tags: String = r.get(5)?;

        Ok(vec![
            auth.into(),
            title.into(),
            body.into(),
            date.into(),
            tags.into(),
        ])
    }))
    .map(|r| error::helper(r))
    .collect::<Vec<Vec<String>>>();

    let mut cache = CACHE.write();
    // Clear it out first
    *cache = BTreeMap::new();

    posts.iter().for_each(|post| {
        (*cache).entry(post[2].clone()).or_insert(post.clone());
    });

    log::info!(
        "Cache loaded. Elapsed time: {} ms",
        start.elapsed().as_millis()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_db() {
        let db = Conn::open("/tmp/test.db");
        db.conn.prepare("SELECT * FROM posts").unwrap();
    }
}
