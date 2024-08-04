use std::ptr::null_mut;

type SearchTree = Option<Box<TreeNode>>;
type Position = *mut TreeNode;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: SearchTree,
    right: SearchTree,
}

impl TreeNode {
    fn new(x: i32) -> Box<TreeNode> {
        Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })
    }
}

fn make_empty(t: &mut SearchTree) {
    if let Some(node) = t {
        make_empty(&mut node.left);
        make_empty(&mut node.right);
        *t = None;
    }
}

fn find(x: i32, t: &SearchTree) -> Position {
    let mut current = t;
    while let Some(node) = current {
        if x < node.element {
            current = &node.left;
        } else if x > node.element {
            current = &node.right;
        } else {
            return current.as_mut() as Position;
        }
    }
    null_mut()
}

fn find_min(t: &SearchTree) -> Position {
    let mut current = t;
    while let Some(node) = current {
        if node.left.is_none() {
            return current.as_mut() as Position;
        }
        current = &node.left;
    }
    null_mut()
}

fn find_max(t: &SearchTree) -> Position {
    let mut current = t;
    while let Some(node) = current {
        if node.right.is_none() {
            return current.as_mut() as Position;
        }
        current = &node.right;
    }
    null_mut()
}

fn insert(x: i32, t: &mut SearchTree) {
    if let Some(node) = t {
        if x < node.element {
            insert(x, &mut node.left);
        } else if x > node.element {
            insert(x, &mut node.right);
        }
    } else {
        *t = Some(TreeNode::new(x));
    }
}

fn delete(x: i32, t: &mut SearchTree) {
    if let Some(mut node) = t.take() {
        if x < node.element {
            delete(x, &mut node.left);
            *t = Some(node);
        } else if x > node.element {
            delete(x, &mut node.right);
            *t = Some(node);
        } else {
            if let Some(mut left) = node.left.take() {
                *t = left;
                if let Some(mut right) = node.right {
                    insert(right.element, t);
                    left.right = right.left.take();
                }
            } else if let Some(mut right) = node.right.take() {
                *t = right;
            } else {
                *t = None;
            }
        }
    }
}

fn retrieve(p: Position) -> i32 {
    unsafe { (*p).element }
}

fn main() {
    let mut t: SearchTree = None;
    for i in 0..50 {
        insert(i, &mut t);
    }
    for i in 0..50 {
        let p = find(i, &t);
        if p.is_null() || retrieve(p) != i {
            println!("Error at {}", i);
        }
    }
    for i in 0..50 {
        if i % 2 == 0 {
            delete(i, &mut t);
        }
    }
    for i in 1..50 {
        if i % 2 == 1 {
            let p = find(i, &t);
            if p.is_null() || retrieve(p) != i {
                println!("Error at {}", i);
            }
        } else {
            let p = find(i, &t);
            if !p.is_null() {
                println!("Error at {}", i);
            }
        }
    }
    println!("Min is {}, Max is {}", retrieve(find_min(&t)), retrieve(find_max(&t)));
}