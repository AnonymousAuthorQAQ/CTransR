use std::env;
use std::process;

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
    fn new() -> SearchTree {
        SearchTree { root: None }
    }

    fn make_empty(&mut self) {
        self.root = None;
    }

    fn find(&self, x: i32) -> Option<&TreeNode> {
        Self::find_node(&self.root, x)
    }

    fn find_node(node: &Option<Box<TreeNode>>, x: i32) -> Option<&TreeNode> {
        if let Some(n) = node {
            if x < n.element {
                Self::find_node(&n.left, x)
            } else if x > n.element {
                Self::find_node(&n.right, x)
            } else {
                Some(n) // Element found
            }
        } else {
            None // Element not found
        }
    }

    fn find_min(&self) -> Option<i32> {
        Self::find_min_node(&self.root).map(|n| n.element)
    }

    fn find_min_node(node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        let mut current = node;
        while let Some(n) = current {
            if n.left.is_none() {
                return Some(n);
            }
            current = &n.left;
        }
        None
    }

    fn find_max(&self) -> Option<i32> {
        Self::find_max_node(&self.root).map(|n| n.element)
    }

    fn find_max_node(node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        let mut current = node;
        while let Some(n) = current {
            if n.right.is_none() {
                return Some(n);
            }
            current = &n.right;
        }
        None
    }

    fn insert(&mut self, x: i32) {
        self.root = self.insert_node(self.root.take(), x);
    }

    fn insert_node(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_node(n.left, x);
                } else if x > n.element {
                    n.right = Self::insert_node(n.right, x);
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

    fn delete(&mut self, x: i32) {
        self.root = self.delete_node(self.root.take(), x);
    }

    fn delete_node(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete_node(n.left, x);
                } else if x > n.element {
                    n.right = Self::delete_node(n.right, x);
                } else {
                    // Found node to delete
                    if n.left.is_none() {
                        return n.right; // 0 or 1 child
                    } else if n.right.is_none() {
                        return n.left; // 1 child
                    }
                    // Node has 2 children
                    let min_node = Self::find_min_node(&n.right);
                    if let Some(min_node) = min_node {
                        n.element = min_node.element;
                        n.right = Self::delete_node(n.right, min_node.element);
                    }
                }
                Some(n)
            }
            None => {
                println!("Element not found");
                None
            }
        }
    }
}

fn main() {
    let mut tree = SearchTree::new();

    for i in 0..50 {
        let j = (i * 7 % 50) as i32;
        tree.insert(j);
    }

    for i in 0..50 {
        match tree.find(i) {
            Some(node) if node.element == i => {}
            _ => println!("Error at {}", i),
        }
    }

    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    for i in (1..50).step_by(2) {
        match tree.find(i) {
            Some(node) if node.element == i => {}
            _ => println!("Error at {}", i),
        }
    }

    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    println!(
        "Min is {:?}, Max is {:?}",
        tree.find_min().unwrap_or(-1),
        tree.find_max().unwrap_or(-1)
    );
}