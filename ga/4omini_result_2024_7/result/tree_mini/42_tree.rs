use std::cmp::Ord;
use std::collections::LinkedList;
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

impl<T: Ord + Copy> SearchTree<T> {
    // Create an empty search tree
    pub fn new() -> Self {
        SearchTree { root: None }
    }

    // Insert an element into the tree
    pub fn insert(&mut self, x: T) {
        self.root = Self::insert_node(self.root.take(), x);
    }

    fn insert_node(node: Option<Box<TreeNode<T>>>, x: T) -> Option<Box<TreeNode<T>>> {
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

    // Find an element in the tree
    pub fn find(&self, x: T) -> Option<&TreeNode<T>> {
        Self::find_node(&self.root, x)
    }

    fn find_node(node: &Option<Box<TreeNode<T>>>, x: T) -> Option<&TreeNode<T>> {
        if let Some(n) = node {
            if x < n.element {
                Self::find_node(&n.left, x)
            } else if x > n.element {
                Self::find_node(&n.right, x)
            } else {
                Some(n)
            }
        } else {
            None
        }
    }

    // Find the minimum element in the tree
    pub fn find_min(&self) -> Option<&T> {
        self.root.as_ref().and_then(Self::find_min_node)
    }

    fn find_min_node(node: &Option<Box<TreeNode<T>>>) -> Option<&T> {
        if let Some(n) = node {
            if n.left.is_some() {
                Self::find_min_node(&n.left)
            } else {
                Some(&n.element)
            }
        } else {
            None
        }
    }

    // Find the maximum element in the tree
    pub fn find_max(&self) -> Option<&T> {
        self.root.as_ref().and_then(Self::find_max_node)
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

    // Delete an element from the tree
    pub fn delete(&mut self, x: T) {
        self.root = Self::delete_node(self.root.take(), x);
    }

    fn delete_node(node: Option<Box<TreeNode<T>>>, x: T) -> Option<Box<TreeNode<T>>> {
        if let Some(mut n) = node {
            if x < n.element {
                n.left = Self::delete_node(n.left.take(), x);
                Some(n)
            } else if x > n.element {
                n.right = Self::delete_node(n.right.take(), x);
                Some(n)
            } else {
                // Node found
                match (n.left.take(), n.right.take()) {
                    (None, None) => None, // Leaf node
                    (Some(left), None) => Some(left), // Only left child
                    (None, Some(right)) => Some(right), // Only right child
                    (Some(left), Some(right)) => {
                        // Node with two children: Get the minimum in the right subtree
                        let min = Self::find_min_node(&Some(right)).unwrap();
                        n.element = *min; // Replace value
                        n.right = Self::delete_node(Some(right), *min);
                        Some(n)
                    }
                }
            }
        } else {
            None // x not found
        }
    }
}

fn main() {
    let mut tree = SearchTree::new();
    let mut j = 0;

    for i in 0..50 {
        j = (j + 7) % 50;
        tree.insert(j);
    }

    for i in 0..50 {
        let result = tree.find(i);
        if result.is_none() || result.unwrap().element != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    for i in (1..50).step_by(2) {
        let result = tree.find(i);
        if result.is_none() || result.unwrap().element != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        let result = tree.find(i);
        if result.is_some() {
            println!("Error at {}", i);
        }
    }

    let min = tree.find_min();
    let max = tree.find_max();
    println!(
        "Min is {:?}, Max is {:?}",
        min.map(|m| m.clone()), // Deref to copy
        max.map(|m| m.clone())
    );
}