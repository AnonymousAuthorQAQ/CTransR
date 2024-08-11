use std::io::{self, BufRead};

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

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(tree: SearchTree) -> SearchTree {
    if let Some(node) = tree {
        make_empty(node.left);
        make_empty(node.right);
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
        if let Some(right) = &node.right {
            find_max(&Some(right.clone()))
        } else {
            Some(node)
        }
    })
}

fn insert(x: i32, tree: SearchTree) -> SearchTree {
    if let Some(node) = tree {
        if x < node.element {
            node.left = insert(x, node.left);
        } else if x > node.element {
            node.right = insert(x, node.right);
        }
        Some(node)
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
        } else if let Some(right) = node.right.take() {
            if let Some(min) = find_min(&Some(right)) {
                node.element = min.element;
                node.right = delete(min.element, Some(right));
            }
        } else if let Some(left) = node.left.take() {
            Some(left)
        } else {
            None
        }
    }
    tree
}

fn retrieve(p: &TreeNode) -> i32 {
    p.element
}

fn print_in_order(tree: &SearchTree) {
    if let Some(node) = tree {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn delete_even_numbers(tree: SearchTree) -> SearchTree {
    if let Some(mut node) = tree {
        node.left = delete_even_numbers(node.left);
        node.right = delete_even_numbers(node.right);
        if node.element % 2 == 0 {
            None
        } else {
            Some(node)
        }
    }
    tree
}

fn main() {
    let stdin = io::stdin();
    let mut tree: SearchTree = None;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                tree = insert(num, tree);
            }
        }
    }

    if tree.is_none() {
        println!("Tree is empty");
    } else {
        println!("Tree after insertion:");
        print_in_order(&tree);
        tree = delete_even_numbers(tree);
        println!("Tree after deleting even numbers:");
        print_in_order(&tree);
        if let Some(node) = find_min(&tree) {
            println!("Min is {}, Max is {}", retrieve(node), retrieve(find_max(&tree).unwrap()));
        }
    }
}