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
            if x < tree.element {
                find(x, &tree.left)
            } else if x > tree.element {
                find(x, &tree.right)
            } else {
                Some(tree.clone())
            }
        }
        None => None,
    }
}

fn find_min(tree: &Option<SearchTree>) -> Option<SearchTree> {
    match tree {
        Some(tree) => {
            let tree = tree.borrow();
            if tree.left.is_none() {
                Some(tree.clone())
            } else {
                find_min(&tree.left)
            }
        }
        None => None,
    }
}

fn find_max(tree: &Option<SearchTree>) -> Option<SearchTree> {
    match tree {
        Some(tree) => {
            let mut tree = tree.borrow();
            while let Some(right) = tree.right.clone() {
                tree = right.borrow();
            }
            Some(tree.clone())
        }
        None => None,
    }
}

fn insert(x: i32, tree: &mut Option<SearchTree>) {
    if tree.is_none() {
        *tree = Some(Rc::new(RefCell::new(TreeNode::new(x))));
    } else {
        let tree = tree.as_ref().unwrap().borrow_mut();
        if x < tree.element {
            insert(x, &mut tree.left);
        } else if x > tree.element {
            insert(x, &mut tree.right);
        }
    }
}

fn delete(x: i32, tree: &mut Option<SearchTree>) {
    if let Some(tree) = tree {
        let mut tree = tree.borrow_mut();
        if x < tree.element {
            delete(x, &mut tree.left);
        } else if x > tree.element {
            delete(x, &mut tree.right);
        } else {
            if let Some(left) = tree.left.clone() {
                if let Some(right) = tree.right.clone() {
                    let min = find_min(&right).unwrap();
                    tree.element = min.borrow().element;
                    delete(min.borrow().element, &mut right);
                } else {
                    *tree = left.borrow().clone();
                }
            } else if let Some(right) = tree.right.clone() {
                *tree = right.borrow().clone();
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
    let mut t: Option<SearchTree> = None;
    for i in 0..50 {
        insert(i, &mut t);
    }
    for i in 0..50 {
        if let Some(p) = find(i, &t) {
            if retrieve(&p) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }
    for i in 0..50 {
        if i % 2 == 0 {
            delete(i, &mut t);
        }
    }
    for i in 1..50 {
        if i % 2 == 1 {
            if let Some(p) = find(i, &t) {
                if retrieve(&p) != i {
                    println!("Error at {}", i);
                }
            } else {
                println!("Error at {}", i);
            }
        } else {
            if let Some(p) = find(i, &t) {
                println!("Error at {}", i);
            }
        }
    }
    if let Some(min) = find_min(&t) {
        println!("Min is {}", retrieve(&min));
    }
    if let Some(max) = find_max(&t) {
        println!("Max is {}", retrieve(&max));
    }
}