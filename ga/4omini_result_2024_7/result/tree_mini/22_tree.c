use std::cmp::Ord;
use std::fmt::Debug;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct TreeNode<T> {
    element: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

pub struct SearchTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Copy + Debug> SearchTree<T> {
    pub fn new() -> Self {
        SearchTree { root: None }
    }

    pub fn make_empty(&mut self) {
        self.root = None;
    }

    pub fn insert(&mut self, x: T) {
        self.root = Self::insert_node(self.root.take(), x);
    }

    fn insert_node(node: Option<Box<TreeNode<T>>>, x: T) -> Option<Box<TreeNode<T>>> {
        if let Some(mut n) = node {
            if x < n.element {
                n.left = Self::insert_node(n.left, x);
            } else if x > n.element {
                n.right = Self::insert_node(n.right, x);
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

    pub fn find(&self, x: T) -> Option<&T> {
        Self::find_node(&self.root, x)
    }

    fn find_node(node: &Option<Box<TreeNode<T>>>, x: T) -> Option<&T> {
        match node {
            Some(n) => {
                if x < n.element {
                    Self::find_node(&n.left, x)
                } else if x > n.element {
                    Self::find_node(&n.right, x)
                } else {
                    Some(&n.element)
                }
            }
            None => None,
        }
    }

    pub fn delete(&mut self, x: T) {
        self.root = Self::delete_node(self.root.take(), x);
    }

    fn delete_node(node: Option<Box<TreeNode<T>>>, x: T) -> Option<Box<TreeNode<T>>> {
        if let Some(mut n) = node {
            if x < n.element {
                n.left = Self::delete_node(n.left, x);
                Some(n)
            } else if x > n.element {
                n.right = Self::delete_node(n.right, x);
                Some(n)
            } else {
                // Node found
                match (n.left, n.right) {
                    (None, None) => None, // No children
                    (Some(left), None) => Some(left), // Only left child
                    (None, Some(right)) => Some(right), // Only right child
                    (Some(left), Some(right)) => {
                        let min_node = Self::find_min(&right).unwrap();
                        n.element = *min_node; // Replace with min of right subtree
                        n.right = Self::delete_node(n.right, *min_node);
                        Some(n)
                    }
                }
            }
        } else {
            None
        }
    }

    pub fn find_min(&self) -> Option<&T> {
        Self::find_min_node(&self.root)
    }

    fn find_min_node(node: &Option<Box<TreeNode<T>>>) -> Option<&T> {
        node.as_ref().and_then(|n| {
            if n.left.is_some() {
                Self::find_min_node(&n.left)
            } else {
                Some(&n.element)
            }
        })
    }

    pub fn find_max(&self) -> Option<&T> {
        Self::find_max_node(&self.root)
    }

    fn find_max_node(node: &Option<Box<TreeNode<T>>>) -> Option<&T> {
        if let Some(n) = node {
            if n.right.is_some() {
                Self::find_max_node(&n.right)
            } else {
                Some(&n.element)
            }
        } else {
            None
        }
    }
}

fn main() {
    let mut tree = SearchTree::new();

    // Insert values
    let mut j = 0;
    for i in 0..50 {
        tree.insert(j);
        j = (j + 7) % 50;
    }

    // Check existence of elements from 0 to 49
    for i in 0..50 {
        if tree.find(i).is_none() {
            eprintln!("Error at {}", i);
        }
    }

    // Delete even values
    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    // Check existence of odd values
    for i in (1..50).step_by(2) {
        if tree.find(i).is_none() {
            eprintln!("Error at {}", i);
        }
    }

    // Check existence of even values
    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            eprintln!("Error at {}", i);
        }
    }

    // Print min and max
    if let (Some(min), Some(max)) = (tree.find_min(), tree.find_max()) {
        println!("Min is {:?}, Max is {:?}", min, max);
    }
}