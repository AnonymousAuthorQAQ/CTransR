use std::cmp::Ord;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

struct SearchTree {
    root: Option<Box<TreeNode>>,
}

impl SearchTree {
    fn new() -> Self {
        Self { root: None }
    }

    fn make_empty(&mut self) {
        self.root = None;
    }

    fn find(&self, x: i32) -> Option<&TreeNode> {
        Self::find_helper(&self.root, x)
    }

    fn find_helper(node: &Option<Box<TreeNode>>, x: i32) -> Option<&TreeNode> {
        match node {
            Some(ref n) if x < n.element => Self::find_helper(&n.left, x),
            Some(ref n) if x > n.element => Self::find_helper(&n.right, x),
            Some(ref n) => Some(n),
            None => None,
        }
    }

    fn find_min(&self) -> Option<&TreeNode> {
        Self::find_min_helper(&self.root)
    }

    fn find_min_helper(node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            Some(ref n) if n.left.is_some() => Self::find_min_helper(&n.left),
            Some(ref n) => Some(n),
            None => None,
        }
    }

    fn find_max(&self) -> Option<&TreeNode> {
        Self::find_max_helper(&self.root)
    }

    fn find_max_helper(node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        let mut current = node;
        while let Some(ref n) = current {
            if n.right.is_some() {
                current = &n.right;
            } else {
                return Some(n);
            }
        }
        None
    }

    fn insert(&mut self, x: i32) {
        self.root = Self::insert_helper(self.root.take(), x);
    }

    fn insert_helper(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_helper(n.left, x);
                } else if x > n.element {
                    n.right = Self::insert_helper(n.right, x);
                }
                Some(n)
            }
        }
    }

    fn delete(&mut self, x: i32) {
        self.root = Self::delete_helper(self.root.take(), x);
    }

    fn delete_helper(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            None => {
                eprintln!("Element not found");
                None
            }
            Some(mut n) if x < n.element => {
                n.left = Self::delete_helper(n.left, x);
                Some(n)
            }
            Some(mut n) if x > n.element => {
                n.right = Self::delete_helper(n.right, x);
                Some(n)
            }
            Some(mut n) => {
                if n.left.is_some() && n.right.is_some() {
                    let min_node = Self::find_min_helper(&n.right).unwrap();
                    n.element = min_node.element;
                    n.right = Self::delete_helper(n.right, min_node.element);
                } else {
                    if n.left.is_none() {
                        return n.right;
                    } else {
                        return n.left;
                    }
                }
                Some(n)
            }
        }
    }

    fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even_helper(self.root.take());
    }

    fn delete_even_helper(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            None => None,
            Some(mut n) => {
                n.left = Self::delete_even_helper(n.left);
                n.right = Self::delete_even_helper(n.right);
                if n.element % 2 == 0 {
                    return Self::delete_helper(Some(n), n.element);
                }
                Some(n)
            }
        }
    }

    fn print_in_order(&self) {
        Self::print_helper(&self.root);
    }

    fn print_helper(node: &Option<Box<TreeNode>>) {
        if let Some(ref n) = node {
            Self::print_helper(&n.left);
            println!("{}", n.element);
            Self::print_helper(&n.right);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut tree = SearchTree::new();
    
    println!("Enter numbers (space-separated):");
    let line = stdin.lock().lines().next().unwrap().unwrap();
    for token in line.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            tree.insert(num);
        }
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

    if let Some(min_node) = tree.find_min() {
        if let Some(max_node) = tree.find_max() {
            println!("Min is {}, Max is {}", min_node.element, max_node.element);
        } else {
            println!("Tree is empty after deleting even numbers");
        }
    } else {
        println!("Tree is empty after deleting even numbers");
    }
}