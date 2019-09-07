//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use rusqlite;
use serde::{Deserialize, Serialize};

use crate::db;
use crate::json;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "kind", content = "cont")]
pub enum Comm {
    Post(db::Entry),
    Delete(u64),
    Update(db::Entry),
    None,
}

#[post("/post", data = "<post>")]
pub fn handle(post: String) -> String {
    let json = serde_json::from_str(&post).unwrap();
    let comm = json::to_comm(json);

    let db = db::CONNECTION.lock();
    let db = &*db;

    let stmt = format!("INSERT INTO posts (author, title, body, date, tags) VALUES (:author, :title, :body, :date, :tags)");
    let mut stmt = db.conn.prepare(&stmt).unwrap();

    let post = match comm {
        Comm::Post(val) => val,
        _ => return "400 Bad Request".into(),
    };
    let tags = post.tags.join(" ");
    match stmt.execute_named(&[
        (":author", &post.author),
        (":title", &post.title),
        (":body", &post.body),
        (":date", &post.date),
        (":tags", &tags),
    ]) {
        Ok(_) => {
            db::load_cache();
            return "200 OK".into();
        }
        Err(_) => "500 Internal Server Error".into(),
    }
}
