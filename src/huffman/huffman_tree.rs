use std::collections::BinaryHeap;

use super::huffman_node::{HuffmanNode, Inode, Lnode};
use super::SymbolWeightPair;

struct HuffmanTree {
    symbol_weight_pairs: Vec<SymbolWeightPair>,
    tree: HuffmanNode,
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
