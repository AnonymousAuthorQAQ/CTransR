use std::cmp::Ordering;
use std::fmt;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(element: i32) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
}

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.element)
    }
}

#[derive(Debug)]
struct SearchTree(Option<Box<TreeNode>>);

impl SearchTree {
    fn make_empty(&mut self) {
        self.0.take();
    }

    fn find(&self, x: i32) -> Option<&TreeNode> {
        let mut current = self.0.as_ref();
        while let Some(node) = current {
            match x.cmp(&node.element) {
                Ordering::Less => current = node.left.as_ref(),
                Ordering::Greater => current = node.right.as_ref(),
                Ordering::Equal => return Some(node),
            }
        }
        None
    }

    fn find_min(&self) -> Option<&TreeNode> {
        let mut current = self.0.as_ref();
        while let Some(node) = current {
            if node.left.is_none() {
                return Some(node);
            }
            current = node.left.as_ref();
        }
        None
    }

    fn find_max(&self) -> Option<&TreeNode> {
        let mut current = self.0.as_ref();
        while let Some(node) = current {
            if node.right.is_none() {
                return Some(node);
            }
            current = node.right.as_ref();
        }
        None
    }

    fn insert(&mut self, x: i32) {
        match self.0.as_mut() {
            None => self.0 = Some(Box::new(TreeNode::new(x))),
            Some(node) => match x.cmp(&node.element) {
                Ordering::Less => {
                    if node.left.is_none() {
                        node.left = Some(Box::new(TreeNode::new(x)));
                    } else {
                        self.insert(x);
                    }
                }
                Ordering::Greater => {
                    if node.right.is_none() {
                        node.right = Some(Box::new(TreeNode::new(x)));
                    } else {
                        self.insert(x);
                    }
                }
                Ordering::Equal => {}
            },
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
                    if let (Some(left), Some(right)) = (node.left.take(), node.right.take()) {
                        self.0 = Some(left);
                        self.insert(right.element);
                    } else if let Some(left) = node.left.take() {
                        self.0 = Some(left);
                    } else if let Some(right) = node.right.take() {
                        self.0 = Some(right);
                    } else {
                        self.0 = None;
                    }
                }
            }
        }
    }

    fn retrieve(&self) -> Option<i32> {
        self.0.as_ref().map(|node| node.element)
    }
}

fn main() {
    let mut tree = SearchTree(None);
    for i in 0..50 {
        tree.insert(i);
    }
    for i in 0..50 {
        assert_eq!(tree.find(i), Some(&TreeNode::new(i)));
    }
    for i in 0..50 {
        if i % 2 == 0 {
            tree.delete(i);
        }
    }
    for i in 0..50 {
        if i % 2 == 1 {
            assert_eq!(tree.find(i), Some(&TreeNode::new(i)));
        } else {
            assert_eq!(tree.find(i), None);
        }
    }
    println!("Min is {}, Max is {}", tree.find_min().unwrap(), tree.find_max().unwrap());
}