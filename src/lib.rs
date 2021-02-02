use std::collections::VecDeque;
use std::error::Error;
use std::fs;

const COMPRESSED_FILE_EXTENSION: &str = ".rgz";

pub struct Config {
    filename: String,
    mode: Mode,
}

enum Mode {
    Compression,
    Decompression,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();
        let mode = {
            if has_compressed_file_extension(&filename) {
                Mode::Decompression
            } else {
                Mode::Compression
            }
        };

        Ok(Config { filename, mode })
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        match self.mode {
            Mode::Compression => compress(self),
            Mode::Decompression => decompress(self),
        }
    }
}

fn has_compressed_file_extension(filename: &str) -> bool {
    let len = filename.len();
    let ext_len = COMPRESSED_FILE_EXTENSION.len();

    filename[len - ext_len..] == COMPRESSED_FILE_EXTENSION.to_string()
}

fn compress(config: Config) -> Result<(), Box<dyn Error>> {
    println!("About to compress file {}", config.filename);

    let bytes = fs::read(config.filename)?;

    print!("With bytes: ");
    for byte in bytes.iter() {
        print!("{:#04X} ", byte);
    }

    Ok(())
}

fn decompress(config: Config) -> Result<(), Box<dyn Error>> {
    println!("About to decompress file {}", config.filename);
    println!("Decompression in not implemented yet");

    Ok(())
}

enum Huffman_node {
    Inode(Box<Huffman_node>, Box<Huffman_node>),
    Lnode(i32),
}

enum Bit {
    Zero,
    One,
}

type Bits = VecDeque<Bit>;

impl Huffman_node {
    fn decode<'a>(&self, bits: &'a mut Bits) -> (i32, &'a Bits) {
        match self {
            Huffman_node::Inode(left_child, right_child) => {
                let current_bit = bits.pop_front().unwrap();
                let child = match current_bit {
                    Zero => left_child,
                    One => right_child,
                };

                child.decode(bits)
            }
            Huffman_node::Lnode(value) => (*value, bits),
        }
    }
}
