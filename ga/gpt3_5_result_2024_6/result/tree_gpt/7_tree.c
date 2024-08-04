use std::ptr;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        node.left = make_empty(node.left);
        node.right = make_empty(node.right);
    }
    None
}

fn find(x: ElementType, t: &SearchTree) -> Option<&Node> {
    match t {
        Some(node) => {
            if x < node.element {
                find(x, &node.left)
            } else if x > node.element {
                find(x, &node.right)
            } else {
                Some(node)
            }
        }
        None => None
    }
}

fn find_min(mut t: &SearchTree) -> Option<&Node> {
    while let Some(node) = t {
        if let Some(left) = &node.left {
            t = Some(left);
        } else {
            return Some(node);
        }
    }
    None
}

fn find_max(mut t: &SearchTree) -> Option<&Node> {
    while let Some(node) = t {
        if let Some(right) = &node.right {
            t = Some(right);
        } else {
            return Some(node);
        }
    }
    None
}

fn insert(x: ElementType, mut t: SearchTree) -> SearchTree {
    match t {
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
        Some(mut node) => {
            if x < node.element {
                node.left = insert(x, node.left);
            } else if x > node.element {
                node.right = insert(x, node.right);
            }
            Some(node)
        }
    }
}

fn delete(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        Some(node) if x < node.element => Some(Box::new(TreeNode {
            element: node.element,
            left: delete(x, node.left),
            right: node.right,
        })),
        Some(node) if x > node.element => Some(Box::new(TreeNode {
            element: node.element,
            left: node.left,
            right: delete(x, node.right),
        })),
        Some(node) if node.left.is_some() && node.right.is_some() => {
            let mut tmp_cell = find_min(&node.right).unwrap().clone();
            node.element = tmp_cell.element;
            node.right = delete(node.element, node.right);
            Some(node)
        }
        Some(node) => {
            if node.left.is_none() {
                node.right
            } else {
                node.left
            }
        }
        None => None,
    }
}

fn retrieve(p: &Node) -> ElementType {
    p.element
}

fn main() {
    let mut t = None;
    let mut j = 0;

    t = make_empty(t);

    for i in 0..50 {
        j = (j + 7) % 50;
        t = insert(j, t);
    }

    for i in 0..50 {
        if let Some(p) = find(i, &t) {
            if retrieve(p) != i {
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
            if retrieve(p) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if find(i, &t).is_some() {
            println!("Error at {}", i);
        }
    }

    if let Some(min_node) = find_min(&t) {
        if let Some(max_node) = find_max(&t) {
            println!("Min is {}, Max is {}", retrieve(min_node), retrieve(max_node));
        }
    }
}