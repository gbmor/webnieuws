//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{BufRead, BufReader};
use tokio::net::TcpStream;
use tokio_io::AsyncRead;

use crate::db;
use crate::json;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "cont")]
pub enum Comm {
    Post(db::Entry),
    Delete(u32),
    Update(db::Entry),
    None,
}

pub fn handle(strm: &mut TcpStream) {
    let (incoming, _outgoing) = strm.split();
    let mut rdr = BufReader::new(incoming);

    let mut input_str = String::new();
    rdr.read_line(&mut input_str).unwrap();
    let input_str = input_str.trim();

    let comm: serde_json::Value = serde_json::from_str(input_str).unwrap();
    let _comm = json::to_comm(comm);
}
