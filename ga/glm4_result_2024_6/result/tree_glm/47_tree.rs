use std::cmp::Ordering;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(element: i32) -> TreeNode {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }

    fn make_empty(&mut self) {
        if let Some(ref mut left) = self.left {
            left.make_empty();
        }
        if let Some(ref mut right) = self.right {
            right.make_empty();
        }
        self.left = None;
        self.right = None;
    }

    fn find(&self, x: i32) -> Option<&TreeNode> {
        match self.element.cmp(&x) {
            Ordering::Less => self.right.as_ref().and_then(|n| n.find(x)),
            Ordering::Greater => self.left.as_ref().and_then(|n| n.find(x)),
            Ordering::Equal => Some(self),
        }
    }

    fn find_min(&self) -> Option<&TreeNode> {
        match self.left.as_ref() {
            None => Some(self),
            Some(node) => node.find_min(),
        }
    }

    fn find_max(&self) -> Option<&TreeNode> {
        match self.right.as_ref() {
            None => Some(self),
            Some(node) => node.find_max(),
        }
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
                // Do nothing for duplicate values
            }
        }
    }

    fn delete(&mut self, x: i32) -> Option<&mut TreeNode> {
        if self.element == x {
            if self.left.is_none() {
                return self.right.take();
            } else if self.right.is_none() {
                return self.left.take();
            } else {
                let mut node = self.right.take().unwrap();
                self.element = node.find_min().unwrap().element;
                self.right = Some(node);
            }
        } else {
            match self.element.cmp(&x) {
                Ordering::Less => {
                    if let Some(ref mut left) = self.left {
                        self.left = left.delete(x);
                    }
                }
                Ordering::Greater => {
                    if let Some(ref mut right) = self.right {
                        self.right = right.delete(x);
                    }
                }
                Ordering::Equal => {
                    // Do nothing for duplicate values
                }
            }
        }
        Some(self)
    }

    fn retrieve(&self) -> i32 {
        self.element
    }
}

fn main() {
    let mut root = Box::new(TreeNode::new(0));
    let mut i = 0;
    while i < 50 {
        root.insert(i);
        i += 1;
    }

    i = 0;
    while i < 50 {
        if root.find(i).unwrap().retrieve() != i {
            println!("Error at {}", i);
        }
        i += 1;
    }

    i = 0;
    while i < 50 {
        root.delete(i);
        i += 2;
    }

    i = 1;
    while i < 50 {
        if root.find(i).unwrap().retrieve() != i {
            println!("Error at {}", i);
        }
        i += 2;
    }

    i = 0;
    while i < 50 {
        if root.find(i).is_some() {
            println!("Error at {}", i);
        }
        i += 2;
    }

    println!("Min is {}, Max is {}", root.find_min().unwrap().retrieve(), root.find_max().unwrap().retrieve());
}