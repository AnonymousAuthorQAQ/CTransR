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
    if let Some(node) = tree.take() {
        make_empty(&mut node.borrow_mut().left);
        make_empty(&mut node.borrow_mut().right);
    }
}

fn find(x: i32, tree: &TreeLink) -> TreeLink {
    let mut current = tree.clone();
    while let Some(n) = current {
        if x < n.borrow().element {
            current = n.borrow_mut().left.clone();
        } else if x > n.borrow().element {
            current = n.borrow_mut().right.clone();
        } else {
            return Some(n);
        }
    }
    None
}

fn find_min(tree: &TreeLink) -> TreeLink {
    let mut current = tree.clone();
    while let Some(n) = current {
        if n.borrow().left.is_none() {
            return Some(n);
        }
        current = n.borrow_mut().left.clone();
    }
    None
}

fn find_max(tree: &TreeLink) -> TreeLink {
    let mut current = tree.clone();
    while let Some(n) = current {
        if n.borrow().right.is_none() {
            return Some(n);
        }
        current = n.borrow_mut().right.clone();
    }
    None
}

fn insert(x: i32, tree: &mut TreeLink) {
    if let Some(n) = tree.clone() {
        if x < n.borrow().element {
            insert(x, &mut n.borrow_mut().left);
        } else if x > n.borrow().element {
            insert(x, &mut n.borrow_mut().right);
        }
    } else {
        *tree = Some(TreeNode::new(x));
    }
}

fn delete(x: i32, tree: &mut TreeLink) {
    if let Some(mut n) = tree.take() {
        if x < n.borrow().element {
            delete(x, &mut n.borrow_mut().left);
            *tree = Some(n);
        } else if x > n.borrow().element {
            delete(x, &mut n.borrow_mut().right);
            *tree = Some(n);
        } else {
            if let Some(mut right) = n.borrow_mut().right.take() {
                if let Some(min) = find_min(&right) {
                    let mut min = min.borrow_mut();
                    if let Some(mut left) = min.left.take() {
                        min.left = Some(left);
                    }
                    min.right = n.borrow_mut().left.take();
                    *tree = Some(min);
                } else {
                    *tree = Some(right);
                }
            } else {
                *tree = n.borrow_mut().left.take();
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
        let p = find(i, &tree);
        if p.is_none() || retrieve(&p.unwrap()) != i {
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
            let p = find(i, &tree);
            if p.is_none() || retrieve(&p.unwrap()) != i {
                println!("Error at {}", i);
            }
        } else {
            let p = find(i, &tree);
            if p.is_some() {
                println!("Error at {}", i);
            }
        }
    }
    println!("Min is {}, Max is {}", retrieve(&find_min(&tree).unwrap()), retrieve(&find_max(&tree).unwrap()));
}