use std::io::{self, Write};
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

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
        Some(node) => {
            if x < node.element {
                let mut new_node = node;
                new_node.left = insert(x, new_node.left);
                Some(new_node)
            } else if x > node.element {
                let mut new_node = node;
                new_node.right = insert(x, new_node.right);
                Some(new_node)
            } else {
                Some(node) // Do not allow duplicates
            }
        }
    }
}

fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    match t {
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

fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => {
            eprintln!("Error: Element not found");
            None
        }
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left);
                Some(node)
            } else if x > node.element {
                node.right = delete(x, node.right);
                Some(node)
            } else {
                // Node with the element found
                if node.left.is_some() && node.right.is_some() {
                    let min_node = find_min(&node.right);
                    node.element = min_node.unwrap().element;
                    node.right = delete(node.element, node.right);
                    Some(node)
                } else {
                    // Only one child or no child
                    let child = if node.left.is_none() { node.right } else { node.left };
                    return child; // Promote the child, if any, or None
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
                    left: left,
                    right: right,
                })))
            } else {
                Some(Box::new(TreeNode {
                    element: node.element,
                    left: left,
                    right: right,
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
    let mut t = make_empty();
    let mut input_line = String::new();

    print!("Enter numbers (space-separated): ");
    io::stdout().flush().unwrap(); // Make sure the prompt is displayed before input

    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let numbers: Vec<i32> = input_line
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    for &num in &numbers {
        t = insert(num, t);
    }

    if t.is_none() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    print_in_order(&t);

    // Delete all even numbers
    t = delete_even_numbers(t);
    println!("Tree after deleting even numbers:");
    print_in_order(&t);

    if let Some(min_node) = find_min(&t) {
        if let Some(max_node) = find_max(&t) {
            println!("Min is {}, Max is {}", min_node.element, max_node.element);
        } else {
            println!("Tree is empty after deletions");
        }
    } else {
        println!("Tree is empty after deletions");
    }
}