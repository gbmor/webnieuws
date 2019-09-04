//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate may;

use ctrlc;
use log;

use std::net::TcpListener;
use std::thread;

mod client;
mod db;
mod logging;

const LSTN_ADDR: &str = "0.0.0.0:9001";

fn main() {
    logging::init();
    db::load_cache();

    //more will have to be done here later
    ctrlc::set_handler(|| {
        log::warn!("^C/SIGINT Caught ... ");
        std::process::exit(0);
    })
    .expect("Failed to set up SIGINT handler");

    log::info!("Starting up ...");

    // This will loop, ensuring if the thread dies it's
    // immediately respawned.
    loop {
        thread::spawn(move || {
            let lstnr = TcpListener::bind(LSTN_ADDR).unwrap();
            log::info!("Listening on {}", LSTN_ADDR);

            for strm in lstnr.incoming() {
                match strm {
                    Ok(mut stream) => {
                        log::info!("New connection: {:?}", stream);
                        go!(move || client::handle(&mut stream));
                    }
                    Err(err) => log::error!("{:?}", err),
                }
            }
        })
        .join()
        .unwrap();
    }
}
