use std::ptr;
use std::mem;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;
type Position = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    if let mut t = t {
        make_empty(t.left.take());
        make_empty(t.right.take());
    }
    None
}

fn find(x: ElementType, t: &SearchTree) -> Position {
    if let Some(ref node) = t {
        if x < node.element {
            return find(x, &node.left);
        } else if x > node.element {
            return find(x, &node.right);
        } else {
            return Some(Box::new(TreeNode { element: node.element, left: node.left.clone(), right: node.right.clone() }));
        }
    }
    None
}

fn find_min(t: &SearchTree) -> Position {
    if let Some(ref node) = t {
        if let Some(ref mut left) = node.left {
            return find_min(&Some(left.clone()));
        }
        return Some(Box::new(TreeNode { element: node.element, left: node.left.clone(), right: node.right.clone() }));
    }
    None
}

fn find_max(t: &SearchTree) -> Position {
    if let Some(ref node) = t {
        let mut current = node;
        while let Some(ref right) = current.right {
            current = right;
        }
        return Some(Box::new(TreeNode { element: current.element, left: current.left.clone(), right: current.right.clone() }));
    }
    None
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => Some(Box::new(TreeNode { element: x, left: None, right: None })),
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
    if let Some(mut t) = t {
        if x < t.element {
            t.left = delete(x, t.left.take());
        } else if x > t.element {
            t.right = delete(x, t.right.take());
        } else {
            let tmp_cell = find_min(&t.right);
            if let Some(tmp_cell) = tmp_cell {
                t.element = tmp_cell.element;
                t.right = delete(t.element, t.right.take());
            } else {
                let tmp_cell = t;
                t = if t.left.is_none() {
                    t.right.take()
                } else if t.right.is_none() {
                    t.left.take()
                } else {
                    unreachable!();
                };
                mem::drop(tmp_cell);
            }
        }
        Some(t)
    } else {
        None
    }
}

fn retrieve(p: &Position) -> ElementType {
    if let Some(ref node) = p {
        node.element
    } else {
        0 // Default value or error handling
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
        if let Some(p) = find(i, &t) {
            if retrieve(&Some(p)) != i {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        if let Some(p) = find(i, &t) {
            if retrieve(&Some(p)) != i {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        if let Some(p) = find(i, &t) {
            println!("Error at {}", i);
        }
    }

    if let Some(min) = find_min(&t) {
        if let Some(max) = find_max(&t) {
            println!("Min is {}, Max is {}", retrieve(&Some(min)), retrieve(&Some(max)));
        }
    }
}