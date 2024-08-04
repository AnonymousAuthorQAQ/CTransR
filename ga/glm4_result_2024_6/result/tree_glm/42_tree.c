use std::rc::Rc;
use std::cell::RefCell;

type SearchTree = Rc<RefCell<TreeNode>>;

#[derive(Clone)]
struct TreeNode {
    element: i32,
    left: SearchTree,
    right: SearchTree,
}

impl TreeNode {
    fn new(element: i32) -> Self {
        TreeNode {
            element,
            left: Rc::new(RefCell::new(TreeNode::null())),
            right: Rc::new(RefCell::new(TreeNode::null())),
        }
    }

    fn null() -> Self {
        TreeNode {
            element: 0,
            left: Rc::new(RefCell::new(TreeNode::null())),
            right: Rc::new(RefCell::new(TreeNode::null())),
        }
    }
}

fn make_empty(t: SearchTree) -> SearchTree {
    Rc::new(RefCell::new(TreeNode::null()))
}

fn find(x: i32, t: SearchTree) -> Option<SearchTree> {
    let mut t = t.borrow_mut();
    if t.element == x {
        return Some(t.clone());
    }
    if x < t.element {
        return find(x, t.left.take());
    }
    find(x, t.right.take())
}

fn find_min(t: SearchTree) -> Option<SearchTree> {
    let mut t = t.borrow_mut();
    if t.left.borrow().element == 0 {
        return Some(t.clone());
    }
    find_min(t.left.take())
}

fn find_max(t: SearchTree) -> Option<SearchTree> {
    let mut t = t.borrow_mut();
    if t.right.borrow().element == 0 {
        return Some(t.clone());
    }
    find_max(t.right.take())
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    let mut t = t.borrow_mut();
    if t.element == 0 {
        t.element = x;
    } else if x < t.element {
        t.left = insert(x, t.left.take());
    } else {
        t.right = insert(x, t.right.take());
    }
    t.clone()
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    let mut t = t.borrow_mut();
    if t.element == 0 {
        return t.clone();
    }
    if x < t.element {
        t.left = delete(x, t.left.take());
    } else if x > t.element {
        t.right = delete(x, t.right.take());
    } else {
        if t.left.borrow().element != 0 && t.right.borrow().element != 0 {
            let min_right = find_min(t.right.take()).unwrap();
            t.element = min_right.borrow().element;
            t.right = delete(t.element, t.right.take());
        } else {
            let mut child = if t.left.borrow().element != 0 {
                t.left.take()
            } else {
                t.right.take()
            };
            return child;
        }
    }
    t.clone()
}

fn retrieve(p: SearchTree) -> i32 {
    p.borrow().element
}

fn main() {
    let mut t = Rc::new(RefCell::new(TreeNode::null()));
    for i in 0..50 {
        t = insert(i, t.clone());
    }
    for i in 0..50 {
        if retrieve(find(i, t.clone()).unwrap()) != i {
            println!("Error at {}", i);
        }
    }
    for i in 0..50 {
        if i % 2 == 0 {
            t = delete(i, t.clone());
        }
    }
    for i in 1..50 {
        if i % 2 == 1 {
            if retrieve(find(i, t.clone()).unwrap()) != i {
                println!("Error at {}", i);
            }
        } else {
            if find(i, t.clone()).is_some() {
                println!("Error at {}", i);
            }
        }
    }
    println!("Min is {}, Max is {}", retrieve(find_min(t.clone()).unwrap()), retrieve(find_max(t.clone()).unwrap()));
}