use std::cmp::PartialOrd;
use std::fmt::Debug;

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

    fn insert(&mut self, x: i32) {
        self.root = Self::insert_node(self.root.take(), x);
    }

    fn insert_node(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_node(n.left, x);
                } else if x > n.element {
                    n.right = Self::insert_node(n.right, x);
                }
                Some(n)
            }
        }
    }

    fn find(&self, x: i32) -> Option<&TreeNode> {
        Self::find_node(&self.root, x)
    }

    fn find_node(node: &Option<Box<TreeNode>>, x: i32) -> Option<&TreeNode> {
        match node {
            None => None,
            Some(n) => {
                if x < n.element {
                    Self::find_node(&n.left, x)
                } else if x > n.element {
                    Self::find_node(&n.right, x)
                } else {
                    Some(n)
                }
            }
        }
    }

    fn find_min(&self) -> Option<i32> {
        self.root.as_ref().and_then(|n| Self::find_min_node(n))
    }

    fn find_min_node(node: &Box<TreeNode>) -> Option<i32> {
        if let Some(ref left) = node.left {
            Self::find_min_node(left)
        } else {
            Some(node.element)
        }
    }

    fn find_max(&self) -> Option<i32> {
        self.root.as_ref().and_then(|n| Self::find_max_node(n))
    }

    fn find_max_node(node: &Box<TreeNode>) -> Option<i32> {
        if let Some(ref right) = node.right {
            Self::find_max_node(right)
        } else {
            Some(node.element)
        }
    }

    fn delete(&mut self, x: i32) {
        self.root = Self::delete_node(self.root.take(), x);
    }

    fn delete_node(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        if let Some(mut n) = node {
            if x < n.element {
                n.left = Self::delete_node(n.left, x);
            } else if x > n.element {
                n.right = Self::delete_node(n.right, x);
            } else if n.left.is_some() && n.right.is_some() {
                n.element = Self::find_min_node(n.right.as_ref().unwrap()).unwrap();
                n.right = Self::delete_node(n.right, n.element);
            } else if n.left.is_none() {
                return n.right;
            } else {
                return n.left;
            }
            Some(n)
        } else {
            panic!("Element not found");
        }
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
        if tree.find(i).is_none() {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

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

    if let (Some(min), Some(max)) = (tree.find_min(), tree.find_max()) {
        println!("Min is {}, Max is {}", min, max);
    }
}