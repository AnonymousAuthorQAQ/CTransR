use std::cmp::Ordering;
use std::fmt;

type SearchTree = Option<Box<TreeNode>>;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: SearchTree,
    right: SearchTree,
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

fn make_empty(t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        node.left = make_empty(node.left);
        node.right = make_empty(node.right);
        let _ = Box::from(node);
    }
    None
}

fn find(x: i32, t: &SearchTree) -> Option<&TreeNode> {
    match t {
        Some(node) => match x.cmp(&node.element) {
            Ordering::Less => find(x, &node.left),
            Ordering::Greater => find(x, &node.right),
            Ordering::Equal => Some(node),
        },
        None => None,
    }
}

fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    match t {
        Some(node) => {
            if node.left.is_none() {
                Some(node)
            } else {
                find_min(&node.left)
            }
        }
        None => None,
    }
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    match t {
        Some(node) => {
            if node.right.is_none() {
                Some(node)
            } else {
                find_max(&node.right)
            }
        }
        None => None,
    }
}

fn insert(x: i32, t: &mut SearchTree) {
    match t {
        Some(node) => match x.cmp(&node.element) {
            Ordering::Less => insert(x, &mut node.left),
            Ordering::Greater => insert(x, &mut node.right),
            Ordering::Equal => return,
        },
        None => *t = Some(Box::new(TreeNode::new(x))),
    }
}

fn delete(x: i32, t: &mut SearchTree) {
    if let Some(mut node) = t.take() {
        match x.cmp(&node.element) {
            Ordering::Less => {
                let left = node.left;
                node.left = None;
                *t = Some(node);
                insert(x, &mut left);
            }
            Ordering::Greater => {
                let right = node.right;
                node.right = None;
                *t = Some(node);
                insert(x, &mut right);
            }
            Ordering::Equal => {
                *t = node.left.take().or(node.right.take());
                if let Some(right) = node.right {
                    insert(node.element, &mut right);
                }
            }
        }
    }
}

fn retrieve(p: &TreeNode) -> i32 {
    p.element
}

fn main() {
    let mut t: SearchTree = None;
    for i in 0..50 {
        insert(i, &mut t);
    }
    for i in 0..50 {
        if let Some(node) = find(i, &t) {
            assert_eq!(retrieve(node), i);
        } else {
            println!("Error at {}", i);
        }
    }
    for i in 0..50 {
        if i % 2 == 0 {
            delete(i, &mut t);
        }
    }
    for i in 1..50 {
        if i % 2 == 1 {
            if let Some(node) = find(i, &t) {
                assert_eq!(retrieve(node), i);
            } else {
                println!("Error at {}", i);
            }
        } else {
            if find(i, &t).is_some() {
                println!("Error at {}", i);
            }
        }
    }
    if let Some(min) = find_min(&t) {
        println!("Min is {}", min);
    }
    if let Some(max) = find_max(&t) {
        println!("Max is {}", max);
    }
}