use std::error::Error;
use std::fs;

pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Compressing file {}", config.filename);

    let bytes = fs::read(config.filename)?;

    println!("With bytes:");
    for byte in bytes.iter() {
        println!("\t{:#010b}", byte);
    }

    Ok(())
}
