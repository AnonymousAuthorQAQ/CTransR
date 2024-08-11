use std::ptr::null_mut;
use std::cmp::Ordering;

type SearchTree = *mut TreeNode;
type Position = *mut TreeNode;

struct TreeNode {
    element: i32,
    left: SearchTree,
    right: SearchTree,
}

fn make_empty(t: SearchTree) -> SearchTree {
    if t.is_null() {
        return null_mut();
    }
    make_empty(t.as_mut().unwrap().left);
    make_empty(t.as_mut().unwrap().right);
    unsafe {
        Box::from_raw(t);
    }
    null_mut()
}

fn find(x: i32, t: SearchTree) -> Position {
    if t.is_null() {
        return null_mut();
    }
    match x.cmp(&t.as_ref().unwrap().element) {
        Ordering::Less => find(x, t.as_mut().unwrap().left),
        Ordering::Greater => find(x, t.as_mut().unwrap().right),
        Ordering::Equal => t,
    }
}

fn find_min(t: SearchTree) -> Position {
    if t.is_null() {
        return null_mut();
    }
    if t.as_ref().unwrap().left.is_null() {
        t
    } else {
        find_min(t.as_mut().unwrap().left)
    }
}

fn find_max(t: SearchTree) -> Position {
    if t.is_null() {
        return null_mut();
    }
    let mut t = t;
    while t.as_ref().unwrap().right.is_some() {
        t = t.as_mut().unwrap().right;
    }
    t
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    if t.is_null() {
        let node = Box::new(TreeNode {
            element: x,
            left: null_mut(),
            right: null_mut(),
        });
        return Box::into_raw(node);
    }
    match x.cmp(&t.as_ref().unwrap().element) {
        Ordering::Less => {
            t.as_mut().unwrap().left = insert(x, t.as_mut().unwrap().left);
        }
        Ordering::Greater => {
            t.as_mut().unwrap().right = insert(x, t.as_mut().unwrap().right);
        }
        _ => {}
    }
    t
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    if t.is_null() {
        panic!("Element not found");
    }
    match x.cmp(&t.as_ref().unwrap().element) {
        Ordering::Less => {
            t.as_mut().unwrap().left = delete(x, t.as_mut().unwrap().left);
        }
        Ordering::Greater => {
            t.as_mut().unwrap().right = delete(x, t.as_mut().unwrap().right);
        }
        _ => {
            if t.as_ref().unwrap().left.is_some() && t.as_ref().unwrap().right.is_some() {
                let min_right = find_min(t.as_mut().unwrap().right);
                t.as_mut().unwrap().element = min_right.as_ref().unwrap().element;
                t.as_mut().unwrap().right = delete(t.as_mut().unwrap().element, t.as_mut().unwrap().right);
            } else {
                let mut tmp = t;
                if t.as_ref().unwrap().left.is_none() {
                    t = t.as_mut().unwrap().right;
                } else if t.as_ref().unwrap().right.is_none() {
                    t = t.as_mut().unwrap().left;
                }
                unsafe {
                    Box::from_raw(tmp);
                }
            }
        }
    }
    t
}

fn retrieve(p: Position) -> i32 {
    p.as_ref().unwrap().element
}

fn main() {
    let mut t: SearchTree = make_empty(null_mut());
    for i in 0..50 {
        t = insert(i, t);
    }
    for i in 0..50 {
        let p = find(i, t);
        if p.is_null() || retrieve(p) != i {
            println!("Error at {}", i);
        }
    }
    for i in 0..50 {
        if i % 2 == 0 {
            t = delete(i, t);
        }
    }
    for i in 1..50 {
        if i % 2 == 1 {
            let p = find(i, t);
            if p.is_null() || retrieve(p) != i {
                println!("Error at {}", i);
            }
        }
    }
    for i in 0..50 {
        if i % 2 == 0 {
            let p = find(i, t);
            if p.is_some() {
                println!("Error at {}", i);
            }
        }
    }
    println!("Min is {}, Max is {}", retrieve(find_min(t)), retrieve(find_max(t)));
}