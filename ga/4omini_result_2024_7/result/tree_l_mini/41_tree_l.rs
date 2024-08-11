use std::io::{self, BufRead};
use std::mem;
use std::ptr;

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

    fn insert(&mut self, value: i32) {
        self.root = Self::insert_node(self.root.take(), value);
    }

    fn insert_node(node: Option<Box<TreeNode>>, value: i32) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if value < n.element {
                    n.left = Self::insert_node(n.left.take(), value);
                } else if value > n.element {
                    n.right = Self::insert_node(n.right.take(), value);
                }
                Some(n)
            }
            None => Some(Box::new(TreeNode {
                element: value,
                left: None,
                right: None,
            })),
        }
    }

    fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even_numbers_node(self.root.take());
    }

    fn delete_even_numbers_node(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                n.left = Self::delete_even_numbers_node(n.left.take());
                n.right = Self::delete_even_numbers_node(n.right.take());

                if n.element % 2 == 0 {
                    // Delete this node and return its right child (or left if right is None)
                    return Self::delete_node(n);
                }
                Some(n)
            }
            None => None,
        }
    }

    fn delete_node(mut node: Box<TreeNode>) -> Option<Box<TreeNode>> {
        // If the node has no children, simply remove it
        if node.left.is_none() && node.right.is_none() {
            return None;
        }

        // If the node has two children, replace it with its in-order successor
        if let Some(left) = node.left.take() {
            if node.right.is_some() {
                let min_node = Self::find_min_node(node.right.as_ref().unwrap());
                node.element = min_node.element;
                node.right = Self::delete_node_from(node.right.take(), min_node.element);
            } else {
                return node.left;
            }
        } else {
            return node.right;
        }
        Some(node)
    }

    fn delete_node_from(node: Option<Box<TreeNode>>, value: i32) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if value < n.element {
                    n.left = Self::delete_node_from(n.left.take(), value);
                } else if value > n.element {
                    n.right = Self::delete_node_from(n.right.take(), value);
                } else {
                    return Self::delete_node(n);
                }
                Some(n)
            }
            None => None,
        }
    }

    fn find_min_node(node: &Box<TreeNode>) -> &Box<TreeNode> {
        if let Some(ref left) = node.left {
            Self::find_min_node(left)
        } else {
            node
        }
    }

    fn find_max_node(node: &Box<TreeNode>) -> &Box<TreeNode> {
        if let Some(ref right) = node.right {
            Self::find_max_node(right)
        } else {
            node
        }
    }

    fn print_in_order(&self) {
        Self::print_in_order_node(&self.root);
    }

    fn print_in_order_node(node: &Option<Box<TreeNode>>) {
        if let Some(ref n) = node {
            Self::print_in_order_node(&n.left);
            println!("{}", n.element);
            Self::print_in_order_node(&n.right);
        }
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn min(&self) -> Option<i32> {
        self.root.as_ref().map(|node| Self::find_min_node(node).element)
    }

    fn max(&self) -> Option<i32> {
        self.root.as_ref().map(|node| Self::find_max_node(node).element)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut tree = SearchTree::new();

    // Read input from the user
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    for &num in &numbers {
        tree.insert(num);
    }

    // Print the tree after insertion
    println!("Tree after insertion:");
    tree.print_in_order();

    // Delete all even numbers
    tree.delete_even_numbers();

    // Print the tree after deleting even numbers
    println!("Tree after deleting even numbers:");
    tree.print_in_order();

    // Print the minimum and maximum values
    match (tree.min(), tree.max()) {
        (Some(min), Some(max)) => {
            println!("Min is {}, Max is {}", min, max);
        }
        _ => {
            println!("Tree is empty");
        }
    }
}