use std::io::{self, Write};
use std::ptr;
use std::collections::VecDeque;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

pub struct SearchTree {
    root: Option<Box<TreeNode>>,
}

impl SearchTree {
    pub fn new() -> Self {
        SearchTree { root: None }
    }

    pub fn insert(&mut self, x: i32) {
        self.root = Self::insert_rec(self.root.take(), x);
    }

    fn insert_rec(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_rec(n.left.take(), x);
                } else if x > n.element {
                    n.right = Self::insert_rec(n.right.take(), x);
                }
                Some(n)
            }
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
        }
    }

    pub fn print_in_order(&self) {
        Self::print_in_order_rec(&self.root);
    }

    fn print_in_order_rec(node: &Option<Box<TreeNode>>) {
        if let Some(n) = node {
            Self::print_in_order_rec(&n.left);
            println!("{}", n.element);
            Self::print_in_order_rec(&n.right);
        }
    }

    pub fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even_numbers_rec(self.root.take());
    }

    fn delete_even_numbers_rec(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        if let Some(mut n) = node {
            n.left = Self::delete_even_numbers_rec(n.left.take());
            n.right = Self::delete_even_numbers_rec(n.right.take());

            if n.element % 2 == 0 {
                return Self::remove_node(n);
            }
            Some(n)
        } else {
            None
        }
    }

    fn remove_node(mut node: Box<TreeNode>) -> Option<Box<TreeNode>> {
        if node.left.is_none() {
            return node.right;
        } else if node.right.is_none() {
            return node.left;
        }

        let min_larger_node = Self::find_min(&node.right);
        node.element = min_larger_node.element;
        node.right = Self::delete_rec(node.right.take(), min_larger_node.element);
        Some(node)
    }

    fn delete_rec(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete_rec(n.left.take(), x);
                } else if x > n.element {
                    n.right = Self::delete_rec(n.right.take(), x);
                } else {
                    return Self::remove_node(n);
                }
                Some(n)
            }
            None => None,
        }
    }

    fn find_min(node: &Option<Box<TreeNode>>) -> &Box<TreeNode> {
        match node {
            Some(n) => {
                if n.left.is_none() {
                    n
                } else {
                    Self::find_min(&n.left)
                }
            }
            None => panic!("Tree should not be empty"),
        }
    }

    fn find_max(node: &Option<Box<TreeNode>>) -> &Box<TreeNode> {
        match node {
            Some(n) => {
                if n.right.is_none() {
                    n
                } else {
                    Self::find_max(&n.right)
                }
            }
            None => panic!("Tree should not be empty"),
        }
    }

    pub fn min(&self) -> Option<i32> {
        self.root.as_ref().map(|n| Self::find_min(&Some(n.clone())).element)
    }

    pub fn max(&self) -> Option<i32> {
        self.root.as_ref().map(|n| Self::find_max(&Some(n.clone())).element)
    }
}

fn main() {
    let mut tree = SearchTree::new();
    let mut input = String::new();

    print!("Enter integers (space-separated): ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    for &num in &numbers {
        tree.insert(num);
    }

    if numbers.is_empty() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    tree.print_in_order();

    // Delete all even numbers
    tree.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    tree.print_in_order();

    match tree.min() {
        Some(min_value) => {
            match tree.max() {
                Some(max_value) => {
                    println!("Min is {}, Max is {}", min_value, max_value);
                }
                None => {
                    println!("Tree is empty after deletion");
                }
            }
        }
        None => {
            println!("Tree is empty after deletion");
        }
    }
}