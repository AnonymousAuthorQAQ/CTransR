use std::cmp::Ordering;

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

    fn make_empty(&mut self) {
        if let Some(left) = self.left.take() {
            left.make_empty();
        }
        if let Some(right) = self.right.take() {
            right.make_empty();
        }
    }

    fn find(&self, x: i32) -> Option<&Self> {
        match self.element.cmp(&x) {
            Ordering::Less => self.right.as_ref().and_then(|r| r.find(x)),
            Ordering::Greater => self.left.as_ref().and_then(|l| l.find(x)),
            Ordering::Equal => Some(self),
        }
    }

    fn find_min(&self) -> Option<&Self> {
        match self.left.as_ref() {
            Some(left) => left.find_min(),
            None => Some(self),
        }
    }

    fn find_max(&self) -> Option<&Self> {
        let mut current = self;
        while let Some(right) = current.right.as_ref() {
            current = right;
        }
        Some(current)
    }

    fn insert(&mut self, x: i32) {
        match self.element.cmp(&x) {
            Ordering::Less => {
                if self.right.is_none() {
                    self.right = Some(Box::new(TreeNode::new(x)));
                } else {
                    self.right.as_mut().unwrap().insert(x);
                }
            }
            Ordering::Greater => {
                if self.left.is_none() {
                    self.left = Some(Box::new(TreeNode::new(x)));
                } else {
                    self.left.as_mut().unwrap().insert(x);
                }
            }
            Ordering::Equal => {
                // Do nothing, duplicates are not allowed in this implementation
            }
        }
    }

    fn delete(&mut self, x: i32) {
        match self.element.cmp(&x) {
            Ordering::Less => {
                if let Some(right) = self.right.as_mut() {
                    right.delete(x);
                }
            }
            Ordering::Greater => {
                if let Some(left) = self.left.as_mut() {
                    left.delete(x);
                }
            }
            Ordering::Equal => {
                if let Some(left) = self.left.take() {
                    self.element = left.find_max().unwrap().element;
                    self.right = left.right;
                    if let Some(right) = self.right.as_mut() {
                        right.delete(self.element);
                    }
                } else if let Some(right) = self.right.take() {
                    self.element = right.find_min().unwrap().element;
                    self.left = right.left;
                    if let Some(left) = self.left.as_mut() {
                        left.delete(self.element);
                    }
                } else {
                    // No children, just remove this node
                }
            }
        }
    }
}

fn main() {
    let mut root: Option<Box<TreeNode>> = None;

    // Insert elements into the tree
    for i in 0..50 {
        let mut node = TreeNode::new(i);
        if let Some(r) = root.as_mut() {
            node.insert(r.element);
            root = Some(node);
        } else {
            root = Some(Box::new(node));
        }
    }

    // Find elements in the tree
    for i in 0..50 {
        if let Some(node) = root.as_ref().and_then(|r| r.find(i)) {
            if node.element != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    // Delete elements from the tree
    for i in 0..50 {
        if i % 2 == 0 {
            if let Some(mut node) = root.as_mut() {
                node.delete(i);
                root = Some(node);
            }
        }
    }

    // Find elements in the tree again
    for i in 0..50 {
        if i % 2 != 0 {
            if let Some(node) = root.as_ref().and_then(|r| r.find(i)) {
                if node.element != i {
                    println!("Error at {}", i);
                }
            } else {
                println!("Error at {}", i);
            }
        } else {
            if root.as_ref().and_then(|r| r.find(i)).is_some() {
                println!("Error at {}", i);
            }
        }
    }

    // Print the minimum and maximum elements in the tree
    if let Some(min) = root.as_ref().and_then(|r| r.find_min()) {
        if let Some(max) = root.as_ref().and_then(|r| r.find_max()) {
            println!("Min is {}, Max is {}", min.element, max.element);
        }
    }
}