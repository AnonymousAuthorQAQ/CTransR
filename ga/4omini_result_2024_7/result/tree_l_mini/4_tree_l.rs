use std::io::{self, Write};
use std::collections::VecDeque;

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
        self.root = Self::insert_rec(self.root.take(), value);
    }

    fn insert_rec(node: Option<Box<TreeNode>>, value: i32) -> Option<Box<TreeNode>> {
        if let Some(mut n) = node {
            if value < n.element {
                n.left = Self::insert_rec(n.left, value);
            } else if value > n.element {
                n.right = Self::insert_rec(n.right, value);
            }
            Some(n)
        } else {
            Some(Box::new(TreeNode {
                element: value,
                left: None,
                right: None,
            }))
        }
    }

    fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even_rec(self.root.take());
    }

    fn delete_even_rec(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        if let Some(mut n) = node {
            n.left = Self::delete_even_rec(n.left);
            n.right = Self::delete_even_rec(n.right);
            if n.element % 2 == 0 {
                return Self::delete(n.element, Some(n));
            }
            Some(n)
        } else {
            None
        }
    }

    fn delete(x: i32, node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        if let Some(mut n) = node {
            if x < n.element {
                n.left = Self::delete(x, n.left);
                Some(n)
            } else if x > n.element {
                n.right = Self::delete(x, n.right);
                Some(n)
            } else {
                // Node with only one child or no child
                if n.left.is_none() {
                    return n.right;
                } else if n.right.is_none() {
                    return n.left;
                }
                // Node with two children
                let min_node = Self::find_min(n.right.as_ref().unwrap());
                n.element = min_node.element;
                n.right = Self::delete(min_node.element, n.right);
                Some(n)
            }
        } else {
            None
        }
    }

    fn find_min(node: &Box<TreeNode>) -> &TreeNode {
        if let Some(ref left) = node.left {
            Self::find_min(left)
        } else {
            node
        }
    }

    fn find_max(node: &Box<TreeNode>) -> &TreeNode {
        if let Some(ref right) = node.right {
            Self::find_max(right)
        } else {
            node
        }
    }

    fn print_in_order(&self) {
        self.print_in_order_rec(&self.root);
    }

    fn print_in_order_rec(&self, node: &Option<Box<TreeNode>>) {
        if let Some(ref n) = node {
            self.print_in_order_rec(&n.left);
            println!("{}", n.element);
            self.print_in_order_rec(&n.right);
        }
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

fn main() {
    let mut tree = SearchTree::new();
    let mut input = String::new();

    print!("Enter integers separated by spaces: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Failed to read input");
    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    if numbers.is_empty() {
        println!("Tree is empty");
        return;
    }

    for &num in &numbers {
        tree.insert(num);
    }

    println!("Tree after insertion:");
    tree.print_in_order();

    // Delete all even numbers
    tree.delete_even_numbers();
    
    println!("Tree after deleting even numbers:");
    tree.print_in_order();

    if !tree.is_empty() {
        if let Some(ref root) = tree.root {
            let min = SearchTree::find_min(root);
            let max = SearchTree::find_max(root);
            println!("Min is {}, Max is {}", min.element, max.element);
        }
    } else {
        println!("Tree is empty");
    }
}