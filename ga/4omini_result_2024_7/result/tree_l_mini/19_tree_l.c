use std::io::{self, BufRead};
use std::cmp::Ord;

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
        }
        if node.right.is_none() {
            return node.left;
        }

        let mut min_node = node.right.as_ref().unwrap();
        while min_node.left.is_some() {
            min_node = min_node.left.as_ref().unwrap();
        }

        let mut new_node = Box::new(TreeNode {
            element: min_node.element,
            left: node.left,
            right: Self::delete_node(node.right.unwrap()),
        });
        Some(new_node)
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
            None => None,
            Some(n) => {
                if n.left.is_none() {
                    Some(n.element)
                } else {
                    Self::find_min_node(&n.left)
                }
            }
        }
    }

    fn find_max(&self) -> Option<i32> {
        Self::find_max_node(&self.root)
    }

    fn find_max_node(node: &Option<Box<TreeNode>>) -> Option<i32> {
        match node {
            None => None,
            Some(n) => {
                if n.right.is_none() {
                    Some(n.element)
                } else {
                    Self::find_max_node(&n.right)
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut tree = SearchTree::new();

    // Read a line of input and split into tokens
    let line = reader.lines().next().unwrap().unwrap();
    let tokens: Vec<&str> = line.split_whitespace().collect();

    for token in tokens {
        if let Ok(num) = token.parse::<i32>() {
            tree.insert(num);
        }
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
        }
    } else {
        println!("Tree is empty");
    }
}