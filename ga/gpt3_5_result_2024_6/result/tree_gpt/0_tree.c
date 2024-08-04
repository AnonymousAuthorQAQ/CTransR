use std::ptr;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;
type Position = Option<*mut TreeNode>;

fn make_empty(t: SearchTree) -> SearchTree {
    if let mut t = t {
        t.left = make_empty(t.left.take());
        t.right = make_empty(t.right.take());
    }
    t
}

fn find(x: ElementType, t: SearchTree) -> Position {
    match t {
        Some(node) => {
            if x < node.element {
                find(x, node.left.as_deref())
            } else if x > node.element {
                find(x, node.right.as_deref())
            } else {
                Some(node.as_mut())
            }
        }
        None => None,
    }
}

fn find_min(t: SearchTree) -> Position {
    match t {
        Some(node) => {
            if let Some(mut left) = node.left.as_deref() {
                find_min(Some(left))
            } else {
                Some(node.as_mut())
            }
        }
        None => None,
    }
}

fn find_max(t: SearchTree) -> Position {
    match t {
        Some(mut node) => {
            while let Some(mut right) = node.right.as_deref() {
                node = right;
            }
            Some(node.as_mut())
        }
        None => None,
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
                if node.left.is_some() && node.right.is_some() {
                    let mut tmp_cell = find_min(node.right.take()).unwrap();
                    node.element = tmp_cell.element;
                    node.right = delete(node.element, node.right.take());
                } else {
                    let tmp_cell = node;
                    if node.left.is_none() {
                        node = node.right.take().unwrap();
                    } else if node.right.is_none() {
                        node = node.left.take().unwrap();
                    }
                    drop(tmp_cell);
                }
            }
            Some(node)
        }
        None => None,
    }
}

fn retrieve(p: Position) -> ElementType {
    unsafe {
        if let Some(p) = p {
            (*p).element
        } else {
            panic!("Position is None");
        }
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
        let p = find(i, t);
        if p.is_none() || retrieve(p) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        let p = find(i, t);
        if p.is_none() || retrieve(p) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if find(i, t).is_some() {
            println!("Error at {}", i);
        }
    }

    if let Some(p) = find_min(t) {
        let min = retrieve(Some(&mut *p));
        if let Some(p) = find_max(t) {
            let max = retrieve(Some(&mut *p));
            println!("Min is {}, Max is {}", min, max);
        }
    }
}