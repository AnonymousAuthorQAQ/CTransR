use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

type SearchTree = Rc<RefCell<TreeNode>>;

#[derive(Clone)]
struct TreeNode {
    element: i32,
    left: Option<SearchTree>,
    right: Option<SearchTree>,
}

impl TreeNode {
    fn new(element: i32) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
}

fn make_empty() -> SearchTree {
    Rc::new(RefCell::new(TreeNode::new(0)))
}

fn find(x: i32, t: &SearchTree) -> Option<SearchTree> {
    let mut t = t.borrow_mut();
    if t.element == x {
        return Some(t.clone());
    }
    if x < t.element {
        t.left.as_ref()?;
        return find(x, t.left.as_ref().unwrap());
    }
    t.right.as_ref()?;
    find(x, t.right.as_ref().unwrap())
}

fn find_min(t: &SearchTree) -> Option<SearchTree> {
    let mut t = t.borrow_mut();
    if t.left.is_none() {
        return Some(t.clone());
    }
    find_min(t.left.as_ref().unwrap())
}

fn find_max(t: &SearchTree) -> Option<SearchTree> {
    let mut t = t.borrow_mut();
    while let Some(right) = t.right.clone() {
        t = right.borrow_mut();
    }
    Some(t.clone())
}

fn insert(x: i32, t: &SearchTree) -> SearchTree {
    let mut t = t.borrow_mut();
    if x < t.element {
        if t.left.is_none() {
            t.left = Some(Rc::new(RefCell::new(TreeNode::new(x))));
        } else {
            insert(x, t.left.as_ref().unwrap());
        }
    } else {
        if t.right.is_none() {
            t.right = Some(Rc::new(RefCell::new(TreeNode::new(x))));
        } else {
            insert(x, t.right.as_ref().unwrap());
        }
    }
    t.clone()
}

fn delete(x: i32, t: &SearchTree) -> SearchTree {
    let mut t = t.borrow_mut();
    if t.element == x {
        if t.left.is_none() {
            return t.right.clone();
        }
        if t.right.is_none() {
            return t.left.clone();
        }
        let mut right_min = find_min(t.right.as_ref().unwrap()).unwrap();
        t.element = right_min.borrow_mut().element;
        t.right = delete(right_min.borrow_mut().element, t.right.as_ref().unwrap());
    } else if x < t.element {
        t.left = delete(x, t.left.as_ref().unwrap());
    } else {
        t.right = delete(x, t.right.as_ref().unwrap());
    }
    t.clone()
}

fn retrieve(p: &SearchTree) -> i32 {
    p.borrow_mut().element
}

fn main() {
    let mut t = make_empty();
    for i in 0..50 {
        t = insert(i, &t);
    }
    for i in 0..50 {
        if let Some(p) = find(i, &t) {
            if retrieve(&p) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }
    for i in 0..50 {
        if i % 2 == 0 {
            t = delete(i, &t);
        }
    }
    for i in 1..50 {
        if i % 2 == 1 {
            if let Some(p) = find(i, &t) {
                if retrieve(&p) != i {
                    println!("Error at {}", i);
                }
            } else {
                println!("Error at {}", i);
            }
        } else {
            if let Some(p) = find(i, &t) {
                println!("Error at {}", i);
            }
        }
    }
    if let Some(min) = find_min(&t) {
        println!("Min is {}", retrieve(&min));
    }
    if let Some(max) = find_max(&t) {
        println!("Max is {}", retrieve(&max));
    }
}