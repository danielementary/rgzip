use std::collections::{BTreeMap, VecDeque};
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

struct SymbolLengthPair {
    symbol: Byte,
    length: i32,
}

struct HuffmanTree {
    symbol_length_pairs: Vec<SymbolLengthPair>, // these pairs order defines lexicograpical order of codes
    lengths_counts: BTreeMap<i32, i32>,
    root: HuffmanNode,
}

enum HuffmanNode {
    Inode(Box<HuffmanNode>, Box<HuffmanNode>),
    Lnode(Byte),
}

enum Bit {
    Zero,
    One,
}

type Bits = VecDeque<Bit>;
type Byte = u8;

struct Decode<'a> {
    symbol: Byte,
    remaining_bits: &'a Bits,
}

impl HuffmanNode {
    fn decode<'a>(&self, bits: &'a mut Bits) -> Decode<'a> {
        match self {
            HuffmanNode::Inode(left_child, right_child) => {
                let current_bit = bits.pop_front().expect("Not enough bits to decode");
                let child = match current_bit {
                    Bit::Zero => left_child,
                    Bit::One => right_child,
                };

                child.decode(bits)
            }
            HuffmanNode::Lnode(symbol) => Decode {
                symbol: *symbol,
                remaining_bits: bits,
            },
        }
    }
}

impl HuffmanTree {
    fn build_lengths_counts(symbol_length_pairs: &Vec<SymbolLengthPair>) -> BTreeMap<i32, i32> {
        let lengths: Vec<i32> = symbol_length_pairs.iter().map(|pair| pair.length).collect();
        let mut lengths_counts: BTreeMap<i32, i32> = BTreeMap::new();

        for length in lengths {
            let count = lengths_counts.entry(length).or_insert(0);
            *count += 1;
        }

        lengths_counts.insert(0, 0);

        lengths_counts
    }

    fn build_lengths_codes(lengths_counts: &BTreeMap<i32, i32>) -> BTreeMap<i32, i32> {
        let mut lengths_codes: BTreeMap<i32, i32> = BTreeMap::new();
        let mut code = 0;
        let mut previous = 0;

        for length in lengths_counts.keys() {
            code = (code + lengths_counts.get(&previous).unwrap()) << 1;
            previous = *length;

            lengths_codes.insert(*length, code);
        }

        lengths_codes
    }

    fn build_symbols_codes(
        symbol_length_pairs: &Vec<SymbolLengthPair>,
        lengths_codes: &mut BTreeMap<i32, i32>,
    ) -> BTreeMap<Byte, i32> {
        let mut symbol_codes: BTreeMap<Byte, i32> = BTreeMap::new();
        for SymbolLengthPair { symbol, length } in symbol_length_pairs.iter() {
            let code = lengths_codes.get_mut(&length).unwrap();
            symbol_codes.insert(*symbol, *code);
            *code += 1;
        }

        symbol_codes
    }

    fn build_huffman_tree(symbol_length_pairs: Vec<SymbolLengthPair>) -> HuffmanTree {
        let lengths_counts = HuffmanTree::build_lengths_counts(&symbol_length_pairs);
        let mut lengths_codes = HuffmanTree::build_lengths_codes(&lengths_counts);
        let symbols_codes =
            HuffmanTree::build_symbols_codes(&symbol_length_pairs, &mut lengths_codes);

        //TODO create proper root
        let root = HuffmanNode::Lnode(0);

        HuffmanTree {
            symbol_length_pairs,
            lengths_counts,
            root,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn symbol_length_pairs_RFC_1951() -> Vec<SymbolLengthPair> {
        let symbol_length_pair_A = SymbolLengthPair {
            symbol: b'A',
            length: 3,
        };

        let symbol_length_pair_B = SymbolLengthPair {
            symbol: b'B',
            length: 3,
        };

        let symbol_length_pair_C = SymbolLengthPair {
            symbol: b'C',
            length: 3,
        };

        let symbol_length_pair_D = SymbolLengthPair {
            symbol: b'D',
            length: 3,
        };

        let symbol_length_pair_E = SymbolLengthPair {
            symbol: b'E',
            length: 3,
        };

        let symbol_length_pair_F = SymbolLengthPair {
            symbol: b'F',
            length: 2,
        };

        let symbol_length_pair_G = SymbolLengthPair {
            symbol: b'G',
            length: 4,
        };

        let symbol_length_pair_H = SymbolLengthPair {
            symbol: b'H',
            length: 4,
        };

        vec![
            symbol_length_pair_A,
            symbol_length_pair_B,
            symbol_length_pair_C,
            symbol_length_pair_D,
            symbol_length_pair_E,
            symbol_length_pair_F,
            symbol_length_pair_G,
            symbol_length_pair_H,
        ]
    }

    fn lengths_counts_RFC_1951() -> BTreeMap<i32, i32> {
        let mut expected_lengths_counts: BTreeMap<i32, i32> = BTreeMap::new();

        expected_lengths_counts.insert(0, 0);
        expected_lengths_counts.insert(2, 1);
        expected_lengths_counts.insert(3, 5);
        expected_lengths_counts.insert(4, 2);

        expected_lengths_counts
    }

    fn lengths_codes_RFC_1951() -> BTreeMap<i32, i32> {
        let mut lengths_codes: BTreeMap<i32, i32> = BTreeMap::new();

        lengths_codes.insert(0, 0);
        lengths_codes.insert(2, 0);
        lengths_codes.insert(3, 2);
        lengths_codes.insert(4, 14);

        lengths_codes
    }

    #[test]
    fn basic_node_decode() {
        let left_value = 10;
        let left_child = HuffmanNode::Lnode(left_value);

        let right_value = 20;
        let right_child = HuffmanNode::Lnode(right_value);

        let tree = HuffmanNode::Inode(Box::new(left_child), Box::new(right_child));

        let mut zero: Bits = VecDeque::from(vec![Bit::Zero]);
        let decoded_zero = tree.decode(&mut zero);
        assert!(decoded_zero.symbol == left_value);
        assert!(decoded_zero.remaining_bits.len() == 0);
        assert!(zero.len() == 0);

        let mut one: Bits = VecDeque::from(vec![Bit::One]);
        let decoded_one = tree.decode(&mut one);
        assert!(decoded_one.symbol == right_value);
        assert!(decoded_one.remaining_bits.len() == 0);
        assert!(one.len() == 0);
    }

    #[test]
    fn build_lengths_counts_RFC_1951() {
        let lengths_counts = HuffmanTree::build_lengths_counts(&symbol_length_pairs_RFC_1951());

        assert_eq!(lengths_counts, lengths_counts_RFC_1951());
    }

    #[test]
    fn build_lengths_codes_RFC_1951() {
        let lengths_codes = HuffmanTree::build_lengths_codes(&lengths_counts_RFC_1951());

        assert_eq!(lengths_codes, lengths_codes_RFC_1951());
    }
}
