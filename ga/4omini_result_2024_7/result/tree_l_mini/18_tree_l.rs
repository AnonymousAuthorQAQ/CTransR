use std::io::{self, BufRead};
use std::collections::VecDeque;

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

fn find(x: i32, t: &SearchTree) -> Option<&TreeNode> {
    match t {
        None => None,
        Some(node) => {
            if x < node.element {
                find(x, &node.left)
            } else if x > node.element {
                find(x, &node.right)
            } else {
                Some(node)
            }
        }
    }
}

fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    match t {
        None => None,
        Some(node) => {
            if node.left.is_none() {
                Some(node)
            } else {
                find_min(&node.left)
            }
        }
    }
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut current = t;
    while let Some(node) = current {
        if node.right.is_none() {
            return Some(node);
        }
        current = &node.right;
    }
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

fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => {
            println!("Element not found");
            None
        }
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left);
            } else if x > node.element {
                node.right = delete(x, node.right);
            } else {
                if node.left.is_some() && node.right.is_some() {
                    if let Some(min_node) = find_min(&node.right) {
                        node.element = min_node.element;
                        node.right = delete(min_node.element, node.right);
                    }
                } else {
                    return if node.left.is_none() {
                        node.right
                    } else {
                        node.left
                    };
                }
            }
            Some(node)
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

fn delete_even_numbers(t: SearchTree) -> SearchTree {
    match t {
        None => None,
        Some(mut node) => {
            node.left = delete_even_numbers(node.left);
            node.right = delete_even_numbers(node.right);

            if node.element % 2 == 0 {
                return delete(node.element, Some(node));
            }
            Some(node)
        }
    }
}

fn main() {
    let mut tree = make_empty();
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line.unwrap();
        let tokens: Vec<_> = line.split_whitespace().collect();
        for token in tokens {
            if let Ok(num) = token.parse::<i32>() {
                tree = insert(num, tree);
            }
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

    match (find_min(&tree), find_max(&tree)) {
        (Some(min), Some(max)) => {
            println!("Min is {}, Max is {}", min.element, max.element);
        }
        _ => {
            println!("Tree is empty");
        }
    }

    // Memory is automatically managed via Rust's ownership model
}