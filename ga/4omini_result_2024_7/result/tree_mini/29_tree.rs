use std::cmp::Ord;
use std::collections::HashSet;
use std::io::{self, Write};

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

    fn insert(&mut self, x: i32) {
        self.root = self.insert_recursive(self.root.take(), x);
    }

    fn insert_recursive(&self, node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        if let Some(mut current_node) = node {
            if x < current_node.element {
                current_node.left = self.insert_recursive(current_node.left, x);
            } else if x > current_node.element {
                current_node.right = self.insert_recursive(current_node.right, x);
            }
            Some(current_node)
        } else {
            Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            }))
        }
    }

    fn find(&self, x: i32) -> Option<&TreeNode> {
        self.find_recursive(&self.root, x)
    }

    fn find_recursive<'a>(&self, node: &'a Option<Box<TreeNode>>, x: i32) -> Option<&'a TreeNode> {
        match node {
            Some(ref n) if x < n.element => self.find_recursive(&n.left, x),
            Some(ref n) if x > n.element => self.find_recursive(&n.right, x),
            Some(ref n) => Some(n),
            None => None,
        }
    }

    fn find_min(&self) -> Option<&TreeNode> {
        self.find_min_recursive(&self.root)
    }

    fn find_min_recursive<'a>(&self, node: &'a Option<Box<TreeNode>>) -> Option<&'a TreeNode> {
        match node {
            Some(ref n) => {
                if n.left.is_some() {
                    self.find_min_recursive(&n.left)
                } else {
                    Some(n)
                }
            },
            None => None,
        }
    }

    fn find_max(&self) -> Option<&TreeNode> {
        self.find_max_recursive(&self.root)
    }

    fn find_max_recursive<'a>(&self, node: &'a Option<Box<TreeNode>>) -> Option<&'a TreeNode> {
        match node {
            Some(ref n) => {
                if n.right.is_some() {
                    self.find_max_recursive(&n.right)
                } else {
                    Some(n)
                }
            },
            None => None,
        }
    }

    fn delete(&mut self, x: i32) {
        self.root = self.delete_recursive(self.root.take(), x);
    }

    fn delete_recursive(&self, node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            Some(mut current_node) => {
                if x < current_node.element {
                    current_node.left = self.delete_recursive(current_node.left, x);
                } else if x > current_node.element {
                    current_node.right = self.delete_recursive(current_node.right, x);
                } else {
                    // Node with `x` found
                    if current_node.left.is_none() {
                        return current_node.right; // Replace with right child
                    } else if current_node.right.is_none() {
                        return current_node.left; // Replace with left child
                    } else {
                        // Node with two children; get the inorder successor (smallest in the right subtree)
                        if let Some(min_node) = self.find_min_recursive(&current_node.right) {
                            current_node.element = min_node.element;
                            current_node.right = self.delete_recursive(current_node.right, min_node.element);
                        }
                    }
                }
                Some(current_node)
            },
            None => {
                eprintln!("Element not found");
                None
            },
        }
    }

    fn retrieve(&self, node: &Option<Box<TreeNode>>) -> Option<i32> {
        node.as_ref().map(|n| n.element)
    }
}

fn main() {
    let mut tree = SearchTree::new();
    let mut inserted_nums = HashSet::new();
    
    for i in 0..50 {
        let j = (i * 7) % 50;
        tree.insert(j);
        inserted_nums.insert(j);
    }

    for i in 0..50 {
        if let Some(node) = tree.find(i) {
            if tree.retrieve(&Some(node.clone())).unwrap() != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    // Delete even numbers
    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    for i in (1..50).step_by(2) {
        if let Some(node) = tree.find(i) {
            if tree.retrieve(&Some(node.clone())).unwrap() != i {
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
            println!("Min is {}, Max is {}", min_node.element, max_node.element);
        }
    }
}