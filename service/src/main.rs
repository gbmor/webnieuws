//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use ctrlc;
use log;
use tokio::net::TcpListener;
use tokio::prelude::*;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::thread;

mod client;
mod db;
mod error;
mod json;
mod logging;
mod post;
mod user;

lazy_static! {
    static ref LSTN_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 9001);
    static ref LSTN_ADDR_SBMT: SocketAddr =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 9002);
}

fn main() {
    logging::init();
    db::load_cache();
    thread::spawn(db::cache_ticker);

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

    // The thread to accept posts can die,
    // but we will block on the requests
    // handle to continue serving posts.
    thread::spawn(listen_for_posts);
    let reqs_handle = thread::spawn(listen_for_reqs);
    reqs_handle.join().unwrap();
}

// Vomits out the posts when a
// connection is made.
fn listen_for_reqs() {
    let lstnr = error::helper(TcpListener::bind(&LSTN_ADDR));

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

// Accepts posts, updates, deletes.
fn listen_for_posts() {
    let lstnr = error::helper(TcpListener::bind(&*LSTN_ADDR_SBMT));
    let srvr = lstnr
        .incoming()
        .for_each(|mut strm| {
            user::handle(&mut strm);
            Ok(())
        })
        .map_err(|err| log::error!("{:?}", err));

    log::info!("Listening on {}", LSTN_ADDR_SBMT.to_string());
    tokio::run(srvr);
}
