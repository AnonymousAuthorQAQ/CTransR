use std::ptr;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type Position = Option<Box<TreeNode>>;
type SearchTree = Position;

fn make_empty(t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        node.left = make_empty(node.left);
        node.right = make_empty(node.right);
    }
    None
}

fn find(x: ElementType, t: SearchTree) -> Position {
    match t {
        None => None,
        Some(node) => {
            if x < node.element {
                find(x, node.left.clone())
            } else if x > node.element {
                find(x, node.right.clone())
            } else {
                Some(node)
            }
        }
    }
}

fn find_min(t: SearchTree) -> Position {
    match t {
        None => None,
        Some(node) => {
            if node.left.is_none() {
                Some(node)
            } else {
                find_min(node.left.clone())
            }
        }
    }
}

fn find_max(t: SearchTree) -> Position {
    match t {
        None => None,
        Some(node) => {
            if node.right.is_none() {
                Some(node)
            } else {
                find_max(node.right.clone())
            }
        }
    }
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
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

fn delete(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => None,
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left.take());
            } else if x > node.element {
                node.right = delete(x, node.right.take());
            } else if let (left, right) = (node.left.take(), node.right.take()) {
                let min_right = find_min(right.clone());
                if let Some(min) = min_right {
                    node.element = min.element;
                    node.right = delete(node.element, Some(right));
                }
            } else {
                return if node.left.is_none() {
                    node.right
                } else {
                    node.left
                };
            }
            Some(node)
        }
    }
}

fn retrieve(p: &Position) -> ElementType {
    if let Some(node) = p {
        node.element
    } else {
        panic!("Position is None");
    }
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
        if let Some(p) = find(i, t.clone()) {
            if retrieve(&p) != i {
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
        if let Some(p) = find(i, t.clone()) {
            if retrieve(&p) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if find(i, t.clone()).is_some() {
            println!("Error at {}", i);
        }
    }

    if let Some(min) = find_min(t.clone()) {
        if let Some(max) = find_max(t.clone()) {
            println!("Min is {}, Max is {}", retrieve(&min), retrieve(&max));
        }
    }
}