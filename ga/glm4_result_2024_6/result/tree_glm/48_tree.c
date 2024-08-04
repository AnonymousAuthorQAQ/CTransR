use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

type SearchTree = Rc<RefCell<TreeNode>>;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<SearchTree>,
    right: Option<SearchTree>,
}

impl TreeNode {
    fn new(x: i32) -> SearchTree {
        Rc::new(RefCell::new(TreeNode {
            element: x,
            left: None,
            right: None,
        }))
    }
}

fn make_empty(tree: &SearchTree) {
    if let Some(left) = tree.borrow_mut().left.take() {
        make_empty(&left);
    }
    if let Some(right) = tree.borrow_mut().right.take() {
        make_empty(&right);
    }
}

fn find(x: i32, tree: &SearchTree) -> Option<SearchTree> {
    match tree.borrow().element.cmp(&x) {
        std::cmp::Ordering::Less => tree.borrow().left.as_ref().and_then(|left| find(x, left)),
        std::cmp::Ordering::Greater => tree.borrow().right.as_ref().and_then(|right| find(x, right)),
        std::cmp::Ordering::Equal => Some(tree.clone()),
    }
}

fn find_min(tree: &SearchTree) -> Option<SearchTree> {
    let mut current = tree.clone();
    while let Some(left) = current.borrow().left.clone() {
        current = left;
    }
    Some(current)
}

fn find_max(tree: &SearchTree) -> Option<SearchTree> {
    let mut current = tree.clone();
    while let Some(right) = current.borrow().right.clone() {
        current = right;
    }
    Some(current)
}

fn insert(x: i32, tree: &SearchTree) -> SearchTree {
    let mut node = tree.borrow_mut();
    match node.element.cmp(&x) {
        std::cmp::Ordering::Less => {
            node.right = Some(insert(x, &node.right.as_ref().unwrap_or(&TreeNode::new(0))))
        }
        std::cmp::Ordering::Greater => {
            node.left = Some(insert(x, &node.left.as_ref().unwrap_or(&TreeNode::new(0))))
        }
        std::cmp::Ordering::Equal => {}
    }
    tree.clone()
}

fn delete(x: i32, tree: &SearchTree) -> Option<SearchTree> {
    let mut node = tree.borrow_mut();
    match node.element.cmp(&x) {
        std::cmp::Ordering::Less => {
            node.right = delete(x, node.right.as_ref().unwrap());
        }
        std::cmp::Ordering::Greater => {
            node.left = delete(x, node.left.as_ref().unwrap());
        }
        std::cmp::Ordering::Equal => {
            if let Some(left) = node.left.take() {
                if let Some(right) = node.right.take() {
                    let mut min_right = find_min(&right).unwrap();
                    min_right.borrow_mut().right = node.right.take();
                    return Some(left);
                } else {
                    return Some(left);
                }
            } else if let Some(right) = node.right.take() {
                return Some(right);
            } else {
                return None;
            }
        }
    }
    Some(tree.clone())
}

fn retrieve(p: &SearchTree) -> i32 {
    p.borrow().element
}

fn main() {
    let mut t = TreeNode::new(0);
    let mut j = 0;
    for i in 0..50 {
        t = insert(j, &t);
        j = (j + 7) % 50;
    }
    for i in 0..50 {
        if find(i, &t).is_none() || retrieve(find(i, &t).unwrap()) != i {
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
            if find(i, &t).is_none() || retrieve(find(i, &t).unwrap()) != i {
                println!("Error at {}", i);
            }
        } else if find(i, &t).is_some() {
            println!("Error at {}", i);
        }
    }
    if let Some(min) = find_min(&t) {
        println!("Min is {}, Max is {}", retrieve(min), retrieve(find_max(&t).unwrap()));
    }
}