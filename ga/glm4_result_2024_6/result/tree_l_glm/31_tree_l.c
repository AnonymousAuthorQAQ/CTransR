use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

#[derive(Debug)]
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

fn make_empty_tree(tree: &mut Option<Box<TreeNode>>) {
    if let Some(mut node) = tree.take() {
        make_empty_tree(&mut node.left);
        make_empty_tree(&mut node.right);
    }
}

fn find_min(tree: &Option<Box<TreeNode>>) -> Option<i32> {
    match tree {
        Some(node) => {
            if let Some(min) = find_min(&node.left) {
                return Some(min);
            }
            if node.left.is_none() {
                return Some(node.value);
            }
            find_min(&node.right)
        }
        None => None,
    }
}

fn find_max(tree: &Option<Box<TreeNode>>) -> Option<i32> {
    match tree {
        Some(node) => {
            if let Some(max) = find_max(&node.right) {
                return Some(max);
            }
            if node.right.is_none() {
                return Some(node.value);
            }
            find_min(&node.left)
        }
        None => None,
    }
}

fn insert(value: i32, tree: &mut Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        if value < node.value {
            insert(value, &mut node.left);
        } else if value > node.value {
            insert(value, &mut node.right);
        }
    } else {
        *tree = Some(Box::new(TreeNode::new(value)));
    }
}

fn delete(value: i32, tree: &mut Option<Box<TreeNode>>) {
    if let Some(mut node) = tree.take() {
        if value < node.value {
            delete(value, &mut node.left);
            *tree = Some(node);
        } else if value > node.value {
            delete(value, &mut node.right);
            *tree = Some(node);
        } else {
            if let Some(mut right) = node.right.take() {
                if let Some(mut left) = node.left.take() {
                    if let Some(min) = find_min(&right) {
                        node.value = min;
                        node.right = Some(right);
                        node.left = Some(left);
                        *tree = Some(node);
                    } else {
                        node.left = Some(left);
                        node.right = Some(right);
                        *tree = Some(node);
                    }
                } else {
                    *tree = Some(node);
                }
            } else {
                *tree = node.left;
            }
        }
    }
}

fn retrieve(tree: &Option<Box<TreeNode>>) -> Option<i32> {
    tree.as_ref().map(|node| node.value)
}

fn print_in_order(tree: &Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        print_in_order(&node.left);
        println!("{}", node.value);
        print_in_order(&node.right);
    }
}

fn delete_even_numbers(tree: &mut Option<Box<TreeNode>>) {
    if let Some(mut node) = tree.take() {
        delete_even_numbers(&mut node.left);
        delete_even_numbers(&mut node.right);
        if node.value % 2 == 0 {
            delete(node.value, tree);
        } else {
            *tree = Some(node);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().and_then(|result| result.ok());
    let mut tree: Option<Box<TreeNode>> = None;

    if let Some(line) = line {
        for token in line.split_whitespace() {
            if let Ok(value) = token.parse::<i32>() {
                insert(value, &mut tree);
            }
        }
    }

    print_in_order(&tree);
    delete_even_numbers(&mut tree);
    print_in_order(&tree);

    if let Some(min) = find_min(&tree) {
        println!("Min is {}", min);
    }
    if let Some(max) = find_max(&tree) {
        println!("Max is {}", max);
    }

    make_empty_tree(&mut tree);
}