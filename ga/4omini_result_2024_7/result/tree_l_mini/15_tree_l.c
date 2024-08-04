use std::io::{self, BufRead};
use std::ptr;
use std::rc::Rc;
use std::cell::RefCell;

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
        self.root = Self::insert_recursive(self.root.take(), x);
    }

    fn insert_recursive(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_recursive(n.left, x);
                } else if x > n.element {
                    n.right = Self::insert_recursive(n.right, x);
                }
                Some(n)
            }
        }
    }

    fn delete(&mut self, x: i32) {
        self.root = Self::delete_recursive(self.root.take(), x);
    }

    fn delete_recursive(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        if let Some(mut n) = node {
            if x < n.element {
                n.left = Self::delete_recursive(n.left, x);
            } else if x > n.element {
                n.right = Self::delete_recursive(n.right, x);
            } else {
                let tmp_cell = n.clone(); // Store reference for use

                if n.left.is_none() {
                    return n.right;
                } else if n.right.is_none() {
                    return n.left;
                } else {
                    let min_larger_node = Self::find_min(n.right.as_ref().unwrap());
                    n.element = min_larger_node.element;
                    n.right = Self::delete_recursive(n.right, min_larger_node.element);
                }
            }
            Some(n)
        } else {
            None
        }
    }

    fn find_min(node: &TreeNode) -> &TreeNode {
        if let Some(ref left) = node.left {
            Self::find_min(left)
        } else {
            node
        }
    }

    fn find_max(node: &TreeNode) -> &TreeNode {
        let mut current = node;
        while let Some(ref right) = current.right {
            current = right;
        }
        current
    }

    fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even_recursive(self.root.take());
    }

    fn delete_even_recursive(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        if let Some(mut n) = node {
            n.left = Self::delete_even_recursive(n.left);
            n.right = Self::delete_even_recursive(n.right);

            if n.element % 2 == 0 {
                return Self::delete_recursive(Some(n), n.element);
            }
            Some(n)
        } else {
            None
        }
    }

    fn print_in_order(&self) {
        Self::print_recursive(&self.root);
    }

    fn print_recursive(node: &Option<Box<TreeNode>>) {
        if let Some(n) = node {
            Self::print_recursive(&n.left);
            println!("{}", n.element);
            Self::print_recursive(&n.right);
        }
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

fn main() {
    let mut tree = SearchTree::new();
    
    let stdin = io::stdin();
    let input_line = stdin.lock().lines().next().unwrap().unwrap();

    let has_input = input_line
        .split_whitespace()
        .filter_map(|token| token.parse::<i32>().ok())
        .fold(false, |acc, num| {
            tree.insert(num);
            true
        });

    if !has_input || tree.is_empty() {
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
        println!(
            "Min is {}, Max is {}",
            Self::find_min(root).element,
            Self::find_max(root).element
        );
    } else {
        println!("Tree is empty");
    }
}