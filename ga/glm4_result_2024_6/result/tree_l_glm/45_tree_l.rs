use std::io::{self, BufRead};
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            value,
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

fn find_min(tree: &Option<Box<TreeNode>>) -> Option<i32> {
    match tree {
        Some(node) => {
            if let Some(min) = find_min(&node.left) {
                Some(min)
            } else {
                Some(node.value)
            }
        }
        None => None,
    }
}

fn find_max(tree: &Option<Box<TreeNode>>) -> Option<i32> {
    match tree {
        Some(node) => {
            if let Some(max) = find_max(&node.right) {
                Some(max)
            } else {
                Some(node.value)
            }
        }
        None => None,
    }
}

fn insert(value: i32, tree: &mut Option<Box<TreeNode>>) {
    match tree {
        Some(node) => {
            if value < node.value {
                insert(value, &mut node.left);
            } else {
                insert(value, &mut node.right);
            }
        }
        None => {
            *tree = Some(Box::new(TreeNode::new(value)));
        }
    }
}

fn delete(value: i32, tree: &mut Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        if value < node.value {
            delete(value, &mut node.left);
        } else if value > node.value {
            delete(value, &mut node.right);
        } else {
            if node.left.is_none() {
                *tree = node.right.take();
            } else if node.right.is_none() {
                *tree = node.left.take();
            } else {
                if let Some(min) = find_min(&node.right) {
                    node.value = min;
                    delete(min, &mut node.right);
                }
            }
        }
    }
}

fn print_in_order(tree: &Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        print_in_order(&node.left);
        println!("{}", node.value);
        print_in_order(&node.right);
    }
}

fn delete_even_numbers(tree: &mut Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        delete_even_numbers(&mut node.left);
        delete_even_numbers(&mut node.right);
        if node.value % 2 == 0 {
            delete(node.value, tree);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().and_then(|line| line.ok());
    let line = line.unwrap_or_else(|| "".to_string());

    let mut tree: Option<Box<TreeNode>> = None;
    for token in line.split_whitespace() {
        if let Ok(value) = token.parse::<i32>() {
            insert(value, &mut tree);
        }
    }

    println!("Tree after insertion:");
    print_in_order(&tree);

    delete_even_numbers(&mut tree);

    println!("Tree after deleting even numbers:");
    print_in_order(&tree);

    match (find_min(&tree), find_max(&tree)) {
        (Some(min), Some(max)) => println!("Min is {}, Max is {}", min, max),
        _ => println!("Tree is empty"),
    }

    make_empty(&mut tree);
}