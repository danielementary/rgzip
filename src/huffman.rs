use std::collections::VecDeque;

mod huffman_lut;
mod huffman_node;
mod huffman_tree;

#[derive(PartialEq, Debug, Clone)]
enum Bit {
    Zero,
    One,
}

type Bits = VecDeque<Bit>;
type SymbolType = u8;

struct SymbolWeightPair {
    symbol: SymbolType,
    weight: i32,
}

struct SymbolLengthPair {
    symbol: SymbolType,
    length: i32,
}
