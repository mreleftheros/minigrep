use std::{env, error, process, fs};

#[derive(Debug)]
struct Env<'a> {
    query: &'a str,
    path: &'a str
}

impl<'a> Env<'a> {
    fn from_args(args: &'a[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        Ok(
            Self {
                query: &args[1],
                path: &args[2]
            }
        )
    }

    fn get_search_results(&self, file: &str) -> Vec<&str> {
        let q = self.query;
        let lines = file.lines();
        let mut results: Vec<&str> = Vec::with_capacity(4);

        for line in lines {
            if line.contains(self.query) {
                results.push(line)
            }
        }

        results
    }
}

fn run(args: &[String]) -> Result<(), Box<dyn error::Error>> {
    let env = Env::from_args(&args)?;
    let file = fs::read_to_string(&env.path)?;
    let results = env.get_search_results(&file);
    println!("Found: {:#?}", results);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(err) = run(&args) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}