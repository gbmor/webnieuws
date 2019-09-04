//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use log;
use rusqlite;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, RwLock};
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

pub fn load_cache() {
    let start = std::time::Instant::now();
    log::info!("Loading cache of posts...");

    let db = match CONNECTION.lock() {
        Ok(val) => val,
        Err(err) => {
            log::error!("{:?}", err);
            return;
        }
    };
    let db = &*db;

    let stmt = format!("SELECT * FROM posts");
    let mut stmt = match db.conn.prepare(&stmt) {
        Ok(val) => val,
        Err(err) => {
            log::error!("{:?}", err);
            return;
        }
    };

    let posts = stmt
        .query_map(rusqlite::NO_PARAMS, |r| {
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
        })
        .unwrap()
        .map(|r| r.unwrap())
        .collect::<Vec<Vec<String>>>();

    let mut cache = match CACHE.write() {
        Ok(val) => val,
        Err(err) => {
            log::error!("{:?}", err);
            return;
        }
    };

    posts.iter().for_each(|post| {
        (*cache).entry(post[3].clone()).or_insert(post.clone());
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
