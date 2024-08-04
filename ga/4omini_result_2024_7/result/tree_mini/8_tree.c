use std::cell::RefCell;
use std::cmp::{min, max};
use std::fmt;
use std::io::{self, Write};
use std::rc::Rc;

type ElementType = i32;

#[derive(Debug)]
struct TreeNode {
    element: ElementType,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

type SearchTree = Option<Rc<RefCell<TreeNode>>>;

impl TreeNode {
    fn new(element: ElementType) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
}

fn make_empty() -> SearchTree {
    None
}

fn find(x: ElementType, tree: SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    match tree {
        None => None,
        Some(node) => {
            let node_ref = node.borrow();
            if x < node_ref.element {
                find(x, node_ref.left.clone())
            } else if x > node_ref.element {
                find(x, node_ref.right.clone())
            } else {
                Some(node.clone())
            }
        }
    }
}

fn find_min(tree: SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    match tree {
        None => None,
        Some(node) => {
            let node_ref = node.borrow();
            if node_ref.left.is_none() {
                Some(node.clone())
            } else {
                find_min(node_ref.left.clone())
            }
        }
    }
}

fn find_max(tree: SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    let mut current = tree;
    while let Some(node) = current {
        let node_ref = node.borrow();
        if node_ref.right.is_none() {
            return Some(node.clone());
        }
        current = node_ref.right.clone();
    }
    None
}

fn insert(x: ElementType, tree: SearchTree) -> SearchTree {
    match tree {
        None => Some(Rc::new(RefCell::new(TreeNode::new(x)))),
        Some(node) => {
            let mut node_ref = node.borrow_mut();
            if x < node_ref.element {
                node_ref.left = insert(x, node_ref.left.clone());
            } else if x > node_ref.element {
                node_ref.right = insert(x, node_ref.right.clone());
            }
            Some(node.clone())
        }
    }
}

fn delete(x: ElementType, tree: SearchTree) -> SearchTree {
    match tree {
        None => {
            eprintln!("Error: Element not found");
            None
        }
        Some(node) => {
            let mut node_ref = node.borrow_mut();
            if x < node_ref.element {
                node_ref.left = delete(x, node_ref.left.clone());
            } else if x > node_ref.element {
                node_ref.right = delete(x, node_ref.right.clone());
            } else {
                if node_ref.left.is_some() && node_ref.right.is_some() {
                    // Node with two children
                    let min_node = find_min(node_ref.right.clone()).unwrap();
                    node_ref.element = min_node.borrow().element;
                    node_ref.right = delete(node_ref.element, node_ref.right.clone());
                } else {
                    // Node with one or no children
                    return if node_ref.left.is_none() {
                        node_ref.right.take()
                    } else {
                        node_ref.left.take()
                    };
                }
            }
            Some(node.clone())
        }
    }
}

fn retrieve(node: Option<Rc<RefCell<TreeNode>>>) -> Option<ElementType> {
    node.map(|n| n.borrow().element)
}

fn main() {
    let mut tree = make_empty();
    let mut j = 0;

    for i in 0..50 {
        j = (j + 7) % 50;
        tree = insert(j, tree);
    }
    
    for i in 0..50 {
        if let Some(p) = find(i, tree.clone()) {
            if retrieve(Some(p)) != Some(i) {
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
        if let Some(p) = find(i, tree.clone()) {
            if retrieve(Some(p)) != Some(i) {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if find(i, tree.clone()).is_some() {
            println!("Error at {}", i);
        }
    }

    let min_node = find_min(tree.clone());
    let max_node = find_max(tree.clone());

    if let (Some(min), Some(max)) = (min_node, max_node) {
        println!("Min is {}, Max is {}", retrieve(Some(min)).unwrap(), retrieve(Some(max)).unwrap());
    }
}