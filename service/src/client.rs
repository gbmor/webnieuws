//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use crate::db;
use rusqlite;

pub fn handle(strm: &mut TcpStream, db: Arc<Mutex<db::Conn>>) -> rusqlite::Result<()> {
    let db = db.lock().unwrap();
    let db = &*db;

    let stmt = format!("SELECT * FROM posts");
    let mut stmt = db.conn.prepare(&stmt).unwrap();

    let posts = stmt.query_map(rusqlite::NO_PARAMS, |r| {
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
    })?;

    let posts = posts.map(|r| r.unwrap()).collect::<Vec<Vec<String>>>();
    let posts = str_to_json(posts).bytes().collect::<Vec<u8>>();

    strm.write_all(&posts).unwrap();

    Ok(())
}

fn str_to_json(data: Vec<Vec<String>>) -> String {
    data.iter().map(|e| {
        format!(
            "{{ \"author\": \"{}\", \"title\": \"{}\", \"body\": \"{}\", \"date\": \"{}\", \"tags\": \"{}\" }}",
            e[0], e[1], e[2], e[3], e[4]
        )
    }).collect::<Vec<String>>().join("").into()
}
