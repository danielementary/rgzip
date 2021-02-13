use std::cmp::Ordering;

use super::{Bit, Bits, SymbolLengthPair, SymbolType};

#[derive(PartialEq, Debug)]
pub struct Inode {
    pub left_child: Box<HuffmanNode>,
    pub right_child: Box<HuffmanNode>,
    pub cached_weight: i32,
}

#[derive(PartialEq, Debug)]
pub struct Lnode {
    pub symbol: SymbolType,
    pub weight: i32,
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

#[derive(Debug)]
pub enum HuffmanNode {
    Inode(Inode),
    Lnode(Lnode),
}

struct Decode<'a> {
    symbol: SymbolType,
    remaining_bits: &'a Bits,
}

impl HuffmanNode {
    pub fn weight(&self) -> i32 {
        match self {
            HuffmanNode::Inode(i_node) => i_node.cached_weight,
            HuffmanNode::Lnode(l_node) => l_node.weight,
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

    fn build_symbol_length_pairs_helper(&self, length: i32, result: &mut Vec<SymbolLengthPair>) {
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::VecDeque;

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
}
