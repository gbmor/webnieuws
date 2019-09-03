//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use ctrlc;
use log;

use std::os::unix::net::UnixListener;
use std::thread;

mod db;
mod logging;

const SOCK_PATH: &str = "/tmp/webnieuws.sock";

fn main() {
    logging::init();

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
        thread::spawn(|| {
            let _db = db::Conn::open(db::PATH);
            log::info!("Database connection opened: {}", db::PATH);

            let lstnr = UnixListener::bind(SOCK_PATH).unwrap();
            log::info!("Listening on socket: {}", SOCK_PATH);
            for strm in lstnr.incoming() {
                match strm {
                    Ok(_stream) => {}
                    Err(err) => log::error!("{:?}", err),
                }
            }
        });
    }
}
