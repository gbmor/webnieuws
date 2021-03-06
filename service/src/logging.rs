//
// webnieuws - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use std::fs;

use chrono::offset::Utc;
use simplelog::*;

pub const FILE: &str = "logs/webnieuws-svc.log";

pub fn init() {
    // If the log file exists on startup,
    // move and timestamp it so we get a
    // fresh log file.
    if fs::metadata(FILE).is_ok() {
        let mut newpath = FILE.to_string();
        let time = Utc::now().to_rfc3339();
        newpath.push_str(".");
        newpath.push_str(&time);
        fs::rename(FILE, newpath).unwrap();
    }

    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Stderr).unwrap(),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            fs::File::create(FILE).unwrap(),
        ),
    ])
    .expect("Unable to initialize logging");
}

#[cfg(test)]
mod tests {
    use super::*;

    use log::info;

    #[test]
    fn init_logs() {
        let blank = " ".bytes().collect::<Vec<u8>>();
        fs::write(FILE, &blank).unwrap();
        init();

        info!("TEST LOG MESSAGE");
        let logfile = fs::read_to_string(FILE).unwrap();
        assert!(logfile.contains("TEST LOG MESSAGE"));
    }
}
