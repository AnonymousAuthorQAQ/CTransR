use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

type TreeLink = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: TreeLink,
    right: TreeLink,
}

impl TreeNode {
    fn new(element: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            element,
            left: None,
            right: None,
        }))
    }
}

fn make_empty(tree: &mut TreeLink) {
    if let Some(link) = tree.take() {
        make_empty(&mut link.borrow_mut().left);
        make_empty(&mut link.borrow_mut().right);
    }
}

fn find(x: i32, tree: &TreeLink) -> TreeLink {
    match tree {
        None => None,
        Some(node) => {
            let node = node.borrow();
            if x < node.element {
                find(x, &node.left)
            } else if x > node.element {
                find(x, &node.right)
            } else {
                Some(node.clone())
            }
        }
    }
}

fn find_min(tree: &TreeLink) -> TreeLink {
    match tree {
        None => None,
        Some(node) => {
            let node = node.borrow();
            if let Some(left) = &node.left {
                find_min(&Some(left.clone()))
            } else {
                Some(node.clone())
            }
        }
    }
}

fn find_max(tree: &TreeLink) -> TreeLink {
    match tree {
        None => None,
        Some(node) => {
            let node = node.borrow();
            if let Some(right) = &node.right {
                find_max(&Some(right.clone()))
            } else {
                Some(node.clone())
            }
        }
    }
}

fn insert(x: i32, tree: &mut TreeLink) {
    if let None = tree {
        *tree = Some(TreeNode::new(x));
    } else {
        let mut node = tree.as_ref().unwrap().borrow_mut();
        if x < node.element {
            insert(x, &mut node.left);
        } else if x > node.element {
            insert(x, &mut node.right);
        }
    }
}

fn delete(x: i32, tree: &mut TreeLink) {
    if let Some(node) = tree.take() {
        let mut node = node.borrow_mut();
        if x < node.element {
            delete(x, &mut node.left);
        } else if x > node.element {
            delete(x, &mut node.right);
        } else {
            if let Some(left) = node.left.take() {
                if let Some(right) = node.right.take() {
                    let min_right = find_min(&Some(right));
                    node.element = min_right.borrow().element;
                    node.left = left;
                    node.right = min_right.borrow_mut().right.take();
                } else {
                    *tree = node.left.take();
                }
            } else {
                *tree = node.right.take();
            }
        }
    }
}

fn retrieve(p: &TreeLink) -> i32 {
    p.as_ref().unwrap().borrow().element
}

fn main() {
    let mut tree: TreeLink = None;
    for i in 0..50 {
        insert(i, &mut tree);
    }
    for i in 0..50 {
        if let None = find(i, &tree) {
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
            if let None = find(i, &tree) {
                println!("Error at {}", i);
            }
        }
    }
    for i in 0..50 {
        if i % 2 == 0 {
            if let Some(_) = find(i, &tree) {
                println!("Error at {}", i);
            }
        }
    }
    if let Some(min) = find_min(&tree) {
        println!("Min is {}, ", retrieve(&min));
    }
    if let Some(max) = find_max(&tree) {
        println!("Max is {}", retrieve(&max));
    }
}