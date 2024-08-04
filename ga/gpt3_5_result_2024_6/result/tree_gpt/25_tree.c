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
    match t {
        Some(mut node) => {
            node.left = make_empty(node.left);
            node.right = make_empty(node.right);
            None
        },
        None => None,
    }
}

fn find(x: ElementType, t: SearchTree) -> Position {
    match t {
        Some(node) => {
            if x < node.element {
                find(x, node.left.clone())
            } else if x > node.element {
                find(x, node.right.clone())
            } else {
                Some(node)
            }
        },
        None => None,
    }
}

fn find_min(t: SearchTree) -> Position {
    match t {
        Some(node) => {
            if let Some(ref left) = node.left {
                find_min(Some(left.clone()))
            } else {
                Some(node)
            }
        },
        None => None,
    }
}

fn find_max(t: SearchTree) -> Position {
    match t {
        Some(mut node) => {
            while let Some(right) = node.right {
                node = right;
            }
            Some(node)
        },
        None => None,
    }
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
        },
        None => {
            Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            }))
        }
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
                if let (Some(left), Some(right)) = (node.left.take(), node.right.take()) {
                    let mut tmp_cell = find_min(Some(right.clone())).unwrap();
                    node.element = tmp_cell.element;
                    node.right = delete(node.element, Some(right));
                } else {
                    let tmp_cell = node;
                    if node.left.is_none() {
                        return node.right;
                    } else if node.right.is_none() {
                        return node.left;
                    }
                    drop(tmp_cell);
                }
            }
            Some(node)
        },
        None => None,
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
        let p = find(i, t.clone());
        if p.is_none() || retrieve(&p) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        let p = find(i, t.clone());
        if p.is_none() || retrieve(&p) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        let p = find(i, t.clone());
        if p.is_some() {
            println!("Error at {}", i);
        }
    }

    match (find_min(t.clone()), find_max(t.clone())) {
        (Some(min), Some(max)) => {
            println!("Min is {}, Max is {}", retrieve(&min), retrieve(&max));
        },
        _ => (),
    }
}