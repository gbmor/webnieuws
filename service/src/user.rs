//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use bcrypt;
use serde::{Deserialize, Serialize};
use serde_json;
use zeroize::Zeroize;

use crate::db;
use crate::json;
use crate::post;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "kind", content = "cont")]
pub enum Comm {
    Post(db::Entry),
    Delete(u64),
    Update(db::Entry),
    None,
}

pub fn handle(strm: &mut TcpStream) {
    let res = auth(strm);
    match res {
        Ok(val) => match val {
            ("auth", user) => {
                log::info!("Auth successful: {}", user);
            }
            ("fail", user) | (_, user) => {
                log::warn!("Auth failure: {}", user);
                let msg = format!("{{ \"res\":0, \"msg\":\"Auth failure\" }}")
                    .bytes()
                    .collect::<Vec<u8>>();
                strm.write_all(&msg).unwrap();
                return;
            }
        },
        Err((_, user)) => {
            log::error!("Auth failure: {}", user);
            let msg = format!("{{ \"res\": 0, \"msg\":\"Auth Failure\" }}")
                .bytes()
                .collect::<Vec<u8>>();
            strm.write_all(&msg).unwrap();
            return;
        }
    }

    let rdr = strm.try_clone().unwrap();
    let mut rdr = BufReader::new(rdr);

    let mut input_str = String::new();
    rdr.read_line(&mut input_str).unwrap();
    let input_str = input_str.trim();
    let comm_json: serde_json::Value = serde_json::from_str(input_str).unwrap();

    let result = match json::to_comm(comm_json) {
        Comm::Post(entry) => post::add(&entry),
        Comm::Delete(id) => post::delete(id),
        Comm::Update(entry) => post::update(&entry),
        _ => Err("Something went wrong"),
    };

    match result {
        Ok(msg) => {
            let msg_b = format!("{{ \"res\":1, \"msg\":\"{}\" }}", msg)
                .bytes()
                .collect::<Vec<u8>>();
            strm.write_all(&msg_b).unwrap();
        }
        Err(msg) => {
            let msg_b = format!("{{ \"res\":0, \"msg\":\"{}\" }}", msg)
                .bytes()
                .collect::<Vec<u8>>();
            strm.write_all(&msg_b).unwrap();
            log::error!("{}", msg);
        }
    }
}

fn auth<'a>(strm: &mut TcpStream) -> Result<(&'a str, String), (&'a str, String)> {
    let db = db::CONNECTION.lock();
    let db = &*db;
    let mut stmt = db
        .conn
        .prepare("SELECT pass FROM users WHERE name = :name")
        .unwrap();

    let incoming = strm.try_clone().unwrap();
    let mut rdr = BufReader::new(incoming);
    let mut in_json = String::new();
    rdr.read_line(&mut in_json).unwrap();
    let in_json = in_json.trim();
    let in_json: serde_json::Value = serde_json::from_str(in_json).unwrap();

    let mut pass = in_json["pass"]
        .as_str()
        .unwrap()
        .bytes()
        .collect::<Vec<u8>>();
    let user = in_json["user"].as_str().unwrap().to_string();
    let mut stored_pass = stmt
        .query_row_named(&[(":name", &user)], |row| row.get::<usize, String>(1))
        .unwrap();

    let res = bcrypt::verify(&pass, &stored_pass);
    match res {
        Ok(val) => match val {
            true => {
                pass.zeroize();
                stored_pass.zeroize();
                return Ok(("auth", user.clone()));
            }
            false => {
                pass.zeroize();
                stored_pass.zeroize();
                return Err(("fail", user.clone()));
            }
        },
        Err(err) => {
            pass.zeroize();
            stored_pass.zeroize();
            log::error!("{:?}", err);
            return Err(("fail", user.clone()));
        }
    }
}
