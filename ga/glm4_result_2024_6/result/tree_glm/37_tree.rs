use std::cmp::Ordering;
use std::ptr::NonNull;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(element: i32) -> Box<TreeNode> {
        Box::new(TreeNode {
            element,
            left: None,
            right: None,
        })
    }
}

#[derive(Debug)]
struct SearchTree(Option<Box<TreeNode>>);

impl SearchTree {
    fn make_empty(&mut self) {
        self.0 = None;
    }

    fn find(&self, x: i32) -> Option<&TreeNode> {
        let mut current = &self.0;
        while let Some(node) = current {
            match x.cmp(&node.element) {
                Ordering::Less => current = &node.left,
                Ordering::Greater => current = &node.right,
                Ordering::Equal => return Some(node),
            }
        }
        None
    }

    fn find_min(&self) -> Option<&TreeNode> {
        let mut current = &self.0;
        while let Some(node) = current {
            if node.left.is_none() {
                return Some(node);
            }
            current = &node.left;
        }
        None
    }

    fn find_max(&self) -> Option<&TreeNode> {
        let mut current = &self.0;
        while let Some(node) = current {
            if node.right.is_none() {
                return Some(node);
            }
            current = &node.right;
        }
        None
    }

    fn insert(&mut self, x: i32) {
        if self.0.is_none() {
            self.0 = Some(TreeNode::new(x));
        } else {
            let mut current = self.0.as_mut();
            while let Some(node) = current {
                match x.cmp(&node.element) {
                    Ordering::Less => {
                        if node.left.is_none() {
                            node.left = Some(TreeNode::new(x));
                            return;
                        }
                        current = node.left.as_mut();
                    }
                    Ordering::Greater => {
                        if node.right.is_none() {
                            node.right = Some(TreeNode::new(x));
                            return;
                        }
                        current = node.right.as_mut();
                    }
                    Ordering::Equal => return,
                }
            }
        }
    }

    fn delete(&mut self, x: i32) {
        if let Some(mut node) = self.0.take() {
            match x.cmp(&node.element) {
                Ordering::Less => {
                    if let Some(left) = node.left.take() {
                        self.0 = Some(left);
                        self.delete(x);
                    }
                }
                Ordering::Greater => {
                    if let Some(right) = node.right.take() {
                        self.0 = Some(right);
                        self.delete(x);
                    }
                }
                Ordering::Equal => {
                    if let Some(left) = node.left.take() {
                        self.0 = Some(left);
                        if let Some(right) = node.right {
                            self.insert(right.element);
                        }
                    } else if let Some(right) = node.right.take() {
                        self.0 = Some(right);
                    } else {
                        self.0 = None;
                    }
                }
            }
        }
    }

    fn retrieve(&self, p: &TreeNode) -> i32 {
        p.element
    }
}

fn main() {
    let mut tree = SearchTree(None);
    for i in 0..50 {
        tree.insert(i);
    }
    for i in 0..50 {
        if tree.find(i).is_none() || tree.retrieve(tree.find(i).unwrap()) != i {
            println!("Error at {}", i);
        }
    }
    for i in 0..50 {
        if i % 2 == 0 {
            tree.delete(i);
        }
    }
    for i in 1..50 {
        if i % 2 == 1 {
            if tree.find(i).is_none() || tree.retrieve(tree.find(i).unwrap()) != i {
                println!("Error at {}", i);
            }
        } else {
            if tree.find(i).is_some() {
                println!("Error at {}", i);
            }
        }
    }
    println!("Min is {}, Max is {}", tree.retrieve(tree.find_min().unwrap()), tree.retrieve(tree.find_max().unwrap()));
}