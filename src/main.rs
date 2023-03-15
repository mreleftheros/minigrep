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

    fn search(&self, file: &str) {
        let q = self.query;
        let lines = file.lines();

        for line in lines {
            println!("{}", line);
        }
    }
}

fn run(args: &[String]) -> Result<(), Box<dyn error::Error>> {
    let env = Env::from_args(&args)?;
    let file = fs::read_to_string(&env.path)?;
    env.search(&file);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(err) = run(&args) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}