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
    fn new(x: i32) -> Box<TreeNode> {
        Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })
    }
}

fn make_empty(tree: &mut SearchTree) {
    if let Some(node) = tree {
        make_empty(&mut node.left);
        make_empty(&mut node.right);
        *tree = None;
    }
}

fn find(x: i32, tree: &SearchTree) -> Option<NonNull<TreeNode>> {
    tree.as_ref().and_then(|node| {
        match x.cmp(&node.element) {
            Ordering::Less => find(x, &node.left),
            Ordering::Greater => find(x, &node.right),
            Ordering::Equal => Some(NonNull::from(node)),
        }
    })
}

fn find_min(tree: &SearchTree) -> Option<NonNull<TreeNode>> {
    tree.as_ref().and_then(|node| {
        node.left.as_ref().map_or_else(
            || Some(NonNull::from(node)),
            |left| find_min(&left),
        )
    })
}

fn find_max(tree: &SearchTree) -> Option<NonNull<TreeNode>> {
    tree.as_ref().and_then(|node| {
        node.right.as_ref().map_or_else(
            || Some(NonNull::from(node)),
            |right| find_max(&right),
        )
    })
}

fn insert(x: i32, tree: &mut SearchTree) {
    match tree {
        None => *tree = Some(TreeNode::new(x)),
        Some(node) => match x.cmp(&node.element) {
            Ordering::Less => insert(x, &mut node.left),
            Ordering::Greater => insert(x, &mut node.right),
            Ordering::Equal => {},
        },
    }
}

fn delete(x: i32, tree: &mut SearchTree) {
    if let Some(mut node) = tree.take() {
        match x.cmp(&node.element) {
            Ordering::Less => {
                *tree = Some(node);
                delete(x, &mut node.left);
            },
            Ordering::Greater => {
                *tree = Some(node);
                delete(x, &mut node.right);
            },
            Ordering::Equal => {
                if let Some(mut right) = node.right.take() {
                    *tree = Some(right);
                    if let Some(mut left) = node.left.take() {
                        insert(left.element, &mut right.left);
                    }
                } else {
                    *tree = node.left;
                }
            },
        }
    }
}

fn retrieve(p: NonNull<TreeNode>) -> i32 {
    p.as_ref().element
}

fn main() {
    let mut t: SearchTree = None;
    for i in 0..50 {
        insert(i, &mut t);
    }
    for i in 0..50 {
        if let Some(p) = find(i, &t) {
            assert_eq!(retrieve(p), i);
        } else {
            panic!("Error at {}", i);
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
                assert_eq!(retrieve(p), i);
            } else {
                panic!("Error at {}", i);
            }
        } else {
            if find(i, &t).is_some() {
                panic!("Error at {}", i);
            }
        }
    }
    if let Some(min) = find_min(&t) {
        println!("Min is {}, Max is {}", retrieve(min), retrieve(find_max(&t).unwrap()));
    }
}