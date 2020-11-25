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

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::{Config, generate_uuid};

    fn _check_regex(test: &String) -> bool {
        let regex = Regex::new(
            r"^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$"
        ).unwrap();

        regex.is_match(test)
    }

    #[test]
    fn generate_1_uuid() {
        let config = Config { count: 1 };
        let uuids = generate_uuid(&config).unwrap();

        assert_eq!(uuids.len(), 1);
        assert!(_check_regex(&uuids[0]));
    }

    #[test]
    fn generate_3_uuids() {
        let config = Config { count: 3 };
        let uuids = generate_uuid(&config).unwrap();

        assert_eq!(uuids.len(), 3);
        assert_ne!(uuids.windows(2).any(|w| w[0] == w[1]), true);

        for uuid in uuids.iter() {
            assert!(_check_regex(&uuid));
        }
    }
}
