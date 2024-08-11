use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    element: i32,
    left: Link,
    right: Link,
}

impl Node {
    fn new(element: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            element,
            left: None,
            right: None,
        }))
    }
}

fn make_empty() -> Link {
    None
}

fn find(x: i32, tree: &Link) -> Option<Weak<RefCell<Node>>> {
    match tree {
        None => None,
        Some(node) => {
            let node = node.borrow();
            if x < node.element {
                find(x, &node.left)
            } else if x > node.element {
                find(x, &node.right)
            } else {
                Some(Rc::downgrade(&node))
            }
        }
    }
}

fn find_min(tree: &Link) -> Option<Weak<RefCell<Node>>> {
    match tree {
        None => None,
        Some(node) => {
            let node = node.borrow();
            if node.left.is_none() {
                Some(Rc::downgrade(&node))
            } else {
                find_min(&node.left)
            }
        }
    }
}

fn find_max(tree: &Link) -> Option<Weak<RefCell<Node>>> {
    match tree {
        None => None,
        Some(node) => {
            let mut node = node.borrow();
            while node.right.is_some() {
                node = node.right.as_ref().unwrap().borrow();
            }
            Some(Rc::downgrade(&node))
        }
    }
}

fn insert(x: i32, tree: &mut Link) {
    match tree {
        None => {
            *tree = Some(Node::new(x));
        }
        Some(node) => {
            let mut node = node.borrow_mut();
            if x < node.element {
                insert(x, &mut node.left);
            } else if x > node.element {
                insert(x, &mut node.right);
            }
        }
    }
}

fn delete(x: i32, tree: &mut Link) {
    if let Some(node) = tree {
        let mut node = node.borrow_mut();
        if x < node.element {
            delete(x, &mut node.left);
        } else if x > node.element {
            delete(x, &mut node.right);
        } else if node.left.is_some() && node.right.is_some() {
            let min_right = find_min(&node.right);
            node.element = min_right.unwrap().upgrade().unwrap().borrow().element;
            delete(node.element, &mut node.right);
        } else {
            *tree = node.left.clone().or(node.right.clone());
        }
    }
}

fn retrieve(node: &Weak<RefCell<Node>>) -> i32 {
    node.upgrade().unwrap().borrow().element
}

fn main() {
    let mut tree: Link = make_empty();
    for i in 0..50 {
        insert(i, &mut tree);
    }

    for i in 0..50 {
        if let Some(node) = find(i, &tree) {
            assert_eq!(retrieve(&node), i);
        } else {
            println!("Error at {}", i);
        }
    }

    for i in 0..50 {
        if i % 2 == 0 {
            delete(i, &mut tree);
        }
    }

    for i in 1..50 {
        if i % 2 == 1 {
            if let Some(node) = find(i, &tree) {
                assert_eq!(retrieve(&node), i);
            } else {
                println!("Error at {}", i);
            }
        } else {
            if find(i, &tree).is_some() {
                println!("Error at {}", i);
            }
        }
    }

    if let Some(min) = find_min(&tree) {
        println!("Min is {}", retrieve(&min));
    }

    if let Some(max) = find_max(&tree) {
        println!("Max is {}", retrieve(&max));
    }
}