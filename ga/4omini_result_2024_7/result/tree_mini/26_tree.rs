use std::cmp::Ord;
use std::io::{self, Write};
use std::ptr;

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
        Self::find_recursive(x, &self.root)
    }

    fn find_recursive(x: i32, node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            Some(n) => {
                if x < n.element {
                    Self::find_recursive(x, &n.left)
                } else if x > n.element {
                    Self::find_recursive(x, &n.right)
                } else {
                    Some(n)
                }
            }
            None => None,
        }
    }

    fn find_min(&self) -> Option<&TreeNode> {
        self.root.as_ref().and_then(|node| Self::find_min_recursive(&node))
    }

    fn find_min_recursive(node: &Box<TreeNode>) -> Option<&TreeNode> {
        if let Some(ref left) = node.left {
            Self::find_min_recursive(left)
        } else {
            Some(node)
        }
    }

    fn find_max(&self) -> Option<&TreeNode> {
        self.root.as_ref().and_then(|node| Self::find_max_recursive(&node))
    }

    fn find_max_recursive(node: &Box<TreeNode>) -> Option<&TreeNode> {
        if let Some(ref right) = node.right {
            Self::find_max_recursive(right)
        } else {
            Some(node)
        }
    }

    fn insert(&mut self, x: i32) {
        self.root = Self::insert_recursive(x, self.root.take());
    }

    fn insert_recursive(x: i32, node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        if let Some(mut n) = node {
            if x < n.element {
                n.left = Self::insert_recursive(x, n.left);
            } else if x > n.element {
                n.right = Self::insert_recursive(x, n.right);
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

    fn delete(&mut self, x: i32) {
        self.root = Self::delete_recursive(x, self.root.take());
    }

    fn delete_recursive(x: i32, node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        if let Some(mut n) = node {
            if x < n.element {
                n.left = Self::delete_recursive(x, n.left);
            } else if x > n.element {
                n.right = Self::delete_recursive(x, n.right);
            } else {
                if n.left.is_some() && n.right.is_some() {
                    let min_node = Self::find_min_recursive(n.right.as_ref().unwrap());
                    n.element = min_node.unwrap().element;
                    n.right = Self::delete_recursive(n.element, n.right);
                } else {
                    return if n.left.is_none() {
                        n.right.take()
                    } else {
                        n.left.take()
                    };
                }
            }
            Some(n)
        } else {
            panic!("Element not found");
        }
    }

    fn retrieve(node: &Option<&TreeNode>) -> Option<i32> {
        node.map(|n| n.element)
    }
}

fn main() {
    let mut tree = SearchTree::new();

    // Inserting numbers into the BST
    for i in 0..50 {
        let j = (i * 7) % 50;
        tree.insert(j);
    }

    // Checking if all elements from 0 to 49 can be found
    for i in 0..50 {
        if let None = tree.find(i) {
            println!("Error at {}", i);
        }
    }

    // Deleting even numbers
    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    // Checking if all remaining odd numbers can be found
    for i in (1..50).step_by(2) {
        if let None = tree.find(i) {
            println!("Error at {}", i);
        }
    }
    
    // Checking if deleted even numbers cannot be found
    for i in (0..50).step_by(2) {
        if let Some(_) = tree.find(i) {
            println!("Error at {}", i);
        }
    }

    // Printing the min and max
    if let Some(min) = tree.find_min() {
        if let Some(max) = tree.find_max() {
            println!("Min is {}, Max is {}", min.element, max.element);
        }
    }
}