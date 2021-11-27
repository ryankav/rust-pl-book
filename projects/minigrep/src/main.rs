use std::env;
use std::fs;
use std::io;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = env::args().collect();
  
	let config = check_args(&args)?;
	
	let _file_string = read_file(&args[2])?;

  config.print_config();

	Ok(())
}

fn check_args(args: &[String]) -> Result<Config, &str> {
	let query = match args.get(1) {
		Some(s) => Ok(s.clone()),
		None => Err("Query string isn't defined"),
	}?;

	let filename = match args.get(2) {
		Some(s) => Ok(s.clone()),
		None => Err("Filename isn't defined"),
	}?;

	Ok(Config { query, filename })
}

fn read_file(filename: &str) -> Result<String, io::Error> {
	fs::read_to_string(filename)
}

struct Config {
	query: String,
	filename: String,
}

impl Config {
	fn print_config(&self) {
		println!("Searching for {}", self.query);
		println!("In file {}", self.filename);
	}
}
