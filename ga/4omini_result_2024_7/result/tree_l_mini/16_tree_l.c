use std::io::{self, BufRead};
use std::ptr::null_mut;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

pub struct SearchTree {
    root: Option<Box<TreeNode>>,
}

impl SearchTree {
    pub fn new() -> Self {
        SearchTree { root: None }
    }

    pub fn insert(&mut self, x: i32) {
        self.root = Self::insert_node(self.root.take(), x);
    }

    fn insert_node(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        if let Some(mut n) = node {
            if x < n.element {
                n.left = Self::insert_node(n.left.take(), x);
            } else if x > n.element {
                n.right = Self::insert_node(n.right.take(), x);
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

    pub fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even_nodes(self.root.take());
    }

    fn delete_even_nodes(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                n.left = Self::delete_even_nodes(n.left);
                n.right = Self::delete_even_nodes(n.right);
                
                if n.element % 2 == 0 {
                    return Self::remove_node(n);
                }
                Some(n)
            }
            None => None,
        }
    }

    fn remove_node(node: Box<TreeNode>) -> Option<Box<TreeNode>> {
        if node.left.is_none() {
            return node.right;
        } else if node.right.is_none() {
            return node.left;
        }
        
        let min_node = Self::find_min(&node.right);
        let mut new_node = Box::new(TreeNode {
            element: min_node.element,
            left: node.left,
            right: Self::remove_min(node.right),
        });
        Some(new_node)
    }

    fn find_min(node: &Option<Box<TreeNode>>) -> &Box<TreeNode> {
        let mut current = node;
        while let Some(ref n) = current {
            if n.left.is_none() {
                return n;
            }
            current = &n.left;
        }
        panic!("Tree is empty, cannot find min.");
    }

    fn remove_min(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) if n.left.is_some() => {
                n.left = Self::remove_min(n.left);
                Some(n)
            }
            Some(n) => n.right,
            None => None,
        }
    }

    pub fn print_in_order(&self) {
        Self::in_order(&self.root);
    }

    fn in_order(node: &Option<Box<TreeNode>>) {
        if let Some(n) = node {
            Self::in_order(&n.left);
            println!("{}", n.element);
            Self::in_order(&n.right);
        }
    }

    pub fn find_min(&self) -> Option<i32> {
        Self::find_min_value(&self.root).map(|node| node.element)
    }

    fn find_min_value(node: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
        let mut current = node;
        while let Some(ref n) = current {
            if n.left.is_none() {
                return Some(n);
            }
            current = &n.left;
        }
        None
    }

    pub fn find_max(&self) -> Option<i32> {
        Self::find_max_value(&self.root).map(|node| node.element)
    }

    fn find_max_value(node: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
        let mut current = node;
        while let Some(ref n) = current {
            if n.right.is_none() {
                return Some(n);
            }
            current = &n.right;
        }
        None
    }
}

fn main() {
    let stdin = io::stdin();
    let mut tree = SearchTree::new();

    println!("Enter numbers separated by spaces:");

    let line = stdin.lock().lines().next().unwrap().unwrap();
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

    if let (Some(min), Some(max)) = (tree.find_min(), tree.find_max()) {
        println!("Min is {}, Max is {}", min, max);
    } else {
        println!("Tree is empty");
    }
}