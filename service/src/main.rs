//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use log;

mod logging;

fn main() {
    logging::init();
    log::info!("Starting up ...");
}
