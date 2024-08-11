use std::rc::Rc;
use std::cell::RefCell;

type SearchTree = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: SearchTree,
    right: SearchTree,
}

impl TreeNode {
    fn new(element: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            element,
            left: None,
            right: None,
        }))
    }
}

fn make_empty(tree: &mut SearchTree) {
    if let Some(node) = tree.take() {
        make_empty(&mut node.borrow_mut().left);
        make_empty(&mut node.borrow_mut().right);
    }
}

fn find(x: i32, tree: &SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    *tree.as_ref()?
        .borrow()
        .left
        .as_ref()
        .and_then(|left| find(x, left))
        .or_else(|| {
            if x < tree.as_ref().unwrap().borrow().element {
                None
            } else {
                *tree.as_ref()?
                    .borrow()
                    .right
                    .as_ref()
                    .and_then(|right| find(x, right))
            }
        })
}

fn find_min(tree: &SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    *tree.as_ref()
        .and_then(|tree| {
            if let Some(left) = tree.borrow().left.as_ref() {
                find_min(left)
            } else {
                Some(tree.clone())
            }
        })
}

fn find_max(tree: &SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    *tree.as_ref()
        .and_then(|tree| {
            if let Some(right) = tree.borrow().right.as_ref() {
                find_max(right)
            } else {
                Some(tree.clone())
            }
        })
}

fn insert(x: i32, tree: &mut SearchTree) {
    if let Some(node) = tree.as_mut() {
        if x < node.borrow().element {
            insert(x, &mut node.borrow_mut().left);
        } else {
            insert(x, &mut node.borrow_mut().right);
        }
    } else {
        *tree = Some(TreeNode::new(x));
    }
}

fn delete(x: i32, tree: &mut SearchTree) {
    if let Some(node) = tree.take() {
        let mut node = node.borrow_mut();
        if x < node.element {
            delete(x, &mut node.left);
            *tree = Some(node);
        } else if x > node.element {
            delete(x, &mut node.right);
            *tree = Some(node);
        } else if let Some(right) = node.right.take() {
            if let Some(min) = find_min(&right) {
                node.element = min.borrow().element;
                delete(node.element, &mut node.right);
            } else {
                node.left = right.borrow_mut().left.take();
                node.right = right.borrow_mut().right.take();
            }
            *tree = Some(node);
        } else {
            *tree = node.left.take();
        }
    }
}

fn retrieve(p: &Rc<RefCell<TreeNode>>) -> i32 {
    p.borrow().element
}

fn main() {
    let mut tree: SearchTree = None;
    for i in 0..50 {
        insert(i, &mut tree);
    }
    for i in 0..50 {
        let p = find(i, &tree);
        if p.is_none() || retrieve(&p.unwrap()) != i {
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
            let p = find(i, &tree);
            if p.is_none() || retrieve(&p.unwrap()) != i {
                println!("Error at {}", i);
            }
        }
    }
    for i in 0..50 {
        if i % 2 == 0 {
            if let Some(p) = find(i, &tree) {
                println!("Error at {}", i);
            }
        }
    }
    println!("Min is {}, Max is {}", retrieve(&find_min(&tree).unwrap()), retrieve(&find_max(&tree).unwrap()));
}