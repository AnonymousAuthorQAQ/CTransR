use std::io::{self, BufRead};
use std::ptr;

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

fn find(x: i32, tree: &SearchTree) -> Option<&TreeNode> {
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

fn find_min(tree: &SearchTree) -> Option<&TreeNode> {
    match tree {
        Some(node) => {
            if node.left.is_none() {
                Some(node)
            } else {
                find_min(&node.left)
            }
        }
        None => None,
    }
}

fn find_max(tree: &SearchTree) -> Option<&TreeNode> {
    let mut current = tree;
    while let Some(node) = current {
        current = &node.right;
    }
    current.map(|node| node.as_ref())
}

fn insert(x: i32, tree: SearchTree) -> SearchTree {
    match tree {
        Some(mut node) => {
            if x < node.element {
                node.left = insert(x, node.left);
                Some(node)
            } else if x > node.element {
                node.right = insert(x, node.right);
                Some(node)
            } else {
                Some(node) // No duplicates allowed, return unchanged
            }
        }
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
    }
}

fn delete(x: i32, tree: SearchTree) -> SearchTree {
    match tree {
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left);
                Some(node)
            } else if x > node.element {
                node.right = delete(x, node.right);
                Some(node)
            } else {
                // Found the node to delete
                match (node.left, node.right) {
                    (None, None) => None, // No children
                    (Some(left), None) => Some(left), // Only left child
                    (None, Some(right)) => Some(right), // Only right child
                    (Some(left), Some(right)) => {
                        let min_node = find_min(&node.right).unwrap();
                        node.element = min_node.element; // Copy min element
                        node.right = delete(min_node.element, node.right);
                        Some(node)
                    }
                }
            }
        }
        None => {
            eprintln!("Element not found");
            tree // Return unchanged if not found
        }
    }
}

fn delete_even_numbers(tree: SearchTree) -> SearchTree {
    match tree {
        Some(mut node) => {
            node.left = delete_even_numbers(node.left);
            node.right = delete_even_numbers(node.right);

            if node.element % 2 == 0 {
                delete(node.element, Some(node))
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn print_in_order(tree: &SearchTree) {
    if let Some(node) = tree {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut tree = make_empty();

    println!("Enter space-separated integers (end with Enter):");
    let mut input = String::new();
    handle.read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    for &num in &numbers {
        tree = insert(num, tree);
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

    if let Some(min_node) = find_min(&tree) {
        if let Some(max_node) = find_max(&tree) {
            println!("Min is {}, Max is {}", min_node.element, max_node.element);
        }
    } else {
        println!("Tree is empty");
    }
}