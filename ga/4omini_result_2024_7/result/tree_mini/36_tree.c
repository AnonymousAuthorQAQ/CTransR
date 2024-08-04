use std::ptr;
use std::mem;
use std::boxed::Box;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty() -> SearchTree {
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
        Some(ref node) => {
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
    let mut current = t.as_deref();
    while let Some(node) = current {
        if node.right.is_none() {
            return Some(node);
        }
        current = node.right.as_deref();
    }
    None
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
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

fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => {
            eprintln!("Error: Element not found");
            None
        }
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left);
                Some(node)
            } else if x > node.element {
                node.right = delete(x, node.right);
                Some(node)
            } else {
                // Node with two children case
                if node.left.is_some() && node.right.is_some() {
                    let min_node = find_min(&node.right).unwrap();
                    node.element = min_node.element;
                    node.right = delete(node.element, node.right);
                } else {
                    let tmp_cell = node;
                    // Node with one or zero children
                    if tmp_cell.left.is_none() {
                        return tmp_cell.right;
                    } else {
                        return tmp_cell.left;
                    }
                }
                Some(node)
            }
        }
    }
}

fn retrieve(p: &Option<&TreeNode>) -> Option<i32> {
    p.map(|node| node.element)
}

fn main() {
    let mut t = make_empty();
    let mut j = 0;

    for i in 0..50 {
        j = (j + 7) % 50;
        t = insert(j, t);
    }

    for i in 0..50 {
        if let None = find(i, &t) {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        if let None = find(i, &t) {
            println!("Error at {}", i);
        }
    }
    
    for i in (0..50).step_by(2) {
        if let Some(_) = find(i, &t) {
            println!("Error at {}", i);
        }
    }

    if let (Some(min), Some(max)) = (find_min(&t), find_max(&t)) {
        println!("Min is {}, Max is {}", min.element, max.element);
    }
}