use std::cmp::Ord;
use std::io::{self, Write};

struct TreeNode<T> {
    element: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

pub struct SearchTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord> SearchTree<T> {
    pub fn new() -> Self {
        SearchTree { root: None }
    }

    pub fn insert(&mut self, x: T) {
        self.root = Self::insert_node(self.root.take(), x);
    }

    fn insert_node(node: Option<Box<TreeNode<T>>>, x: T) -> Option<Box<TreeNode<T>>> {
        match node {
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_node(n.left.take(), x);
                } else if x > n.element {
                    n.right = Self::insert_node(n.right.take(), x);
                }
                Some(n)
            }
        }
    }

    pub fn find(&self, x: T) -> Option<&TreeNode<T>> {
        self.find_node(&self.root, x)
    }

    fn find_node<'a>(&'a self, node: &'a Option<Box<TreeNode<T>>>, x: T) -> Option<&'a TreeNode<T>> {
        match node {
            None => None,
            Some(n) => {
                if x < n.element {
                    self.find_node(&n.left, x)
                } else if x > n.element {
                    self.find_node(&n.right, x)
                } else {
                    Some(n)
                }
            }
        }
    }

    pub fn find_min(&self) -> Option<&T> {
        self.find_min_node(&self.root)
    }

    fn find_min_node<'a>(&'a self, node: &'a Option<Box<TreeNode<T>>>) -> Option<&'a T> {
        match node {
            None => None,
            Some(n) => {
                if n.left.is_none() {
                    Some(&n.element)
                } else {
                    self.find_min_node(&n.left)
                }
            }
        }
    }

    pub fn find_max(&self) -> Option<&T> {
        self.find_max_node(&self.root)
    }

    fn find_max_node<'a>(&'a self, node: &'a Option<Box<TreeNode<T>>>) -> Option<&'a T> {
        match node {
            None => None,
            Some(n) => {
                if n.right.is_none() {
                    Some(&n.element)
                } else {
                    self.find_max_node(&n.right)
                }
            }
        }
    }

    pub fn delete(&mut self, x: T) -> Result<(), &'static str> {
        self.root = Self::delete_node(self.root.take(), x)
    }

    fn delete_node(node: Option<Box<TreeNode<T>>>, x: T) -> Option<Box<TreeNode<T>>> {
        match node {
            None => Err("Element not found").ok(),
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete_node(n.left.take(), x);
                    Some(n)
                } else if x > n.element {
                    n.right = Self::delete_node(n.right.take(), x);
                    Some(n)
                } else {
                    // Node with only one child or no child
                    match (n.left.take(), n.right.take()) {
                        (None, None) => None, // No children
                        (Some(left), None) => Some(left), // Only left child
                        (None, Some(right)) => Some(right), // Only right child
                        (Some(left), Some(right)) => {
                            let min_node = Self::find_min_node(&Some(right.clone()));
                            n.element = min_node.unwrap().element;
                            n.right = Self::delete_node(Some(right), n.element);
                            Some(n)
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let mut tree = SearchTree::new();
    let mut j = 0;

    // Insert elements 0 to 49 in a specific order
    for i in 0..50 {
        j = (j + 7) % 50;
        tree.insert(j);
    }

    // Verify elements were inserted correctly
    for i in 0..50 {
        match tree.find(i) {
            Some(node) if node.element == i => {}
            _ => println!("Error at {}", i),
        }
    }

    // Delete even elements
    for i in (0..50).step_by(2) {
        tree.delete(i).unwrap();
    }

    // Verify remaining elements are odd
    for i in (1..50).step_by(2) {
        match tree.find(i) {
            Some(node) if node.element == i => {}
            _ => println!("Error at {}", i),
        }
    }
    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    // Print the min and max
    match (tree.find_min(), tree.find_max()) {
        (Some(min), Some(max)) => {
            println!("Min is {}, Max is {}", min, max);
        }
        _ => println!("Tree is empty"),
    }
}