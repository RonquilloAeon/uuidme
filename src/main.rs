use std::env;
use std::process;

use uuidme::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error in arguments: {}", err);
        process::exit(1);
    });

    let uuids = uuidme::generate_uuid(&config).unwrap_or_else(|err| {
        eprintln!("Unable to generate a new UUID: {}", err);
        process::exit(1);
    });

    match config.count() {
        1 => println!("Generated 1 UUID"),
        count => println!("Generated {} UUIDs:", count),
    }

    for uuid in uuids.iter() {
        println!("{}", uuid);
    }

    process::exit(0);
}
