use std::cmp::Ord;
use std::fmt::Debug;
use std::io::{self, Write};

#[derive(Debug)]
struct TreeNode<T: Ord + Copy + Debug> {
    element: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

struct SearchTree<T: Ord + Copy + Debug> {
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
        Self::find_recursive(&self.root, x)
    }

    fn find_recursive(node: &Option<Box<TreeNode<T>>>, x: T) -> Option<&TreeNode<T>> {
        match node {
            None => None,
            Some(ref n) => {
                if x < n.element {
                    Self::find_recursive(&n.left, x)
                } else if x > n.element {
                    Self::find_recursive(&n.right, x)
                } else {
                    Some(n)
                }
            }
        }
    }

    fn find_min(&self) -> Option<&TreeNode<T>> {
        Self::find_min_recursive(&self.root)
    }

    fn find_min_recursive(node: &Option<Box<TreeNode<T>>>) -> Option<&TreeNode<T>> {
        match node {
            None => None,
            Some(ref n) => {
                if n.left.is_none() {
                    Some(n)
                } else {
                    Self::find_min_recursive(&n.left)
                }
            }
        }
    }

    fn find_max(&self) -> Option<&TreeNode<T>> {
        Self::find_max_recursive(&self.root)
    }

    fn find_max_recursive(node: &Option<Box<TreeNode<T>>>) -> Option<&TreeNode<T>> {
        match node {
            None => None,
            Some(ref n) => {
                if n.right.is_none() {
                    Some(n)
                } else {
                    Self::find_max_recursive(&n.right)
                }
            }
        }
    }

    fn insert(&mut self, x: T) {
        self.root = Self::insert_recursive(self.root.take(), x);
    }

    fn insert_recursive(node: Option<Box<TreeNode<T>>>, x: T) -> Option<Box<TreeNode<T>>> {
        match node {
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_recursive(n.left, x);
                } else if x > n.element {
                    n.right = Self::insert_recursive(n.right, x);
                }
                Some(n)
            }
        }
    }

    fn delete(&mut self, x: T) {
        self.root = Self::delete_recursive(self.root.take(), x);
    }

    fn delete_recursive(node: Option<Box<TreeNode<T>>>, x: T) -> Option<Box<TreeNode<T>>> {
        match node {
            None => {
                eprintln!("Error: Element not found");
                return None;
            }
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete_recursive(n.left, x);
                    return Some(n);
                } else if x > n.element {
                    n.right = Self::delete_recursive(n.right, x);
                    return Some(n);
                } else {
                    if n.left.is_some() && n.right.is_some() {
                        let min_node = Self::find_min_recursive(&n.right);
                        if let Some(min_node) = min_node {
                            n.element = min_node.element;
                            n.right = Self::delete_recursive(n.right, min_node.element);
                        }
                    } else {
                        return if n.left.is_none() {
                            n.right
                        } else {
                            n.left
                        };
                    }
                }
            }
        }
    }

    fn retrieve(node: &Option<Box<TreeNode<T>>>) -> Option<T> {
        node.as_ref().map(|n| n.element)
    }
}

fn main() {
    let mut tree: SearchTree<i32> = SearchTree::new();

    for i in (0..50).map(|j| (j * 7) % 50) {
        tree.insert(i);
    }

    for i in 0..50 {
        if let Some(node) = tree.find(i) {
            if node.element != i {
                eprintln!("Error at {}", i);
            }
        } else {
            eprintln!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    for i in (1..50).step_by(2) {
        if let Some(node) = tree.find(i) {
            if node.element != i {
                eprintln!("Error at {}", i);
            }
        } else {
            eprintln!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            eprintln!("Error at {}", i);
        }
    }

    println!(
        "Min is {:?}, Max is {:?}",
        tree.retrieve(tree.find_min()),
        tree.retrieve(tree.find_max())
    );
}