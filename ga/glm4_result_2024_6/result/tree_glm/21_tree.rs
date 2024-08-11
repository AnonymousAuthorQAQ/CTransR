use std::ptr::null_mut;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: *mut TreeNode,
    right: *mut TreeNode,
}

impl TreeNode {
    fn new(x: i32) -> *mut TreeNode {
        let node = Box::into_raw(Box::new(TreeNode {
            element: x,
            left: null_mut(),
            right: null_mut(),
        }));
        node
    }
}

fn make_empty(t: *mut TreeNode) -> *mut TreeNode {
    if t.is_null() {
        return null_mut();
    }
    unsafe {
        make_empty((*t).left);
        make_empty((*t).right);
        Box::from_raw(t);
    }
    null_mut()
}

fn find(x: i32, t: *mut TreeNode) -> *mut TreeNode {
    if t.is_null() {
        return null_mut();
    }
    if x < (*t).element {
        return find(x, (*t).left);
    } else if x > (*t).element {
        return find(x, (*t).right);
    } else {
        return t;
    }
}

fn find_min(t: *mut TreeNode) -> *mut TreeNode {
    if t.is_null() {
        return null_mut();
    } else if (*t).left.is_null() {
        return t;
    } else {
        return find_min((*t).left);
    }
}

fn find_max(t: *mut TreeNode) -> *mut TreeNode {
    if t.is_null() {
        return null_mut();
    }
    let mut t = t;
    while !(*t).right.is_null() {
        t = (*t).right;
    }
    t
}

fn insert(x: i32, t: *mut TreeNode) -> *mut TreeNode {
    if t.is_null() {
        return TreeNode::new(x);
    } else if x < (*t).element {
        (*t).left = insert(x, (*t).left);
    } else if x > (*t).element {
        (*t).right = insert(x, (*t).right);
    }
    t
}

fn delete(x: i32, t: *mut TreeNode) -> *mut TreeNode {
    if t.is_null() {
        panic!("Element not found");
    } else if x < (*t).element {
        (*t).left = delete(x, (*t).left);
    } else if x > (*t).element {
        (*t).right = delete(x, (*t).right);
    } else {
        let mut tmp = t;
        if !(*t).left.is_null() && !(*t).right.is_null() {
            let min_right = find_min((*t).right);
            (*t).element = (*min_right).element;
            (*t).right = delete((*min_right).element, (*t).right);
        } else {
            if (*t).left.is_null() {
                tmp = (*t).right;
            } else if (*t).right.is_null() {
                tmp = (*t).left;
            }
            unsafe {
                Box::from_raw(t);
            }
            tmp
        }
    }
}

fn retrieve(p: *mut TreeNode) -> i32 {
    unsafe { (*p).element }
}

fn main() {
    let mut t: *mut TreeNode = null_mut();
    let mut p: *mut TreeNode;
    let mut i = 0;
    let mut j = 0;

    t = make_empty(t);
    for i in 0..50 {
        j = (j + 7) % 50;
        t = insert(j, t);
    }
    for i in 0..50 {
        p = find(i, t);
        if p.is_null() || retrieve(p) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        p = find(i, t);
        if p.is_null() || retrieve(p) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        p = find(i, t);
        if !p.is_null() {
            println!("Error at {}", i);
        }
    }

    println!("Min is {}, Max is {}", retrieve(find_min(t)), retrieve(find_max(t)));
}