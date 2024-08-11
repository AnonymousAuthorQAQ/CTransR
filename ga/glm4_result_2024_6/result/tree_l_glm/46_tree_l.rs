use std::io;
use std::collections::Vec;
use std::io::prelude::*;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|token| token.parse::<i32>().ok())
        .collect();

    let mut bst = BinarySearchTree::new();
    for &num in &numbers {
        bst.insert(num);
    }

    println!("Tree after insertion:");
    bst.print_in_order();

    bst.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    bst.print_in_order();

    if let Some((min, max)) = bst.min_max() {
        println!("Min is {}, Max is {}", min, max);
    } else {
        println!("Tree is empty");
    }
}

#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

struct BinarySearchTree {
    root: Option<Box<TreeNode>>,
}

impl BinarySearchTree {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn insert(&mut self, value: i32) {
        self.root = Some(Self::insert_into(value, self.root.take()));
    }

    fn insert_into(value: i32, mut node: Option<Box<TreeNode>>) -> Box<TreeNode> {
        match node {
            None => Box::new(TreeNode::new(value)),
            Some(mut node) => {
                if value < node.value {
                    node.left = Self::insert_into(value, node.left.take());
                } else {
                    node.right = Self::insert_into(value, node.right.take());
                }
                node
            }
        }
    }

    fn delete_even_numbers(&mut self) {
        if let Some(root) = self.root.take() {
            self.root = Some(Self::delete_even_numbers_from(root));
        }
    }

    fn delete_even_numbers_from(mut node: Box<TreeNode>) -> Box<TreeNode> {
        if node.value % 2 == 0 {
            if let Some(right) = node.right.take() {
                return Self::delete_even_numbers_from(right);
            } else if let Some(left) = node.left.take() {
                return Self::delete_even_numbers_from(left);
            } else {
                return Box::new(TreeNode::new(0)); // Return an empty node
            }
        }

        node.left = node.left.take().map(|left| Self::delete_even_numbers_from(left));
        node.right = node.right.take().map(|right| Self::delete_even_numbers_from(right));
        node
    }

    fn print_in_order(&self) {
        if let Some(root) = &self.root {
            Self::print_in_order_from(root);
        }
    }

    fn print_in_order_from(node: &Box<TreeNode>) {
        if let Some(left) = &node.left {
            Self::print_in_order_from(left);
        }
        println!("{}", node.value);
        if let Some(right) = &node.right {
            Self::print_in_order_from(right);
        }
    }

    fn min_max(&self) -> Option<(i32, i32)> {
        if let Some(root) = &self.root {
            Some(Self::min_max_from(root))
        } else {
            None
        }
    }

    fn min_max_from(node: &Box<TreeNode>) -> (i32, i32) {
        let (mut min, mut max) = (node.value, node.value);
        if let Some(left) = &node.left {
            let (left_min, left_max) = Self::min_max_from(left);
            min = std::cmp::min(min, left_min);
            max = std::cmp::max(max, left_max);
        }
        if let Some(right) = &node.right {
            let (right_min, right_max) = Self::min_max_from(right);
            min = std::cmp::min(min, right_min);
            max = std::cmp::max(max, right_max);
        }
        (min, max)
    }
}