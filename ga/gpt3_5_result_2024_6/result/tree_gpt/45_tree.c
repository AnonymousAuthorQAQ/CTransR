use std::ptr;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(element: ElementType) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
}

type SearchTree = Option<Box<TreeNode>>;
type Position = Option<Box<TreeNode>>;

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
                find(x, node.left.clone())
            } else if x > node.element {
                find(x, node.right.clone())
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
            if let Some(mut left_node) = node.left.clone() {
                find_min(Some(left_node))
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
            while let Some(right_node) = node.right.clone() {
                node = right_node;
            }
            Some(node)
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
        None => Some(Box::new(TreeNode::new(x))),
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
                if let Some(right_node) = node.right.clone() {
                    let mut tmp_cell = find_min(Some(right_node)).unwrap();
                    node.element = tmp_cell.element;
                    node.right = delete(tmp_cell.element, node.right);
                } else {
                    let tmp_cell = node;
                    if node.left.is_none() {
                        return node.right;
                    } else if node.right.is_none() {
                        return node.left;
                    }
                    return None;
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
        if let Some(p) = find(i, t.clone()) {
            if retrieve(Some(p)) != i {
                println!("Error at {}", i);
            }
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
        }
    }
    for i in (0..50).step_by(2) {
        if find(i, t.clone()).is_some() {
            println!("Error at {}", i);
        }
    }

    if let (Some(min), Some(max)) = (find_min(t.clone()), find_max(t.clone())) {
        println!("Min is {}, Max is {}", retrieve(min), retrieve(max));
    }
}