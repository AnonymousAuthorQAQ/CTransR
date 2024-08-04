use std::cmp::Ord;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T: Ord + Copy + Debug> {
    element: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct SearchTree<T: Ord + Copy + Debug> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Copy + Debug> SearchTree<T> {
    fn new() -> Self {
        SearchTree { root: None }
    }

    fn make_empty(&mut self) {
        self.root = None;
    }

    fn find(&self, x: T) -> Option<&TreeNode<T>> {
        Self::find_node(&self.root, x)
    }

    fn find_node(node: &Option<Box<TreeNode<T>>>, x: T) -> Option<&TreeNode<T>> {
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

    fn find_min(&self) -> Option<T> {
        Self::find_min_node(&self.root)
    }

    fn find_min_node(node: &Option<Box<TreeNode<T>>>) -> Option<T> {
        match node {
            Some(n) => {
                if n.left.is_none() {
                    Some(n.element)
                } else {
                    Self::find_min_node(&n.left)
                }
            }
            None => None,
        }
    }

    fn find_max(&self) -> Option<T> {
        Self::find_max_node(&self.root)
    }

    fn find_max_node(node: &Option<Box<TreeNode<T>>>) -> Option<T> {
        let mut current = node;
        while let Some(n) = current {
            if n.right.is_none() {
                return Some(n.element);
            }
            current = &n.right;
        }
        None
    }

    fn insert(&mut self, x: T) {
        self.root = Self::insert_node(self.root.take(), x);
    }

    fn insert_node(node: Option<Box<TreeNode<T>>>, x: T) -> Option<Box<TreeNode<T>>> {
        if let Some(mut n) = node {
            if x < n.element {
                n.left = Self::insert_node(n.left, x);
            } else if x > n.element {
                n.right = Self::insert_node(n.right, x);
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

    fn delete(&mut self, x: T) {
        self.root = Self::delete_node(self.root.take(), x);
    }

    fn delete_node(node: Option<Box<TreeNode<T>>>, x: T) -> Option<Box<TreeNode<T>>> {
        if let Some(mut n) = node {
            if x < n.element {
                n.left = Self::delete_node(n.left, x);
                return Some(n);
            } else if x > n.element {
                n.right = Self::delete_node(n.right, x);
                return Some(n);
            } else {
                if n.left.is_some() && n.right.is_some() {
                    // Two children: Get the inorder successor (min in right subtree)
                    let min_node = Self::find_min_node(&n.right).unwrap();
                    n.element = min_node; // Update element to the min
                    n.right = Self::delete_node(n.right, min_node); // Delete the successor
                } else {
                    if n.left.is_none() {
                        return n.right; // Only right child or no child
                    } else {
                        return n.left; // Only left child
                    }
                }
            }
        }
        None
    }

    fn retrieve(&self, node: &Option<Box<TreeNode<T>>>) -> Option<T> {
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
        if let Some(_) = tree.find(i) {
            continue;
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    for i in (1..50).step_by(2) {
        if let Some(_) = tree.find(i) {
            continue;
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if let Some(_) = tree.find(i) {
            println!("Error at {}", i);
        }
    }

    let min_val = match tree.find_min() {
        Some(val) => val,
        None => -1, // Arbitrary value if tree is empty
    };
    
    let max_val = match tree.find_max() {
        Some(val) => val,
        None => -1, // Arbitrary value if tree is empty
    };

    println!("Min is {}, Max is {}", min_val, max_val);
}