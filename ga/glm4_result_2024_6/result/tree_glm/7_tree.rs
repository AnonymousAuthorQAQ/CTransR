use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

type Element = i32;
type Tree = Rc<RefCell<Node>>;

struct Node {
    element: Element,
    left: Option<Tree>,
    right: Option<Tree>,
}

impl Node {
    fn new(element: Element) -> Tree {
        Rc::new(RefCell::new(Node {
            element,
            left: None,
            right: None,
        }))
    }
}

fn make_empty() -> Option<Tree> {
    None
}

fn find(x: Element, tree: Option<Tree>) -> Option<Tree> {
    tree.and_then(|t| {
        if x < t.borrow().element {
            find(x, t.borrow().left.clone())
        } else if x > t.borrow().element {
            find(x, t.borrow().right.clone())
        } else {
            Some(t)
        }
    })
}

fn find_min(tree: Option<Tree>) -> Option<Tree> {
    tree.and_then(|t| {
        match t.borrow().left.clone() {
            None => Some(t),
            Some(left) => find_min(Some(left)),
        }
    })
}

fn find_max(tree: Option<Tree>) -> Option<Tree> {
    tree.and_then(|t| {
        if let Some(right) = t.borrow().right.clone() {
            find_max(Some(right))
        } else {
            Some(t)
        }
    })
}

fn insert(x: Element, tree: Option<Tree>) -> Tree {
    match tree {
        None => Node::new(x),
        Some(t) => {
            if x < t.borrow().element {
                let mut borrowed = t.borrow_mut();
                borrowed.left = insert(x, borrowed.left.take());
                t
            } else if x > t.borrow().element {
                let mut borrowed = t.borrow_mut();
                borrowed.right = insert(x, borrowed.right.take());
                t
            } else {
                t
            }
        }
    }
}

fn delete(x: Element, tree: Option<Tree>) -> Option<Tree> {
    if let Some(mut t) = tree {
        if x < t.borrow().element {
            t.borrow_mut().left = delete(x, t.borrow().left.clone());
        } else if x > t.borrow().element {
            t.borrow_mut().right = delete(x, t.borrow().right.clone());
        } else {
            if let Some(mut left) = t.borrow_mut().left.take() {
                if let Some(mut right) = t.borrow_mut().right.take() {
                    let min_right = find_min(Some(right));
                    t.borrow_mut().element = min_right.unwrap().borrow().element;
                    t.borrow_mut().right = delete(t.borrow().element, Some(right));
                } else {
                    left
                }
            } else {
                t.borrow_mut().right.take()
            }
        }
        Some(t)
    } else {
        None
    }
}

fn retrieve(tree: Option<Tree>) -> Option<Element> {
    tree.map(|t| t.borrow().element)
}

fn main() {
    let mut tree = make_empty();
    for i in 0..50 {
        tree = insert(i, tree);
    }

    for i in 0..50 {
        if let Some(t) = find(i, tree.clone()) {
            assert_eq!(retrieve(Some(t)), Some(i));
        } else {
            println!("Error at {}", i);
        }
    }

    for i in 0..50 {
        if i % 2 == 0 {
            tree = delete(i, tree);
        }
    }

    for i in 1..50 {
        if i % 2 == 1 {
            if let Some(t) = find(i, tree.clone()) {
                assert_eq!(retrieve(Some(t)), Some(i));
            } else {
                println!("Error at {}", i);
            }
        } else {
            if find(i, tree.clone()).is_some() {
                println!("Error at {}", i);
            }
        }
    }

    println!("Min is {}, Max is {}", retrieve(find_min(tree)).unwrap(), retrieve(find_max(tree)).unwrap());
}