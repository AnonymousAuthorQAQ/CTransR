use std::cmp::Ord;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::io::{self, Write};

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

struct SearchTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Copy + Debug> SearchTree<T> {
    fn new() -> Self {
        SearchTree { root: None }
    }

    fn make_empty(&mut self) {
        self.root = None;
    }

    fn find(&self, x: T) -> Option<&TreeNode<T>> {
        self.find_recursive(x, &self.root)
    }

    fn find_recursive(&self, x: T, node: &Option<Box<TreeNode<T>>>) -> Option<&TreeNode<T>> {
        match node {
            Some(n) => {
                if x < n.element {
                    self.find_recursive(x, &n.left)
                } else if x > n.element {
                    self.find_recursive(x, &n.right)
                } else {
                    Some(n)
                }
            }
            None => None,
        }
    }

    fn find_min(&self) -> Option<&TreeNode<T>> {
        self.find_min_recursive(&self.root)
    }

    fn find_min_recursive(&self, node: &Option<Box<TreeNode<T>>>) -> Option<&TreeNode<T>> {
        match node {
            Some(n) => {
                if n.left.is_none() {
                    Some(n)
                } else {
                    self.find_min_recursive(&n.left)
                }
            }
            None => None,
        }
    }

    fn find_max(&self) -> Option<&TreeNode<T>> {
        self.find_max_recursive(&self.root)
    }

    fn find_max_recursive(&self, node: &Option<Box<TreeNode<T>>>) -> Option<&TreeNode<T>> {
        let mut current = node;
        while let Some(n) = current {
            if n.right.is_none() {
                return Some(n);
            }
            current = &n.right;
        }
        None
    }

    fn insert(&mut self, x: T) {
        self.root = self.insert_recursive(x, self.root.take());
    }

    fn insert_recursive(&self, x: T, node: Option<Box<TreeNode<T>>>) -> Option<Box<TreeNode<T>>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = self.insert_recursive(x, n.left);
                } else if x > n.element {
                    n.right = self.insert_recursive(x, n.right);
                }
                Some(n)
            }
            None => {
                Some(Box::new(TreeNode {
                    element: x,
                    left: None,
                    right: None,
                }))
            }
        }
    }

    fn delete(&mut self, x: T) {
        self.root = self.delete_recursive(x, self.root.take());
    }

    fn delete_recursive(&self, x: T, node: Option<Box<TreeNode<T>>>) -> Option<Box<TreeNode<T>>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = self.delete_recursive(x, n.left);
                    Some(n)
                } else if x > n.element {
                    n.right = self.delete_recursive(x, n.right);
                    Some(n)
                } else {
                    // Node to be deleted found
                    if n.left.is_none() {
                        return n.right;
                    } else if n.right.is_none() {
                        return n.left;
                    } else {
                        // Node with two children
                        let min_node = self.find_min_recursive(&n.right).unwrap();
                        n.element = min_node.element;
                        n.right = self.delete_recursive(n.element, n.right);
                        Some(n)
                    }
                }
            }
            None => {
                eprintln!("Error: Element not found");
                None
            }
        }
    }

    fn retrieve(&self, node: &TreeNode<T>) -> T {
        node.element
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
        if let Some(node) = tree.find(i) {
            if tree.retrieve(node) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    for i in (1..50).step_by(2) {
        if let Some(node) = tree.find(i) {
            if tree.retrieve(node) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    if let Some(min_node) = tree.find_min() {
        if let Some(max_node) = tree.find_max() {
            println!("Min is {:?}, Max is {:?}", tree.retrieve(min_node), tree.retrieve(max_node));
        }
    }
}