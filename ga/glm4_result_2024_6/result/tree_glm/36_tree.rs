use std::cmp::Ordering;

type SearchTree = Option<Box<TreeNode>>;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: SearchTree,
    right: SearchTree,
}

impl TreeNode {
    fn new(x: i32) -> Self {
        TreeNode {
            element: x,
            left: None,
            right: None,
        }
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
    if let Some(node) = t {
        match x.cmp(&node.element) {
            Ordering::Less => find(x, &node.left),
            Ordering::Greater => find(x, &node.right),
            Ordering::Equal => Some(node),
        }
    } else {
        None
    }
}

fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    if let Some(node) = t {
        if let None = node.left {
            return Some(node);
        }
        find_min(&node.left)
    } else {
        None
    }
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    if let Some(node) = t {
        if let None = node.right {
            return Some(node);
        }
        find_max(&node.right)
    } else {
        None
    }
}

fn insert(x: i32, t: &mut SearchTree) {
    if let None = t {
        *t = Some(Box::new(TreeNode::new(x)));
    } else {
        match x.cmp(&t.as_ref().unwrap().element) {
            Ordering::Less => insert(x, &mut t.as_mut().unwrap().left),
            Ordering::Greater => insert(x, &mut t.as_mut().unwrap().right),
            _ => (),
        }
    }
}

fn delete(x: i32, t: &mut SearchTree) {
    if let Some(mut node) = t.take() {
        match x.cmp(&node.element) {
            Ordering::Less => {
                let left = node.left.take();
                *t = Some(node);
                insert(x, &mut left);
            }
            Ordering::Greater => {
                let right = node.right.take();
                *t = Some(node);
                insert(x, &mut right);
            }
            Ordering::Equal => {
                if let Some(left) = node.left.take() {
                    *t = Some(left);
                } else if let Some(right) = node.right.take() {
                    *t = Some(right);
                } else {
                    *t = None;
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
        if let None = find(i, &t) {
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
            if let None = find(i, &t) {
                println!("Error at {}", i);
            }
        } else {
            if let Some(_) = find(i, &t) {
                println!("Error at {}", i);
            }
        }
    }
    if let Some(min) = find_min(&t) {
        println!("Min is {}, Max is {}", retrieve(min), retrieve(find_max(&t).unwrap()));
    }
}