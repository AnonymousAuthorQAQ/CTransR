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
        Some(tree) => {
            if x < tree.element {
                return find(x, tree.left);
            } else if x > tree.element {
                return find(x, tree.right);
            } else {
                return Some(tree);
            }
        }
        None => None,
    }
}

fn find_min(t: SearchTree) -> Position {
    let mut current = t;
    while let Some(node) = current {
        if let Some(ref left) = node.left {
            current = Some(left.clone());
        } else {
            return Some(node);
        }
    }
    None
}

fn find_max(t: SearchTree) -> Position {
    let mut current = t;
    while let Some(node) = current {
        if let Some(ref right) = node.right {
            current = Some(right.clone());
        } else {
            return Some(node);
        }
    }
    None
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        Some(mut tree) => {
            if x < tree.element {
                tree.left = insert(x, tree.left);
            } else if x > tree.element {
                tree.right = insert(x, tree.right);
            }
            Some(tree)
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
        Some(mut tree) => {
            if x < tree.element {
                tree.left = delete(x, tree.left);
            } else if x > tree.element {
                tree.right = delete(x, tree.right);
            } else {
                if let (Some(left), Some(right)) = (tree.left.take(), tree.right.take()) {
                    let min_right = find_min(Some(right)).unwrap();
                    tree.element = min_right.element;
                    tree.right = delete(min_right.element, Some(right));
                } else {
                    return if tree.left.is_some() {
                        tree.left
                    } else {
                        tree.right
                    };
                }
            }
            Some(tree)
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
        let p = find(i, t);
        if p.is_none() || retrieve(p) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        let p = find(i, t);
        if p.is_none() || retrieve(p) != i {
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
            println!("Min is {}, Max is {}", retrieve(min), retrieve(max));
        }
    }
}