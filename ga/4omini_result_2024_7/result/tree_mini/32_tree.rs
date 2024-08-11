use std::cmp::Ord;
use std::mem;

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

struct SearchTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord> SearchTree<T> {
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
            Some(ref n) => {
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
            Some(ref n) => {
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
            Some(ref n) => {
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
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
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

    fn delete(&mut self, x: T) {
        self.root = Self::delete_recursive(self.root.take(), x);
    }

    fn delete_recursive(node: Option<Box<TreeNode<T>>>, x: T) -> Option<Box<TreeNode<T>>> {
        match node {
            None => {
                println!("Element not found");
                None
            }
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete_recursive(n.left, x);
                } else if x > n.element {
                    n.right = Self::delete_recursive(n.right, x);
                } else {
                    // Node to be deleted found
                    if n.left.is_some() && n.right.is_some() {
                        let min_right = Self::find_min_recursive(&n.right).unwrap();
                        n.element = min_right.element.clone();
                        n.right = Self::delete_recursive(n.right, n.element.clone());
                    } else {
                        // One of the children or both are None
                        let tmp_node = if n.left.is_none() {
                            n.right.take()
                        } else {
                            n.left.take()
                        };
                        return tmp_node;
                    }
                }
                Some(n)
            }
        }
    }
}

fn retrieve<T>(node: &TreeNode<T>) -> &T {
    &node.element
}

fn main() {
    let mut tree = SearchTree::new();

    // Insert elements
    let mut j = 0;
    for i in 0..50 {
        tree.insert(j);
        j = (j + 7) % 50;
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

    // Check if odd elements are still present
    for i in (1..50).step_by(2) {
        if tree.find(i).is_none() {
            println!("Error at {}", i);
        }
    }

    // Check if even elements are deleted
    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    // Print min and max
    if let Some(min) = tree.find_min() {
        if let Some(max) = tree.find_max() {
            println!("Min is {:?}, Max is {:?}", retrieve(min), retrieve(max));
        }
    }
}