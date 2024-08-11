use std::ptr;
use std::mem;

type ElementType = i32;

#[derive(Debug)]
struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;
type Position = Option<&mut Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    t
}

fn find(x: ElementType, t: &SearchTree) -> Position {
    match t {
        None => None,
        Some(node) => {
            if x < node.element {
                find(x, &node.left)
            } else if x > node.element {
                find(x, &node.right)
            } else {
                Some(node)
            }
        }
    }
}

fn find_min(t: &SearchTree) -> Position {
    match t {
        None => None,
        Some(node) => {
            if node.left.is_none() {
                Some(node)
            } else {
                find_min(&node.left)
            }
        }
    }
}

fn find_max(t: &SearchTree) -> Position {
    let mut current = t;
    while let Some(node) = current {
        if node.right.is_none() {
            return Some(node)
        }
        current = &node.right;
    }
    None
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => {
            Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            }))
        }
        Some(mut node) => {
            if x < node.element {
                node.left = insert(x, mem::replace(&mut node.left, None));
            } else if x > node.element {
                node.right = insert(x, mem::replace(&mut node.right, None));
            }
            Some(node)
        }
    }
}

fn delete(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => t,
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, mem::replace(&mut node.left, None));
            } else if x > node.element {
                node.right = delete(x, mem::replace(&mut node.right, None));
            } else {
                if node.left.is_some() && node.right.is_some() {
                    let tmp = find_min(&node.right)
                        .map(|node| mem::replace(&mut node.element, 0))
                        .unwrap_or_default();
                    node.element = tmp;
                    node.right = delete(tmp, mem::replace(&mut node.right, None));
                } else {
                    let tmp = node;
                    if node.left.is_none() {
                        return node.right;
                    } else if node.right.is_none() {
                        return node.left;
                    }
                    drop(tmp); // Ensure tmp is dropped before returning
                }
            }
            Some(node)
        }
    }
}

fn retrieve(p: &Position) -> ElementType {
    p.as_ref().unwrap().element
}

fn main() {
    let mut t: SearchTree = None;
    let mut j = 0;

    for i in 0..50 {
        j = (j + 7) % 50;
        t = insert(j, t);
    }

    for i in 0..50 {
        if let Some(p) = find(i, &t) {
            if retrieve(&Some(&mut p)) != i {
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
            if retrieve(&Some(&mut p)) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if let Some(_) = find(i, &t) {
            println!("Error at {}", i);
        }
    }

    if let Some(min) = find_min(&t) {
        if let Some(max) = find_max(&t) {
            println!("Min is {}, Max is {}", retrieve(&Some(&mut min)), retrieve(&Some(&mut max)));
        }
    }
}