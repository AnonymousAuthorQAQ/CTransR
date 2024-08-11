use std::cmp::{Ord, PartialOrd};
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

    fn find(&self, x: i32) -> Option<&TreeNode> {
        self.find_recursive(&self.root, x)
    }

    fn find_recursive(&self, node: &Option<Box<TreeNode>>, x: i32) -> Option<&TreeNode> {
        match node {
            None => None,
            Some(ref n) => {
                if x < n.element {
                    self.find_recursive(&n.left, x)
                } else if x > n.element {
                    self.find_recursive(&n.right, x)
                } else {
                    Some(n)
                }
            }
        }
    }

    fn find_min(&self) -> Option<&TreeNode> {
        self.find_min_recursive(&self.root)
    }

    fn find_min_recursive(&self, node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            None => None,
            Some(ref n) => {
                if n.left.is_none() {
                    Some(n)
                } else {
                    self.find_min_recursive(&n.left)
                }
            }
        }
    }

    fn find_max(&self) -> Option<&TreeNode> {
        self.find_max_recursive(&self.root)
    }

    fn find_max_recursive(&self, node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            None => None,
            Some(ref n) => {
                if n.right.is_none() {
                    Some(n)
                } else {
                    self.find_max_recursive(&n.right)
                }
            }
        }
    }

    fn insert(&mut self, x: i32) {
        self.root = self.insert_recursive(self.root.take(), x);
    }

    fn insert_recursive(&self, node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
            Some(mut n) => {
                if x < n.element {
                    n.left = self.insert_recursive(n.left, x);
                } else if x > n.element {
                    n.right = self.insert_recursive(n.right, x);
                }
                Some(n)
            }
        }
    }

    fn delete(&mut self, x: i32) {
        self.root = self.delete_recursive(self.root.take(), x);
    }

    fn delete_recursive(&self, node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            None => {
                println!("Error: Element {} not found", x);
                None
            }
            Some(mut n) => {
                if x < n.element {
                    n.left = self.delete_recursive(n.left, x);
                } else if x > n.element {
                    n.right = self.delete_recursive(n.right, x);
                } else {
                    if n.left.is_some() && n.right.is_some() {
                        if let Some(min_node) = self.find_min_recursive(&n.right) {
                            n.element = min_node.element;
                            n.right = self.delete_recursive(n.right, min_node.element);
                        }
                    } else {
                        if n.left.is_none() {
                            return n.right;
                        } else {
                            return n.left;
                        }
                    }
                }
                Some(n)
            }
        }
    }

    fn retrieve(&self, node: &Option<Box<TreeNode>>) -> Option<i32> {
        node.as_ref().map(|n| n.element)
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

    // Check all elements from 0 to 49
    for i in 0..50 {
        match tree.find(i) {
            Some(node) if tree.retrieve(&Some(node.clone())).unwrap() != i => {
                println!("Error at {}", i);
            }
            None => {
                println!("Error at {}", i);
            }
            _ => {}
        }
    }

    // Delete even elements
    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    // Check remaining odd elements
    for i in (1..50).step_by(2) {
        match tree.find(i) {
            Some(node) if tree.retrieve(&Some(node.clone())).unwrap() != i => {
                println!("Error at {}", i);
            }
            None => {
                println!("Error at {}", i);
            }
            _ => {}
        }
    }

    // Check that even elements are deleted
    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    // Output min and max
    if let (Some(min), Some(max)) = (tree.find_min(), tree.find_max()) {
        println!("Min is {}, Max is {}", min.element, max.element);
    }
}