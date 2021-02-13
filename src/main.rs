use std::env;
use std::process;

mod configuration;
pub use configuration::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = config.run() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
