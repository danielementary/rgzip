pub mod configuration {
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
}

mod huffman {
    use std::cmp::Ordering;
    use std::collections::{BTreeMap, BinaryHeap, VecDeque};

    struct SymbolLengthPair {
        symbol: Byte,
        length: i32,
    }

    struct SymbolWeightPair {
        symbol: Byte,
        weight: i32,
    }

    struct HuffmanTree {
        symbol_weight_pairs: Vec<SymbolWeightPair>,
        tree: HuffmanNode,
    }

    struct HuffmanLUT {
        symbol_length_pairs: Vec<SymbolLengthPair>, // these pairs order defines lexicograpical order of codes
        lookup_table: BTreeMap<Byte, Bits>,
    }

    #[derive(PartialEq)]
    struct Inode {
        left_child: Box<HuffmanNode>,
        right_child: Box<HuffmanNode>,
        cached_weight: i32,
    }

    #[derive(PartialEq)]
    struct Lnode {
        symbol: Byte,
        weight: i32,
    }

    impl PartialEq for HuffmanNode {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (HuffmanNode::Inode(s), HuffmanNode::Inode(o)) => s == o,
                (HuffmanNode::Lnode(s), HuffmanNode::Lnode(o)) => s == o,
                _ => false,
            }
        }
    }

    impl Eq for HuffmanNode {}

    impl PartialOrd for HuffmanNode {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for HuffmanNode {
        fn cmp(&self, other: &Self) -> Ordering {
            self.weight().cmp(&other.weight())
        }
    }

    enum HuffmanNode {
        Inode(Inode),
        Lnode(Lnode),
    }

    #[derive(PartialEq, Debug, Clone)]
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

    impl HuffmanTree {
        fn build_huffman_tree(symbol_weight_pairs: Vec<SymbolWeightPair>) -> HuffmanTree {
            let mut forest: BinaryHeap<HuffmanNode> = symbol_weight_pairs
                .iter()
                .map(|SymbolWeightPair { symbol, weight }| {
                    HuffmanNode::Lnode(Lnode {
                        symbol: *symbol,
                        weight: *weight,
                    })
                })
                .collect();

            if forest.len() < 2 {
                panic!("forest is too small to build a huffman tree");
            }

            while forest.len() > 1 {
                let left_child = Box::new(forest.pop().unwrap());
                let right_child = Box::new(forest.pop().unwrap());
                let cached_weight = left_child.weight() + right_child.weight();

                let new_tree = HuffmanNode::Inode(Inode {
                    left_child,
                    right_child,
                    cached_weight,
                });

                forest.push(new_tree);
            }

            HuffmanTree {
                symbol_weight_pairs,
                tree: forest.pop().unwrap(),
            }
        }
    }

    impl HuffmanNode {
        fn weight(&self) -> i32 {
            match self {
                HuffmanNode::Inode(iNode) => iNode.cached_weight,
                HuffmanNode::Lnode(lNode) => lNode.weight,
            }
        }

        fn decode<'a>(&self, bits: &'a mut Bits) -> Decode<'a> {
            match self {
                HuffmanNode::Inode(Inode {
                    left_child,
                    right_child,
                    ..
                }) => {
                    let current_bit = bits.pop_front().expect("Not enough bits to decode");
                    let child = match current_bit {
                        Bit::Zero => left_child,
                        Bit::One => right_child,
                    };

                    child.decode(bits)
                }
                HuffmanNode::Lnode(Lnode { symbol, .. }) => Decode {
                    symbol: *symbol,
                    remaining_bits: bits,
                },
            }
        }

        fn build_symbol_length_pairs_helper(
            &self,
            length: i32,
            result: &mut Vec<SymbolLengthPair>,
        ) {
            match self {
                HuffmanNode::Inode(Inode {
                    left_child,
                    right_child,
                    ..
                }) => {
                    left_child.build_symbol_length_pairs_helper(length + 1, result);
                    right_child.build_symbol_length_pairs_helper(length + 1, result);
                }
                HuffmanNode::Lnode(Lnode { symbol, .. }) => {
                    result.push(SymbolLengthPair {
                        symbol: *symbol,
                        length,
                    });
                }
            }
        }

        fn build_symbol_length_pairs(&self) -> Vec<SymbolLengthPair> {
            let mut result: Vec<SymbolLengthPair> = Vec::new();

            self.build_symbol_length_pairs_helper(0, &mut result);

            result
        }
    }

    impl HuffmanLUT {
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

        fn build_lookup_table(
            symbol_length_pairs: &Vec<SymbolLengthPair>,
            lengths_codes: &mut BTreeMap<i32, i32>,
        ) -> BTreeMap<Byte, Bits> {
            let mut lookup_table: BTreeMap<Byte, Bits> = BTreeMap::new();
            for SymbolLengthPair { symbol, length } in symbol_length_pairs.iter() {
                let numerical_code = lengths_codes.get_mut(&length).unwrap();
                let mut code = VecDeque::new();
                let mut i = length - 1;
                while i >= 0 {
                    let bit = if *numerical_code & (1 << i) == 0 {
                        Bit::Zero
                    } else {
                        Bit::One
                    };

                    code.push_back(bit);

                    i -= 1;
                }
                lookup_table.insert(*symbol, code);
                *numerical_code += 1;
            }

            lookup_table
        }

        fn build_huffman_tree(symbol_length_pairs: Vec<SymbolLengthPair>) -> HuffmanLUT {
            let lengths_counts = HuffmanLUT::build_lengths_counts(&symbol_length_pairs);
            let mut lengths_codes = HuffmanLUT::build_lengths_codes(&lengths_counts);
            let lookup_table =
                HuffmanLUT::build_lookup_table(&symbol_length_pairs, &mut lengths_codes);

            HuffmanLUT {
                symbol_length_pairs,
                lookup_table: lookup_table,
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

        fn lookup_table_RFC_1951() -> BTreeMap<Byte, Bits> {
            let mut lookup_table: BTreeMap<Byte, Bits> = BTreeMap::new();

            lookup_table.insert(b'A', VecDeque::from(vec![Bit::Zero, Bit::One, Bit::Zero]));
            lookup_table.insert(b'B', VecDeque::from(vec![Bit::Zero, Bit::One, Bit::One]));
            lookup_table.insert(b'C', VecDeque::from(vec![Bit::One, Bit::Zero, Bit::Zero]));
            lookup_table.insert(b'D', VecDeque::from(vec![Bit::One, Bit::Zero, Bit::One]));
            lookup_table.insert(b'E', VecDeque::from(vec![Bit::One, Bit::One, Bit::Zero]));
            lookup_table.insert(b'F', VecDeque::from(vec![Bit::Zero, Bit::Zero]));
            lookup_table.insert(
                b'G',
                VecDeque::from(vec![Bit::One, Bit::One, Bit::One, Bit::Zero]),
            );
            lookup_table.insert(
                b'H',
                VecDeque::from(vec![Bit::One, Bit::One, Bit::One, Bit::One]),
            );

            lookup_table
        }

        #[test]
        fn basic_node_decode() {
            let left_symbol = 10;
            let left_weight = 1;
            let left_child = HuffmanNode::Lnode(Lnode {
                symbol: left_symbol,
                weight: left_weight,
            });

            let right_symbol = 20;
            let right_weight = 1;
            let right_child = HuffmanNode::Lnode(Lnode {
                symbol: right_symbol,
                weight: right_weight,
            });

            let tree = HuffmanNode::Inode(Inode {
                left_child: Box::new(left_child),
                right_child: Box::new(right_child),
                cached_weight: left_weight + right_weight,
            });

            let mut zero: Bits = VecDeque::from(vec![Bit::Zero]);
            let decoded_zero = tree.decode(&mut zero);
            assert_eq!(decoded_zero.symbol, left_symbol);
            assert_eq!(decoded_zero.remaining_bits.len(), 0);
            assert_eq!(zero.len(), 0);

            let mut one: Bits = VecDeque::from(vec![Bit::One]);
            let decoded_one = tree.decode(&mut one);
            assert_eq!(decoded_one.symbol, right_symbol);
            assert_eq!(decoded_one.remaining_bits.len(), 0);
            assert_eq!(one.len(), 0);
        }

        #[test]
        fn build_lengths_counts_RFC_1951() {
            let lengths_counts = HuffmanLUT::build_lengths_counts(&symbol_length_pairs_RFC_1951());

            assert_eq!(lengths_counts, lengths_counts_RFC_1951());
        }

        #[test]
        fn build_lengths_codes_RFC_1951() {
            let lengths_codes = HuffmanLUT::build_lengths_codes(&lengths_counts_RFC_1951());

            assert_eq!(lengths_codes, lengths_codes_RFC_1951());
        }

        #[test]
        fn build_lookup_table_RFC_1951() {
            let lookup_table = HuffmanLUT::build_lookup_table(
                &symbol_length_pairs_RFC_1951(),
                &mut lengths_codes_RFC_1951(),
            );

            assert_eq!(lookup_table, lookup_table_RFC_1951());
        }
    }
}
