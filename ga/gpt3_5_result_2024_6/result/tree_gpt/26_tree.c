use std::ptr;

struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;
type Position = *mut TreeNode;

impl TreeNode {
    fn new(element: i32) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
}

fn make_empty(t: SearchTree) -> SearchTree {
    unsafe {
        if let mut t = t {
            make_empty(t.left.take());
            make_empty(t.right.take());
        }
    }
    None
}

fn find(x: i32, mut t: SearchTree) -> Option<Position> {
    while let Some(ptr) = t.as_deref_mut() {
        if x < ptr.element {
            t = ptr.left.take();
        } else if x > ptr.element {
            t = ptr.right.take();
        } else {
            return Some(ptr);
        }
    }
    None
}

fn find_min(mut t: SearchTree) -> Position {
    let mut current = t.as_mut().unwrap();
    while let Some(left) = current.left.as_deref_mut() {
        current = left;
    }
    let ptr: *mut TreeNode = current;
    ptr
}

fn find_max(mut t: SearchTree) -> Position {
    let mut current = t.as_mut().unwrap();
    while let Some(right) = current.right.as_deref_mut() {
        current = right;
    }
    let ptr: *mut TreeNode = current;
    ptr
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => {
            Some(Box::new(TreeNode::new(x)))
        }
        Some(mut ptr) => {
            if x < ptr.element {
                ptr.left = insert(x, ptr.left.take());
            } else if x > ptr.element {
                ptr.right = insert(x, ptr.right.take());
            }
            Some(ptr)
        }
    }
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => t,
        Some(mut node) if x < node.element => {
            node.left = delete(x, node.left.take());
            Some(node)
        }
        Some(mut node) if x > node.element => {
            node.right = delete(x, node.right.take());
            Some(node)
        }
        Some(node) if node.left.is_some() && node.right.is_some() => {
            let to_delete = find_min(node.right.take());
            node.element = unsafe { (*to_delete).element };
            node.right = delete(node.element, node.right.take());
            Some(node)
        }
        Some(node) => {
            let new_node = if let Some(ptr) = node.left.take() {
                ptr
            } else {
                node.right.take().unwrap_or_default()
            };
            Some(new_node)
        }
    }
}

fn retrieve(p: Position) -> i32 {
    let node = unsafe { &*p };
    node.element
}

fn main() {
    let mut t = make_empty(None);
    for i in 0..50 {
        let j = (7 * i) % 50;
        t = insert(j, t);
    }
    
    for i in 0..50 {
        if let Some(p) = find(i, t) {
            if retrieve(p) != i {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        if let Some(p) = find(i, t) {
            if retrieve(p) != i {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        if let Some(_) = find(i, t) {
            println!("Error at {}", i);
        }
    }

    unsafe {
        let min = retrieve(find_min(t));
        let max = retrieve(find_max(t));
        println!("Min is {}, Max is {}", min, max);
    }
}