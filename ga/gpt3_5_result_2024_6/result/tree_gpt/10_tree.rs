use std::ptr;
use std::mem;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type Position = Option<Box<TreeNode>>;
type SearchTree = Option<Box<TreeNode>>;

impl TreeNode {
    pub fn new(element: ElementType) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
}

fn make_empty(t: SearchTree) -> SearchTree {
    match t {
        None => None,
        Some(mut node) => {
            node.left = make_empty(node.left);
            node.right = make_empty(node.right);
            None
        }
    }
}

fn find(x: ElementType, t: &SearchTree) -> Position {
    match t {
        None => None,
        Some(node) => {
            if x < node.element {
                find(x, &node.left)
            } else if x > node.element {
                find(x, &node.right)
            } else {
                Some(node.clone())
            }
        }
    }
}

fn find_min(t: &SearchTree) -> Position {
    match t {
        None => None,
        Some(node) => {
            if node.left.is_none() {
                Some(node.clone())
            } else {
                find_min(&node.left)
            }
        }
    }
}

fn find_max(t: &SearchTree) -> Position {
    t.as_ref().and_then(|node| {
        node.right.as_ref().map_or(Some(node), |right| find_max(&Some(right.clone())))
    })
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => Some(Box::new(TreeNode::new(x))),
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
    if let Some(mut node) = t {
        if x < node.element {
            node.left = delete(x, node.left.take());
        } else if x > node.element {
            node.right = delete(x, node.right.take());
        } else {
            let temp_cell = match find_min(&node.right) {
                None => {
                    Some(node)
                }
                Some(min_node) => {
                    node.element = min_node.element;
                    node.right = delete(node.element, node.right.take());
                    None
                }
            };

            if let Some(cell) = temp_cell {
                if cell.left.is_none() {
                    return cell.right;
                } else if cell.right.is_none() {
                    return cell.left;
                }
            }
        }
        Some(node)
    } else {
        None
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
        if let Some(p) = find(i, &t) {
            if retrieve(&Some(p)) != i {
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
        if let Some(p) = find(i, &t) {
            if retrieve(&Some(p)) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if let Some(p) = find(i, &t) {
            println!("Error at {}", i);
        }
    }

    if let Some(min_node) = find_min(&t) {
        if let Some(max_node) = find_max(&t) {
            println!("Min is {}, Max is {}", retrieve(&Some(min_node)), retrieve(&Some(max_node)));
        }
    }
}