//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use std::thread;

use serde::{Deserialize, Serialize};

use crate::db;
use crate::error;
use crate::json;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "kind", content = "cont")]
pub enum Comm {
    Post(db::Entry),
    Delete(u64),
    Update(db::Entry),
    None,
}

#[post("/", data = "<post>")]
pub fn handle(post: String) -> String {
    let json = error::helper(serde_json::from_str(&post));
    let comm = json::to_comm(json);

    let db = db::CONNECTION.lock();
    let db = &*db;

    let stmt = format!("INSERT INTO posts (author, title, body, date, tags) VALUES (:author, :title, :body, :date, :tags)");
    let mut stmt = error::helper(db.conn.prepare(&stmt));

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
            thread::spawn(db::load_cache);
            return "200 OK".into();
        }
        Err(_) => "500 Internal Server Error".into(),
    }
}

#[delete("/<id>")]
pub fn del_post<'a>(id: u32) -> &'a str {
    let db = db::CONNECTION.lock();
    let db = &*db;

    let stmt = format!("DELETE FROM posts WHERE id = :id");
    let mut stmt = db.conn.prepare(&stmt).unwrap();

    match stmt.execute_named(&[(":id", &id)]) {
        Ok(_) => "200 OK",
        Err(err) => {
            log::error!("{:?}", err);
            "500 Internal Server Error"
        }
    }
}
