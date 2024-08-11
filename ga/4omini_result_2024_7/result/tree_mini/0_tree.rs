use std::cmp::Ord;
use std::io;
use std::ptr;

#[derive(Debug)]
pub struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

pub type SearchTree = Option<Box<TreeNode>>;

pub fn make_empty() -> SearchTree {
    None
}

pub fn find(x: i32, t: &SearchTree) -> Option<&TreeNode> {
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

pub fn find_min(t: &SearchTree) -> Option<&TreeNode> {
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

pub fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut current = t;
    while let Some(node) = current {
        current = &node.right;
    }
    current
}

pub fn insert(x: i32, t: SearchTree) -> SearchTree {
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
                node.left = insert(x, node.left);
                Some(node)
            } else if x > node.element {
                node.right = insert(x, node.right);
                Some(node)
            } else {
                Some(node) // don't insert duplicates
            }
        }
    }
}

pub fn delete(x: i32, t: SearchTree) -> SearchTree {
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
                // Node found
                if node.left.is_none() {
                    return node.right; // replace with right child
                } else if node.right.is_none() {
                    return node.left; // replace with left child
                } else {
                    // Node has two children
                    let min_node = find_min(&node.right).unwrap(); // Unwrap safe because we know it's there
                    node.element = min_node.element;
                    node.right = delete(min_node.element, node.right);
                    Some(node)
                }
            }
        }
    }
}

pub fn retrieve(node: &TreeNode) -> i32 {
    node.element
}

fn main() {
    let mut t = make_empty();

    // Insert elements
    let mut j = 0;
    for i in 0..50 {
        j = (j + 7) % 50;
        t = insert(j, t);
    }

    // Check all elements were inserted correctly
    for i in 0..50 {
        match find(i, &t) {
            Some(p) if retrieve(p) != i => {
                println!("Error at {}", i);
            }
            None => {
                println!("Error at {}", i);
            }
            _ => {}
        }
    }

    // Delete even elements
    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    // Verify odd elements remain
    for i in 1..50 {
        if i % 2 == 1 {
            match find(i, &t) {
                Some(p) if retrieve(p) != i => {
                    println!("Error at {}", i);
                }
                None => {
                    println!("Error at {}", i);
                }
                _ => {}
            }
        }
    }

    // Verify even elements are gone
    for i in (0..50).step_by(2) {
        match find(i, &t) {
            Some(_) => {
                println!("Error at {}", i);
            }
            None => {}
        }
    }

    // Print min and max
    match (find_min(&t), find_max(&t)) {
        (Some(min), Some(max)) => {
            println!("Min is {}, Max is {}", retrieve(min), retrieve(max));
        }
        _ => println!("Tree is empty."),
    }
}