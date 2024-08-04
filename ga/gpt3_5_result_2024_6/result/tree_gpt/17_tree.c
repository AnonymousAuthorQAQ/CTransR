use std::ptr;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;
type Position = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        node.left = make_empty(node.left);
        node.right = make_empty(node.right);
    }
    t
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
        Some(node) => {
            if let Some(mut left_node) = &node.left {
                find_min(Some(left_node.clone()))
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_max(t: SearchTree) -> Position {
    let mut max = t;
    if let Some(mut node) = t {
        while let Some(right_node) = &node.right {
            max = Some(right_node.clone());
        }
    }
    max
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            if x < node.element {
                node.left = insert(x, node.left);
            } else if x > node.element {
                node.right = insert(x, node.right);
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

fn delete(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left);
            } else if x > node.element {
                node.right = delete(x, node.right);
            } else {
                if let Some(right) = node.right.take() {
                    let min = find_min(Some(right.clone()));
                    node.element = min.unwrap().element;
                    node.right = delete(node.element, node.right);
                } else {
                    return node.left;
                }
            }
            Some(node)
        }
        None => None,
    }
}

fn retrieve(p: Position) -> ElementType {
    p.unwrap().element
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
            if retrieve(Some(p)) != i {
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
            if retrieve(Some(p)) != i {
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

    if let Some(min) = find_min(t) {
        if let Some(max) = find_max(t) {
            println!("Min is {}, Max is {}", retrieve(Some(min)), retrieve(Some(max));
        }
    }
}