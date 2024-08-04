use std::cmp::Ord;
use std::collections::BinaryHeap;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

struct SearchTree<T> {
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
        Self::find_recursive(&self.root, x)
    }

    fn find_recursive(node: &Option<Box<TreeNode<T>>>, x: T) -> Option<&TreeNode<T>> {
        match node {
            Some(n) => {
                if x < n.element {
                    Self::find_recursive(&n.left, x)
                } else if x > n.element {
                    Self::find_recursive(&n.right, x)
                } else {
                    Some(n)
                }
            }
            None => None,
        }
    }

    fn find_min(&self) -> Option<&TreeNode<T>> {
        Self::find_min_recursive(&self.root)
    }

    fn find_min_recursive(node: &Option<Box<TreeNode<T>>>) -> Option<&TreeNode<T>> {
        match node {
            Some(n) => {
                if n.left.is_none() {
                    Some(n)
                } else {
                    Self::find_min_recursive(&n.left)
                }
            }
            None => None,
        }
    }

    fn find_max(&self) -> Option<&TreeNode<T>> {
        Self::find_max_recursive(&self.root)
    }

    fn find_max_recursive(node: &Option<Box<TreeNode<T>>>) -> Option<&TreeNode<T>> {
        match node {
            Some(n) => {
                if n.right.is_none() {
                    Some(n)
                } else {
                    Self::find_max_recursive(&n.right)
                }
            }
            None => None,
        }
    }

    fn insert(&mut self, x: T) {
        self.root = Self::insert_recursive(self.root.take(), x);
    }

    fn insert_recursive(node: Option<Box<TreeNode<T>>>, x: T) -> Option<Box<TreeNode<T>>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_recursive(n.left, x);
                } else if x > n.element {
                    n.right = Self::insert_recursive(n.right, x);
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

    fn delete(&mut self, x: T) {
        self.root = Self::delete_recursive(self.root.take(), x);
    }

    fn delete_recursive(node: Option<Box<TreeNode<T>>>, x: T) -> Option<Box<TreeNode<T>>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete_recursive(n.left, x);
                    Some(n)
                } else if x > n.element {
                    n.right = Self::delete_recursive(n.right, x);
                    Some(n)
                } else {
                    // Node to delete found
                    if n.left.is_some() && n.right.is_some() {
                        let min_node = Self::find_min_recursive(&n.right);
                        if let Some(min) = min_node {
                            n.element = min.element;
                            n.right = Self::delete_recursive(n.right, min.element);
                        }
                    } else {
                        return if n.left.is_none() {
                            n.right
                        } else {
                            n.left
                        };
                    }
                    Some(n)
                }
            }
            None => None,
        }
    }

    fn retrieve(node: &Option<Box<TreeNode<T>>>) -> Option<T> {
        node.as_ref().map(|n| n.element)
    }
}

fn main() {
    let mut tree = SearchTree::new();
    let mut j = 0;

    for i in 0..50 {
        j = (j + 7) % 50;
        tree.insert(j);
    }

    for i in 0..50 {
        if let Some(p) = tree.find(i) {
            if p.element != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    for i in (1..50).step_by(2) {
        if let Some(p) = tree.find(i) {
            if p.element != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    if let (Some(min), Some(max)) = (tree.find_min(), tree.find_max()) {
        println!("Min is {:?}, Max is {:?}", min.element, max.element);
    }
}