use std::io::{self, BufRead};
use std::fmt;
use std::ptr;

#[derive(Debug)]
pub struct TreeNode {
    pub element: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

pub struct SearchTree {
    root: Option<Box<TreeNode>>,
}

impl SearchTree {
    pub fn new() -> Self {
        SearchTree { root: None }
    }

    pub fn insert(&mut self, x: i32) {
        self.root = Self::insert_node(self.root.take(), x);
    }

    fn insert_node(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_node(n.left, x);
                } else if x > n.element {
                    n.right = Self::insert_node(n.right, x);
                }
                Some(n)
            }
        }
    }

    pub fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even(self.root.take());
    }

    fn delete_even(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        if let Some(mut n) = node {
            n.left = Self::delete_even(n.left);
            n.right = Self::delete_even(n.right);

            if n.element % 2 == 0 {
                return Self::delete_node(n);
            }
            return Some(n);
        }
        None
    }

    fn delete_node(node: Box<TreeNode>) -> Option<Box<TreeNode>> {
        if node.left.is_none() {
            return node.right;
        } else if node.right.is_none() {
            return node.left;
        }

        let mut current = node.right;
        let mut min_node = current.as_mut();
        while let Some(ref mut n) = min_node {
            if n.left.is_some() {
                current = n.left.take();
                min_node = current.as_mut();
            } else {
                let min_value = n.element;
                current = node.right.take();
                return Self::insert_node(current, min_value);
            }
        }

        None
    }

    pub fn print_in_order(&self) {
        Self::in_order(&self.root);
    }

    fn in_order(node: &Option<Box<TreeNode>>) {
        if let Some(n) = node {
            Self::in_order(&n.left);
            println!("{}", n.element);
            Self::in_order(&n.right);
        }
    }

    pub fn find_min(&self) -> Option<i32> {
        Self::find_min_node(&self.root).map(|node| node.element)
    }

    fn find_min_node(node: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
        let mut current = node;
        while let Some(ref n) = current {
            if n.left.is_none() {
                return Some(n);
            }
            current = &n.left;
        }
        None
    }

    pub fn find_max(&self) -> Option<i32> {
        Self::find_max_node(&self.root).map(|node| node.element)
    }

    fn find_max_node(node: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
        let mut current = node;
        while let Some(ref n) = current {
            if n.right.is_none() {
                return Some(n);
            }
            current = &n.right;
        }
        None
    }
}

fn main() {
    let stdin = io::stdin();
    let mut tree = SearchTree::new();

    let line = stdin.lock().lines().next().unwrap().unwrap();
    let numbers: Vec<i32> = line
        .split_whitespace()
        .filter_map(|token| token.parse().ok())
        .collect();

    for num in numbers {
        tree.insert(num);
    }

    if tree.root.is_none() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    tree.print_in_order();

    // Delete all even numbers
    tree.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    tree.print_in_order();

    match (tree.find_min(), tree.find_max()) {
        (Some(min), Some(max)) => {
            println!("Min is {}, Max is {}", min, max);
        }
        _ => {
            println!("Tree is empty");
        }
    }
}