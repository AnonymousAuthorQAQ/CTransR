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

fn make_empty(tree: &SearchTree) {
    if let Some(left) = tree.borrow_mut().left.take() {
        make_empty(&left);
    }
    if let Some(right) = tree.borrow_mut().right.take() {
        make_empty(&right);
    }
}

fn find(x: i32, tree: &SearchTree) -> Option<SearchTree> {
    if let Some(node) = tree.borrow() {
        if x < node.element {
            find(x, &node.left)
        } else if x > node.element {
            find(x, &node.right)
        } else {
            Some(tree.clone())
        }
    } else {
        None
    }
}

fn find_min(tree: &SearchTree) -> Option<SearchTree> {
    if let Some(node) = tree.borrow() {
        if let Some(left) = &node.left {
            find_min(left)
        } else {
            Some(tree.clone())
        }
    } else {
        None
    }
}

fn find_max(tree: &SearchTree) -> Option<SearchTree> {
    if let Some(node) = tree.borrow() {
        if let Some(right) = &node.right {
            find_max(right)
        } else {
            Some(tree.clone())
        }
    } else {
        None
    }
}

fn insert(x: i32, tree: &SearchTree) -> SearchTree {
    if let Some(node) = tree.borrow_mut() {
        if x < node.element {
            node.left = Some(insert(x, &node.left.clone().unwrap_or_else(|| Rc::new(RefCell::new(TreeNode::new(0)))));
        } else if x > node.element {
            node.right = Some(insert(x, &node.right.clone().unwrap_or_else(|| Rc::new(RefCell::new(TreeNode::new(0)))));
        }
    }
    tree.clone()
}

fn delete(x: i32, tree: &SearchTree) -> SearchTree {
    if let Some(node) = tree.borrow_mut() {
        if x < node.element {
            node.left = Some(delete(x, &node.left.clone().unwrap()));
        } else if x > node.element {
            node.right = Some(delete(x, &node.right.clone().unwrap()));
        } else {
            if let Some(left) = node.left.clone() {
                if let Some(right) = node.right.clone() {
                    let min = find_min(&right).unwrap();
                    node.element = min.borrow().element;
                    node.right = Some(delete(min.borrow().element, &right));
                } else {
                    return left;
                }
            } else if let Some(right) = node.right.clone() {
                return right;
            } else {
                return Rc::new(RefCell::new(TreeNode::new(0)));
            }
        }
    }
    tree.clone()
}

fn retrieve(tree: &SearchTree) -> i32 {
    tree.borrow().element
}

fn main() {
    let mut tree: SearchTree = Rc::new(RefCell::new(TreeNode::new(0)));
    for i in 0..50 {
        tree = insert(i, &tree);
    }
    for i in 0..50 {
        if find(i, &tree).is_none() || retrieve(&find(i, &tree).unwrap()) != i {
            println!("Error at {}", i);
        }
    }
    for i in 0..50 {
        if i % 2 == 0 {
            tree = delete(i, &tree);
        }
    }
    for i in 1..50 {
        if i % 2 == 1 {
            if find(i, &tree).is_none() || retrieve(&find(i, &tree).unwrap()) != i {
                println!("Error at {}", i);
            }
        } else {
            if find(i, &tree).is_some() {
                println!("Error at {}", i);
            }
        }
    }
    println!("Min is {}, Max is {}", retrieve(&find_min(&tree).unwrap()), retrieve(&find_max(&tree).unwrap()));
}