use std::env;
use std::error::Error;

use uuid::Uuid;

pub struct Config {
    count: u8,
}

impl Config {
    pub fn count(&self) -> &u8 {
        &self.count
    }
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, Box<dyn Error>> {
        args.next();

        let count = match args.next() {
            Some(arg) => arg.parse::<u8>()?,
            None => 1,
        };

        Ok(Config { count })
    }
}

pub fn generate_uuid(config: &Config) -> Result<Vec<String>, Box<dyn Error>> {
    let mut uuids: Vec<String> = vec![];

    for _ in 0..config.count {
        uuids.push(Uuid::new_v4().to_hyphenated().to_string())
    }

    Ok(uuids)
}
