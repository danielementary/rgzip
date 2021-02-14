use std::error::Error;
use std::{fs, str};

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
    println!("Decompression implementation is in progress...");

    let bytes = fs::read(config.filename)?;
    let mut bytes = bytes.iter();

    // refactor repetitions
    let id1 = bytes.next().expect("This file does not contain ID1");
    if *id1 != 0x1f {
        return Err("ID1 does not match".into());
    }

    let id2 = bytes.next().expect("This file does not contain ID2");
    if *id2 != 0x8b {
        return Err("ID2 does not match".into());
    }

    let cm = bytes.next().expect("This file does not contain CM");
    if *cm != 0x8 {
        return Err("Only 0x8 is a valid CM standing for deflate".into());
    }

    let flg = bytes.next().expect("This file does not contain FLG");
    if *flg != 0x8 {
        return Err("Only 0x8 is a valid FLG standing for original filename".into());
    }

    // TODO: interpret date
    let mtime_0 = bytes.next().expect("This file does not contain MTIME");
    let mtime_1 = bytes.next().expect("This file does not contain MTIME");
    let mtime_2 = bytes.next().expect("This file does not contain MTIME");
    let mtime_3 = bytes.next().expect("This file does not contain MTIME");

    let xfl = bytes.next().expect("This file does not contain XFL");
    if *xfl != 0x0 {
        println!("XFL is ignored as it is different from 0");
    }

    let os = bytes.next().expect("This file does not contain OS");
    if *os != 0x3 {
        return Err("Only 0x3 is a valid OS standing for Unix".into());
    }

    let mut filename: Vec<u8> = vec![];
    while let Some(byte) = bytes.next() {
        if *byte == 0x0 {
            break;
        }

        filename.push(*byte);
    }
    let filename = str::from_utf8(&filename)?;

    // read compressed blocks
    let header = bytes.next().expect("This block does not contain an header");

    let bfinal_set = header & 1 == 1;
    let btype_0_set = header >> 1 & 1 == 1;
    let btype_1_set = header >> 2 & 1 == 1;

    if !bfinal_set {
        panic!("This implementation only supports one block compressed files for now");
    }

    match (btype_0_set, btype_1_set) {
        (false, false) => {} // no compression
        (false, true) => {}  // compressed with fixed Huffman codes
        (true, false) => {}  //compressed with dynamic Huffman codes
        (true, true) => panic!("BTYPE is 11"),
    }

    // read CRC32

    // read ISIZE

    Ok(())
}
