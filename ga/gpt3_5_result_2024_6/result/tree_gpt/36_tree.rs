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
        Some(node) => {
            if x < node.element {
                find(x, node.left.as_ref().map(|n| &**n))
            } else if x > node.element {
                find(x, node.right.as_ref().map(|n| &**n))
            } else {
                Some(node.as_ptr())
            }
        }
        None => None,
    }
}

fn find_min(t: SearchTree) -> Position {
    t.as_ref().and_then(|node| {
        if let Some(mut left_node) = &node.left {
            find_min(Some(left_node.as_mut()))
        } else {
            Some(node.as_ptr())
        }
    })
}

fn find_max(t: SearchTree) -> Position {
    t.as_ref().map(|mut node| {
        while let Some(mut right_node) = &node.right {
            node = right_node.as_mut();
        }
        node.as_ptr()
    })
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
        None => {
            Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            }))
        }
    }
}

fn delete(x: ElementType, mut t: SearchTree) -> SearchTree {
    if let Some(node) = t.as_mut() {
        if x < node.element {
            node.left = delete(x, node.left.take());
        } else if x > node.element {
            node.right = delete(x, node.right.take());
        } else {
            let tmp_cell = find_min(node.right.take());
            if let Some(tmp_node) = tmp_cell {
                node.element = unsafe { (*tmp_node).element };
                node.right = delete(node.element, node.right.take());
            } else {
                let tmp_cell = t.take();
                if let Some(tmp_node) = tmp_cell {
                    if tmp_node.left.is_none() {
                        t = tmp_node.right.take();
                    } else if tmp_node.right.is_none() {
                        t = tmp_node.left.take();
                    }
                    drop(tmp_node);
                }
            }
        }
    }
    t
}

fn retrieve(p: Position) -> ElementType {
    unsafe { (*p.unwrap()).element }
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
        let p = find(i, t);
        if p.is_some() {
            println!("Error at {}", i);
        }
    }

    if let Some(min_ptr) = find_min(t) {
        let min_value = retrieve(Some(min_ptr));
        if let Some(max_ptr) = find_max(t) {
            let max_value = retrieve(Some(max_ptr));
            println!("Min is {}, Max is {}", min_value, max_value);
        }
    }
}