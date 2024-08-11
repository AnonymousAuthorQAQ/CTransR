use std::io::{self, BufRead};
use std::cmp::{Ord, Ordering};
use std::option::Option;

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
                    n.left = Self::insert_node(n.left.take(), x);
                } else if x > n.element {
                    n.right = Self::insert_node(n.right.take(), x);
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

    fn delete_node(mut node: Box<TreeNode>) -> Option<Box<TreeNode>> {
        if node.left.is_none() {
            return node.right;
        } else if node.right.is_none() {
            return node.left;
        } else {
            let min_node = Self::find_min(node.right.as_ref().unwrap());
            node.element = min_node.element;
            node.right = Self::delete_node_from(node.right, min_node.element);
            return Some(node);
        }
    }

    fn delete_node_from(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            None => None,
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete_node_from(n.left, x);
                } else if x > n.element {
                    n.right = Self::delete_node_from(n.right, x);
                } else {
                    return Self::delete_node(n);
                }
                Some(n)
            }
        }
    }

    fn find_min(node: &Box<TreeNode>) -> &Box<TreeNode> {
        if let Some(ref left) = node.left {
            Self::find_min(left)
        } else {
            node
        }
    }

    fn find_max(node: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
        match node {
            Some(n) => {
                let mut current = n;
                while let Some(ref right) = current.right {
                    current = right;
                }
                Some(current)
            }
            None => None,
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
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut tree = SearchTree::new();

    let mut has_input = false;

    for line in handle.lines() {
        let line = line.unwrap();
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                tree.insert(num);
                has_input = true;
            }
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

    if let Some(max_node) = SearchTree::find_max(tree.root.as_ref()) {
        if let Some(min_node) = SearchTree::find_min(max_node) {
            println!("Min is {}, Max is {}", min_node.element, max_node.element);
        }
    } else {
        println!("Tree is empty");
    }
}