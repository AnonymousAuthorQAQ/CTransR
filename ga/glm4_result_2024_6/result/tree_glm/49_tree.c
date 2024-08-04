use std::rc::Rc;
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
    match tree {
        None => None,
        Some(node) => {
            node.borrow_mut().left = make_empty(node.borrow().left.clone());
            node.borrow_mut().right = make_empty(node.borrow().right.clone());
            Rc::try_unwrap(node).ok()
        }
    }
}

fn find(x: i32, tree: &Option<SearchTree>) -> Option<Rc<RefCell<TreeNode>>> {
    match tree {
        None => None,
        Some(node) => {
            if x < node.borrow().element {
                find(x, &node.borrow().left)
            } else if x > node.borrow().element {
                find(x, &node.borrow().right)
            } else {
                Some(node.clone())
            }
        }
    }
}

fn find_min(tree: &Option<SearchTree>) -> Option<Rc<RefCell<TreeNode>>> {
    match tree {
        None => None,
        Some(node) => {
            if node.borrow().left.is_none() {
                Some(node.clone())
            } else {
                find_min(&node.borrow().left)
            }
        }
    }
}

fn find_max(tree: &Option<SearchTree>) -> Option<Rc<RefCell<TreeNode>>> {
    match tree {
        None => None,
        Some(node) => {
            let mut current = node.clone();
            while let Some(right) = current.borrow().right.clone() {
                current = right;
            }
            Some(current)
        }
    }
}

fn insert(x: i32, tree: Option<SearchTree>) -> Option<SearchTree> {
    match tree {
        None => Some(Rc::new(RefCell::new(TreeNode::new(x)))),
        Some(node) => {
            if x < node.borrow().element {
                node.borrow_mut().left = insert(x, node.borrow().left.clone());
            } else if x > node.borrow().element {
                node.borrow_mut().right = insert(x, node.borrow().right.clone());
            }
            Some(node.clone())
        }
    }
}

fn delete(x: i32, tree: Option<SearchTree>) -> Option<SearchTree> {
    match tree {
        None => None,
        Some(node) => {
            if x < node.borrow().element {
                node.borrow_mut().left = delete(x, node.borrow().left.clone());
            } else if x > node.borrow().element {
                node.borrow_mut().right = delete(x, node.borrow().right.clone());
            } else {
                if node.borrow().left.is_none() {
                    return node.borrow().right.clone();
                } else if node.borrow().right.is_none() {
                    return node.borrow().left.clone();
                }
                let min_right = find_min(&node.borrow().right).unwrap();
                node.borrow_mut().element = min_right.borrow().element;
                node.borrow_mut().right = delete(min_right.borrow().element, node.borrow().right.clone());
            }
            Some(node.clone())
        }
    }
}

fn retrieve(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    node.as_ref().unwrap().borrow().element
}

fn main() {
    let mut tree: Option<SearchTree> = None;
    for i in 0..50 {
        tree = insert(i, tree);
    }
    for i in 0..50 {
        let node = find(i, &tree);
        assert!(node.is_some() && retrieve(&node) == i);
    }
    for i in 0..50 {
        if i % 2 == 0 {
            tree = delete(i, tree);
        }
    }
    for i in 1..50 {
        if i % 2 == 1 {
            let node = find(i, &tree);
            assert!(node.is_some() && retrieve(&node) == i);
        }
    }
    for i in 0..50 {
        if i % 2 == 0 {
            let node = find(i, &tree);
            assert!(node.is_none());
        }
    }
    let min = retrieve(&find_min(&tree).unwrap());
    let max = retrieve(&find_max(&tree).unwrap());
    println!("Min is {}, Max is {}", min, max);
}