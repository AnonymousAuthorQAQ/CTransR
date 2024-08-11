use std::io::{self, Read};
use std::collections::Vec;
use std::str::FromStr;

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

    fn insert(&mut self, value: i32) {
        match value.cmp(&self.value) {
            std::cmp::Ordering::Less => {
                if self.left.is_none() {
                    self.left = Some(Box::new(TreeNode::new(value)));
                } else {
                    self.left.as_mut().unwrap().insert(value);
                }
            }
            std::cmp::Ordering::Greater => {
                if self.right.is_none() {
                    self.right = Some(Box::new(TreeNode::new(value)));
                } else {
                    self.right.as_mut().unwrap().insert(value);
                }
            }
            std::cmp::Ordering::Equal => {
                // Do nothing for duplicate values
            }
        }
    }

    fn delete_even_numbers(&mut self) {
        if let Some(left) = self.left.take() {
            if left.value % 2 == 0 {
                self.left = left.left;
            } else {
                self.left = Some(left);
            }
            if let Some(left) = self.left {
                left.delete_even_numbers();
            }
        }

        if let Some(right) = self.right.take() {
            if right.value % 2 == 0 {
                self.right = right.left;
            } else {
                self.right = Some(right);
            }
            if let Some(right) = self.right {
                right.delete_even_numbers();
            }
        }
    }

    fn in_order_traversal(&self) {
        if let Some(left) = &self.left {
            left.in_order_traversal();
        }
        println!("{}", self.value);
        if let Some(right) = &self.right {
            right.in_order_traversal();
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    let mut root = TreeNode::new(numbers[0]);
    for &value in &numbers[1..] {
        root.insert(value);
    }

    println!("Tree after insertion:");
    root.in_order_traversal();

    root.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    root.in_order_traversal();
}