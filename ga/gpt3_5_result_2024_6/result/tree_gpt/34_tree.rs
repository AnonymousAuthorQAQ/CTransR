use std::ptr;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type Position = Option<Box<TreeNode>>;
type SearchTree = Option<Box<TreeNode>>;

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
            if let Some(mut left) = node.left {
                find_min(Some(left))
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
            if let Some(mut right) = node.right {
                find_max(Some(right))
            } else {
                Some(node)
            }
        }
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
                if let Some(mut right) = node.right {
                    let mut tmp_cell = find_min(Some(right)).unwrap().clone();
                    node.element = tmp_cell.element;
                    node.right = delete(tmp_cell.element, node.right);
                } else {
                    let tmp_cell = node;
                    if tmp_cell.left.is_none() {
                        return tmp_cell.right;
                    } else if tmp_cell.right.is_none() {
                        return tmp_cell.left;
                    }
                    ptr::null::<TreeNode>() as SearchTree
                }
            }
            Some(node)
        }
        None => None,
    }
}

fn retrieve(p: Position) -> ElementType {
    p.as_ref().unwrap().element
}

fn main() {
    let mut t: SearchTree = None;
    
    for i in 0..50 {
        let j = (7 + i) % 50;
        t = insert(j, t);
    }

    for i in 0..50 {
        let p = find(i, t);
        match p {
            Some(node) => {
                if retrieve(p) != i {
                    println!("Error at {}", i);
                }
            },
            None => println!("Error at {}", i),
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        let p = find(i, t);
        match p {
            Some(node) => {
                if retrieve(p) != i {
                    println!("Error at {}", i);
                }
            },
            None => println!("Error at {}", i),
        }
    }

    for i in (0..50).step_by(2) {
        if let Some(p) = find(i, t) {
            println!("Error at {}", i);
        }
    }

    match (find_min(t), find_max(t)) {
        (Some(min), Some(max)) => {
            println!("Min is {}, Max is {}", retrieve(min), retrieve(max));
        }
        _ => println!("No elements found in the tree."),
    }
}