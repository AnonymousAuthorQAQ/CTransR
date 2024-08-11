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
    if let Some(mut tree) = t {
        tree.left = make_empty(tree.left);
        tree.right = make_empty(tree.right);
    }
    None
}

fn find(x: ElementType, t: SearchTree) -> Position {
    match t {
        Some(ref node) => {
            if x < node.element {
                find(x, node.left.clone())
            } else if x > node.element {
                find(x, node.right.clone())
            } else {
                t
            }
        }
        None => None,
    }
}

fn find_min(t: SearchTree) -> Position {
    match t {
        Some(ref node) => {
            if let Some(ref mut left) = node.left {
                find_min(Some(left.clone()))
            } else {
                t
            }
        }
        None => None,
    }
}

fn find_max(t: SearchTree) -> Position {
    let mut tree = t;
    while let Some(ref node) = tree {
        if let Some(ref mut right) = node.right {
            tree = Some(right.clone());
        } else {
            break;
        }
    }
    tree
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
    let mut tree = t;
    if let Some(mut node) = tree.take() {
        if x < node.element {
            node.left = delete(x, node.left);
            tree = Some(node);
        } else if x > node.element {
            node.right = delete(x, node.right);
            tree = Some(node);
        } else if let (Some(left), Some(right)) = (node.left.take(), node.right.take()) {
            let min = find_min(Some(right.clone()).clone());
            if let Some(min_node) = min {
                node.element = min_node.element;
                node.right = delete(node.element, Some(right));
            }
            tree = Some(node);
        } else if let Some(left) = node.left.take() {
            tree = Some(left);
        } else if let Some(right) = node.right.take() {
            tree = Some(right);
        }
    }
    tree
}

fn retrieve(p: Position) -> ElementType {
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
        let p = find(i, t.clone());
        if p.is_none() || retrieve(p) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        let p = find(i, t.clone());
        if p.is_none() || retrieve(p) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        let p = find(i, t.clone());
        if p.is_some() {
            println!("Error at {}", i);
        }
    }

    if let Some(min_node) = find_min(t.clone()) {
        if let Some(max_node) = find_max(t.clone()) {
            println!("Min is {}, Max is {}", retrieve(Some(min_node)), retrieve(Some(max_node)));
        }
    }
}