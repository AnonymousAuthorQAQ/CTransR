use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

type ElementType = i32;
type SearchTree = Rc<RefCell<TreeNode>>;

#[derive(Clone)]
struct TreeNode {
    element: ElementType,
    left: Option<SearchTree>,
    right: Option<SearchTree>,
}

impl TreeNode {
    fn new(element: ElementType) -> Self {
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

fn find(x: ElementType, tree: &SearchTree) -> Option<SearchTree> {
    if let Some(ref node) = tree.borrow().left {
        if x < node.borrow().element {
            return find(x, node);
        }
    }
    if let Some(ref node) = tree.borrow().right {
        if x > node.borrow().element {
            return find(x, node);
        }
    }
    if tree.borrow().element == x {
        return Some(tree.clone());
    }
    None
}

fn find_min(tree: &SearchTree) -> Option<SearchTree> {
    let mut node = tree.clone();
    while let Some(ref left) = node.borrow().left {
        node = left.clone();
    }
    Some(node)
}

fn find_max(tree: &SearchTree) -> Option<SearchTree> {
    let mut node = tree.clone();
    while let Some(ref right) = node.borrow().right {
        node = right.clone();
    }
    Some(node)
}

fn insert(x: ElementType, tree: &mut Option<SearchTree>) {
    if let None = *tree {
        *tree = Some(Rc::new(RefCell::new(TreeNode::new(x))));
    } else {
        let mut node = tree.as_ref().unwrap().clone();
        loop {
            if x < node.borrow().element {
                if let None = node.borrow().left {
                    node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(x))));
                    break;
                } else {
                    node = node.borrow().left.as_ref().unwrap().clone();
                }
            } else {
                if let None = node.borrow().right {
                    node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(x))));
                    break;
                } else {
                    node = node.borrow().right.as_ref().unwrap().clone();
                }
            }
        }
    }
}

fn delete(x: ElementType, tree: &mut Option<SearchTree>) {
    if let Some(mut node) = tree.take() {
        if node.borrow().element == x {
            if let None = node.borrow().left {
                *tree = node.borrow().right.clone();
            } else if let None = node.borrow().right {
                *tree = node.borrow().left.clone();
            } else {
                let mut right = node.borrow().right.clone();
                *tree = node.borrow().left.clone();
                insert(right.borrow().element, tree);
            }
        } else if let Some(ref mut left) = node.borrow_mut().left {
            if left.borrow().element == x {
                *tree = Some(left.clone());
                node.borrow_mut().left = left.borrow().left.clone();
            } else {
                delete(x, &mut node.borrow_mut().left);
            }
        } else if let Some(ref mut right) = node.borrow_mut().right {
            if right.borrow().element == x {
                *tree = Some(right.clone());
                node.borrow_mut().right = right.borrow().right.clone();
            } else {
                delete(x, &mut node.borrow_mut().right);
            }
        }
        *tree = Some(node);
    }
}

fn retrieve(node: &SearchTree) -> ElementType {
    node.borrow().element
}

fn main() {
    let mut tree: Option<SearchTree> = None;
    for i in 0..50 {
        insert(i, &mut tree);
    }
    for i in 0..50 {
        if let None = find(i, tree.as_ref().unwrap()) {
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
            if let None = find(i, tree.as_ref().unwrap()) {
                println!("Error at {}", i);
            }
        } else {
            if let Some(node) = find(i, tree.as_ref().unwrap()) {
                println!("Error at {}", i);
            }
        }
    }
    if let Some(min) = find_min(tree.as_ref().unwrap()) {
        println!("Min is {}", retrieve(&min));
    }
    if let Some(max) = find_max(tree.as_ref().unwrap()) {
        println!("Max is {}", retrieve(&max));
    }
}