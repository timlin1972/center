use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use actix_web::{web, Responder};
use log::info;
use serde::Serialize;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Serialize, Debug)]
struct Log {
    log: Vec<String>,
}

impl Default for Log {
    fn default() -> Self {
        Self::new()
    }
}

impl Log {
    pub fn new() -> Log {
        Log { log: vec![] }
    }
}

pub async fn get() -> impl Responder {
    info!(">>> recv: get");

    let mut log = Log::new();

    info!(">>> read: {}", common::LOG_FILE);
    if let Ok(lines) = read_lines(common::LOG_FILE) {
        for line in lines.flatten() {
            if line.contains(common::LOG_SERVER) {
                log.log.clear();
            }
            log.log.push(line);
        }
    }

    web::Json(log)
}
