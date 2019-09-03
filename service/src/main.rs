//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use ctrlc;
use log;

use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;

mod db;
mod logging;

fn main() {
    logging::init();

    //more will have to be done here later
    ctrlc::set_handler(move || {
        log::warn!("^C/SIGINT Caught ... ");
        std::process::exit(0);
    });

    log::info!("Starting up ...");

    thread::spawn(db::Conn::open(db::PATH));
}
