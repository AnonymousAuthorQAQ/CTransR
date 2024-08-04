use std::ptr;
use std::mem;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(element: ElementType) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
}

type Position = Option<Box<TreeNode>>;
type SearchTree = Position;

fn make_empty(t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            node.left = make_empty(node.left);
            node.right = make_empty(node.right);
            None
        }
        None => None,
    }
}

fn find(x: ElementType, t: SearchTree) -> Position {
    match t {
        Some(node) => {
            if x < node.element {
                find(x, node.left)
            } else if x > node.element {
                find(x, node.right)
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_min(t: SearchTree) -> Position {
    match t {
        Some(mut node) => {
            if let Some(mut left_node) = node.left {
                find_min(Some(left_node))
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_max(t: SearchTree) -> Position {
    match t {
        Some(mut node) => {
            while let Some(mut right_node) = node.right {
                node = right_node;
            }
            Some(node)
        }
        None => None,
    }
}

fn insert(x: ElementType, mut t: SearchTree) -> SearchTree {
    if let Some(mut node) = t.take() {
        if x < node.element {
            node.left = insert(x, node.left);
        } else if x > node.element {
            node.right = insert(x, node.right);
        }
        Some(node)
    } else {
        Some(Box::new(TreeNode::new(x)))
    }
}

fn delete(x: ElementType, mut t: SearchTree) -> SearchTree {
    if let Some(mut node) = t.take() {
        if x < node.element {
            node.left = delete(x, node.left);
        } else if x > node.element {
            node.right = delete(x, node.right);
        } else if node.left.is_some() && node.right.is_some() {
            let mut tmp_cell = find_min(node.right.take()).unwrap();
            node.element = tmp_cell.element;
            node.right = delete(tmp_cell.element, node.right);
        } else {
            let tmp_cell = node;
            if node.left.is_none() {
                t = node.right.take();
            } else {
                t = node.left.take();
            }
            mem::drop(tmp_cell);
        }
        t
    } else {
        panic!("Element not found");
    }
}

fn retrieve(p: &Position) -> ElementType {
    p.as_ref().unwrap().element
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
        if let Some(p) = find(i, t) {
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
        if let Some(p) = find(i, t) {
            if retrieve(&p) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }
    for i in (0..50).step_by(2) {
        if let Some(p) = find(i, t) {
            println!("Error at {}", i);
        }
    }

    if let Some(min_node) = find_min(t) {
        if let Some(max_node) = find_max(t) {
            println!("Min is {}, Max is {}", retrieve(&min_node), retrieve(&max_node));
        }
    }
}