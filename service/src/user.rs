//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{BufRead, BufReader, Write};
use tokio::net::TcpStream;
use tokio_io::AsyncRead;

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

pub fn handle(strm: &mut TcpStream) {
    let (incoming, mut outgoing) = strm.split();
    let mut rdr = BufReader::new(incoming);

    let mut input_str = String::new();
    rdr.read_line(&mut input_str).unwrap();
    let input_str = input_str.trim();
    let comm_json: serde_json::Value = serde_json::from_str(input_str).unwrap();

    let result = match json::to_comm(comm_json) {
        Comm::Post(entry) => add_post(&entry),
        Comm::Delete(id) => delete_post(id),
        Comm::Update(entry) => update_post(&entry),
        _ => Err("Something went wrong"),
    };

    match result {
        Ok(msg) => {
            let msg_b = format!("{{ \"res\":1, \"msg\":\"{}\" }}", msg)
                .bytes()
                .collect::<Vec<u8>>();
            outgoing.write_all(&msg_b).unwrap();
        }
        Err(msg) => {
            let msg_b = format!("{{ \"res\":0, \"msg\":\"{}\" }}", msg)
                .bytes()
                .collect::<Vec<u8>>();
            outgoing.write_all(&msg_b).unwrap();
            log::error!("{}", msg);
        }
    }
}

fn add_post<'a>(_entry: &db::Entry) -> Result<&'a str, &'a str> {
    unimplemented!();
}

fn update_post<'a>(_entry: &db::Entry) -> Result<&'a str, &'a str> {
    unimplemented!();
}

fn delete_post<'a>(_id: u64) -> Result<&'a str, &'a str> {
    unimplemented!();
}
