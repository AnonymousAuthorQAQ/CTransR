use std::cmp::Ord;
use std::io::{self, Write};

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

    fn make_empty(&mut self) {
        self.root = None;
    }

    fn insert(&mut self, x: i32) {
        self.root = Self::insert_node(self.root.take(), x);
    }

    fn insert_node(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_node(n.left, x);
                } else if x > n.element {
                    n.right = Self::insert_node(n.right, x);
                }
                Some(n)
            }
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
        }
    }

    fn find(&self, x: i32) -> Option<&Box<TreeNode>> {
        Self::find_node(&self.root, x)
    }

    fn find_node(node: &Option<Box<TreeNode>>, x: i32) -> Option<&Box<TreeNode>> {
        match node {
            Some(n) => {
                if x < n.element {
                    Self::find_node(&n.left, x)
                } else if x > n.element {
                    Self::find_node(&n.right, x)
                } else {
                    Some(n)
                }
            }
            None => None,
        }
    }

    fn find_min(&self) -> Option<&Box<TreeNode>> {
        Self::find_min_node(&self.root)
    }

    fn find_min_node(node: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
        match node {
            Some(n) => {
                if n.left.is_none() {
                    Some(n)
                } else {
                    Self::find_min_node(&n.left)
                }
            }
            None => None,
        }
    }

    fn find_max(&self) -> Option<&Box<TreeNode>> {
        Self::find_max_node(&self.root)
    }

    fn find_max_node(node: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
        match node {
            Some(n) => {
                if n.right.is_none() {
                    Some(n)
                } else {
                    Self::find_max_node(&n.right)
                }
            }
            None => None,
        }
    }

    fn delete(&mut self, x: i32) {
        self.root = Self::delete_node(self.root.take(), x);
    }

    fn delete_node(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete_node(n.left, x);
                    Some(n)
                } else if x > n.element {
                    n.right = Self::delete_node(n.right, x);
                    Some(n)
                } else {
                    // Node with one child or no child
                    if n.left.is_none() {
                        return n.right;
                    } else if n.right.is_none() {
                        return n.left;
                    }

                    // Node with two children
                    let min_node = Self::find_min_node(&n.right);
                    if let Some(min_node) = min_node {
                        n.element = min_node.element;
                        n.right = Self::delete_node(n.right, min_node.element);
                    }
                    Some(n)
                }
            }
            None => {
                eprintln!("Error: Element not found");
                None
            }
        }
    }

    fn retrieve(node: &Option<Box<TreeNode>>) -> Option<i32> {
        node.as_ref().map(|n| n.element)
    }
}

fn main() {
    let mut tree = SearchTree::new();
    
    // Insert numbers according to the algorithm
    let mut j = 0;
    for i in 0..50 {
        tree.insert(j);
        j = (j + 7) % 50;
    }

    // Verify that numbers 0 to 49 can be found
    for i in 0..50 {
        if let Some(node) = tree.find(i) {
            if SearchTree::retrieve(Some(node)) != Some(i) {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    // Delete even numbers
    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    // Verify that odd numbers remain
    for i in (1..50).step_by(2) {
        if let Some(node) = tree.find(i) {
            if SearchTree::retrieve(Some(node)) != Some(i) {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    // Check that even numbers have been deleted
    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    // Print minimum and maximum values
    if let Some(min_node) = tree.find_min() {
        let min_value = SearchTree::retrieve(Some(min_node)).unwrap();
        if let Some(max_node) = tree.find_max() {
            let max_value = SearchTree::retrieve(Some(max_node)).unwrap();
            println!("Min is {}, Max is {}", min_value, max_value);
        }
    }
}