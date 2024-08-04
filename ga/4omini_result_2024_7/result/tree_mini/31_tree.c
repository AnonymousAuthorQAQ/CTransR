use std::io::{self, Write};

type ElementType = i32;

#[derive(Debug)]
pub struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

pub type SearchTree = Option<Box<TreeNode>>;

pub fn make_empty(t: SearchTree) -> SearchTree {
    None
}

pub fn find(x: ElementType, t: &SearchTree) -> Option<&TreeNode> {
    match t {
        Some(node) => {
            if x < node.element {
                find(x, &node.left)
            } else if x > node.element {
                find(x, &node.right)
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

pub fn find_min(t: &SearchTree) -> Option<&TreeNode> {
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

pub fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut current = t;

    while let Some(ref node) = current {
        if node.right.is_none() {
            return Some(node);
        }
        current = &node.right;
    }

    None
}

pub fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
        Some(mut node) => {
            if x < node.element {
                node.left = insert(x, node.left);
            } else if x > node.element {
                node.right = insert(x, node.right);
            }
            Some(node)
        }
    }
}

pub fn delete(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => {
            eprintln!("Element not found");
            None
        }
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left);
            } else if x > node.element {
                node.right = delete(x, node.right);
            } else {
                match (node.left.take(), node.right.take()) {
                    (None, None) => return None,
                    (Some(left), None) => return Some(left),
                    (None, Some(right)) => return Some(right),
                    (Some(left), Some(right)) => {
                        let min_node = find_min(&Some(right.clone())).unwrap();
                        node.element = min_node.element;
                        node.right = delete(min_node.element, Some(right));
                    }
                }
            }
            Some(node)
        }
    }
}

pub fn retrieve(p: &TreeNode) -> ElementType {
    p.element
}

fn main() {
    let mut t: SearchTree = make_empty(None);
    let mut j = 0;

    for i in 0..50 {
        j = (j + 7) % 50;
        t = insert(j, t);
    }

    for i in 0..50 {
        if let Some(p) = find(i, &t) {
            if retrieve(p) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        if let Some(p) = find(i, &t) {
            if retrieve(p) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if find(i, &t).is_some() {
            println!("Error at {}", i);
        }
    }

    if let Some(min) = find_min(&t) {
        println!("Min is {}, Max is {}", retrieve(min), retrieve(find_max(&t).unwrap()));
    }
}