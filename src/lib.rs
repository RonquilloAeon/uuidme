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

pub fn generate_uuid(count: &u8) -> Result<Vec<String>, Box<dyn Error>> {
    let mut uuids: Vec<String> = vec![];

    for _ in 0..*count {
        uuids.push(Uuid::new_v4().to_hyphenated().to_string())
    }

    Ok(uuids)
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let uuids = generate_uuid(&config.count())?;

    // Print message
    let message = match config.count() {
        1 => format!("Generated 1 UUID"),
        count => format!("Generated {} UUIDs", count),
    };

    match term_size::dimensions() {
        Some((w, _)) => {
            let dash_size = ((w - message.len()) / 2) - 2;
            let dash = (0..dash_size).map(|_| "-").collect::<String>();

            println!("{}| {} |{}", dash, message, dash);
        },
        _ => println!("{}", message),
    };

    // Print UUID(s)
    for uuid in uuids.iter() {
        println!("{}", uuid);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::generate_uuid;

    fn _check_regex(test: &String) -> bool {
        let regex = Regex::new(
            r"^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$"
        ).unwrap();

        regex.is_match(test)
    }

    #[test]
    fn generate_1_uuid() {
        let uuids = generate_uuid(&(1 as u8)).unwrap();

        assert_eq!(uuids.len(), 1);
        assert!(_check_regex(&uuids[0]));
    }

    #[test]
    fn generate_3_uuids() {
        let uuids = generate_uuid(&(3 as u8)).unwrap();

        assert_eq!(uuids.len(), 3);
        assert_ne!(uuids.windows(2).any(|w| w[0] == w[1]), true);

        for uuid in uuids.iter() {
            assert!(_check_regex(&uuid));
        }
    }
}
