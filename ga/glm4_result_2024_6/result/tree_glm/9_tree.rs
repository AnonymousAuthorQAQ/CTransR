use std::cmp::Ordering;
use std::fmt::Debug;
use std::mem;

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Debug> TreeNode<T> {
    fn new(element: T) -> Self {
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

    fn find(&self, x: &T) -> Option<&Self> {
        match x.cmp(&self.element) {
            Ordering::Less => self.left.as_ref().and_then(|left| left.find(x)),
            Ordering::Greater => self.right.as_ref().and_then(|right| right.find(x)),
            Ordering::Equal => Some(self),
        }
    }

    fn find_min(&self) -> Option<&Self> {
        self.left.as_ref().and_then(|left| left.find_min()).or(Some(self))
    }

    fn find_max(&self) -> Option<&Self> {
        self.right.as_ref().and_then(|right| right.find_max()).or(Some(self))
    }

    fn insert(&mut self, x: T) {
        match x.cmp(&self.element) {
            Ordering::Less => {
                if self.left.is_none() {
                    self.left = Some(Box::new(TreeNode::new(x)));
                } else {
                    self.left.as_mut().unwrap().insert(x);
                }
            }
            Ordering::Greater => {
                if self.right.is_none() {
                    self.right = Some(Box::new(TreeNode::new(x)));
                } else {
                    self.right.as_mut().unwrap().insert(x);
                }
            }
            Ordering::Equal => {}
        }
    }

    fn delete(&mut self, x: &T) -> Option<T> {
        match x.cmp(&self.element) {
            Ordering::Less => {
                if let Some(left) = self.left.as_mut() {
                    left.delete(x)
                } else {
                    None
                }
            }
            Ordering::Greater => {
                if let Some(right) = self.right.as_mut() {
                    right.delete(x)
                } else {
                    None
                }
            }
            Ordering::Equal => {
                if self.left.is_none() {
                    if let Some(right) = self.right.take() {
                        mem::replace(self, *right);
                    } else {
                        return Some(mem::replace(&mut self.element, T::default()));
                    }
                } else if self.right.is_none() {
                    if let Some(left) = self.left.take() {
                        mem::replace(self, *left);
                    } else {
                        return Some(mem::replace(&mut self.element, T::default()));
                    }
                } else {
                    if let Some(min) = self.right.as_ref().and_then(|right| right.find_min()) {
                        self.element = min.element;
                        self.right.as_mut().unwrap().delete(&min.element);
                    }
                }
                Some(mem::replace(&mut self.element, T::default()))
            }
        }
    }
}

fn main() {
    let mut tree: Option<Box<TreeNode<i32>>> = None;
    let mut i = 0;
    let mut j = 0;

    while i < 50 {
        tree = Some(Box::new(TreeNode::new(j)));
        i += 1;
        j = (j + 7) % 50;
    }

    for i in 0..50 {
        if tree.as_ref().and_then(|tree| tree.find(&i)).is_none() {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        tree.as_mut().unwrap().delete(&i);
    }

    for i in (1..50).step_by(2) {
        if tree.as_ref().and_then(|tree| tree.find(&i)).is_none() {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if tree.as_ref().and_then(|tree| tree.find(&i)).is_some() {
            println!("Error at {}", i);
        }
    }

    println!("Min is {}, Max is {}", tree.as_ref().and_then(|tree| tree.find_min()).map(|node| node.element), tree.as_ref().and_then(|tree| tree.find_max()).map(|node| node.element));
}