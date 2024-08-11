use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

type SearchTree = Rc<RefCell<TreeNode>>;

#[derive(Clone)]
struct TreeNode {
    element: i32,
    left: Option<SearchTree>,
    right: Option<SearchTree>,
}

impl TreeNode {
    fn new(element: i32) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }

    fn make_empty(&mut self) {
        self.left.take();
        self.right.take();
    }
}

fn make_empty(tree: SearchTree) -> SearchTree {
    Rc::new(RefCell::new(TreeNode::new(0)))
}

fn find(x: i32, tree: &SearchTree) -> Option<SearchTree> {
    let tree = tree.borrow();
    if tree.element == x {
        Some(tree.clone())
    } else if x < tree.element {
        tree.left.as_ref().and_then(|left| find(x, left))
    } else {
        tree.right.as_ref().and_then(|right| find(x, right))
    }
}

fn find_min(tree: &SearchTree) -> Option<SearchTree> {
    let mut tree = tree.borrow_mut();
    while let Some(left) = tree.left.take() {
        tree = left.borrow_mut();
    }
    Some(tree.clone())
}

fn find_max(tree: &SearchTree) -> Option<SearchTree> {
    let mut tree = tree.borrow_mut();
    while let Some(right) = tree.right.take() {
        tree = right.borrow_mut();
    }
    Some(tree.clone())
}

fn insert(x: i32, tree: &mut SearchTree) {
    let mut tree = tree.borrow_mut();
    if x < tree.element {
        if let None = tree.left {
            tree.left = Some(Rc::new(RefCell::new(TreeNode::new(x))));
        } else {
            insert(x, tree.left.as_mut().unwrap());
        }
    } else {
        if let None = tree.right {
            tree.right = Some(Rc::new(RefCell::new(TreeNode::new(x))));
        } else {
            insert(x, tree.right.as_mut().unwrap());
        }
    }
}

fn delete(x: i32, tree: &mut SearchTree) {
    let mut tree = tree.borrow_mut();
    if tree.element == x {
        if let Some(left) = tree.left.take() {
            if let Some(right) = tree.right.take() {
                let mut min = find_min(&right).unwrap();
                tree.element = min.borrow().element;
                tree.left = left;
                tree.right = min.borrow_mut().right.take();
            } else {
                tree.element = left.borrow().element;
                tree.left = left.borrow_mut().left.take();
                tree.right = left.borrow_mut().right.take();
            }
        } else if let Some(right) = tree.right.take() {
            tree.element = right.borrow().element;
            tree.left = right.borrow_mut().left.take();
            tree.right = right.borrow_mut().right.take();
        } else {
            tree.make_empty();
        }
    } else if x < tree.element {
        if let Some(left) = tree.left.as_mut() {
            delete(x, left);
        }
    } else {
        if let Some(right) = tree.right.as_mut() {
            delete(x, right);
        }
    }
}

fn retrieve(tree: &SearchTree) -> i32 {
    tree.borrow().element
}

fn main() {
    let mut tree = make_empty(Rc::new(RefCell::new(TreeNode::new(0))));
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
        println!("Min is {}, ", retrieve(&min));
    }
    if let Some(max) = find_max(&tree) {
        println!("Max is {}", retrieve(&max));
    }
}