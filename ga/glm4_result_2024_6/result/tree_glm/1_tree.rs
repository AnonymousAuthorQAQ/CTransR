use std::ptr::null_mut;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: *mut TreeNode,
    right: *mut TreeNode,
}

impl TreeNode {
    fn new(element: ElementType) -> *mut TreeNode {
        let node = Box::into_raw(Box::new(TreeNode {
            element,
            left: null_mut(),
            right: null_mut(),
        }));
        node
    }
}

type SearchTree = *mut TreeNode;

fn make_empty(t: SearchTree) -> SearchTree {
    if t.is_null() {
        return null_mut();
    }
    make_empty(unsafe { (*t).left });
    make_empty(unsafe { (*t).right });
    unsafe {
        Box::from_raw(t);
    }
    null_mut()
}

fn find(x: ElementType, t: SearchTree) -> SearchTree {
    if t.is_null() {
        return null_mut();
    }
    if x < unsafe { (*t).element } {
        return find(x, unsafe { (*t).left });
    } else if x > unsafe { (*t).element } {
        return find(x, unsafe { (*t).right });
    } else {
        return t;
    }
}

fn find_min(t: SearchTree) -> SearchTree {
    if t.is_null() {
        return null_mut();
    } else if unsafe { (*t).left }.is_null() {
        return t;
    } else {
        return find_min(unsafe { (*t).left });
    }
}

fn find_max(t: SearchTree) -> SearchTree {
    let mut t = t;
    while !t.is_null() && !unsafe { (*t).right }.is_null() {
        t = unsafe { (*t).right };
    }
    t
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    if t.is_null() {
        return TreeNode::new(x);
    } else if x < unsafe { (*t).element } {
        unsafe { (*t).left = insert(x, (*t).left); }
    } else if x > unsafe { (*t).element } {
        unsafe { (*t).right = insert(x, (*t).right); }
    }
    t
}

fn delete(x: ElementType, t: SearchTree) -> SearchTree {
    let mut tmp_cell = null_mut();
    if t.is_null() {
        panic!("Element not found");
    } else if x < unsafe { (*t).element } {
        unsafe { (*t).left = delete(x, (*t).left); }
    } else if x > unsafe { (*t).element } {
        unsafe { (*t).right = delete(x, (*t).right); }
    } else if !unsafe { (*t).left }.is_null() && !unsafe { (*t).right }.is_null() {
        tmp_cell = find_min(unsafe { (*t).right });
        unsafe { (*t).element = (*tmp_cell).element; }
        unsafe { (*t).right = delete((*tmp_cell).element, (*t).right); }
    } else {
        tmp_cell = t;
        if unsafe { (*t).left }.is_null() {
            t = unsafe { (*t).right };
        } else if unsafe { (*t).right }.is_null() {
            t = unsafe { (*t).left };
        }
        unsafe {
            Box::from_raw(tmp_cell);
        }
    }
    t
}

fn retrieve(p: SearchTree) -> ElementType {
    unsafe { (*p).element }
}

fn main() {
    let mut t: SearchTree = null_mut();
    let mut p: SearchTree;
    let mut i = 0;
    let mut j = 0;

    t = make_empty(t);
    while i < 50 {
        j = (j + 7) % 50;
        t = insert(j, t);
        i += 1;
    }

    i = 0;
    while i < 50 {
        p = find(i, t);
        if p.is_null() || retrieve(p) != i {
            println!("Error at {}", i);
        }
        i += 1;
    }

    i = 0;
    while i < 50 {
        t = delete(i, t);
        i += 2;
    }

    i = 1;
    while i < 50 {
        p = find(i, t);
        if p.is_null() || retrieve(p) != i {
            println!("Error at {}", i);
        }
        i += 2;
    }

    i = 0;
    while i < 50 {
        p = find(i, t);
        if !p.is_null() {
            println!("Error at {}", i);
        }
        i += 2;
    }

    println!("Min is {}, Max is {}", retrieve(find_min(t)), retrieve(find_max(t)));
}