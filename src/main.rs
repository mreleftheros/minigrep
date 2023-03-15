use std::env;

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
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let env = Env::from_args(&args);

    println!("{:?}", env);
}