use std::{env::Args, error, fs};

#[derive(Debug)]
pub struct Config {
    path: String,
    query: String,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Self, &'static str> {
        args.next().ok_or("first argument should be valid")?;
        let path = args.next().ok_or("must provide a path")?;
        let query = args.next().ok_or("must provide a query")?;

        Ok(Self { path, query })
    }
}

fn get_results<'a, 'b>(file: &'a str, query: &'b str) -> Vec<&'a str> {
    file.lines()
        .filter(|&l| l.contains(query))
        .collect::<Vec<_>>()
}

pub fn run(mut args: Args) -> Result<(), Box<dyn error::Error>> {
    let config = Config::new(args)?;
    let file = fs::read_to_string(&config.path)?;
    let results = get_results(&file, &config.query);
    dbg!(results);
    Ok(())
}
