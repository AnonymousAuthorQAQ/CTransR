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
        Some(ref node) if x < node.element => find(x, node.left.as_ref().map(|n| n.as_ref())),
        Some(ref node) if x > node.element => find(x, node.right.as_ref().map(|n| n.as_ref())),
        _ => t,
    }
}

fn find_min(t: SearchTree) -> Position {
    match t {
        Some(ref node) if node.left.is_none() => t,
        Some(ref node) => find_min(node.left.as_ref().map(|n| n.as_ref())),
        _ => None,
    }
}

fn find_max(t: SearchTree) -> Position {
    let mut node = t;
    while let Some(ref n) = node.as_ref() {
        node = n.right.as_ref().map(|n| n.as_ref());
    }
    node
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
        Some(mut node) if x < node.element => {
            node.left = insert(x, node.left.take());
            Some(node)
        }
        Some(mut node) if x > node.element => {
            node.right = insert(x, node.right.take());
            Some(node)
        }
        _ => t,
    }
}

fn delete(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => None,
        Some(mut node) if x < node.element => {
            node.left = delete(x, node.left.take());
            Some(node)
        }
        Some(mut node) if x > node.element => {
            node.right = delete(x, node.right.take());
            Some(node)
        }
        Some(mut node) if node.left.is_some() && node.right.is_some() => {
            let mut tmp_cell = find_min(node.right.take().unwrap());
            node.element = tmp_cell.as_ref().unwrap().element;
            node.right = delete(node.element, node.right.take());
            Some(node)
        }
        Some(node) => {
            let tmp_cell = node;
            let next_node = if node.left.is_none() {
                node.right.take()
            } else {
                node.left.take()
            };
            next_node
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
    let mut T: SearchTree = None;
    let mut j = 0;
    
    T = make_empty(T);
    for i in 0..50 {
        j = (j + 7) % 50;
        T = insert(j, T);
    }
    
    for i in 0..50 {
        if let Some(p) = find(i, T) {
            if retrieve(&p) != i {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        T = delete(i, T);
    }

    for i in (1..50).step_by(2) {
        if let Some(p) = find(i, T) {
            if retrieve(&p) != i {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        if let Some(p) = find(i, T) {
            println!("Error at {}", i);
        }
    }

    if let Some(min) = find_min(T) {
        if let Some(max) = find_max(T) {
            println!("Min is {}, Max is {}", retrieve(&min), retrieve(&max));
        }
    }
}