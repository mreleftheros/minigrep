use std::{env, error, fs};

#[derive(Debug)]
pub struct Config {
    path: String,
    query: String,
    case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next().ok_or("first argument should be valid")?;
        let path = args.next().ok_or("must provide a path")?;
        let query = args.next().ok_or("must provide a query")?;
        let case_insensitive = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            path,
            query,
            case_insensitive,
        })
    }
}

fn get_results<'a, 'b>(file: &'a str, query: &'b str) -> Vec<&'a str> {
    file.lines()
        .filter(|&l| l.contains(query))
        .collect::<Vec<_>>()
}

fn get_results_insensitive<'a, 'b>(file: &'a str, query: &'b str) -> Vec<&'a str> {
    file.lines()
        .filter(|&l| {
            l.to_lowercase()
                .as_str()
                .contains(query.to_lowercase().as_str())
        })
        .collect::<Vec<_>>()
}

pub fn run(mut args: env::Args) -> Result<(), Box<dyn error::Error>> {
    let config = Config::new(args)?;
    let file = fs::read_to_string(&config.path)?;
    let results = if config.case_insensitive {
        get_results_insensitive(&file, &config.query)
    } else {
        get_results(&file, &config.query)
    };
    dbg!(results);
    Ok(())
}
