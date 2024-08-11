use std::ptr::null;
use std::ptr::NonNull;

type ElementType = i32;

#[derive(Debug)]
struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    if let Some(node) = t {
        make_empty(node.left);
        make_empty(node.right);
        std::mem::drop(node);
    }
    None
}

fn find(x: ElementType, t: SearchTree) -> Option<*const TreeNode> {
    let mut current = t;
    while let Some(node) = current {
        if x < node.element {
            current = node.left;
        } else if x > node.element {
            current = node.right;
        } else {
            return Some(node);
        }
    }
    None
}

fn find_min(t: SearchTree) -> Option<*const TreeNode> {
    let mut current = t;
    while let Some(node) = current {
        if node.left.is_none() {
            return Some(node);
        }
        current = node.left;
    }
    None
}

fn find_max(t: SearchTree) -> Option<*const TreeNode> {
    let mut current = t;
    while let Some(node) = current {
        if node.right.is_none() {
            return Some(node);
        }
        current = node.right;
    }
    None
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    if t.is_none() {
        Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        }))
    } else {
        let mut current = t;
        while let Some(node) = current {
            if x < node.element {
                current = node.left;
            } else if x > node.element {
                current = node.right;
            } else {
                return t;
            }
        }
        None
    }
}

fn delete(x: ElementType, t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        if x < node.element {
            node.left = delete(x, node.left);
        } else if x > node.element {
            node.right = delete(x, node.right);
        } else if node.left.is_some() && node.right.is_some() {
            let min_right = find_min(node.right).unwrap();
            node.element = min_right.element;
            node.right = delete(min_right.element, node.right);
        } else {
            if node.left.is_none() {
                return node.right;
            } else if node.right.is_none() {
                return node.left;
            }
        }
        Some(node)
    } else {
        None
    }
}

fn retrieve(p: *const TreeNode) -> ElementType {
    unsafe { (*p).element }
}

fn main() {
    let mut t: SearchTree = None;
    for i in 0..50 {
        t = insert(i, t);
    }
    for i in 0..50 {
        if find(i, t).is_none() || retrieve(find(i, t).unwrap()) != i {
            println!("Error at {}", i);
        }
    }
    for i in 0..50 {
        if i % 2 == 0 {
            t = delete(i, t);
        }
    }
    for i in 1..50 {
        if i % 2 == 1 {
            if find(i, t).is_none() || retrieve(find(i, t).unwrap()) != i {
                println!("Error at {}", i);
            }
        } else {
            if find(i, t).is_some() {
                println!("Error at {}", i);
            }
        }
    }
    println!("Min is {}, Max is {}", retrieve(find_min(t).unwrap()), retrieve(find_max(t).unwrap()));
}