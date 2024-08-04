use std::io::{self, Read};
use std::str::FromStr;

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

fn make_empty(tree: &mut Option<Box<TreeNode>>) {
    if let Some(mut node) = tree.take() {
        make_empty(&mut node.left);
        make_empty(&mut node.right);
    }
}

fn find(x: i32, tree: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
    match tree {
        Some(node) => {
            if x < node.element {
                find(x, &node.left)
            } else if x > node.element {
                find(x, &node.right)
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_min(tree: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
    match tree {
        Some(node) => {
            if let Some(left) = &node.left {
                find_min(&Some(left.clone()))
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_max(tree: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
    let mut node = tree;
    while let Some(current) = node {
        if let Some(right) = &current.right {
            node = Some(right.clone());
        } else {
            return Some(current);
        }
    }
    None
}

fn insert(x: i32, tree: &mut Option<Box<TreeNode>>) {
    if let Some(mut node) = tree.take() {
        if x < node.element {
            insert(x, &mut node.left);
        } else if x > node.element {
            insert(x, &mut node.right);
        }
        *tree = Some(node);
    } else {
        *tree = Some(Box::new(TreeNode::new(x)));
    }
}

fn delete(x: i32, tree: &mut Option<Box<TreeNode>>) {
    if let Some(mut node) = tree.take() {
        if x < node.element {
            delete(x, &mut node.left);
        } else if x > node.element {
            delete(x, &mut node.right);
        } else if let (Some(left), Some(right)) = (node.left.take(), node.right.take()) {
            let min_node = find_min(&right).unwrap();
            node.element = min_node.element;
            insert(min_node.element, &mut node.right);
            node.left = left;
            node.right = right;
        } else {
            if node.left.is_none() {
                *tree = node.right;
            } else {
                *tree = node.left;
            }
        }
    }
}

fn retrieve(p: &TreeNode) -> i32 {
    p.element
}

fn print_in_order(tree: &Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn delete_even_numbers(tree: &mut Option<Box<TreeNode>>) {
    if let Some(mut node) = tree.take() {
        delete_even_numbers(&mut node.left);
        delete_even_numbers(&mut node.right);
        if node.element % 2 == 0 {
            delete(node.element, tree);
        } else {
            *tree = Some(node);
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut tree: Option<Box<TreeNode>> = None;
    for token in input.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            insert(num, &mut tree);
        }
    }

    if let Some(tree) = &tree {
        println!("Tree after insertion:");
        print_in_order(&Some(tree.clone()));
    } else {
        println!("Tree is empty");
    }

    delete_even_numbers(&mut tree);

    if let Some(tree) = &tree {
        println!("Tree after deleting even numbers:");
        print_in_order(&Some(tree.clone()));
    } else {
        println!("Tree is empty");
    }

    if let Some(tree) = &tree {
        if let (Some(min), Some(max)) = (find_min(&Some(tree.clone())), find_max(&Some(tree.clone()))) {
            println!("Min is {}, Max is {}", retrieve(min), retrieve(max));
        }
    }
}