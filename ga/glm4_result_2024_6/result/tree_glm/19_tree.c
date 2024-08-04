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
}

fn make_empty(tree: &mut Option<SearchTree>) {
    *tree = None;
}

fn find(x: i32, tree: &SearchTree) -> Option<SearchTree> {
    let tree = tree.borrow();
    if let Some(left) = &tree.left {
        if x < tree.element {
            return find(x, left);
        }
    }
    if x == tree.element {
        return Some(tree.clone());
    }
    if let Some(right) = &tree.right {
        if x > tree.element {
            return find(x, right);
        }
    }
    None
}

fn find_min(tree: &SearchTree) -> Option<SearchTree> {
    let mut tree = tree.borrow_mut();
    while let Some(left) = &tree.left {
        tree = left.borrow_mut();
    }
    Some(tree.clone())
}

fn find_max(tree: &SearchTree) -> Option<SearchTree> {
    let mut tree = tree.borrow_mut();
    while let Some(right) = &tree.right {
        tree = right.borrow_mut();
    }
    Some(tree.clone())
}

fn insert(x: i32, tree: &mut Option<SearchTree>) {
    if let Some(tree) = tree {
        if x < tree.borrow().element {
            insert(x, &mut tree.borrow_mut().left);
        } else if x > tree.borrow().element {
            insert(x, &mut tree.borrow_mut().right);
        }
    } else {
        *tree = Some(Rc::new(RefCell::new(TreeNode::new(x))));
    }
}

fn delete(x: i32, tree: &mut Option<SearchTree>) {
    if let Some(tree) = tree {
        if x < tree.borrow().element {
            delete(x, &mut tree.borrow_mut().left);
        } else if x > tree.borrow().element {
            delete(x, &mut tree.borrow_mut().right);
        } else {
            if let Some(left) = tree.borrow_mut().left.take() {
                if let Some(right) = tree.borrow_mut().right.take() {
                    let mut min = find_min(&right).unwrap();
                    min.borrow_mut().right = Some(right);
                    *tree = Some(min);
                } else {
                    *tree = Some(left);
                }
            } else if let Some(right) = tree.borrow_mut().right.take() {
                *tree = Some(right);
            } else {
                *tree = None;
            }
        }
    }
}

fn retrieve(tree: &SearchTree) -> i32 {
    tree.borrow().element
}

fn main() {
    let mut tree: Option<SearchTree> = None;
    for i in 0..50 {
        insert(i, &mut tree);
    }
    for i in 0..50 {
        if let Some(node) = find(i, &tree.unwrap()) {
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
            if let Some(node) = find(i, &tree.unwrap()) {
                assert_eq!(retrieve(&node), i);
            } else {
                panic!("Error at {}", i);
            }
        } else {
            if find(i, &tree.unwrap()).is_some() {
                panic!("Error at {}", i);
            }
        }
    }
    println!("Min is {}, Max is {}", retrieve(&find_min(&tree.unwrap()).unwrap()), retrieve(&find_max(&tree.unwrap()).unwrap()));
}