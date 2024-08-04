use std::ptr;
use std::io::{self, Write};

type ElementType = i32;

struct TreeNode {
    element: ElementType,
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

    fn find(&self, x: ElementType) -> Option<&TreeNode> {
        Self::find_recursive(&self.root, x)
    }

    fn find_recursive(node: &Option<Box<TreeNode>>, x: ElementType) -> Option<&TreeNode> {
        match node {
            None => None,
            Some(n) => {
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

    fn find_min(&self) -> Option<&TreeNode> {
        Self::find_min_recursive(&self.root)
    }

    fn find_min_recursive(node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            None => None,
            Some(n) => {
                if n.left.is_none() {
                    Some(n)
                } else {
                    Self::find_min_recursive(&n.left)
                }
            }
        }
    }

    fn find_max(&self) -> Option<&TreeNode> {
        Self::find_max_recursive(&self.root)
    }

    fn find_max_recursive(node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            None => None,
            Some(n) => {
                let mut current = n;
                while current.right.is_some() {
                    current = current.right.as_ref().unwrap();
                }
                Some(current)
            }
        }
    }

    fn insert(&mut self, x: ElementType) {
        self.root = Self::insert_recursive(self.root.take(), x);
    }

    fn insert_recursive(node: Option<Box<TreeNode>>, x: ElementType) -> Option<Box<TreeNode>> {
        match node {
            None => {
                let new_node = Box::new(TreeNode {
                    element: x,
                    left: None,
                    right: None,
                });
                Some(new_node)
            }
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

    fn delete(&mut self, x: ElementType) {
        self.root = Self::delete_recursive(self.root.take(), x);
    }

    fn delete_recursive(node: Option<Box<TreeNode>>, x: ElementType) -> Option<Box<TreeNode>> {
        match node {
            None => {
                eprintln!("Error: Element not found");
                None
            }
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete_recursive(n.left, x);
                } else if x > n.element {
                    n.right = Self::delete_recursive(n.right, x);
                } else {
                    // Found the node to delete
                    if n.left.is_some() && n.right.is_some() {
                        // Node with two children
                        let min_right = Self::find_min_recursive(&n.right);
                        if let Some(min_node) = min_right {
                            n.element = min_node.element;
                            n.right = Self::delete_recursive(n.right, min_node.element);
                        }
                    } else {
                        // Node with one or no children
                        if n.left.is_none() {
                            return n.right; // Replace with right child
                        } else {
                            return n.left; // Replace with left child
                        }
                    }
                }
                Some(n)
            }
        }
    }

    fn retrieve(&self, node: &Option<Box<TreeNode>>) -> Option<ElementType> {
        node.as_ref().map(|n| n.element)
    }
}

fn main() {
    let mut tree = SearchTree::new();

    for i in 0..50 {
        let j = (i * 7) % 50;
        tree.insert(j);
    }

    for i in 0..50 {
        match tree.find(i) {
            None => println!("Error at {}", i),
            Some(node) => {
                if node.element != i {
                    println!("Error at {}", i);
                }
            }
        }
    }

    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    for i in (1..50).step_by(2) {
        match tree.find(i) {
            None => println!("Error at {}", i),
            Some(node) => {
                if node.element != i {
                    println!("Error at {}", i);
                }
            }
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