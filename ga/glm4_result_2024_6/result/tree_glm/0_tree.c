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
    fn new(element: i32) -> SearchTree {
        Rc::new(RefCell::new(TreeNode {
            element,
            left: None,
            right: None,
        }))
    }

    fn make_empty(&mut self) {
        self.left.take();
        self.right.take();
    }
}

fn find_min(tree: &SearchTree) -> Option<SearchTree> {
    let mut current = tree.clone();
    while let Some(left) = current.borrow_mut().left.take() {
        current = left;
    }
    Some(current)
}

fn find_max(tree: &SearchTree) -> Option<SearchTree> {
    let mut current = tree.clone();
    while let Some(right) = current.borrow_mut().right.take() {
        current = right;
    }
    Some(current)
}

fn find(x: i32, tree: &SearchTree) -> Option<SearchTree> {
    let mut current = tree.clone();
    while let Some(node) = current.as_ref() {
        if x < node.borrow().element {
            current = node.borrow_mut().left.take();
        } else if x > node.borrow().element {
            current = node.borrow_mut().right.take();
        } else {
            return Some(current);
        }
    }
    None
}

fn insert(x: i32, tree: &mut SearchTree) {
    if let Some(mut node) = tree.as_mut() {
        if x < node.borrow().element {
            if node.borrow_mut().left.is_none() {
                node.borrow_mut().left = Some(TreeNode::new(x));
            } else {
                insert(x, &mut node.borrow_mut().left);
            }
        } else if x > node.borrow().element {
            if node.borrow_mut().right.is_none() {
                node.borrow_mut().right = Some(TreeNode::new(x));
            } else {
                insert(x, &mut node.borrow_mut().right);
            }
        }
    }
}

fn delete(x: i32, tree: &mut SearchTree) {
    if let Some(mut node) = tree.as_mut() {
        if x < node.borrow().element {
            delete(x, &mut node.borrow_mut().left);
        } else if x > node.borrow().element {
            delete(x, &mut node.borrow_mut().right);
        } else {
            if let Some(left) = node.borrow_mut().left.take() {
                if let Some(right) = node.borrow_mut().right.take() {
                    if let Some(min) = find_min(&right) {
                        node.borrow_mut().element = min.borrow().element;
                        node.borrow_mut().right = Some(right);
                        delete(min.borrow().element, &mut node.borrow_mut().right);
                    } else {
                        node.borrow_mut().left = Some(left);
                    }
                } else {
                    node.borrow_mut().left = Some(left);
                }
            } else if let Some(right) = node.borrow_mut().right.take() {
                node.borrow_mut().right = Some(right);
            } else {
                *tree = None;
            }
        }
    }
}

fn retrieve(tree: &SearchTree) -> Option<i32> {
    tree.as_ref().map(|node| node.borrow().element)
}

fn main() {
    let mut tree: Option<SearchTree> = None;
    for i in 0..50 {
        insert(i, &mut tree);
    }
    for i in 0..50 {
        if let Some(node) = find(i, &tree) {
            assert_eq!(retrieve(&node), Some(i));
        } else {
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
            if let Some(node) = find(i, &tree) {
                assert_eq!(retrieve(&node), Some(i));
            } else {
                println!("Error at {}", i);
            }
        } else {
            if let Some(node) = find(i, &tree) {
                println!("Error at {}", i);
            }
        }
    }
    if let Some(min) = find_min(&tree) {
        println!("Min is {}", retrieve(&min).unwrap());
    }
    if let Some(max) = find_max(&tree) {
        println!("Max is {}", retrieve(&max).unwrap());
    }
}