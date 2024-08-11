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
    match t {
        Some(mut node) => {
            make_empty(node.left.take());
            make_empty(node.right.take());
        }
        None => {}
    }
    None
}

fn find(x: ElementType, t: SearchTree) -> Position {
    match t {
        Some(ref node) => {
            if x < node.element {
                find(x, node.left.as_ref().map(|n| n.as_ref()))
            } else if x > node.element {
                find(x, node.right.as_ref().map(|n| n.as_ref()))
            } else {
                Some(Box::new(TreeNode {
                    element: node.element,
                    left: node.left.clone(),
                    right: node.right.clone(),
                }))
            }
        }
        None => None
    }
}

fn find_min(t: SearchTree) -> Position {
    match t {
        Some(ref node) => {
            if let Some(ref mut left_node) = node.left {
                find_min(Some(left_node.as_ref()))
            } else {
                Some(Box::new(TreeNode {
                    element: node.element,
                    left: node.left.clone(),
                    right: node.right.clone(),
                }))
            }
        }
        None => None
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
        None => None
    }
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
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
    match t {
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left.take());
            } else if x > node.element {
                node.right = delete(x, node.right.take());
            } else {
                let new_node = if node.left.is_none() {
                    node.right.take()
                } else if node.right.is_none() {
                    node.left.take()
                } else {
                    let mut right_min = find_min(node.right.take());
                    let right_min_val = mem::replace(&mut right_min.as_mut().unwrap().element, x);
                    let right_updated = delete(right_min_val, node.right.take());
                    *node.right.as_mut().unwrap() = right_updated.unwrap();
                    right_min
                };
                return new_node;
            }
            Some(node)
        }
        None => None
    }
}

fn retrieve(p: Position) -> ElementType {
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
        println!("Min is {}, Max is {}", retrieve(Some(min)), retrieve(find_max(t)));
    }
}