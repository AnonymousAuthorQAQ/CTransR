use std::io::Write;

#[macro_export]
macro_rules! fatal_error {
    ($($arg:tt)*) => ({
        eprintln!($($arg)*);
        std::process::exit(1);
    })
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        eprintln!($($arg)*);
    })
}

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
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

impl Drop for TreeNode {
    fn drop(&mut self) {
        if let Some(left) = self.left.take() {
            drop(left);
        }
        if let Some(right) = self.right.take() {
            drop(right);
        }
    }
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(tree: SearchTree) -> SearchTree {
    if let Some(mut node) = tree {
        node.left = make_empty(node.left);
        node.right = make_empty(node.right);
    }
    None
}

fn find(x: i32, tree: &SearchTree) -> Option<&TreeNode> {
    if let Some(node) = tree {
        if x < node.element {
            find(x, &node.left)
        } else if x > node.element {
            find(x, &node.right)
        } else {
            Some(node)
        }
    } else {
        None
    }
}

fn find_min(tree: &SearchTree) -> Option<&TreeNode> {
    if let Some(node) = tree {
        if node.left.is_none() {
            Some(node)
        } else {
            find_min(&node.left)
        }
    } else {
        None
    }
}

fn find_max(tree: &SearchTree) -> Option<&TreeNode> {
    let mut node = tree;
    while let Some(current) = node {
        if current.right.is_none() {
            return Some(current);
        }
        node = &current.right;
    }
    None
}

fn insert(x: i32, tree: SearchTree) -> SearchTree {
    if let Some(mut node) = tree {
        if x < node.element {
            node.left = insert(x, node.left);
        } else if x > node.element {
            node.right = insert(x, node.right);
        }
    } else {
        Some(Box::new(TreeNode::new(x)))
    }
}

fn delete(x: i32, tree: SearchTree) -> SearchTree {
    if let Some(mut node) = tree {
        if x < node.element {
            node.left = delete(x, node.left);
        } else if x > node.element {
            node.right = delete(x, node.right);
        } else {
            if let Some(left) = node.left {
                if let Some(right) = node.right {
                    let mut min_node = right;
                    while min_node.left.is_some() {
                        min_node = min_node.left.as_ref().unwrap();
                    }
                    node.element = min_node.element;
                    node.right = delete(min_node.element, node.right);
                } else {
                    return node.left;
                }
            } else if let Some(right) = node.right {
                return node.right;
            } else {
                return None;
            }
        }
        Some(node)
    } else {
        error!("Element not found");
        None
    }
}

fn retrieve(position: &TreeNode) -> i32 {
    position.element
}

fn main() {
    let mut tree: SearchTree = None;
    let mut i = 0;
    let mut j = 0;

    while i < 50 {
        j = (j + 7) % 50;
        tree = insert(j, tree);
        i += 1;
    }

    for i in 0..50 {
        if let None = find(i, &tree) {
            fatal_error!("Error at {}", i);
        }
    }

    for i in 0..50 {
        if i % 2 == 0 {
            tree = delete(i, tree);
        }
    }

    for i in 1..50 {
        if i % 2 == 1 {
            if let None = find(i, &tree) {
                fatal_error!("Error at {}", i);
            }
        } else if let Some(_) = find(i, &tree) {
            fatal_error!("Error at {}", i);
        }
    }

    if let Some(min) = find_min(&tree) {
        println!("Min is {}, ", min.element);
    }

    if let Some(max) = find_max(&tree) {
        println!("Max is {}", max.element);
    }
}