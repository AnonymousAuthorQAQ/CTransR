use std::io::{self, Write};
use std::collections::VecDeque;

// Define the TreeNode structure
#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,  // Use Option for potentially missing child nodes
    right: Option<Box<TreeNode>>,
}

// Define the SearchTree type as an Option of Box<TreeNode>
type SearchTree = Option<Box<TreeNode>>;

// Create an empty tree
fn make_empty() -> SearchTree {
    None
}

// Insert an element into the tree
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
            Some(node)  // return the node (boxed)
        }
    }
}

// In-order print
fn print_in_order(t: &SearchTree) {
    if let Some(node) = t {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

// Find the minimum element
fn find_min(t: &SearchTree) -> Option<i32> {
    let mut current = t;
    while let Some(ref node) = current {
        if node.left.is_none() {
            return Some(node.element);
        }
        current = &node.left;
    }
    None
}

// Find the maximum element
fn find_max(t: &SearchTree) -> Option<i32> {
    let mut current = t;
    while let Some(ref node) = current {
        if node.right.is_none() {
            return Some(node.element);
        }
        current = &node.right;
    }
    None
}

// Delete an element from the tree
fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => None,  // Element not found
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left);
                Some(node)
            } else if x > node.element {
                node.right = delete(x, node.right);
                Some(node)
            } else {
                // Node to delete found
                if node.left.is_none() {
                    return node.right;  // Replace node with right child
                } else if node.right.is_none() {
                    return node.left;   // Replace node with left child
                } else {
                    // Node has two children, find the minimum from the right subtree
                    if let Some(min_right) = find_min(&node.right) {
                        node.element = min_right;
                        node.right = delete(min_right, node.right);
                    }
                    Some(node)
                }
            }
        }
    }
}

// Delete all even numbers from the tree
fn delete_even_numbers(t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        node.left = delete_even_numbers(node.left);
        node.right = delete_even_numbers(node.right);

        if node.element % 2 == 0 {
            return delete(node.element, Some(node));  // Remove the even node
        }
        Some(node)  // return the current node
    } else {
        None
    }
}

fn main() {
    let mut input = String::new();
    print!("Enter numbers separated by spaces: ");
    io::stdout().flush().unwrap();  // Ensure prompt is printed

    // Read input
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Initialize the tree
    let mut tree: SearchTree = make_empty();

    // Parse and insert numbers into the tree
    for token in input.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            tree = insert(num, tree);
        }
    }

    // Print the tree after insertion
    println!("Tree after insertion:");
    print_in_order(&tree);

    // Delete all even numbers
    tree = delete_even_numbers(tree);

    // Print the tree after deleting even numbers
    println!("Tree after deleting even numbers:");
    print_in_order(&tree);

    // Display min and max
    match find_min(&tree) {
        Some(min) => match find_max(&tree) {
            Some(max) => println!("Min is {}, Max is {}", min, max),
            None => println!("Tree is empty after deletion"),
        },
        None => println!("Tree is empty after deletion"),
    }
}