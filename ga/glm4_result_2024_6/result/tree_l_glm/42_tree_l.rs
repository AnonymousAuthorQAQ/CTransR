use std::io::{self, Read};
use std::collections::Vec;
use std::fmt;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(element: i32) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }

    fn make_empty(&mut self) {
        self.left = None;
        self.right = None;
    }
}

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.element)
    }
}

fn find_min(root: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    match root {
        Some(node) => {
            if node.left.is_none() {
                Some(node)
            } else {
                find_min(&node.left)
            }
        }
        None => None,
    }
}

fn find_max(root: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    match root {
        Some(node) => {
            if node.right.is_none() {
                Some(node)
            } else {
                find_max(&node.right)
            }
        }
        None => None,
    }
}

fn insert(root: &mut Option<Box<TreeNode>>, element: i32) {
    match root {
        Some(node) => {
            if element < node.element {
                insert(&mut node.left, element);
            } else if element > node.element {
                insert(&mut node.right, element);
            }
        }
        None => {
            *root = Some(Box::new(TreeNode::new(element)));
        }
    }
}

fn delete(root: &mut Option<Box<TreeNode>>, element: i32) {
    if let Some(node) = root {
        if element < node.element {
            delete(&mut node.left, element);
        } else if element > node.element {
            delete(&mut node.right, element);
        } else {
            if node.left.is_none() {
                *root = node.right.take();
            } else if node.right.is_none() {
                *root = node.left.take();
            } else {
                let mut min_right = node.right.take();
                let min_node = find_min(&min_right).unwrap();
                node.element = min_node.element;
                insert(&mut node.left, min_node.element);
                delete(&mut min_right, min_node.element);
                node.right = min_right;
            }
        }
    }
}

fn retrieve(root: &Option<Box<TreeNode>>) -> Option<i32> {
    root.as_ref().map(|node| node.element)
}

fn print_in_order(root: &Option<Box<TreeNode>>) {
    if let Some(node) = root {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn delete_even_numbers(root: &mut Option<Box<TreeNode>>) {
    if let Some(node) = root {
        delete_even_numbers(&mut node.left);
        delete_even_numbers(&mut node.right);
        if node.element % 2 == 0 {
            delete(root, node.element);
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut tree: Option<Box<TreeNode>> = None;
    let tokens: Vec<i32> = input
        .split_whitespace()
        .filter_map(|token| token.parse().ok())
        .collect();

    for token in tokens {
        insert(&mut tree, token);
    }

    print_in_order(&tree);
    delete_even_numbers(&mut tree);
    println!("Tree after deleting even numbers:");
    print_in_order(&tree);

    if let Some(node) = tree {
        if let (Some(min), Some(max)) = (find_min(&tree), find_max(&tree)) {
            println!("Min is {}, Max is {}", min.element, max.element);
        }
    } else {
        println!("Tree is empty");
    }
}