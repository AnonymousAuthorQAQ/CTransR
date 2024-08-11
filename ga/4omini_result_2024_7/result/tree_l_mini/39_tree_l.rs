use std::io::{self, BufRead};
use std::iter::FromIterator;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    None
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
        Some(node) => {
            if x < node.element {
                let left = insert(x, node.left);
                Some(Box::new(TreeNode {
                    element: node.element,
                    left,
                    right: node.right,
                }))
            } else if x > node.element {
                let right = insert(x, node.right);
                Some(Box::new(TreeNode {
                    element: node.element,
                    left: node.left,
                    right,
                }))
            } else {
                Some(node) // Duplicate values are ignored
            }
        }
    }
}

fn find_min(t: &SearchTree) -> Option<i32> {
    match t {
        Some(node) => {
            let mut current = node;
            while let Some(ref left) = current.left {
                current = left;
            }
            Some(current.element)
        }
        None => None,
    }
}

fn find_max(t: &SearchTree) -> Option<i32> {
    let mut current = t;
    while let Some(ref node) = current {
        current = &node.right;
    }
    current.as_ref().map(|node| node.element)
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => None, // Element not found
        Some(node) => {
            if x < node.element {
                let left = delete(x, node.left);
                Some(Box::new(TreeNode {
                    element: node.element,
                    left,
                    right: node.right,
                }))
            } else if x > node.element {
                let right = delete(x, node.right);
                Some(Box::new(TreeNode {
                    element: node.element,
                    left: node.left,
                    right,
                }))
            } else if node.left.is_some() && node.right.is_some() {
                let min_val = find_min(&node.right);
                match min_val {
                    Some(min) => {
                        let right = delete(min, node.right);
                        Some(Box::new(TreeNode {
                            element: min,
                            left: node.left,
                            right,
                        }))
                    }
                    None => None,
                }
            } else {
                if node.left.is_none() {
                    node.right // Only right child
                } else {
                    node.left // Only left child
                }
            }
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
                delete(node.element, Some(Box::new(TreeNode {
                    element: node.element,
                    left,
                    right,
                })))
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
    let handle = stdin.lock();

    let mut tree: SearchTree = None;

    for line in handle.lines() {
        let line = line.unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|token| token.parse().ok())
            .collect();

        for num in nums {
            tree = insert(num, tree);
        }
    }

    // Print tree after insertion
    println!("Tree after insertion:");
    print_in_order(&tree);

    // Delete all even numbers
    tree = delete_even_numbers(tree);

    // Print tree after deleting even numbers
    println!("Tree after deleting even numbers:");
    print_in_order(&tree);

    // Print min and max
    if let Some(min) = find_min(&tree) {
        if let Some(max) = find_max(&tree) {
            println!("Min is {}, Max is {}", min, max);
        }
    } else {
        println!("Tree is empty");
    }
}