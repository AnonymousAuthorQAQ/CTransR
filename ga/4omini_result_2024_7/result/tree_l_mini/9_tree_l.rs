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

    fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even_numbers_node(self.root.take());
    }

    fn delete_even_numbers_node(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        if let Some(n) = node {
            let left = Self::delete_even_numbers_node(n.left);
            let right = Self::delete_even_numbers_node(n.right);
            
            if n.element % 2 == 0 {
                return Self::delete_node(n.element, left, right);
            }

            Some(Box::new(TreeNode {
                element: n.element,
                left,
                right,
            }))
        } else {
            None
        }
    }

    fn delete_node(x: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        if left.is_none() {
            return right;
        }
        if right.is_none() {
            return left;
        }
        let mut min_node = right.as_ref().unwrap();
        let min_value = min_node.element;
        let new_right = Self::delete_node(min_value, left, min_node.right.clone());
        Some(Box::new(TreeNode {
            element: min_value,
            left: left,
            right: new_right,
        }))
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
        let mut current = node;
        while let Some(n) = current {
            if n.right.is_none() {
                return Some(n.element);
            }
            current = &n.right;
        }
        None
    }
}

fn main() {
    let mut tree = SearchTree::new();
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    let mut has_input = false;

    for token in input.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            tree.insert(num);
            has_input = true;
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

    match tree.find_min() {
        Some(min) => match tree.find_max() {
            Some(max) => println!("Min is {}, Max is {}", min, max),
            None => println!("Tree is empty"),
        },
        None => println!("Tree is empty"),
    }
}