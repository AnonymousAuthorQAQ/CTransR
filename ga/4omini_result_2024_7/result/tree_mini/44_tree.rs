use std::cmp::Ord;
use std::io::{self, Write};

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

struct BST {
    root: Option<Box<TreeNode>>,
}

impl BST {
    fn new() -> Self {
        BST { root: None }
    }

    fn make_empty(&mut self) {
        self.root = None;
    }

    fn find(&self, x: i32) -> Option<&TreeNode> {
        self.find_recursive(x, &self.root)
    }

    fn find_recursive(&self, x: i32, node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
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

    fn find_min(&self) -> Option<&TreeNode> {
        self.find_min_recursive(&self.root)
    }

    fn find_min_recursive(&self, node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
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

    fn find_max(&self) -> Option<&TreeNode> {
        self.find_max_recursive(&self.root)
    }

    fn find_max_recursive(&self, node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        let mut current = node;
        while let Some(ref n) = current {
            if n.right.is_none() {
                return Some(n);
            }
            current = &n.right;
        }
        None
    }

    fn insert(&mut self, x: i32) {
        self.root = self.insert_recursive(x, self.root.take());
    }

    fn insert_recursive(&self, x: i32, node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = self.insert_recursive(x, n.left);
                } else if x > n.element {
                    n.right = self.insert_recursive(x, n.right);
                }
                Some(n)
            }
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
        }
    }

    fn delete(&mut self, x: i32) {
        self.root = self.delete_recursive(x, self.root.take());
    }

    fn delete_recursive(&self, x: i32, node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = self.delete_recursive(x, n.left);
                    Some(n)
                } else if x > n.element {
                    n.right = self.delete_recursive(x, n.right);
                    Some(n)
                } else {
                    if n.left.is_none() {
                        return n.right;
                    } else if n.right.is_none() {
                        return n.left;
                    }
                    
                    let min_node = self.find_min_recursive(&n.right).unwrap();
                    n.element = min_node.element;
                    n.right = self.delete_recursive(min_node.element, n.right);
                    Some(n)
                }
            }
            None => {
                eprintln!("Error: Element not found");
                node
            }
        }
    }
}

fn retrieve(node: &Option<Box<TreeNode>>) -> Option<i32> {
    node.as_ref().map(|n| n.element)
}

fn main() {
    let mut bst = BST::new();
    let mut j = 0;

    for i in 0..50 {
        j = (j + 7) % 50;
        bst.insert(j);
    }

    for i in 0..50 {
        if bst.find(i).is_none() || retrieve(&bst.find(i)).unwrap() != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        bst.delete(i);
    }

    for i in (1..50).step_by(2) {
        if bst.find(i).is_none() || retrieve(&bst.find(i)).unwrap() != i {
            println!("Error at {}", i);
        }
    }
    
    for i in (0..50).step_by(2) {
        if bst.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    if let (Some(min), Some(max)) = (bst.find_min(), bst.find_max()) {
        println!("Min is {}, Max is {}", min.element, max.element);
    } else {
        println!("Tree is empty.");
    }
}