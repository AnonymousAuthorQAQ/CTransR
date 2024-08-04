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
        let mut tree = tree.borrow_mut();
        tree.left = make_empty(tree.left.clone());
        tree.right = make_empty(tree.right.clone());
        if tree.left.is_none() && tree.right.is_none() {
            None
        } else {
            Some(tree)
        }
    } else {
        None
    }
}

fn find(x: i32, tree: &Option<SearchTree>) -> Option<SearchTree> {
    if let Some(tree) = tree {
        let tree = tree.borrow();
        if x < tree.element {
            find(x, &tree.left)
        } else if x > tree.element {
            find(x, &tree.right)
        } else {
            Some(tree.clone())
        }
    } else {
        None
    }
}

fn find_min(tree: &Option<SearchTree>) -> Option<SearchTree> {
    if let Some(tree) = tree {
        let tree = tree.borrow();
        if let Some(left) = tree.left.clone() {
            find_min(&left)
        } else {
            Some(tree.clone())
        }
    } else {
        None
    }
}

fn find_max(tree: &Option<SearchTree>) -> Option<SearchTree> {
    if let Some(tree) = tree {
        let mut tree = tree.borrow_mut();
        while let Some(right) = tree.right.clone() {
            tree = right.borrow_mut();
        }
        Some(tree.clone())
    } else {
        None
    }
}

fn insert(x: i32, tree: &mut Option<SearchTree>) {
    if let Some(tree) = tree {
        let mut tree = tree.borrow_mut();
        if x < tree.element {
            insert(x, &mut tree.left);
        } else if x > tree.element {
            insert(x, &mut tree.right);
        }
    } else {
        *tree = Some(Rc::new(RefCell::new(TreeNode::new(x))));
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
                    let mut right = right.borrow_mut();
                    *tree = left.borrow().clone();
                    insert(right.element, &mut tree.borrow_mut().right);
                } else {
                    *tree = left;
                }
            } else if let Some(right) = tree.right.clone() {
                *tree = right;
            } else {
                *tree = None;
            }
        }
    }
}

fn retrieve(tree: &Option<SearchTree>) -> Option<i32> {
    if let Some(tree) = tree {
        Some(tree.borrow().element)
    } else {
        None
    }
}

fn main() {
    let mut tree: Option<SearchTree> = None;
    for i in 0..50 {
        insert(i, &mut tree);
    }
    for i in 0..50 {
        if retrieve(find(i, &tree)) != Some(i) {
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
            if retrieve(find(i, &tree)) != Some(i) {
                println!("Error at {}", i);
            }
        } else if find(i, &tree).is_some() {
            println!("Error at {}", i);
        }
    }
    println!("Min is {}, Max is {}", retrieve(find_min(&tree)).unwrap(), retrieve(find_max(&tree)).unwrap());
}