use std::cmp::Ord;
use std::ptr::null_mut;
use std::mem;

#[derive(Clone, Copy)]
pub struct ElementType(i32);

pub struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

pub struct SearchTree {
    root: Option<Box<TreeNode>>,
}

impl SearchTree {
    pub fn new() -> Self {
        SearchTree { root: None }
    }

    pub fn make_empty(&mut self) {
        self.root = None;
    }

    pub fn find(&self, x: ElementType) -> Option<&TreeNode> {
        self.find_helper(x, &self.root)
    }

    fn find_helper(&self, x: ElementType, node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            Some(n) => {
                if x.0 < n.element.0 {
                    self.find_helper(x, &n.left)
                } else if x.0 > n.element.0 {
                    self.find_helper(x, &n.right)
                } else {
                    Some(n)
                }
            }
            None => None,
        }
    }

    pub fn find_min(&self) -> Option<&TreeNode> {
        self.find_min_helper(&self.root)
    }

    fn find_min_helper(&self, node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            Some(n) => {
                if n.left.is_none() {
                    Some(n)
                } else {
                    self.find_min_helper(&n.left)
                }
            }
            None => None,
        }
    }

    pub fn find_max(&self) -> Option<&TreeNode> {
        self.find_max_helper(&self.root)
    }

    fn find_max_helper(&self, node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            Some(n) => {
                let mut current = n;
                while let Some(ref right) = current.right {
                    current = right;
                }
                Some(current)
            }
            None => None,
        }
    }

    pub fn insert(&mut self, x: ElementType) {
        self.root = self.insert_helper(x, self.root);
    }

    fn insert_helper(&self, x: ElementType, node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if x.0 < n.element.0 {
                    n.left = self.insert_helper(x, n.left);
                    Some(n)
                } else if x.0 > n.element.0 {
                    n.right = self.insert_helper(x, n.right);
                    Some(n)
                } else {
                    Some(n) // Duplicate values are ignored
                }
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

    pub fn delete(&mut self, x: ElementType) {
        self.root = self.delete_helper(x, self.root);
    }

    fn delete_helper(&self, x: ElementType, node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if x.0 < n.element.0 {
                    n.left = self.delete_helper(x, n.left);
                    Some(n)
                } else if x.0 > n.element.0 {
                    n.right = self.delete_helper(x, n.right);
                    Some(n)
                } else {
                    if n.left.is_some() && n.right.is_some() {
                        let min_node = self.find_min_helper(&n.right).unwrap().clone();
                        n.element = min_node.element; // Replace with in-order successor
                        n.right = self.delete_helper(min_node.element, n.right);
                        Some(n)
                    } else {
                        let tmp_node = n.clone();
                        if n.left.is_none() {
                            return n.right; // Replace with right child
                        } else {
                            return n.left; // Replace with left child
                        }
                    }
                }
            }
            None => {
                panic!("Element not found");
            }
        }
    }

    pub fn retrieve(&self, node: &TreeNode) -> ElementType {
        node.element
    }
}

fn main() {
    let mut tree = SearchTree::new();

    for i in 0..50 {
        let j = (i * 7) % 50; // Equivalent to the C code logic
        tree.insert(ElementType(j));
    }

    for i in 0..50 {
        if let Some(p) = tree.find(ElementType(i)) {
            if tree.retrieve(p) != ElementType(i) {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        tree.delete(ElementType(i));
    }

    for i in (1..50).step_by(2) {
        if let Some(p) = tree.find(ElementType(i)) {
            if tree.retrieve(p) != ElementType(i) {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if tree.find(ElementType(i)).is_some() {
            println!("Error at {}", i);
        }
    }

    if let Some(min) = tree.find_min() {
        let min_value = tree.retrieve(min);
        if let Some(max) = tree.find_max() {
            let max_value = tree.retrieve(max);
            println!("Min is {}, Max is {}", min_value.0, max_value.0);
        }
    }
}