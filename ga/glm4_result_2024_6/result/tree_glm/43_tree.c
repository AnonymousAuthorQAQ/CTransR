use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

type TreeLink = Rc<RefCell<TreeNode>>;
type WeakTreeLink = Weak<RefCell<TreeNode>>;

#[derive(Clone)]
struct TreeNode {
    element: i32,
    left: Option<TreeLink>,
    right: Option<TreeLink>,
}

impl TreeNode {
    fn new(element: i32) -> TreeLink {
        Rc::new(RefCell::new(TreeNode {
            element,
            left: None,
            right: None,
        }))
    }
}

fn make_empty(tree: &Option<TreeLink>) {
    if let Some(node) = tree.clone() {
        make_empty(&node.borrow().left);
        make_empty(&node.borrow().right);
        let mut node = node.borrow_mut();
        node.left = None;
        node.right = None;
    }
}

fn find(x: i32, tree: &Option<TreeLink>) -> Option<TreeLink> {
    if let Some(node) = tree.clone() {
        if x < node.borrow().element {
            return find(x, &node.borrow().left);
        } else if x > node.borrow().element {
            return find(x, &node.borrow().right);
        } else {
            return Some(node);
        }
    }
    None
}

fn find_min(tree: &Option<TreeLink>) -> Option<TreeLink> {
    if let Some(node) = tree.clone() {
        if node.borrow().left.is_none() {
            return Some(node);
        } else {
            return find_min(&node.borrow().left);
        }
    }
    None
}

fn find_max(tree: &Option<TreeLink>) -> Option<TreeLink> {
    if let Some(node) = tree.clone() {
        if node.borrow().right.is_none() {
            return Some(node);
        } else {
            return find_max(&node.borrow().right);
        }
    }
    None
}

fn insert(x: i32, tree: &mut Option<TreeLink>) {
    if let Some(node) = tree.clone() {
        if x < node.borrow().element {
            insert(x, &mut node.borrow_mut().left);
        } else if x > node.borrow().element {
            insert(x, &mut node.borrow_mut().right);
        }
    } else {
        *tree = Some(TreeNode::new(x));
    }
}

fn delete(x: i32, tree: &mut Option<TreeLink>) {
    if let Some(node) = tree.clone() {
        if x < node.borrow().element {
            delete(x, &mut node.borrow_mut().left);
        } else if x > node.borrow().element {
            delete(x, &mut node.borrow_mut().right);
        } else {
            if node.borrow().left.is_some() && node.borrow().right.is_some() {
                let min_right = find_min(&node.borrow().right);
                node.borrow_mut().element = min_right.unwrap().borrow().element;
                delete(min_right.unwrap().borrow().element, &mut node.borrow_mut().right);
            } else {
                let mut node = node.borrow_mut();
                *tree = node.left.clone().or(node.right.clone());
            }
        }
    }
}

fn retrieve(node: &TreeLink) -> i32 {
    node.borrow().element
}

fn main() {
    let mut tree: Option<TreeLink> = None;
    for i in 0..50 {
        insert(i, &mut tree);
    }
    for i in 0..50 {
        if let Some(node) = find(i, &tree) {
            assert_eq!(retrieve(&node), i);
        } else {
            panic!("Error at {}", i);
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
                panic!("Error at {}", i);
            }
        } else {
            if find(i, &tree).is_some() {
                panic!("Error at {}", i);
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