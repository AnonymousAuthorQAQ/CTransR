use std::cell::RefCell;
use std::cmp::{Ord, Ordering};
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

type SearchTree = Option<Rc<RefCell<TreeNode>>>;

fn make_empty() -> SearchTree {
    None
}

fn find(x: i32, t: &SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    match t {
        None => None,
        Some(node) => {
            let n = node.borrow();

            match x.cmp(&n.element) {
                Ordering::Less => find(x, &n.left),
                Ordering::Greater => find(x, &n.right),
                Ordering::Equal => Some(Rc::clone(node)),
            }
        }
    }
}

fn find_min(t: &SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    match t {
        None => None,
        Some(node) => {
            let n = node.borrow();
            if n.left.is_none() {
                Some(Rc::clone(node))
            } else {
                find_min(&n.left)
            }
        }
    }
}

fn find_max(t: &SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    match t {
        None => None,
        Some(node) => {
            let n = node.borrow();
            if n.right.is_none() {
                Some(Rc::clone(node))
            } else {
                find_max(&n.right)
            }
        }
    }
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => {
            Some(Rc::new(RefCell::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })))
        }
        Some(node) => {
            let mut n = node.borrow_mut();
            match x.cmp(&n.element) {
                Ordering::Less => {
                    n.left = insert(x, n.left.take());
                }
                Ordering::Greater => {
                    n.right = insert(x, n.right.take());
                }
                _ => {} // No duplicates in the tree
            }
            Some(node) // Return the unchanged node
        }
    }
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => {
            println!("Element not found");
            None
        }
        Some(node) => {
            let mut n = node.borrow_mut();
            match x.cmp(&n.element) {
                Ordering::Less => {
                    n.left = delete(x, n.left.take());
                    Some(node)
                }
                Ordering::Greater => {
                    n.right = delete(x, n.right.take());
                    Some(node)
                }
                Ordering::Equal => {
                    match (n.left.take(), n.right.take()) {
                        (None, None) => None,
                        (Some(left), None) => Some(left),
                        (None, Some(right)) => Some(right),
                        (Some(left), Some(right)) => {
                            let min_node = find_min(&Some(right)).unwrap(); // Safe to unwrap
                            n.element = min_node.borrow().element; // Replace with min element from right
                            n.right = delete(n.element, Some(right)); // Delete the min node
                            Some(node)
                        }
                    }
                }
            }
        }
    }
}

fn retrieve(p: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    p.as_ref().map_or(0, |node| node.borrow().element)
}

fn main() {
    let mut tree: SearchTree = make_empty();

    for i in 0..50 {
        let j = (i * 7) % 50;
        tree = insert(j, tree);
    }
    
    for i in 0..50 {
        if let Some(p) = find(i, &tree) {
            if retrieve(&Some(p)) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        tree = delete(i, tree);
    }

    for i in (1..50).step_by(2) {
        if let Some(p) = find(i, &tree) {
            if retrieve(&Some(p)) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }
    
    for i in (0..50).step_by(2) {
        if let Some(p) = find(i, &tree) {
            println!("Error at {}", i);
        }
    }

    if let Some(min) = find_min(&tree) {
        if let Some(max) = find_max(&tree) {
            println!("Min is {}, Max is {}", retrieve(&Some(min)), retrieve(&Some(max)));
        }
    }
}