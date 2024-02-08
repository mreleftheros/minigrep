use minigrep::{Config, run};
use std::{env, process};

fn main() {
    let config = Config::from(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("{}", e);
        process::exit(1);
    }
}