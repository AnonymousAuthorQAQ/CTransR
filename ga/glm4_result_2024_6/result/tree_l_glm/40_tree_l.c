use std::io::{self, Read};
use std::collections::BinaryHeap;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct TreeNode {
    value: i32,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode { value }
    }
}

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for TreeNode {}

impl std::fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreeNode")
            .field("value", &self.value)
            .finish()
    }
}

struct SearchTree {
    root: Option<Box<TreeNode>>,
}

impl SearchTree {
    fn new() -> Self {
        SearchTree { root: None }
    }

    fn insert(&mut self, value: i32) {
        let node = Box::new(TreeNode::new(value));
        match self.root {
            None => self.root = Some(node),
            Some(ref mut root) => {
                let mut current = root;
                while *current <= node {
                    if node < *current {
                        if let Some(ref mut left) = current.left {
                            current = left;
                        } else {
                            current.left = Some(node);
                            return;
                        }
                    } else {
                        if let Some(ref mut right) = current.right {
                            current = right;
                        } else {
                            current.right = Some(node);
                            return;
                        }
                    }
                }
            }
        }
    }

    fn delete(&mut self, value: i32) {
        if let Some(mut root) = self.root.take() {
            let mut current = &mut root;
            let mut parent = None;
            while *current < TreeNode::new(value) || *current > TreeNode::new(value) {
                parent = Some(current);
                if *current < TreeNode::new(value) {
                    current = current.right.as_mut().unwrap();
                } else {
                    current = current.left.as_mut().unwrap();
                }
            }
            if let Some(mut left) = current.left.take() {
                if let Some(mut right) = current.right.take() {
                    let mut min = right;
                    while let Some(mut left) = min.left.take() {
                        min.left = left.right.take();
                        left.right = Some(min);
                        min = left;
                    }
                    if let Some(mut parent) = parent {
                        if parent.left.as_ref() == Some(current) {
                            parent.left = Some(min);
                        } else {
                            parent.right = Some(min);
                        }
                    } else {
                        self.root = Some(min);
                    }
                    if let Some(mut left) = current.left.take() {
                        min.left = Some(left);
                    }
                    if let Some(mut right) = current.right.take() {
                        min.right = Some(right);
                    }
                } else {
                    if let Some(mut parent) = parent {
                        if parent.left.as_ref() == Some(current) {
                            parent.left = current.left.take();
                        } else {
                            parent.right = current.left.take();
                        }
                    } else {
                        self.root = current.left.take();
                    }
                }
            } else if let Some(mut right) = current.right.take() {
                if let Some(mut parent) = parent {
                    if parent.left.as_ref() == Some(current) {
                        parent.left = Some(right);
                    } else {
                        parent.right = Some(right);
                    }
                } else {
                    self.root = Some(right);
                }
            } else if let Some(mut parent) = parent {
                if parent.left.as_ref() == Some(current) {
                    parent.left = None;
                } else {
                    parent.right = None;
                }
            } else {
                self.root = None;
            }
            self.root = Some(root);
        }
    }

    fn find(&self, value: i32) -> Option<&TreeNode> {
        let mut current = self.root.as_ref()?;
        while *current < TreeNode::new(value) || *current > TreeNode::new(value) {
            if *current < TreeNode::new(value) {
                current = current.right.as_ref()?;
            } else {
                current = current.left.as_ref()?;
            }
        }
        Some(current)
    }

    fn find_min(&self) -> Option<&TreeNode> {
        let mut current = self.root.as_ref()?;
        while let Some(left) = current.left.as_ref() {
            current = left;
        }
        Some(current)
    }

    fn find_max(&self) -> Option<&TreeNode> {
        let mut current = self.root.as_ref()?;
        while let Some(right) = current.right.as_ref() {
            current = right;
        }
        Some(current)
    }

    fn print_in_order(&self) {
        let mut stack = BinaryHeap::new();
        let mut current = self.root.as_ref();
        while let Some(node) = current {
            stack.push(node);
            current = node.left.as_ref();
        }
        while let Some(node) = stack.pop() {
            println!("{:?}", node);
            if let Some(right) = node.right.as_ref() {
                let mut current = right;
                while let Some(left) = current.left.as_ref() {
                    current = left;
                }
                stack.push(current);
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tree = SearchTree::new();
    for token in input.split_whitespace() {
        if let Ok(value) = token.parse::<i32>() {
            tree.insert(value);
        }
    }
    tree.print_in_order();
    tree.delete(2);
    tree.print_in_order();
    if let Some(min) = tree.find_min() {
        println!("Min: {:?}", min);
    }
    if let Some(max) = tree.find_max() {
        println!("Max: {:?}", max);
    }
}