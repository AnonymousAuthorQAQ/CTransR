use std::cmp::Ord;
use std::fmt;
use std::io::{self, Write};

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    None
}

fn find(x: i32, t: &SearchTree) -> Option<&TreeNode> {
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
    let mut current = t;
    while let Some(node) = current {
        if node.right.is_none() {
            return Some(node);
        }
        current = &node.right;
    }
    None
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            if x < node.element {
                node.left = insert(x, node.left);
            } else if x > node.element {
                node.right = insert(x, node.right);
            }
            Some(node)
        }
        None => {
            Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            }))
        }
    }
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left);
                Some(node)
            } else if x > node.element {
                node.right = delete(x, node.right);
                Some(node)
            } else {
                if node.left.is_some() && node.right.is_some() {
                    let min_node = find_min(&node.right).unwrap();
                    node.element = min_node.element;
                    node.right = delete(node.element, node.right);
                    Some(node)
                } else {
                    if node.left.is_none() {
                        Some(node.right.take().unwrap())
                    } else {
                        Some(node.left.take().unwrap())
                    }
                }
            }
        }
        None => {
            eprintln!("Error: Element not found");
            None
        }
    }
}

fn retrieve(node: &Option<Box<TreeNode>>) -> i32 {
    node.as_ref().map_or(0, |n| n.element)
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
            if retrieve(&Some(p)) != i {
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
            if retrieve(&Some(p)) != i {
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
        if let Some(max) = find_max(&t) {
            println!("Min is {}, Max is {}", min.element, max.element);
        }
    }
}