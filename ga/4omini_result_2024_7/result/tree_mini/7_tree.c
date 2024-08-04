use std::cmp::Ord;
use std::fmt;
use std::mem;
use std::ptr;

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

pub struct SearchTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Copy + fmt::Debug> SearchTree<T> {
    pub fn new() -> Self {
        SearchTree { root: None }
    }

    pub fn make_empty(&mut self) {
        self.root = None;
    }

    pub fn find(&self, x: T) -> Option<&T> {
        self.find_node(&self.root, x)
    }

    fn find_node(&self, node: &Option<Box<TreeNode<T>>>, x: T) -> Option<&T> {
        match node {
            Some(n) => {
                if x < n.element {
                    self.find_node(&n.left, x)
                } else if x > n.element {
                    self.find_node(&n.right, x)
                } else {
                    Some(&n.element)
                }
            }
            None => None,
        }
    }

    pub fn find_min(&self) -> Option<&T> {
        self.find_min_node(&self.root)
    }

    fn find_min_node(&self, node: &Option<Box<TreeNode<T>>>) -> Option<&T> {
        match node {
            Some(n) => {
                if n.left.is_none() {
                    Some(&n.element)
                } else {
                    self.find_min_node(&n.left)
                }
            }
            None => None,
        }
    }

    pub fn find_max(&self) -> Option<&T> {
        self.find_max_node(&self.root)
    }

    fn find_max_node(&self, node: &Option<Box<TreeNode<T>>>) -> Option<&T> {
        match node {
            Some(n) => {
                if n.right.is_none() {
                    Some(&n.element)
                } else {
                    self.find_max_node(&n.right)
                }
            }
            None => None,
        }
    }

    pub fn insert(&mut self, x: T) {
        self.root = self.insert_node(self.root.take(), x);
    }

    fn insert_node(&self, node: Option<Box<TreeNode<T>>>, x: T) -> Option<Box<TreeNode<T>>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = self.insert_node(n.left, x);
                    Some(n)
                } else if x > n.element {
                    n.right = self.insert_node(n.right, x);
                    Some(n)
                } else {
                    Some(n) // Already exists
                }
            }
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
        }
    }

    pub fn delete(&mut self, x: T) {
        self.root = self.delete_node(self.root.take(), x);
    }

    fn delete_node(&self, node: Option<Box<TreeNode<T>>>, x: T) -> Option<Box<TreeNode<T>>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = self.delete_node(n.left, x);
                    Some(n)
                } else if x > n.element {
                    n.right = self.delete_node(n.right, x);
                    Some(n)
                } else {
                    // Node found
                    if n.left.is_some() && n.right.is_some() {
                        let min_right = self.find_min_node(&n.right).unwrap(); // Safe to unwrap
                        n.element = *min_right; // Replace with min from right subtree
                        n.right = self.delete_node(n.right, *min_right);
                        Some(n)
                    } else {
                        // One child or no child
                        let child = if n.left.is_some() {
                            n.left.take()
                        } else {
                            n.right.take()
                        };
                        child
                    }
                }
            }
            None => {
                println!("Error: Element not found");
                None
            }
        }
    }

    pub fn retrieve(&self, node: &Option<Box<TreeNode<T>>>) -> Option<&T> {
        node.as_ref().map(|n| &n.element)
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
        if tree.find(i).is_none() {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    for i in (1..50).step_by(2) {
        if tree.find(i).is_none() {
            println!("Error at {}", i);
        }
    }
    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    if let (Some(min), Some(max)) = (tree.find_min(), tree.find_max()) {
        println!("Min is {:?}, Max is {:?}", min, max);
    }
}