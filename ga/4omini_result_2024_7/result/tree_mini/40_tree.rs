use std::cmp::Ord;
use std::collections::HashSet;

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
        Self::find_node(&self.root, x)
    }

    fn find_node(node: &Option<Box<TreeNode>>, x: i32) -> Option<&TreeNode> {
        match node {
            Some(n) => {
                if x < n.element {
                    Self::find_node(&n.left, x)
                } else if x > n.element {
                    Self::find_node(&n.right, x)
                } else {
                    Some(n)
                }
            }
            None => None,
        }
    }

    fn find_min(&self) -> Option<&TreeNode> {
        Self::find_min_node(&self.root)
    }

    fn find_min_node(node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            Some(n) => {
                if n.left.is_none() {
                    Some(n)
                } else {
                    Self::find_min_node(&n.left)
                }
            }
            None => None,
        }
    }

    fn find_max(&self) -> Option<&TreeNode> {
        Self::find_max_node(&self.root)
    }

    fn find_max_node(node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            Some(n) => {
                if n.right.is_none() {
                    Some(n)
                } else {
                    Self::find_max_node(&n.right)
                }
            }
            None => None,
        }
    }

    fn insert(&mut self, x: i32) {
        self.root = Self::insert_node(self.root.take(), x);
    }

    fn insert_node(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            None => {
                Some(Box::new(TreeNode {
                    element: x,
                    left: None,
                    right: None,
                }))
            }
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_node(n.left.take(), x);
                } else if x > n.element {
                    n.right = Self::insert_node(n.right.take(), x);
                }
                Some(n)
            }
        }
    }

    fn delete(&mut self, x: i32) {
        self.root = Self::delete_node(self.root.take(), x);
    }

    fn delete_node(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            None => {
                eprintln!("Error: Element not found");
                return None;
            }
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete_node(n.left.take(), x);
                } else if x > n.element {
                    n.right = Self::delete_node(n.right.take(), x);
                } else {
                    if n.left.is_some() && n.right.is_some() {
                        let min_node = Self::find_min_node(&n.right).unwrap();
                        n.element = min_node.element;
                        n.right = Self::delete_node(n.right.take(), min_node.element);
                    } else {
                        return if n.left.is_none() {
                            n.right.take()
                        } else {
                            n.left.take()
                        };
                    }
                }
                Some(n)
            }
        }
    }
}

fn main() {
    let mut tree = SearchTree::new();
    let mut j = 0;

    for _ in 0..50 {
        tree.insert(j);
        j = (j + 7) % 50;
    }

    // Verify insertion
    for i in 0..50 {
        if tree.find(i).is_none() {
            println!("Error at {}", i);
        }
    }

    // Delete even numbers
    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    // Verify deletion
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
    let min_value = tree.find_min().map_or("None".to_string(), |n| n.element.to_string());
    let max_value = tree.find_max().map_or("None".to_string(), |n| n.element.to_string());
    println!("Min is {}, Max is {}", min_value, max_value);
}