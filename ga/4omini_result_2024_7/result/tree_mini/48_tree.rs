use std::cmp::Ord;
use std::collections::BinaryHeap;
use std::io::{self, Write};
use std::ptr::null_mut;

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

    fn find(&self, x: i32) -> Option<&TreeNode> {
        Self::find_node(x, &self.root)
    }

    fn find_node(x: i32, node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            Some(n) => {
                if x < n.element {
                    Self::find_node(x, &n.left)
                } else if x > n.element {
                    Self::find_node(x, &n.right)
                } else {
                    Some(n)
                }
            }
            None => None,
        }
    }

    fn find_min(&self) -> Option<&TreeNode> {
        self.root.as_ref().and_then(|n| Self::find_min_node(n))
    }

    fn find_min_node(node: &Box<TreeNode>) -> Option<&TreeNode> {
        if let Some(ref left) = node.left {
            Self::find_min_node(left)
        } else {
            Some(node)
        }
    }

    fn find_max(&self) -> Option<&TreeNode> {
        self.root.as_ref().and_then(|n| Self::find_max_node(n))
    }

    fn find_max_node(node: &Box<TreeNode>) -> Option<&TreeNode> {
        if let Some(ref right) = node.right {
            Self::find_max_node(right)
        } else {
            Some(node)
        }
    }

    fn insert(&mut self, x: i32) {
        self.root = Self::insert_node(x, self.root.take());
    }

    fn insert_node(x: i32, node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_node(x, n.left);
                } else if x > n.element {
                    n.right = Self::insert_node(x, n.right);
                }
                Some(n)
            }
        }
    }

    fn delete(&mut self, x: i32) {
        self.root = Self::delete_node(x, self.root.take());
    }

    fn delete_node(x: i32, node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            None => {
                eprintln!("Error: element not found");
                None
            }
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete_node(x, n.left);
                    Some(n)
                } else if x > n.element {
                    n.right = Self::delete_node(x, n.right);
                    Some(n)
                } else {
                    if n.left.is_none() {
                        return n.right;
                    }
                    if n.right.is_none() {
                        return n.left;
                    }
                    let min_node = Self::find_min_node(n.right.as_ref().unwrap());
                    n.element = min_node.unwrap().element;
                    n.right = Self::delete_node(n.element, n.right);
                    Some(n)
                }
            }
        }
    }

    fn retrieve(node: &Option<Box<TreeNode>>) -> Option<i32> {
        node.as_ref().map(|n| n.element)
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
            if node.element != i {
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
            if node.element != i {
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

    if let Some(min) = tree.find_min() {
        if let Some(max) = tree.find_max() {
            println!("Min is {}, Max is {}", min.element, max.element);
        }
    }
}