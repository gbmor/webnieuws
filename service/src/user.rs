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
        Ok(_) => {}
        Err(err) => {
            log::error!("Auth failure: {}", err);
            let msg = format!("{{ \"res\": 0, \"msg\":\"Auth Failure\" }}")
                .bytes()
                .collect::<Vec<u8>>();
            strm.write_all(&msg).unwrap();
            return;
        }
    }

    let (incoming, mut outgoing) = strm.split();
    let mut rdr = BufReader::new(incoming);

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

fn auth<'a>(_strm: &mut TcpStream) -> Result<&'a str, &'a str> {
    unimplemented!();
}
