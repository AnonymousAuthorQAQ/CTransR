use std::ptr;
use std::rc::Rc;
use std::cell::RefCell;

type ElementType = i32;

#[derive(Debug)]
struct TreeNode {
    element: ElementType,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

type SearchTree = Option<Rc<RefCell<TreeNode>>>;

fn make_empty() -> SearchTree {
    None
}

fn find(x: ElementType, t: &SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    match t {
        Some(node) => {
            let node_borrowed = node.borrow();
            if x < node_borrowed.element {
                find(x, &node_borrowed.left)
            } else if x > node_borrowed.element {
                find(x, &node_borrowed.right)
            } else {
                Some(Rc::clone(node))
            }
        }
        None => None,
    }
}

fn find_min(t: &SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    match t {
        Some(node) => {
            let node_borrowed = node.borrow();
            if node_borrowed.left.is_none() {
                Some(Rc::clone(node))
            } else {
                find_min(&node_borrowed.left)
            }
        }
        None => None,
    }
}

fn find_max(t: &SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    let mut current = t;
    while let Some(node) = current {
        let node_borrowed = node.borrow();
        if node_borrowed.right.is_none() {
            return Some(Rc::clone(node));
        }
        current = &node_borrowed.right;
    }
    None
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        Some(node) => {
            let mut node_borrowed = node.borrow_mut();
            if x < node_borrowed.element {
                node_borrowed.left = insert(x, node_borrowed.left.clone());
            } else if x > node_borrowed.element {
                node_borrowed.right = insert(x, node_borrowed.right.clone());
            }
            Some(node)
        }
        None => {
            let new_node = Rc::new(RefCell::new(TreeNode {
                element: x,
                left: None,
                right: None,
            }));
            Some(new_node)
        }
    }
}

fn delete(x: ElementType, t: SearchTree) -> SearchTree {
    if t.is_none() {
        eprintln!("Error: Element not found");
        return t;
    }
    let mut node_borrowed = t.as_ref().unwrap().borrow_mut();
    if x < node_borrowed.element {
        node_borrowed.left = delete(x, node_borrowed.left.clone());
    } else if x > node_borrowed.element {
        node_borrowed.right = delete(x, node_borrowed.right.clone());
    } else {
        if node_borrowed.left.is_some() && node_borrowed.right.is_some() {
            let min_node = find_min(&node_borrowed.right);
            if let Some(min_node) = min_node {
                let min_node_borrowed = min_node.borrow();
                node_borrowed.element = min_node_borrowed.element;
                node_borrowed.right = delete(min_node_borrowed.element, node_borrowed.right.clone());
            }
        } else {
            let tmp_cell = Rc::clone(&t.as_ref().unwrap());
            if node_borrowed.left.is_none() {
                return node_borrowed.right.clone();
            } else {
                return node_borrowed.left.clone();
            }
        }
    }
    Some(Rc::clone(&t.as_ref().unwrap()))
}

fn retrieve(p: &Option<Rc<RefCell<TreeNode>>>) -> ElementType {
    if let Some(node) = p {
        node.borrow().element
    } else {
        panic!("Attempted to retrieve from None");
    }
}

fn main() {
    let mut t = make_empty();
    let mut j = 0;

    // Insert elements
    for i in 0..50 {
        j = (j + 7) % 50;
        t = insert(j, t);
    }

    // Check if all elements from 0 to 49 are present
    for i in 0..50 {
        let p = find(i, &t);
        if p.is_none() || retrieve(&p) != i {
            println!("Error at {}", i);
        }
    }

    // Delete even elements
    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    // Check if all odd elements are present
    for i in (1..50).step_by(2) {
        let p = find(i, &t);
        if p.is_none() || retrieve(&p) != i {
            println!("Error at {}", i);
        }
    }

    // Check if all even elements are absent
    for i in (0..50).step_by(2) {
        let p = find(i, &t);
        if p.is_some() {
            println!("Error at {}", i);
        }
    }

    // Display min and max
    let min = find_min(&t);
    let max = find_max(&t);
    println!(
        "Min is {}, Max is {}",
        min.map_or(-1, |node| retrieve(&Some(node))),
        max.map_or(-1, |node| retrieve(&Some(node)))
    );
}