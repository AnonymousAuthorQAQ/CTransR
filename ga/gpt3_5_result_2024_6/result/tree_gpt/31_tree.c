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
        }
        None => None,
    }
}

fn find(x: ElementType, t: SearchTree) -> Position {
    match t {
        Some(mut node) => {
            if x < node.element {
                find(x, node.left.take())
            } else if x > node.element {
                find(x, node.right.take())
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
            if let Some(mut left) = node.left.take() {
                find_min(Some(left))
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_max(t: SearchTree) -> Position {
    let mut current_node = t;
    while let Some(mut node) = current_node {
        if let Some(right) = node.right.take() {
            current_node = Some(right);
        } else {
            return Some(node);
        }
    }
    None
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            if x < node.element {
                node.left = insert(x, node.left.take());
            } else if x > node.element {
                node.right = insert(x, node.right.take());
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
                node.left = delete(x, node.left.take());
            } else if x > node.element {
                node.right = delete(x, node.right.take());
            } else {
                let tmp_cell = match find_min(node.right.take()) {
                    Some(mut tmp) => {
                        let tmp_element = tmp.element;
                        tmp.right = delete(tmp_element, tmp.right.take());
                        tmp
                    }
                    None => {
                        let tmp_cell = node;
                        if node.left.is_none() {
                            return node.right.take();
                        } else {
                            return node.left.take();
                        }
                    }
                };
                node.element = tmp_cell.element;
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
        if let Some(p) = find(i, t) {
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
        if let Some(p) = find(i, t) {
            if retrieve(Some(p)) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if find(i, t).is_some() {
            println!("Error at {}", i);
        }
    }

    if let Some(min_node) = find_min(t) {
        if let Some(max_node) = find_max(t) {
            println!("Min is {}, Max is {}", retrieve(Some(min_node)), retrieve(Some(max_node)));
        }
    }
}