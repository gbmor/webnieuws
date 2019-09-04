//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

#[macro_use]
extern crate lazy_static;

use ctrlc;
use log;
use tokio::net::TcpListener;
use tokio::prelude::*;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod client;
mod db;
mod logging;

lazy_static! {
    static ref LSTN_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 9001);
}

fn main() {
    logging::init();
    db::load_cache();

    // Handles SIGINT signals.
    // More will have to be done here later.
    if let Err(err) = ctrlc::set_handler(|| {
        log::warn!("^C/SIGINT Caught ... ");
        std::process::exit(0);
    }) {
        log::error!("Failed to set up SIGINT handler: {:?}", err);
        std::process::exit(1);
    }

    // Next we'll asynchronously handle incoming requests.
    log::info!("Starting up ...");

    let lstnr = match TcpListener::bind(&LSTN_ADDR) {
        Ok(val) => val,
        Err(err) => {
            log::error!("Failed to open TCP socket: {:?}", err);
            std::process::exit(1);
        }
    };

    let srvr = lstnr
        .incoming()
        .for_each(|mut strm| {
            client::handle(&mut strm);
            Ok(())
        })
        .map_err(|err| log::error!("{:?}", err));

    log::info!("Listening on {}", LSTN_ADDR.to_string());
    tokio::run(srvr);
}
