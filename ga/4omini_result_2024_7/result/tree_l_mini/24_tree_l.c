use std::io::{self, BufRead};
use std::ptr::null_mut;

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

    fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even_nodes(self.root.take());
    }

    fn delete_even_nodes(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            None => None,
            Some(mut n) => {
                n.left = Self::delete_even_nodes(n.left);
                n.right = Self::delete_even_nodes(n.right);
                if n.element % 2 == 0 {
                    return Self::delete_node(n);
                }
                Some(n)
            }
        }
    }

    fn delete_node(node: Box<TreeNode>) -> Option<Box<TreeNode>> {
        if node.left.is_none() {
            return node.right;
        } else if node.right.is_none() {
            return node.left;
        }
        let mut min_node = node.right.as_ref().unwrap().clone();
        while min_node.left.is_some() {
            min_node = min_node.left.as_ref().unwrap().clone();
        }
        let mut new_node = Box::new(TreeNode {
            element: min_node.element,
            left: node.left,
            right: Self::delete_node_from_right(node.right, min_node.element),
        });
        Some(new_node)
    }

    fn delete_node_from_right(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            None => None,
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete_node_from_right(n.left, x);
                    Some(n)
                } else if x > n.element {
                    n.right = Self::delete_node_from_right(n.right, x);
                    Some(n)
                } else {
                    Self::delete_node(n)
                }
            }
        }
    }

    fn print_in_order(&self) {
        Self::print_in_order_node(&self.root);
    }

    fn print_in_order_node(node: &Option<Box<TreeNode>>) {
        if let Some(n) = node {
            Self::print_in_order_node(&n.left);
            println!("{}", n.element);
            Self::print_in_order_node(&n.right);
        }
    }

    fn find_min(&self) -> Option<i32> {
        Self::find_min_node(&self.root)
    }

    fn find_min_node(node: &Option<Box<TreeNode>>) -> Option<i32> {
        match node {
            Some(n) => {
                if n.left.is_none() {
                    Some(n.element)
                } else {
                    Self::find_min_node(&n.left)
                }
            }
            None => None,
        }
    }

    fn find_max(&self) -> Option<i32> {
        Self::find_max_node(&self.root)
    }

    fn find_max_node(node: &Option<Box<TreeNode>>) -> Option<i32> {
        match node {
            Some(n) => {
                if n.right.is_none() {
                    Some(n.element)
                } else {
                    Self::find_max_node(&n.right)
                }
            }
            None => None,
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut tree = SearchTree::new();

    // Read input line
    let mut line = String::new();
    handle.read_line(&mut line).expect("Failed to read line");

    // Parse integers and insert into the tree
    for token in line.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            tree.insert(num);
        }
    }

    // Print tree after insertion
    println!("Tree after insertion:");
    tree.print_in_order();

    // Delete all even numbers
    tree.delete_even_numbers();

    // Print tree after deleting even numbers
    println!("Tree after deleting even numbers:");
    tree.print_in_order();

    // Print min and max
    if let (Some(min), Some(max)) = (tree.find_min(), tree.find_max()) {
        println!("Min is {}, Max is {}", min, max);
    } else {
        println!("Tree is empty");
    }
}