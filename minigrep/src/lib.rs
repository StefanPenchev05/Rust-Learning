use std::{ error::Error, fs::File, io::{ BufRead, BufReader, ErrorKind } };
use colored::Colorize;
pub struct Config {
    query: String,
    file_path: String,
}

pub trait Args {
    fn new(args: &[String]) -> Result<Self, Box<dyn Error>> where Self: Sized;
    fn run(&self) -> Result<(), Box<dyn Error>>;
}

impl Args for Config {
    fn new(args: &[String]) -> Result<Self, Box<dyn Error>> {
        let query = args.get(1).ok_or("No query provided")?.clone();
        let file_path = args.get(2).ok_or("No file path provided")?.clone();

        Ok(Self { query, file_path })
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        let file = File::open(&self.file_path).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                println!("File not found!");
                std::process::exit(1);
            } else {
                println!("An error occurred: {}", error);
                std::process::exit(1);
            }
        });

        let reader = BufReader::new(file);
        let mut lines: Vec<String> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line.contains(&self.query) {
                lines.push(line.yellow().to_string());
            } else {
                lines.push(line.to_string());
            }
        }

        for line in lines {
            println!("{line}");
        }
        Ok(())
    }
}
