use std::io::{self, BufRead};
use std::collections::binary_tree::Node;
use std::collections::binary_tree::Node::*;

fn main() {
    let mut tree = None;
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let tokens: Vec<&str> = line.split_whitespace().collect();
    for token in tokens {
        if let Ok(num) = token.parse::<i32>() {
            tree = Some(insert(num, tree));
        }
    }

    println!("Tree after insertion:");
    print_in_order(&tree);

    tree = delete_even_numbers(tree);

    println!("Tree after deleting even numbers:");
    print_in_order(&tree);

    if let Some(node) = tree {
        println!("Min is {}, Max is {}", find_min(&node).unwrap(), find_max(&node).unwrap());
    } else {
        println!("Tree is empty");
    }
}

fn insert(x: i32, tree: Option<Node<i32>>) -> Option<Node<i32>> {
    match tree {
        None => Some(Branch(x)),
        Some(Branch(value)) => {
            if x < value {
                Some(Branch(value, insert(x, value.left), value.right))
            } else {
                Some(Branch(value, value.left, insert(x, value.right)))
            }
        }
    }
}

fn delete(x: i32, tree: Option<Node<i32>>) -> Option<Node<i32>> {
    match tree {
        None => None,
        Some(Branch(value)) => {
            if x < value {
                Some(Branch(value, delete(x, value.left), value.right))
            } else if x > value {
                Some(Branch(value, value.left, delete(x, value.right)))
            } else if let Some(left) = value.left {
                if let Some(right) = value.right {
                    Some(Branch(find_min(right).unwrap(), left, delete(find_min(right).unwrap(), right)))
                } else {
                    Some(left)
                }
            } else {
                value.right
            }
        }
    }
}

fn find(x: i32, tree: &Option<Node<i32>>) -> Option<&Node<i32>> {
    match tree {
        None => None,
        Some(Branch(value)) => {
            if x < value {
                find(x, &value.left)
            } else if x > value {
                find(x, &value.right)
            } else {
                Some(tree)
            }
        }
    }
}

fn find_min(tree: &Option<Node<i32>>) -> Option<i32> {
    match tree {
        None => None,
        Some(Branch(value)) => {
            if let None = value.left {
                Some(value)
            } else {
                find_min(&value.left)
            }
        }
    }
}

fn find_max(tree: &Option<Node<i32>>) -> Option<i32> {
    match tree {
        None => None,
        Some(Branch(value)) => {
            if let None = value.right {
                Some(value)
            } else {
                find_max(&value.right)
            }
        }
    }
}

fn delete_even_numbers(tree: Option<Node<i32>>) -> Option<Node<i32>> {
    match tree {
        None => None,
        Some(Branch(value)) => {
            let left = delete_even_numbers(value.left);
            let right = delete_even_numbers(value.right);
            if value % 2 == 0 {
                None
            } else {
                Some(Branch(value, left, right))
            }
        }
    }
}

fn print_in_order(tree: &Option<Node<i32>>) {
    match tree {
        None => (),
        Some(Branch(value)) => {
            print_in_order(&value.left);
            println!("{}", value);
            print_in_order(&value.right);
        }
    }
}