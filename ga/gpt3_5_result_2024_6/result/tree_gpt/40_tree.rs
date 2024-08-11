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
        None => None,
        Some(mut node) => {
            node.left = make_empty(node.left);
            node.right = make_empty(node.right);
            None
        }
    }
}

fn find(x: ElementType, t: SearchTree) -> Position {
    match t {
        None => None,
        Some(ref node) => {
            if x < node.element {
                find(x, node.left.as_ref().map(|n| n.as_ref())).flatten()
            } else if x > node.element {
                find(x, node.right.as_ref().map(|n| n.as_ref())).flatten()
            } else {
                Some(node.clone())
            }
        }
    }
}

fn find_min(t: SearchTree) -> Position {
    match t {
        None => None,
        Some(ref node) => {
            if let Some(ref left) = node.left {
                find_min(Some(left.as_ref().clone()))
            } else {
                Some(node.clone())
            }
        }
    }
}

fn find_max(t: SearchTree) -> Position {
    match t {
        None => None,
        Some(ref mut node) => {
            let mut current = node;

            while let Some(ref mut right) = current.right {
                current = right;
            }

            Some(current.clone())
        }
    }
}

fn insert(x: ElementType, mut t: SearchTree) -> SearchTree {
    if let None = t {
        return Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        }));
    }

    if let Some(ref mut node) = t {
        if x < node.element {
            node.left = insert(x, node.left.take()).into();
        } else if x > node.element {
            node.right = insert(x, node.right.take()).into();
        }
    }

    t
}

fn delete(x: ElementType, mut t: SearchTree) -> SearchTree {
    if let None = t {
        return None;
    }

    let mut temp_cell: Position;

    if let Some(ref mut node) = t {
        if x < node.element {
            node.left = delete(x, node.left.take()).into();
        } else if x > node.element {
            node.right = delete(x, node.right.take()).into();
        } else if let (Some(left), Some(right)) = (&node.left, &node.right) {
            temp_cell = find_min(Some(right.as_ref().clone()));
            if let Some(ref cell) = temp_cell {
                node.element = cell.element;
                node.right = delete(cell.element, node.right.take()).into();
            }
        } else {
            temp_cell = t;
            t = if node.left.is_none() { node.right.take() } else { node.left.take() };
        }

        if let Some(cell) = temp_cell {
            drop(cell);
        }
    }

    t
}

fn retrieve(p: &Position) -> ElementType {
    if let Some(ref node) = p {
        return node.element;
    }
    panic!("Cannot retrieve from None position");
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
            if retrieve(&p) != i {
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
            if retrieve(&p) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if let Some(p) = find(i, t.clone()) {
            println!("Error at {}", i);
        }
    }

    if let Some(min) = find_min(t.clone()) {
        if let Some(max) = find_max(t.clone()) {
            println!("Min is {}, Max is {}", retrieve(&min), retrieve(&max));
        }
    }
}