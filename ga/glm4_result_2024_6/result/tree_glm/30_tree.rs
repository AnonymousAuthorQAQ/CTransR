use std::cmp::Ordering;
use std::ptr::NonNull;

type SearchTree = Option<Box<TreeNode>>;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: SearchTree,
    right: SearchTree,
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

fn make_empty(tree: &mut SearchTree) {
    if let Some(node) = tree {
        make_empty(&mut node.left);
        make_empty(&mut node.right);
        unsafe {
            Box::from_raw(node);
        }
    }
    *tree = None;
}

fn find(x: i32, tree: &SearchTree) -> Option<NonNull<TreeNode>> {
    let mut current = tree.as_ref()?;
    loop {
        match x.cmp(&current.element) {
            Ordering::Less => current = current.left.as_ref(),
            Ordering::Greater => current = current.right.as_ref(),
            Ordering::Equal => return Some(NonNull::from(current)),
        }
    }
}

fn find_min(tree: &SearchTree) -> Option<NonNull<TreeNode>> {
    let mut current = tree.as_ref()?;
    while let Some(left) = current.left.as_ref() {
        current = left;
    }
    Some(NonNull::from(current))
}

fn find_max(tree: &SearchTree) -> Option<NonNull<TreeNode>> {
    let mut current = tree.as_ref()?;
    while let Some(right) = current.right.as_ref() {
        current = right;
    }
    Some(NonNull::from(current))
}

fn insert(x: i32, tree: &mut SearchTree) {
    if let None = tree {
        *tree = Some(Box::new(TreeNode::new(x)));
    } else {
        let mut current = tree.as_mut().unwrap();
        loop {
            match x.cmp(&current.element) {
                Ordering::Less => {
                    if current.left.is_none() {
                        current.left = Some(Box::new(TreeNode::new(x)));
                        return;
                    }
                    current = current.left.as_mut().unwrap();
                }
                Ordering::Greater => {
                    if current.right.is_none() {
                        current.right = Some(Box::new(TreeNode::new(x)));
                        return;
                    }
                    current = current.right.as_mut().unwrap();
                }
                Ordering::Equal => return,
            }
        }
    }
}

fn delete(x: i32, tree: &mut SearchTree) {
    if let Some(mut node) = tree.take() {
        match x.cmp(&node.element) {
            Ordering::Less => {
                node.left = delete(x, &mut node.left);
            }
            Ordering::Greater => {
                node.right = delete(x, &mut node.right);
            }
            Ordering::Equal => {
                if let Some(left) = node.left.take() {
                    if let Some(right) = node.right.take() {
                        let mut min = find_min(&right).unwrap();
                        node.element = min.as_ref().element;
                        node.right = Some(min.into());
                        node.left = left;
                    } else {
                        node.left = left;
                    }
                } else {
                    node.right = delete(x, &mut node.right);
                }
            }
        }
        *tree = Some(node);
    }
}

fn retrieve(p: NonNull<TreeNode>) -> i32 {
    p.as_ref().element
}

fn main() {
    let mut tree: SearchTree = None;
    for i in 0..50 {
        insert(i, &mut tree);
    }
    for i in 0..50 {
        if let None = find(i, &tree) {
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
            if let None = find(i, &tree) {
                println!("Error at {}", i);
            }
        } else {
            if let Some(_) = find(i, &tree) {
                println!("Error at {}", i);
            }
        }
    }
    if let Some(min) = find_min(&tree) {
        println!("Min is {}", retrieve(min));
    }
    if let Some(max) = find_max(&tree) {
        println!("Max is {}", retrieve(max));
    }
}