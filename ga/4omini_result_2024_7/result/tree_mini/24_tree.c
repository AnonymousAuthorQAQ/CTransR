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
        Self::find_rec(&self.root, x)
    }

    fn find_rec(node: &Option<Box<TreeNode>>, x: i32) -> Option<&TreeNode> {
        match node {
            Some(n) => {
                if x < n.element {
                    Self::find_rec(&n.left, x)
                } else if x > n.element {
                    Self::find_rec(&n.right, x)
                } else {
                    Some(n)
                }
            }
            None => None,
        }
    }

    fn find_min(&self) -> Option<&TreeNode> {
        Self::find_min_rec(&self.root)
    }

    fn find_min_rec(node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            Some(n) => {
                if n.left.is_none() {
                    Some(n)
                } else {
                    Self::find_min_rec(&n.left)
                }
            }
            None => None,
        }
    }

    fn find_max(&self) -> Option<&TreeNode> {
        Self::find_max_rec(&self.root)
    }

    fn find_max_rec(node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            Some(n) => {
                if n.right.is_none() {
                    Some(n)
                } else {
                    Self::find_max_rec(&n.right)
                }
            }
            None => None,
        }
    }

    fn insert(&mut self, x: i32) {
        self.root = Self::insert_rec(self.root.take(), x);
    }

    fn insert_rec(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_rec(n.left, x);
                } else if x > n.element {
                    n.right = Self::insert_rec(n.right, x);
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
        self.root = Self::delete_rec(self.root.take(), x);
    }

    fn delete_rec(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete_rec(n.left, x);
                } else if x > n.element {
                    n.right = Self::delete_rec(n.right, x);
                } else {
                    // Node found
                    if n.left.is_some() && n.right.is_some() {
                        let min_node = Self::find_min_rec(&n.right).unwrap();
                        n.element = min_node.element;
                        n.right = Self::delete_rec(n.right, n.element);
                    } else {
                        return if n.left.is_none() {
                            n.right
                        } else {
                            n.left
                        };
                    }
                }
                Some(n)
            }
            None => {
                println!("Error: Element not found");
                None
            }
        }
    }

    fn retrieve(node: &Option<Box<TreeNode>>) -> Option<i32> {
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

    // Check if all elements from 0 to 49 are present
    for i in 0..50 {
        if tree.find(i).is_none() {
            println!("Error at {}", i);
        }
    }

    // Delete even elements
    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    // Check if only odd elements remain
    for i in (1..50).step_by(2) {
        if tree.find(i).is_none() {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    // Print min and max
    if let (Some(min), Some(max)) = (tree.find_min(), tree.find_max()) {
        println!("Min is {}, Max is {}", min.element, max.element);
    }
}