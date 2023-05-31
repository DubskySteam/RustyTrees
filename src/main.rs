#![allow(non_snake_case)]

mod bst_tree;

use bst_tree::BST;

fn main() {

    //Binary Search Tree
    let example_bst = vec![15, 10, 22, 4, 12, 18, 24, 50, 35, 31, 44, 70, 66, 90];

    let mut _tree: BST = BST::new(25);
    for value in example_bst {
        _tree.insert(value);
    }
    _tree.print();
}
