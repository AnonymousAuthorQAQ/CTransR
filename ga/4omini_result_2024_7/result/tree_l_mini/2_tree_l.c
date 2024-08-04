use std::io::{self, BufRead};
use std::ops::Deref;

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
                    n.left = Self::insert_node(n.left.take(), x);
                } else if x > n.element {
                    n.right = Self::insert_node(n.right.take(), x);
                }
                Some(n)
            }
            None => {
                Some(Box::new(TreeNode {
                    element: x,
                    left: None,
                    right: None,
                }))
            }
        }
    }

    fn delete(&mut self, x: i32) {
        self.root = Self::delete_node(self.root.take(), x);
    }

    fn delete_node(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        if let Some(mut n) = node {
            if x < n.element {
                n.left = Self::delete_node(n.left.take(), x);
            } else if x > n.element {
                n.right = Self::delete_node(n.right.take(), x);
            } else {
                if n.left.is_none() {
                    return n.right;
                }
                if n.right.is_none() {
                    return n.left;
                }
                let min_node = Self::find_min(n.right.as_ref());
                n.element = min_node.element;
                n.right = Self::delete_node(n.right.take(), min_node.element);
            }
            Some(n)
        } else {
            None
        }
    }

    fn find_min(node: Option<&Box<TreeNode>>) -> &TreeNode {
        let mut current = node.unwrap();
        while let Some(ref left) = current.left {
            current = left.as_ref();
        }
        current
    }

    fn find_max(node: Option<&Box<TreeNode>>) -> &TreeNode {
        let mut current = node.unwrap();
        while let Some(ref right) = current.right {
            current = right.as_ref();
        }
        current
    }

    fn print_in_order(&self) {
        Self::in_order_helper(&self.root);
    }

    fn in_order_helper(node: &Option<Box<TreeNode>>) {
        if let Some(ref n) = node {
            Self::in_order_helper(&n.left);
            println!("{}", n.element);
            Self::in_order_helper(&n.right);
        }
    }

    fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even_node(self.root.take());
    }

    fn delete_even_node(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                n.left = Self::delete_even_node(n.left.take());
                n.right = Self::delete_even_node(n.right.take());

                if n.element % 2 == 0 {
                    n.right // Return the right child which can handle the case of "n.left" being None
                } else {
                    Some(n)
                }
            }
            None => None,
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut tree = SearchTree::new();
    let mut has_input = false;

    println!("Enter numbers (space-separated):");

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let tokens: Vec<&str> = line.split_whitespace().collect();
        for token in tokens {
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

    if let Some(ref root) = tree.root {
        let min = SearchTree::find_min(Some(root)).element;
        let max = SearchTree::find_max(Some(root)).element;
        println!("Min is {}, Max is {}", min, max);
    } else {
        println!("Tree is empty");
    }
}