use std::mem;

struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

impl TreeNode {
    fn new(element: i32) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
}

fn make_empty(t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        node.left = make_empty(node.left);
        node.right = make_empty(node.right);
        drop(node);
    }
    None
}

fn find(x: i32, t: &SearchTree) -> Option<&TreeNode> {
    match t {
        None => None,
        Some(node) => {
            if x < node.element {
                find(x, &node.left)
            } else if x > node.element {
                find(x, &node.right)
            } else {
                Some(&*node)
            }
        }
    }
}

fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    match t {
        None => None,
        Some(node) => {
            if node.left.is_none() {
                Some(&*node)
            } else {
                find_min(&node.left)
            }
        }
    }
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut current = t;
    while let Some(node) = current {
        if node.right.is_none() {
            return Some(&*node);
        }
        current = &node.right;
    }
    None
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => Some(Box::new(TreeNode::new(x))),
        Some(mut node) => {
            if x < node.element {
                node.left = insert(x, node.left.take());
            } else if x > node.element {
                node.right = insert(x, node.right.take());
            }
            Some(node)
        }
    }
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => panic!("Element not found"),
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left.take());
            } else if x > node.element {
                node.right = delete(x, node.right.take());
            } else if node.left.is_some() && node.right.is_some() {
                let min_right = find_min(&node.right).unwrap().element;
                node.element = min_right;
                node.right = delete(min_right, node.right.take());
            } else {
                let tmp_cell = node;
                if node.left.is_none() {
                    return node.right;
                } else {
                    return node.left;
                }
                drop(tmp_cell);
            }
            Some(node)
        }
    }
}

fn retrieve(p: &TreeNode) -> i32 {
    p.element
}

fn main() {
    let mut t: SearchTree = None;
    let mut j = 0;

    t = make_empty(t);
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
        if let Some(max) = find_max(&t) {
            println!("Min is {}, Max is {}", retrieve(min), retrieve(max));
        }
    }
}