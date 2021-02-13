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
type Byte = u8;

struct SymbolWeightPair {
    symbol: Byte,
    weight: i32,
}

struct SymbolLengthPair {
    symbol: Byte,
    length: i32,
}
