use std::cmp::Ord;
use std::ptr::null_mut;

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
        current = &node.right;
    }
    current
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
            println!("Element not found");
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
                // Node to be deleted found
                if node.left.is_some() && node.right.is_some() {
                    // Node has two children
                    if let Some(min_node) = find_min(&node.right) {
                        node.element = min_node.element;
                        node.right = delete(node.element, node.right);
                    }
                    Some(node)
                } else {
                    // Node has one or zero children
                    if node.left.is_none() {
                        return node.right; // Return right child
                    } else {
                        return node.left; // Return left child
                    }
                }
            }
        }
    }
}

fn retrieve(p: &Option<&TreeNode>) -> Option<i32> {
    p.map(|node| node.element)
}

fn main() {
    let mut t: SearchTree = make_empty();
    let mut j = 0;

    for i in 0..50 {
        j = (j + 7) % 50;
        t = insert(j, t);
    }

    for i in 0..50 {
        if let Some(p) = find(i, &t) {
            if retrieve(&Some(p)) != Some(i) {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in 1..50 {
        if i % 2 == 1 {
            if let Some(p) = find(i, &t) {
                if retrieve(&Some(p)) != Some(i) {
                    println!("Error at {}", i);
                }
            } else {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        if find(i, &t).is_some() {
            println!("Error at {}", i);
        }
    }

    if let Some(min_node) = find_min(&t) {
        if let Some(max_node) = find_max(&t) {
            println!("Min is {}, Max is {}", min_node.element, max_node.element);
        }
    }
}