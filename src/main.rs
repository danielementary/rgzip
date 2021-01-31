use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let bytes = fs::read(filename).expect("Something went wrong reading the file");

    println!("Compressing file {}", filename);

    println!("With bytes:");
    for byte in bytes.iter() {
        println!("\t{:#010b}", byte);
    }
}
