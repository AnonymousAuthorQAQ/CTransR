use std::mem;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            node.left = make_empty(node.left);
            node.right = make_empty(node.right);
            Some(node)
        }
        None => None,
    }
}

fn find(x: i32, t: SearchTree) -> Option<&TreeNode> {
    match t {
        Some(ref node) => {
            if x == node.element {
                Some(node)
            } else if x < node.element {
                find(x, node.left.as_ref())
            } else {
                find(x, node.right.as_ref())
            }
        }
        None => None,
    }
}

fn find_min(t: SearchTree) -> Option<&TreeNode> {
    match t {
        Some(ref node) => {
            if let Some(ref mut left) = node.left {
                find_min(Some(left))
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_max(t: SearchTree) -> Option<&TreeNode> {
    match t {
        Some(ref mut node) => {
            if let Some(ref mut right) = node.right {
                find_max(Some(right))
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            if x < node.element {
                node.left = insert(x, mem::replace(&mut node.left, None));
            } else if x > node.element {
                node.right = insert(x, mem::replace(&mut node.right, None));
            }
            Some(node)
        }
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
    }
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, mem::replace(&mut node.left, None));
            } else if x > node.element {
                node.right = delete(x, mem::replace(&mut node.right, None));
            } else if node.left.is_some() && node.right.is_some() {
                let right_min = find_min(node.right.clone()).unwrap().element;
                node.element = right_min;
                node.right = delete(right_min, mem::replace(&mut node.right, None));
            } else {
                let left_child = mem::replace(&mut node.left, None);
                return left_child;
            }
            Some(node)
        }
        None => None,
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
        if let Some(found_node) = find(i, t) {
            if find(i, t).is_none() || found_node.element != i {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        if let Some(found_node) = find(i, t) {
            if find(i, t).is_none() || found_node.element != i {
                println!("Error at {}", i);
            }
        }
    }
    for i in (0..50).step_by(2) {
        if find(i, t).is_some() {
            println!("Error at {}", i);
        }
    }

    if let Some(min_node) = find_min(t) {
        if let Some(max_node) = find_max(t) {
            println!("Min is {}, Max is {}", min_node.element, max_node.element);
        }
    }
}