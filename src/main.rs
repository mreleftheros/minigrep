use std::env;

struct Env<'a> {
    query: &'a str,
    path: &'a str
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
}