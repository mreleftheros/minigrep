use minigrep::run;
use std::{env, process};

fn main() {
    minigrep::run(env::args()).unwrap_or_else(|e| {
        eprint!("{}", e);
        process::exit(1);
    })
}
