//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

#![feature(proc_macro_hygiene, decl_macro)]
#![feature(test)]
extern crate test;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;

use ctrlc;
use log;
use rocket::config::{Config, Environment};

use std::net::TcpListener;
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
    static ref LSTN_ADDR: SocketAddr =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9001);
    static ref LSTN_ADDR_SBMT: SocketAddr =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9002);
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

    let config = Config::build(Environment::Staging)
        .address("127.0.0.1")
        .port(9001)
        .workers(8)
        .unwrap();

    rocket::custom(config)
        .mount("/", routes![client::handle, user::handle])
        .launch();
}
