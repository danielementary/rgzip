use std::collections::{BTreeMap, VecDeque};

use super::{Bit, Bits, SymbolLengthPair, SymbolType};

struct HuffmanLUT {
    symbol_length_pairs: Vec<SymbolLengthPair>, // these pairs order defines lexicograpical order of codes
    lookup_table: BTreeMap<SymbolType, Bits>,
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
    ) -> BTreeMap<SymbolType, Bits> {
        let mut lookup_table: BTreeMap<SymbolType, Bits> = BTreeMap::new();
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

    fn build_huffman_lut(symbol_length_pairs: Vec<SymbolLengthPair>) -> HuffmanLUT {
        let lengths_counts = HuffmanLUT::build_lengths_counts(&symbol_length_pairs);
        let mut lengths_codes = HuffmanLUT::build_lengths_codes(&lengths_counts);
        let lookup_table = HuffmanLUT::build_lookup_table(&symbol_length_pairs, &mut lengths_codes);

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

    fn lookup_table_RFC_1951() -> BTreeMap<SymbolType, Bits> {
        let mut lookup_table: BTreeMap<SymbolType, Bits> = BTreeMap::new();

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
