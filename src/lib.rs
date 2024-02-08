use std::{env::Args, error, fmt::Error};

pub struct Config {
  path: String,
  query: String
}

impl Config {
  pub fn from(mut args: Args) -> Result<Self, &'static str> {
    let path = match args.next() {
      Some(v) => v,
      None => return Err("No path input.")
    };

    let query = match args.next() {
      Some(v) => v,
      None => return Err("No query input.")
    };

    Ok(
      Self {
        path,
        query
      }
    )
  }
}

pub fn run(c: Config) -> Result<(), Box<dyn error::Error>> {
  println!("run");
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(1, 1);
  }
}