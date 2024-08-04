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
    None
}

fn find(x: ElementType, t: SearchTree) -> Position {
    match t {
        Some(ref node) => {
            if x < node.element {
                return find(x, node.left.clone());
            } else if x > node.element {
                return find(x, node.right.clone());
            } else {
                return Some(Box::new(TreeNode {
                    element: node.element,
                    left: node.left.clone(),
                    right: node.right.clone(),
                }));
            }
        }
        None => None,
    }
}

fn find_min(t: SearchTree) -> Position {
    match t {
        Some(ref node) => {
            if let Some(ref mut left) = node.left {
                return find_min(Some(left.as_ref().clone()));
            } else {
                return Some(Box::new(TreeNode {
                    element: node.element,
                    left: node.left.clone(),
                    right: node.right.clone(),
                }));
            }
        }
        None => None,
    }
}

fn find_max(t: SearchTree) -> Position {
    match t {
        Some(ref node) => {
            let mut current = node;
            while let Some(ref right) = current.right {
                current = right.as_ref();
            }
            return Some(Box::new(TreeNode {
                element: current.element,
                left: current.left.clone(),
                right: current.right.clone(),
            }));
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
                let tmp_cell = match find_min(node.right.clone()) {
                    Some(min) => min,
                    None => return node.left,
                };
                node.element = tmp_cell.element;
                node.right = delete(tmp_cell.element, node.right);
            }
            Some(node)
        }
        None => None,
    }
}

fn retrieve(p: Position) -> ElementType {
    match p {
        Some(node) => node.element,
        None => panic!("Position is None"),
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
        if let Some(p) = find(i, t.clone()) {
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
        if let Some(p) = find(i, t.clone()) {
            if retrieve(Some(p)) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if find(i, t.clone()).is_some() {
            println!("Error at {}", i);
        }
    }

    if let Some(min) = find_min(t.clone()) {
        if let Some(max) = find_max(t.clone()) {
            println!("Min is {}, Max is {}", retrieve(Some(min)), retrieve(Some(max)));
        }
    }
}