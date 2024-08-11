use std::cmp::Ord;
use std::io::{self, Write};

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

pub struct SearchTree {
    root: Option<Box<TreeNode>>,
}

impl SearchTree {
    pub fn new() -> Self {
        SearchTree { root: None }
    }

    pub fn make_empty(&mut self) {
        self.root = None;
    }

    pub fn find(&self, x: i32) -> Option<&Box<TreeNode>> {
        Self::find_helper(&self.root, x)
    }

    fn find_helper(node: &Option<Box<TreeNode>>, x: i32) -> Option<&Box<TreeNode>> {
        match node {
            None => None,
            Some(n) => {
                if x < n.element {
                    Self::find_helper(&n.left, x)
                } else if x > n.element {
                    Self::find_helper(&n.right, x)
                } else {
                    Some(n)
                }
            }
        }
    }

    pub fn find_min(&self) -> Option<&Box<TreeNode>> {
        Self::find_min_helper(&self.root)
    }

    fn find_min_helper(node: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
        match node {
            None => None,
            Some(n) => {
                if n.left.is_none() {
                    Some(n)
                } else {
                    Self::find_min_helper(&n.left)
                }
            }
        }
    }

    pub fn find_max(&self) -> Option<&Box<TreeNode>> {
        Self::find_max_helper(&self.root)
    }

    fn find_max_helper(node: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
        match node {
            None => None,
            Some(n) => {
                let mut current = n;
                while let Some(ref right) = current.right {
                    current = right;
                }
                Some(current)
            }
        }
    }

    pub fn insert(&mut self, x: i32) {
        self.root = Self::insert_helper(self.root.take(), x);
    }

    fn insert_helper(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_helper(n.left, x);
                } else if x > n.element {
                    n.right = Self::insert_helper(n.right, x);
                }
                Some(n)
            }
        }
    }

    pub fn delete(&mut self, x: i32) {
        self.root = Self::delete_helper(self.root.take(), x);
    }

    fn delete_helper(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            None => {
                eprintln!("Error: Element not found");
                None
            }
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete_helper(n.left, x);
                } else if x > n.element {
                    n.right = Self::delete_helper(n.right, x);
                } else {
                    // Node with one child or no child
                    if n.left.is_none() {
                        return n.right;
                    } else if n.right.is_none() {
                        return n.left;
                    }
                    // Node with two children: Get the inorder successor (smallest in the right subtree)
                    let min_node = Self::find_min_helper(&n.right);
                    if let Some(min_node) = min_node {
                        n.element = min_node.element;
                        n.right = Self::delete_helper(n.right, min_node.element);
                    }
                }
                Some(n)
            }
        }
    }

    pub fn retrieve(&self, node: &Box<TreeNode>) -> i32 {
        node.element
    }

    pub fn print_tree(&self) {
        println!("Tree: {:?}", self.root);
    }
}

fn main() {
    let mut tree = SearchTree::new();
    let mut j = 0;

    // Inserting integers
    for i in 0..50 {
        j = (j + 7) % 50;
        tree.insert(j);
    }

    // Finding and checking all integers 0 to 49
    for i in 0..50 {
        if tree.find(i).is_none() {
            eprintln!("Error at {}", i);
        }
    }

    // Deleting all even integers
    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    // Checking remaining odd integers
    for i in (1..50).step_by(2) {
        if tree.find(i).is_none() {
            eprintln!("Error at {}", i);
        }
    }

    // Checking that no even integers remain
    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            eprintln!("Error at {}", i);
        }
    }

    // Printing Min and Max
    if let (Some(min), Some(max)) = (tree.find_min(), tree.find_max()) {
        println!("Min is {}, Max is {}", tree.retrieve(min), tree.retrieve(max));
    }
}