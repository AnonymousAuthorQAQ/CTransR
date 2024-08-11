use std::io::{self, Write};
use std::ptr;
use std::option::Option;

type ElementType = i32;

pub struct TreeNode {
    pub element: ElementType,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

pub struct SearchTree {
    root: Option<Box<TreeNode>>,
}

impl SearchTree {
    pub fn new() -> Self {
       SearchTree { root: None }
    }

    pub fn insert(&mut self, x: ElementType) {
        self.root = Self::insert_node(self.root.take(), x);
    }

    fn insert_node(node: Option<Box<TreeNode>>, x: ElementType) -> Option<Box<TreeNode>> {
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

    pub fn find(&self, x: ElementType) -> Option<&TreeNode> {
        Self::find_node(&self.root, x)
    }

    fn find_node(node: &Option<Box<TreeNode>>, x: ElementType) -> Option<&TreeNode> {
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

    pub fn find_min(&self) -> Option<&TreeNode> {
        Self::find_min_node(&self.root)
    }

    fn find_min_node(node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
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

    pub fn find_max(&self) -> Option<&TreeNode> {
        Self::find_max_node(&self.root)
    }

    fn find_max_node(node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        let mut curr_node = node;
        while let Some(n) = curr_node {
            if n.right.is_none() {
                return Some(n);
            }
            curr_node = &n.right;
        }
        None
    }

    pub fn delete(&mut self, x: ElementType) {
        self.root = Self::delete_node(self.root.take(), x);
    }

    fn delete_node(node: Option<Box<TreeNode>>, x: ElementType) -> Option<Box<TreeNode>> {
        if let Some(mut n) = node {
            if x < n.element {
                n.left = Self::delete_node(n.left, x);
                Some(n)
            } else if x > n.element {
                n.right = Self::delete_node(n.right, x);
                Some(n)
            } else {
                // Node found
                if n.left.is_none() {
                    return n.right;
                } else if n.right.is_none() {
                    return n.left;
                } else {
                    // Node with two children
                    let min_node = Self::find_min_node(&n.right).unwrap();
                    n.element = min_node.element;
                    n.right = Self::delete_node(n.right, min_node.element);
                    Some(n)
                }
            }
        } else {
            None
        }
    }
}

pub fn retrieve(node: &Option<Box<TreeNode>>) -> Option<ElementType> {
    node.as_ref().map(|n| n.element)
}

fn main() {
    let mut tree = SearchTree::new();
    let mut j = 0;

    for i in 0..50 {
        j = (j + 7) % 50;
        tree.insert(j);
    }

    for i in 0..50 {
        match tree.find(i) {
            Some(node) if retrieve(&Some(node.clone())).unwrap() != i => {
                eprintln!("Error at {}", i);
            }
            None => {
                eprintln!("Error at {}", i);
            }
            _ => {}
        }
    }

    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    for i in (1..50).step_by(2) {
        match tree.find(i) {
            Some(node) if retrieve(&Some(node.clone())).unwrap() != i => {
                eprintln!("Error at {}", i);
            }
            None => {
                eprintln!("Error at {}", i);
            }
            _ => {}
        }
    }

    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            eprintln!("Error at {}", i);
        }
    }

    if let Some(min) = tree.find_min() {
        if let Some(max) = tree.find_max() {
            println!("Min is {}, Max is {}", retrieve(&Some(min.clone())).unwrap(), retrieve(&Some(max.clone())).unwrap());
        }
    }
}