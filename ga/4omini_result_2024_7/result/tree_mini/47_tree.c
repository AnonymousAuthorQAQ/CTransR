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
        self.find_rec(&self.root, x)
    }

    fn find_rec(&self, node: &Option<Box<TreeNode>>, x: i32) -> Option<&TreeNode> {
        match node {
            Some(n) => {
                if x < n.element {
                    self.find_rec(&n.left, x)
                } else if x > n.element {
                    self.find_rec(&n.right, x)
                } else {
                    Some(n)
                }
            }
            None => None,
        }
    }

    fn find_min(&self) -> Option<&TreeNode> {
        self.find_min_rec(&self.root)
    }

    fn find_min_rec(&self, node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            Some(n) => {
                if n.left.is_none() {
                    Some(n)
                } else {
                    self.find_min_rec(&n.left)
                }
            }
            None => None,
        }
    }

    fn find_max(&self) -> Option<&TreeNode> {
        self.find_max_rec(&self.root)
    }

    fn find_max_rec(&self, node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        let mut current = node;
        while let Some(n) = current {
            current = &n.right;
        }
        current.and_then(|n| Some(n.as_ref()))
    }

    fn insert(&mut self, x: i32) {
        self.root = self.insert_rec(self.root.take(), x);
    }

    fn insert_rec(&self, node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = self.insert_rec(n.left, x);
                } else if x > n.element {
                    n.right = self.insert_rec(n.right, x);
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
        self.root = self.delete_rec(self.root.take(), x);
    }

    fn delete_rec(&self, node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        let mut node = node?;

        if x < node.element {
            node.left = self.delete_rec(node.left, x);
        } else if x > node.element {
            node.right = self.delete_rec(node.right, x);
        } else {
            if node.left.is_some() && node.right.is_some() {
                let min_node = self.find_min_rec(&node.right).unwrap();
                node.element = min_node.element;
                node.right = self.delete_rec(node.right, min_node.element);
            } else {
                return if node.left.is_none() { node.right } else { node.left };
            }
        }
        Some(node)
    }

    fn retrieve(&self, node: &TreeNode) -> i32 {
        node.element
    }

    fn print_error(&self, expected: i32) {
        println!("Error at {}", expected);
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
            Some(node) => {
                if tree.retrieve(node) != i {
                    tree.print_error(i);
                }
            }
            None => {
                tree.print_error(i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    for i in (1..50).step_by(2) {
        match tree.find(i) {
            Some(node) => {
                if tree.retrieve(node) != i {
                    tree.print_error(i);
                }
            }
            None => {
                tree.print_error(i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    match tree.find_min() {
        Some(min_node) => {
            match tree.find_max() {
                Some(max_node) => {
                    println!("Min is {}, Max is {}", tree.retrieve(min_node), tree.retrieve(max_node));
                }
                None => println!("Max not found"),
            }
        }
        None => println!("Min not found"),
    }
}