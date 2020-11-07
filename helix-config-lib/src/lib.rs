#[macro_use]
extern crate serde_derive;

use crate::version::Version;
use std::env;
pub mod version;

pub struct Configuration {
    version: Version,
}

impl Configuration {
    pub fn new(version: Version) -> Self {
        //Load configuration into env variables.
        match env::args().len() {
            1 => {
                //_Defaut file loaded.
                println!("[Configuration file] Default file loaded");
                dotenv::dotenv().expect("File .env not found");
            }
            _ => {
                //_Load first file params.
                let arguments: Vec<String> = env::args().collect();
                println!("[Configuration file] '{}' file loaded.", arguments[1]);
                dotenv::from_filename(&arguments[1]).expect("File .env not found");
            }
        }

        Configuration { version }
    }

    pub fn get_served_addr(&self) -> String {
        let mut addr = String::new();
        let ip = env::var("IP").expect("IP not found.");
        let port = env::var("PORT").expect("PORT not found.");

        //Return string of {IP}:{PORT}
        addr.push_str(&ip);
        addr.push_str(&":".to_owned());
        addr.push_str(&port);
        addr
    }

    pub fn get_workers_number(&self) -> usize {
        let str_value = env::var("ACTIX_WORKERS").expect("ACTIX_WORKERS not found.");
        str_value.parse::<usize>().unwrap()
    }

    pub fn get_shutdown_time_out(&self) -> u64 {
        let str_value =
            env::var("ACTIX_SHUTDOWN_TIMEOUT").expect("ACTIX_SHUTDOWN_TIMEOUT not found.");
        str_value.parse::<u64>().unwrap()
    }

    pub fn get_keep_alive(&self) -> usize {
        let str_value = env::var("ACTIX_KEEP_ALIVE").expect("ACTIX_KEEP_ALIVE not found.");
        str_value.parse::<usize>().unwrap()
    }

    pub fn version(&self) -> &Version {
        &self.version
    }
}
