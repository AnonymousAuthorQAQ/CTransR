use std::io::{self, BufRead};
use std::mem;

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
        self.root = Self::insert_rec(self.root.take(), x);
    }

    fn insert_rec(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_rec(n.left, x);
                } else if x > n.element {
                    n.right = Self::insert_rec(n.right, x);
                }
                Some(n)
            }
        }
    }

    fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even_numbers_rec(self.root.take());
    }

    fn delete_even_numbers_rec(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            None => None,
            Some(mut n) => {
                n.left = Self::delete_even_numbers_rec(n.left);
                n.right = Self::delete_even_numbers_rec(n.right);

                if n.element % 2 == 0 {
                    return Self::delete(n.element, Some(n));
                }
                Some(n)
            }
        }
    }

    fn delete(x: i32, node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            None => None,
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete(x, n.left);
                } else if x > n.element {
                    n.right = Self::delete(x, n.right);
                } else {
                    // Node with only one child or no child
                    if n.left.is_none() {
                        return n.right; // No left child
                    } else if n.right.is_none() {
                        return n.left; // No right child
                    }

                    // Node with two children, get the inorder successor (smallest in the right subtree)
                    n.element = Self::find_min(n.right.as_ref().unwrap()).element;

                    // Delete the inorder successor
                    n.right = Self::delete(n.element, n.right);
                }
                Some(n)
            }
        }
    }

    fn find_min(node: &TreeNode) -> &TreeNode {
        match &node.left {
            None => node,
            Some(left) => Self::find_min(left),
        }
    }

    fn find_max(node: &TreeNode) -> &TreeNode {
        match &node.right {
            None => node,
            Some(right) => Self::find_max(right),
        }
    }

    fn print_in_order(&self) {
        Self::print_in_order_rec(self.root.as_ref());
    }

    fn print_in_order_rec(node: Option<&Box<TreeNode>>) {
        if let Some(n) = node {
            Self::print_in_order_rec(n.left.as_ref());
            println!("{}", n.element);
            Self::print_in_order_rec(n.right.as_ref());
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut tree = SearchTree::new();

    for line in lines {
        let input_line = line.unwrap();
        let nums: Vec<i32> = input_line
            .split_whitespace()
            .filter_map(|token| token.parse().ok())
            .collect();

        for &num in &nums {
            tree.insert(num);
        }
        
        if nums.is_empty() {
            println!("Tree is empty");
            return;
        }

        println!("Tree after insertion:");
        tree.print_in_order();

        // Delete all even numbers
        tree.delete_even_numbers();

        println!("Tree after deleting even numbers:");
        tree.print_in_order();

        if let Some(root) = &tree.root {
            println!(
                "Min is {}, Max is {}",
                SearchTree::find_min(root).element,
                SearchTree::find_max(root).element
            );
        } else {
            println!("Tree is empty");
        }
    }
}