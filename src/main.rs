use std::env;
use std::process;

use uuidme::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error in arguments: {}", err);
        process::exit(1);
    });

    uuidme::run(&config).unwrap_or_else(|err| {
        println!("Unable to generate UUID - {}", err);
        process::exit(1);
    });

    process::exit(0);
}
