use std::io::{self, BufRead};

type ElementType = i32;

#[derive(Debug)]
struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(element: ElementType) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, x: ElementType) {
        if x < self.element {
            if let None = self.left {
                self.left = Some(Box::new(TreeNode::new(x)));
            } else {
                self.left.as_mut().unwrap().insert(x);
            }
        } else {
            if let None = self.right {
                self.right = Some(Box::new(TreeNode::new(x)));
            } else {
                self.right.as_mut().unwrap().insert(x);
            }
        }
    }

    fn find(&self, x: ElementType) -> Option<&Self> {
        if x == self.element {
            return Some(self);
        }
        if x < self.element {
            self.left.as_ref().and_then(|left| left.find(x))
        } else {
            self.right.as_ref().and_then(|right| right.find(x))
        }
    }

    fn find_min(&self) -> Option<&Self> {
        match self.left {
            None => Some(self),
            Some(ref left) => left.find_min(),
        }
    }

    fn find_max(&self) -> Option<&Self> {
        let mut current = self;
        while let Some(ref right) = current.right {
            current = right;
        }
        Some(current)
    }

    fn delete(&mut self, x: ElementType) -> Option<Box<TreeNode>> {
        if x < self.element {
            if let Some(mut left) = self.left.take() {
                self.left = left.delete(x);
                return Some(self.into());
            }
        } else if x > self.element {
            if let Some(mut right) = self.right.take() {
                self.right = right.delete(x);
                return Some(self.into());
            }
        } else {
            if let (None, None) = (self.left.as_ref(), self.right.as_ref()) {
                return None;
            } else if let (Some(left), None) = (self.left.as_ref(), self.right.as_ref()) {
                return Some(left.into());
            } else if let (None, Some(right)) = (self.left.as_ref(), self.right.as_ref()) {
                return Some(right.into());
            } else {
                let mut left = self.left.take().unwrap();
                let mut right = self.right.take().unwrap();
                let min = right.find_min().unwrap();
                self.element = min.element;
                self.left = left.delete(min.element);
                self.right = right.delete(min.element);
                return Some(self.into());
            }
        }
        Some(self.into())
    }

    fn into(self) -> Box<TreeNode> {
        Box::new(self)
    }
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut root = TreeNode::new(0);
    for token in line.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            root.insert(num);
        }
    }

    if let Some(node) = root.find_min() {
        println!("Min is {}", node.element);
    }
    if let Some(node) = root.find_max() {
        println!("Max is {}", node.element);
    }

    let mut stack = Vec::new();
    let mut current = &root;
    while let Some(node) = current {
        stack.push(node);
        current = node.left.as_ref();
    }

    while let Some(node) = stack.pop() {
        println!("{}", node.element);
        if let Some(right) = node.right.as_ref() {
            let mut current = right;
            while let Some(node) = current {
                stack.push(node);
                current = node.left.as_ref();
            }
        }
    }
}