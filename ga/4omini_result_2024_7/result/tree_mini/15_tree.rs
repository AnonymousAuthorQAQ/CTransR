use std::cell::RefCell;
use std::fmt;
use std::ops::Deref;
use std::ptr;
use std::rc::Rc;

type ElementType = i32;

#[derive(Debug)]
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

    fn insert(&mut self, x: ElementType) {
        self.root = Self::insert_node(self.root.take(), x);
    }

    fn insert_node(node: Option<Box<TreeNode>>, x: ElementType) -> Option<Box<TreeNode>> {
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

    fn find(&self, x: ElementType) -> Option<&Box<TreeNode>> {
        Self::find_node(&self.root, x)
    }

    fn find_node(node: &Option<Box<TreeNode>>, x: ElementType) -> Option<&Box<TreeNode>> {
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

    fn find_min(&self) -> Option<&Box<TreeNode>> {
        Self::find_min_node(&self.root)
    }

    fn find_min_node(node: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
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

    fn find_max(&self) -> Option<&Box<TreeNode>> {
        Self::find_max_node(&self.root)
    }

    fn find_max_node(node: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
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

    fn delete(&mut self, x: ElementType) {
        self.root = Self::delete_node(self.root.take(), x);
    }

    fn delete_node(node: Option<Box<TreeNode>>, x: ElementType) -> Option<Box<TreeNode>> {
        if let Some(mut n) = node {
            if x < n.element {
                n.left = Self::delete_node(n.left, x);
            } else if x > n.element {
                n.right = Self::delete_node(n.right, x);
            } else {
                // Found the node to be deleted
                if n.left.is_some() && n.right.is_some() {
                    // Node with two children
                    let min_node = Self::find_min_node(&n.right);
                    if let Some(min) = min_node {
                        n.element = min.element;
                        n.right = Self::delete_node(n.right, min.element);
                    }
                } else {
                    // Node with one or no child
                    return if n.left.is_none() {
                        n.right
                    } else {
                        n.left
                    };
                }
            }
            Some(n)
        } else {
            None
        }
    }

    fn retrieve(node: &Option<Box<TreeNode>>) -> Option<ElementType> {
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
            if tree.retrieve(&Some(p)).unwrap() != i {
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
            if tree.retrieve(&Some(p)).unwrap() != i {
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
        println!("Min is {}, Max is {}", min.element, max.element);
    }
}