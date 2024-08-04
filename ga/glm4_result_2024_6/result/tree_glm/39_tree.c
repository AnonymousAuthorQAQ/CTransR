use std::ptr::NonNull;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(element: i32) -> Box<TreeNode> {
        Box::new(TreeNode {
            element,
            left: None,
            right: None,
        })
    }
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(tree: SearchTree) -> SearchTree {
    if let Some(node) = tree {
        let mut left = node.left;
        let mut right = node.right;
        left = make_empty(left);
        right = make_empty(right);
        Box::leak(node);
    }
    None
}

fn find(x: i32, tree: &SearchTree) -> Option<&TreeNode> {
    tree.as_ref().and_then(|node| {
        if x < node.element {
            find(x, &node.left)
        } else if x > node.element {
            find(x, &node.right)
        } else {
            Some(node)
        }
    })
}

fn find_min(tree: &SearchTree) -> Option<&TreeNode> {
    tree.as_ref().and_then(|node| {
        if let Some(left) = &node.left {
            find_min(&Some(left.clone()))
        } else {
            Some(node)
        }
    })
}

fn find_max(tree: &SearchTree) -> Option<&TreeNode> {
    tree.as_ref().and_then(|node| {
        let mut current = node;
        while let Some(right) = &current.right {
            current = right;
        }
        Some(current)
    })
}

fn insert(x: i32, tree: &mut SearchTree) {
    if let Some(node) = tree {
        if x < node.element {
            insert(x, &mut node.left);
        } else if x > node.element {
            insert(x, &mut node.right);
        }
    } else {
        *tree = Some(TreeNode::new(x));
    }
}

fn delete(x: i32, tree: &mut SearchTree) {
    if let Some(node) = tree {
        if x < node.element {
            delete(x, &mut node.left);
        } else if x > node.element {
            delete(x, &mut node.right);
        } else if let Some(left) = node.left.take() {
            if let Some(right) = node.right.take() {
                *tree = Some(TreeNode {
                    element: find_min(&Some(right)).unwrap().element,
                    left: Some(left),
                    right: Some(right),
                });
            } else {
                *tree = Some(left);
            }
        } else if let Some(right) = node.right.take() {
            *tree = Some(right);
        } else {
            *tree = None;
        }
    }
}

fn retrieve(p: &TreeNode) -> i32 {
    p.element
}

fn main() {
    let mut t: SearchTree = None;
    for i in 0..50 {
        insert(i, &mut t);
    }
    for i in 0..50 {
        if let Some(node) = find(i, &t) {
            assert_eq!(retrieve(node), i);
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
            if let Some(node) = find(i, &t) {
                assert_eq!(retrieve(node), i);
            } else {
                println!("Error at {}", i);
            }
        } else {
            if find(i, &t).is_some() {
                println!("Error at {}", i);
            }
        }
    }
    if let Some(min) = find_min(&t) {
        println!("Min is {}, Max is {}", retrieve(min), retrieve(find_max(&t).unwrap()));
    }
}