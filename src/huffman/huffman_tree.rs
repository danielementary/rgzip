use std::cmp::Ordering;
use std::collections::BinaryHeap;

use super::huffman_node::{HuffmanNode, Inode, Lnode};
use super::SymbolWeightPair;

struct HuffmanTree {
    symbol_weight_pairs: Vec<SymbolWeightPair>,
    tree: HuffmanNode,
}

#[derive(PartialEq, Eq)]
struct WrappedHuffmanNode {
    huffman_node: HuffmanNode,
}

impl PartialOrd for WrappedHuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WrappedHuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.weight().cmp(&other.weight())
        // invert definition to mimic a min-heap using a max-heap
        other.huffman_node.weight().cmp(&self.huffman_node.weight())
    }
}

impl HuffmanTree {
    fn build_huffman_tree(symbol_weight_pairs: Vec<SymbolWeightPair>) -> HuffmanTree {
        let mut forest: BinaryHeap<WrappedHuffmanNode> = symbol_weight_pairs
            .iter()
            .map(|SymbolWeightPair { symbol, weight }| WrappedHuffmanNode {
                huffman_node: HuffmanNode::Lnode(Lnode {
                    symbol: *symbol,
                    weight: *weight,
                }),
            })
            .collect();

        if forest.len() < 2 {
            panic!("forest is too small to build a huffman tree");
        }

        while forest.len() > 1 {
            let left_child = Box::new(forest.pop().unwrap().huffman_node);
            let right_child = Box::new(forest.pop().unwrap().huffman_node);
            let cached_weight = left_child.weight() + right_child.weight();

            let new_tree = HuffmanNode::Inode(Inode {
                left_child,
                right_child,
                cached_weight,
            });

            forest.push(WrappedHuffmanNode {
                huffman_node: new_tree,
            });
        }

        HuffmanTree {
            symbol_weight_pairs,
            tree: forest.pop().unwrap().huffman_node,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn empty_forest() {
        let symbol_weight_pairs = vec![];

        HuffmanTree::build_huffman_tree(symbol_weight_pairs);
    }

    #[test]
    #[should_panic]
    fn small_forest() {
        let symbol_weight_pairs = vec![SymbolWeightPair {
            symbol: b'A',
            weight: 10,
        }];

        HuffmanTree::build_huffman_tree(symbol_weight_pairs);
    }

    #[test]
    fn simple_forest() {
        let symbol_weight_pair_A = SymbolWeightPair {
            symbol: b'A',
            weight: 10,
        };
        let symbol_weight_pair_B = SymbolWeightPair {
            symbol: b'B',
            weight: 20,
        };
        let symbol_weight_pair_C = SymbolWeightPair {
            symbol: b'C',
            weight: 5,
        };

        let symbol_weight_pairs = vec![
            symbol_weight_pair_A,
            symbol_weight_pair_B,
            symbol_weight_pair_C,
        ];
        let expected_symbol_weight_pairs = symbol_weight_pairs.clone();

        let huffman_tree = HuffmanTree::build_huffman_tree(symbol_weight_pairs);

        let left_right_symbol = b'A';
        let left_right_weight = 10;
        let left_right_child = HuffmanNode::Lnode(Lnode {
            symbol: left_right_symbol,
            weight: left_right_weight,
        });

        let right_symbol = b'B';
        let right_weight = 20;
        let right_child = HuffmanNode::Lnode(Lnode {
            symbol: right_symbol,
            weight: right_weight,
        });

        let left_left_symbol = b'C';
        let left_left_weight = 5;
        let left_left_child = HuffmanNode::Lnode(Lnode {
            symbol: left_left_symbol,
            weight: left_left_weight,
        });

        let expected_left_subtree = HuffmanNode::Inode(Inode {
            left_child: Box::new(left_left_child),
            right_child: Box::new(left_right_child),
            cached_weight: left_left_weight + left_right_weight,
        });
        let expected_left_subtree_weight = expected_left_subtree.weight();

        let expected_tree = HuffmanNode::Inode(Inode {
            left_child: Box::new(expected_left_subtree),
            right_child: Box::new(right_child),
            cached_weight: expected_left_subtree_weight + right_weight,
        });

        assert_eq!(
            huffman_tree.symbol_weight_pairs,
            expected_symbol_weight_pairs
        );
        assert_eq!(huffman_tree.tree, expected_tree);
    }
}
