use std::io::{self, BufRead};
use std::ptr;
use std::collections::VecDeque;

type ElementType = i32;

#[derive(Debug)]
struct TreeNode {
    element: ElementType,
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

    fn insert(&mut self, x: ElementType) {
        self.root = Self::insert_rec(self.root.take(), x);
    }

    fn insert_rec(node: Option<Box<TreeNode>>, x: ElementType) -> Option<Box<TreeNode>> {
        if let Some(mut n) = node {
            if x < n.element {
                n.left = Self::insert_rec(n.left, x);
            } else if x > n.element {
                n.right = Self::insert_rec(n.right, x);
            }
            Some(n)
        } else {
            Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            }))
        }
    }

    fn find_min(&self) -> Option<ElementType> {
        Self::find_min_rec(&self.root)
    }

    fn find_min_rec(node: &Option<Box<TreeNode>>) -> Option<ElementType> {
        match node {
            Some(n) => {
                if n.left.is_none() {
                    Some(n.element)
                } else {
                    Self::find_min_rec(&n.left)
                }
            }
            None => None,
        }
    }

    fn find_max(&self) -> Option<ElementType> {
        Self::find_max_rec(&self.root)
    }

    fn find_max_rec(node: &Option<Box<TreeNode>>) -> Option<ElementType> {
        match node {
            Some(n) => {
                if n.right.is_none() {
                    Some(n.element)
                } else {
                    Self::find_max_rec(&n.right)
                }
            }
            None => None,
        }
    }

    fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even_numbers_rec(self.root.take());
    }

    fn delete_even_numbers_rec(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        if let Some(mut n) = node {
            n.left = Self::delete_even_numbers_rec(n.left);
            n.right = Self::delete_even_numbers_rec(n.right);
            if n.element % 2 == 0 {
                return Self::delete_node(n);
            }
            Some(n)
        } else {
            None
        }
    }

    fn delete_node(node: Box<TreeNode>) -> Option<Box<TreeNode>> {
        if node.left.is_none() {
            return node.right;
        }
        if node.right.is_none() {
            return node.left;
        }

        // Find the minimum node in the right subtree
        let mut right_min = node.right.clone();
        let min_value = Self::find_min_rec(&right_min);
        let new_right = Self::delete_min_node(right_min.take());
        let mut new_node = Box::new(TreeNode {
            element: min_value.unwrap(),
            left: node.left,
            right: new_right,
        });
        Some(new_node)
    }

    fn delete_min_node(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        if let Some(mut n) = node {
            if n.left.is_none() {
                return n.right;
            }
            n.left = Self::delete_min_node(n.left);
            Some(n)
        } else {
            None
        }
    }

    fn print_in_order(&self) {
        Self::print_in_order_rec(&self.root);
    }

    fn print_in_order_rec(node: &Option<Box<TreeNode>>) {
        if let Some(n) = node {
            Self::print_in_order_rec(&n.left);
            println!("{}", n.element);
            Self::print_in_order_rec(&n.right);
        }
    }
}

fn main() {
    let mut tree = SearchTree::new();
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut has_input = false;

    for line in reader.lines() {
        let line = line.unwrap();
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<ElementType>() {
                tree.insert(num);
                has_input = true;
            }
        }
    }

    if !has_input || tree.root.is_none() {
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
        }
    } else {
        println!("Tree is empty");
    }
}