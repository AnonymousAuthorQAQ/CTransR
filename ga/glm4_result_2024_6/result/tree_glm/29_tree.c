use std::cmp::Ordering;
use std::rc::Rc;
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

fn make_empty(tree: Option<SearchTree>) -> Option<SearchTree> {
    if let Some(tree) = tree {
        tree.borrow_mut().left = make_empty(tree.borrow().left.clone());
        tree.borrow_mut().right = make_empty(tree.borrow().right.clone());
    }
    None
}

fn find(x: i32, tree: &Option<SearchTree>) -> Option<SearchTree> {
    match tree {
        Some(tree) => {
            let tree = tree.borrow();
            match x.cmp(&tree.element) {
                Ordering::Less => find(x, &tree.left),
                Ordering::Greater => find(x, &tree.right),
                Ordering::Equal => Some(tree.clone()),
            }
        }
        None => None,
    }
}

fn find_min(tree: &Option<SearchTree>) -> Option<SearchTree> {
    match tree {
        Some(tree) => {
            let tree = tree.borrow();
            if let Some(left) = &tree.left {
                find_min(&Some(left.clone()))
            } else {
                Some(tree.clone())
            }
        }
        None => None,
    }
}

fn find_max(tree: &Option<SearchTree>) -> Option<SearchTree> {
    match tree {
        Some(tree) => {
            let tree = tree.borrow();
            if let Some(right) = &tree.right {
                find_max(&Some(right.clone()))
            } else {
                Some(tree.clone())
            }
        }
        None => None,
    }
}

fn insert(x: i32, tree: &mut Option<SearchTree>) {
    if let Some(tree) = tree {
        let mut tree = tree.borrow_mut();
        match x.cmp(&tree.element) {
            Ordering::Less => {
                if tree.left.is_none() {
                    tree.left = Some(Rc::new(RefCell::new(TreeNode::new(x))));
                } else {
                    insert(x, &mut tree.left);
                }
            }
            Ordering::Greater => {
                if tree.right.is_none() {
                    tree.right = Some(Rc::new(RefCell::new(TreeNode::new(x))));
                } else {
                    insert(x, &mut tree.right);
                }
            }
            Ordering::Equal => (),
        }
    } else {
        *tree = Some(Rc::new(RefCell::new(TreeNode::new(x))));
    }
}

fn delete(x: i32, tree: &mut Option<SearchTree>) {
    if let Some(tree) = tree {
        let mut tree = tree.borrow_mut();
        match x.cmp(&tree.element) {
            Ordering::Less => delete(x, &mut tree.left),
            Ordering::Greater => delete(x, &mut tree.right),
            Ordering::Equal => {
                if let Some(left) = tree.left.clone() {
                    if let Some(right) = tree.right.clone() {
                        let min = find_min(&Some(right)).unwrap();
                        tree.element = min.borrow().element;
                        delete(min.borrow().element, &mut Some(right));
                    } else {
                        *tree = tree.left.take();
                    }
                } else if let Some(right) = tree.right.clone() {
                    *tree = tree.right.take();
                } else {
                    *tree = None;
                }
            }
        }
    }
}

fn retrieve(tree: &Option<SearchTree>) -> Option<i32> {
    tree.as_ref().map(|tree| tree.borrow().element)
}

fn main() {
    let mut tree: Option<SearchTree> = None;
    for i in 0..50 {
        insert(i, &mut tree);
    }
    for i in 0..50 {
        if let Some(tree) = find(i, &tree) {
            assert_eq!(retrieve(&Some(tree)), Some(i));
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
            if let Some(tree) = find(i, &tree) {
                assert_eq!(retrieve(&Some(tree)), Some(i));
            } else {
                panic!("Error at {}", i);
            }
        } else {
            if find(i, &tree).is_some() {
                panic!("Error at {}", i);
            }
        }
    }
    println!("Min is {}, Max is {}", retrieve(find_min(&tree)).unwrap(), retrieve(find_max(&tree)).unwrap());
}