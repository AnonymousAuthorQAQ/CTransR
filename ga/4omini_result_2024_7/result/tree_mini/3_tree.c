use std::cmp::Ord;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::io;

#[derive(Debug)]
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
        Self { root: None }
    }

    pub fn insert(&mut self, x: T) {
        self.root = Self::insert_rec(self.root.take(), x);
    }

    fn insert_rec(node: Option<Box<TreeNode<T>>>, x: T) -> Option<Box<TreeNode<T>>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_rec(n.left, x);
                } else if x > n.element {
                    n.right = Self::insert_rec(n.right, x);
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

    pub fn delete(&mut self, x: T) {
        self.root = Self::delete_rec(self.root.take(), x);
    }

    fn delete_rec(node: Option<Box<TreeNode<T>>>, x: T) -> Option<Box<TreeNode<T>>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete_rec(n.left, x);
                    Some(n)
                } else if x > n.element {
                    n.right = Self::delete_rec(n.right, x);
                    Some(n)
                } else {
                    // Found the node to delete
                    match (n.left.take(), n.right.take()) {
                        (None, None) => None, // No children
                        (Some(left), None) => Some(left), // Only left child
                        (None, Some(right)) => Some(right), // Only right child
                        (Some(left), Some(right)) => {
                            let min = Self::find_min(right.as_ref().unwrap());
                            n.element = min.element; // Replace with min element from right subtree
                            n.right = Self::delete_rec(Some(right), min.element);
                            Some(n)
                        }
                    }
                }
            }
            None => {
                eprintln!("Element not found");
                None
            }
        }
    }

    pub fn find(&self, x: T) -> Option<&T> {
        Self::find_rec(&self.root, x)
    }

    fn find_rec(node: &Option<Box<TreeNode<T>>>, x: T) -> Option<&T> {
        match node {
            Some(n) => {
                if x < n.element {
                    Self::find_rec(&n.left, x)
                } else if x > n.element {
                    Self::find_rec(&n.right, x)
                } else {
                    Some(&n.element)
                }
            }
            None => None,
        }
    }

    pub fn find_min(&self) -> Option<&T> {
        self.root.as_ref().and_then(Self::find_min_rec)
    }

    fn find_min_rec(node: &Box<TreeNode<T>>) -> Option<&T> {
        if let Some(left) = &node.left {
            Self::find_min_rec(left)
        } else {
            Some(&node.element)
        }
    }

    pub fn find_max(&self) -> Option<&T> {
        self.root.as_ref().and_then(Self::find_max_rec)
    }

    fn find_max_rec(node: &Box<TreeNode<T>>) -> Option<&T> {
        if let Some(right) = &node.right {
            Self::find_max_rec(right)
        } else {
            Some(&node.element)
        }
    }

    pub fn make_empty(&mut self) {
        self.root = None;
    }
}

fn main() {
    let mut tree = SearchTree::new();
    let mut j = 0;

    // Insert elements
    for i in 0..50 {
        j = (j + 7) % 50;
        tree.insert(j);
    }

    // Verify elements
    for i in 0..50 {
        if tree.find(i).is_none() {
            println!("Error at {}", i);
        }
    }

    // Delete even elements
    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    // Verify remaining odd elements
    for i in (1..50).step_by(2) {
        if tree.find(i).is_none() {
            println!("Error at {}", i);
        }
    }

    // Verify deleted even elements
    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    // Print min and max
    if let Some(min) = tree.find_min() {
        if let Some(max) = tree.find_max() {
            println!("Min is {:?}, Max is {:?}", min, max);
        }
    }
}