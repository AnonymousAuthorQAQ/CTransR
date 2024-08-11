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
        Some(mut node) => {
            if let Some(mut left_node) = node.left {
                find_min(Some(left_node))
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
            if let Some(mut right_node) = node.right {
                find_max(Some(right_node))
            } else {
                Some(node)
            }
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
            let new_node = TreeNode {
                element: x,
                left: None,
                right: None,
            };
            Some(Box::new(new_node))
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
                if node.left.is_some() && node.right.is_some() {
                    let mut tmp_cell = find_min(node.right.clone()).unwrap();
                    node.element = tmp_cell.element;
                    node.right = delete(tmp_cell.element, node.right);
                } else {
                    let tmp_cell = node;
                    if node.left.is_none() {
                        node = node.right.unwrap();
                    } else if node.right.is_none() {
                        node = node.left.unwrap();
                    }
                    drop(tmp_cell);
                }
            }
            Some(node)
        },
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
        if let Some(p) = find(i, t.clone()) {
            if retrieve(p) != i {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        if let Some(p) = find(i, t.clone()) {
            if retrieve(p) != i {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        if let Some(p) = find(i, t.clone()) {
            println!("Error at {}", i);
        }
    }

    if let Some(min_node) = find_min(t.clone()) {
        if let Some(max_node) = find_max(t.clone()) {
            println!("Min is {}, Max is {}", retrieve(min_node), retrieve(max_node));
        }
    }
}