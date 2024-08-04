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

fn insert(x: i32, tree: &SearchTree) -> SearchTree {
    if let Some(mut node) = tree.as_ref().and_then(|node| node.borrow_mut().left.take()) {
        if node.borrow().element < x {
            node.borrow_mut().right = Some(insert(x, &node));
        } else {
            node.borrow_mut().left = Some(insert(x, &node));
        }
    } else {
        let mut node = TreeNode::new(x);
        node.left = tree.take();
        tree.replace(node);
    }
    tree.clone()
}

fn delete(x: i32, tree: &SearchTree) -> SearchTree {
    if let Some(mut node) = tree.as_ref().and_then(|node| node.borrow_mut().left.take()) {
        if node.borrow().element < x {
            node.borrow_mut().left = delete(x, &node);
        } else if node.borrow().element > x {
            node.borrow_mut().right = delete(x, &node);
        } else {
            if let Some(right) = node.borrow_mut().right.take() {
                if let Some(min) = find_min(&right) {
                    node.borrow_mut().element = min.borrow().element;
                    node.borrow_mut().right = delete(min.borrow().element, &right);
                } else {
                    node.borrow_mut().element = right.borrow().element;
                    node.borrow_mut().right = right.borrow_mut().right.take();
                }
            } else {
                return node.borrow_mut().left.take();
            }
        }
        Some(node)
    } else {
        None
    }
}

fn main() {
    let mut tree: Option<SearchTree> = None;
    for i in 0..50 {
        tree = Some(insert(i, &tree.unwrap()));
    }
    for i in 0..50 {
        if find(i, &tree.unwrap()).is_none() {
            println!("Error at {}", i);
        }
    }
    for i in 0..50 {
        if i % 2 == 0 {
            tree = Some(delete(i, &tree.unwrap()));
        }
    }
    for i in 1..50 {
        if i % 2 == 1 {
            if find(i, &tree.unwrap()).is_none() {
                println!("Error at {}", i);
            }
        } else {
            if find(i, &tree.unwrap()).is_some() {
                println!("Error at {}", i);
            }
        }
    }
    println!("Min is {}, Max is {}", find_min(&tree.unwrap()).unwrap().borrow().element, find_max(&tree.unwrap()).unwrap().borrow().element);
}