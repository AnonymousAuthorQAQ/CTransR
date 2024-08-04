use std::cmp::Ordering;
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
    None
}

fn find(x: i32, tree: &SearchTree) -> Option<&TreeNode> {
    tree.as_ref().and_then(|node| match x.cmp(&node.element) {
        Ordering::Less => find(x, &node.left),
        Ordering::Greater => find(x, &node.right),
        Ordering::Equal => Some(node),
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
        if let Some(right) = &node.right {
            find_max(&Some(right.clone()))
        } else {
            Some(node)
        }
    })
}

fn insert(x: i32, tree: SearchTree) -> SearchTree {
    match tree {
        None => Some(TreeNode::new(x)),
        Some(node) => match x.cmp(&node.element) {
            Ordering::Less => {
                node.left = insert(x, node.left);
                Some(node)
            }
            Ordering::Greater => {
                node.right = insert(x, node.right);
                Some(node)
            }
            Ordering::Equal => Some(node),
        },
    }
}

fn delete(x: i32, tree: SearchTree) -> SearchTree {
    if let Some(mut node) = tree {
        match x.cmp(&node.element) {
            Ordering::Less => {
                node.left = delete(x, node.left);
                Some(node)
            }
            Ordering::Greater => {
                node.right = delete(x, node.right);
                Some(node)
            }
            Ordering::Equal => {
                if let Some(left) = node.left.take() {
                    if let Some(right) = node.right.take() {
                        let mut min_node = find_min(&Some(right)).unwrap();
                        if let Some(mut right) = min_node.right.take() {
                            min_node.right = Some(right);
                        }
                        min_node.left = left;
                        Some(min_node)
                    } else {
                        Some(left)
                    }
                } else {
                    node.right
                }
            }
        }
    } else {
        None
    }
}

fn retrieve(p: Option<&TreeNode>) -> Option<i32> {
    p.map(|node| node.element)
}

fn main() {
    let mut t: SearchTree = None;
    for i in 0..50 {
        t = insert(i, t);
    }

    for i in 0..50 {
        if let Some(p) = find(i, &t) {
            assert_eq!(retrieve(Some(p)), Some(i));
        } else {
            println!("Error at {}", i);
        }
    }

    for i in 0..50 {
        if i % 2 == 0 {
            t = delete(i, t);
        }
    }

    for i in 1..50 {
        if i % 2 == 1 {
            if let Some(p) = find(i, &t) {
                assert_eq!(retrieve(Some(p)), Some(i));
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
        println!("Min is {}, Max is {}", retrieve(Some(min)), retrieve(find_max(&t).unwrap()));
    }
}