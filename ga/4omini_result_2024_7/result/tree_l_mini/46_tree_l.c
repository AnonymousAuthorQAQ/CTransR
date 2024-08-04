use std::io::{self, BufRead};
use std::cmp::Ord;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty() -> SearchTree {
    None
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
        Some(mut node) => {
            if x < node.element {
                node.left = insert(x, node.left);
            } else if x > node.element {
                node.right = insert(x, node.right);
            }
            Some(node)
        }
    }
}

fn find_min(t: &SearchTree) -> Option<i32> {
    match t {
        None => None,
        Some(node) => {
            let mut current = node;
            while let Some(ref left_child) = current.left {
                current = left_child;
            }
            Some(current.element)
        }
    }
}

fn find_max(t: &SearchTree) -> Option<i32> {
    match t {
        None => None,
        Some(node) => {
            let mut current = node;
            while let Some(ref right_child) = current.right {
                current = right_child;
            }
            Some(current.element)
        }
    }
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => {
            eprintln!("Element not found");
            None
        }
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left);
            } else if x > node.element {
                node.right = delete(x, node.right);
            } else if node.left.is_some() && node.right.is_some() {
                node.element = find_min(&node.right).unwrap();
                node.right = delete(node.element, node.right);
            } else {
                return if node.left.is_none() {
                    node.right
                } else {
                    node.left
                };
            }
            Some(node)
        }
    }
}

fn delete_even_numbers(t: SearchTree) -> SearchTree {
    match t {
        None => None,
        Some(node) => {
            let left = delete_even_numbers(node.left);
            let right = delete_even_numbers(node.right);
            if node.element % 2 == 0 {
                delete(node.element, Some(node)).unwrap_or_else(|| left.or(right))
            } else {
                Some(Box::new(TreeNode {
                    element: node.element,
                    left,
                    right,
                }))
            }
        }
    }
}

fn print_in_order(t: &SearchTree) {
    if let Some(node) = t {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    let mut tree: SearchTree = make_empty();

    for token in input.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            tree = insert(num, tree);
        }
    }

    if tree.is_none() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    print_in_order(&tree);

    // Delete all even numbers
    tree = delete_even_numbers(tree);

    println!("Tree after deleting even numbers:");
    print_in_order(&tree);

    if let Some(min) = find_min(&tree) {
        if let Some(max) = find_max(&tree) {
            println!("Min is {}, Max is {}", min, max);
        }
    } else {
        println!("Tree is empty");
    }
}