use std::cmp::Ord;
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
        self._find(x, &self.root)
    }

    fn _find(&self, x: i32, node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            Some(n) => {
                if x < n.element {
                    self._find(x, &n.left)
                } else if x > n.element {
                    self._find(x, &n.right)
                } else {
                    Some(n)
                }
            }
            None => None,
        }
    }

    fn find_min(&self) -> Option<&TreeNode> {
        self._find_min(&self.root)
    }

    fn _find_min(&self, node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            Some(n) => {
                if n.left.is_none() {
                    Some(n)
                } else {
                    self._find_min(&n.left)
                }
            }
            None => None,
        }
    }

    fn find_max(&self) -> Option<&TreeNode> {
        self._find_max(&self.root)
    }

    fn _find_max(&self, node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
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
        self.root = self._insert(x, self.root.take());
    }

    fn _insert(&self, x: i32, node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = self._insert(x, n.left);
                    Some(n)
                } else if x > n.element {
                    n.right = self._insert(x, n.right);
                    Some(n)
                } else {
                    Some(n) // Duplicate value, do not insert
                }
            }
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
        }
    }

    fn delete(&mut self, x: i32) {
        self.root = self._delete(x, self.root.take());
    }

    fn _delete(&self, x: i32, node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = self._delete(x, n.left);
                    Some(n)
                } else if x > n.element {
                    n.right = self._delete(x, n.right);
                    Some(n)
                } else {
                    // Found the node to delete
                    if n.left.is_none() {
                        return n.right;
                    } else if n.right.is_none() {
                        return n.left;
                    } else {
                        let min_node = self._find_min(&n.right).unwrap();
                        n.element = min_node.element;
                        n.right = self._delete(min_node.element, n.right);
                        Some(n)
                    }
                }
            }
            None => {
                eprintln!("Element not found");
                None
            }
        }
    }

    fn retrieve(&self, node: &Box<TreeNode>) -> i32 {
        node.element
    }
}

fn main() {
    let mut tree = SearchTree::new();
    let mut j: usize = 0;

    for i in 0..50 {
        j = (j + 7) % 50;
        tree.insert(j as i32);
    }

    for i in 0..50 {
        if let Some(p) = tree.find(i as i32) {
            if tree.retrieve(p) != i as i32 {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        tree.delete(i as i32);
    }

    for i in (1..50).step_by(2) {
        if let Some(p) = tree.find(i as i32) {
            if tree.retrieve(p) != i as i32 {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if tree.find(i as i32).is_some() {
            println!("Error at {}", i);
        }
    }

    if let Some(min_node) = tree.find_min() {
        if let Some(max_node) = tree.find_max() {
            println!("Min is {}, Max is {}", tree.retrieve(min_node), tree.retrieve(max_node));
        }
    }
}