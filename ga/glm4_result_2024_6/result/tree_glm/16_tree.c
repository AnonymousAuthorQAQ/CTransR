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
        self.left = None;
        self.right = None;
    }

    fn find(&self, x: &T) -> Option<&Self> {
        match self {
            None => None,
            Some(node) => match x.cmp(&node.element) {
                Ordering::Less => node.left.find(x),
                Ordering::Greater => node.right.find(x),
                Ordering::Equal => Some(node),
            },
        }
    }

    fn find_min(&self) -> Option<&Self> {
        match self {
            None => None,
            Some(node) => node.left.find_min().or(Some(node)),
        }
    }

    fn find_max(&self) -> Option<&Self> {
        match self {
            None => None,
            Some(node) => node.right.find_max().or(Some(node)),
        }
    }

    fn insert(&mut self, x: T) {
        match self {
            None => {
                *self = Some(Box::new(TreeNode::new(x)));
            }
            Some(node) => match x.cmp(&node.element) {
                Ordering::Less => node.left.insert(x),
                Ordering::Greater => node.right.insert(x),
                Ordering::Equal => (),
            },
        }
    }

    fn delete(&mut self, x: &T) {
        match self {
            None => return,
            Some(node) => match x.cmp(&node.element) {
                Ordering::Less => node.left.delete(x),
                Ordering::Greater => node.right.delete(x),
                Ordering::Equal => {
                    if let (Some(l), Some(r)) = (&mut node.left, &mut node.right) {
                        if let Some(min) = r.find_min() {
                            node.element = min.element;
                            r.delete(&min.element);
                        } else {
                            *self = mem::replace(&mut node.left, None);
                        }
                    } else {
                        *self = mem::replace(&mut node.left, None).or(node.right);
                    }
                }
            },
        }
    }
}

impl<T: Debug> TreeNode<T> {
    fn retrieve(&self) -> Option<&T> {
        self.as_ref().map(|node| &node.element)
    }
}

fn main() {
    let mut tree: Option<Box<TreeNode<i32>>> = None;
    let mut i = 0;
    let mut j = 0;

    tree = Some(Box::new(TreeNode::new(0)));

    for _ in 0..50 {
        tree.as_mut().unwrap().insert(j);
        i += 1;
        j = (j + 7) % 50;
    }

    for i in 0..50 {
        if let Some(node) = tree.as_ref().unwrap().find(&i) {
            assert_eq!(node.element, i);
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        tree.as_mut().unwrap().delete(&i);
    }

    for i in (1..50).step_by(2) {
        if let Some(node) = tree.as_ref().unwrap().find(&i) {
            assert_eq!(node.element, i);
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if tree.as_ref().unwrap().find(&i).is_some() {
            println!("Error at {}", i);
        }
    }

    println!("Min is {}, Max is {}", tree.as_ref().unwrap().find_min().unwrap().element, tree.as_ref().unwrap().find_max().unwrap().element);
}