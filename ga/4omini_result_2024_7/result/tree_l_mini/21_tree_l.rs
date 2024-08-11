use std::collections::VecDeque;
use std::io::{self, BufRead};

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
        SearchTree { root: None }
    }

    fn insert(&mut self, x: i32) {
        self.root = Self::insert_node(self.root.take(), x);
    }

    fn insert_node(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_node(n.left, x);
                } else if x > n.element {
                    n.right = Self::insert_node(n.right, x);
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

    fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even_node(self.root.take());
    }

    fn delete_even_node(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                n.left = Self::delete_even_node(n.left);
                n.right = Self::delete_even_node(n.right);

                if n.element % 2 == 0 {
                    return Self::delete_node(n);
                }
                Some(n)
            }
            None => None,
        }
    }

    fn delete_node(node: Box<TreeNode>) -> Option<Box<TreeNode>> {
        if node.left.is_none() {
            return node.right;
        } else if node.right.is_none() {
            return node.left;
        }

        let mut right_min_node = node.right.as_ref().unwrap().clone();
        while let Some(left) = right_min_node.left {
            right_min_node = left;
        }

        let new_node = Box::new(TreeNode {
            element: right_min_node.element,
            left: node.left,
            right: Self::delete_node(right_min_node),
        });
        Some(new_node)
    }

    fn print_in_order(&self) {
        Self::in_order_traversal(&self.root);
    }

    fn in_order_traversal(node: &Option<Box<TreeNode>>) {
        if let Some(n) = node {
            Self::in_order_traversal(&n.left);
            println!("{}", n.element);
            Self::in_order_traversal(&n.right);
        }
    }

    fn find_min(&self) -> Option<i32> {
        Self::find_min_node(&self.root)
    }

    fn find_min_node(node: &Option<Box<TreeNode>>) -> Option<i32> {
        let mut current = node;
        while let Some(n) = current {
            if n.left.is_none() {
                return Some(n.element);
            }
            current = &n.left;
        }
        None
    }

    fn find_max(&self) -> Option<i32> {
        Self::find_max_node(&self.root)
    }

    fn find_max_node(node: &Option<Box<TreeNode>>) -> Option<i32> {
        let mut current = node;
        while let Some(n) = current {
            if n.right.is_none() {
                return Some(n.element);
            }
            current = &n.right;
        }
        None
    }
}

fn main() {
    let mut tree = SearchTree::new();
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut has_input = false;

    for line in handle.lines() {
        let line = line.expect("Failed to read line");
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|token| token.parse::<i32>().ok())
            .collect();

        for &num in &numbers {
            tree.insert(num);
            has_input = true;
        }
    }

    if !has_input {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    tree.print_in_order();

    // Delete all even numbers
    tree.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    tree.print_in_order();

    if let Some(min) = tree.find_min() {
        if let Some(max) = tree.find_max() {
            println!("Min is {}, Max is {}", min, max);
        } else {
            println!("Tree is empty after deletion");
        }
    } else {
        println!("Tree is empty after deletion");
    }
}