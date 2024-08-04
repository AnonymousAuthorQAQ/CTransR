use std::cmp::Ord;
use std::io::{self, Write};

// Define a type for the elements in the tree
type ElementType = i32;

// Define a node structure for the binary search tree
struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

// Type alias for the search tree
type SearchTree = Option<Box<TreeNode>>;

// Function to create an empty tree
fn make_empty() -> SearchTree {
    None
}

// Function to find an element in the tree
fn find(x: ElementType, t: &SearchTree) -> Option<&TreeNode> {
    match t {
        None => None,
        Some(ref node) => {
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

// Function to find the minimum element in the tree
fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    match t {
        None => None,
        Some(ref node) => {
            if node.left.is_none() {
                Some(node)
            } else {
                find_min(&node.left)
            }
        }
    }
}

// Function to find the maximum element in the tree
fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut current = t;
    while let Some(ref node) = current {
        if node.right.is_none() {
            return Some(node);
        }
        current = &node.right;
    }
    None
}

// Function to insert an element into the tree
fn insert(x: ElementType, t: SearchTree) -> SearchTree {
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
            Some(node) // Return the unchanged node
        }
    }
}

// Function to delete an element from the tree
fn delete(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => {
            println!("Error: Element not found");
            None
        }
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left);
            } else if x > node.element {
                node.right = delete(x, node.right);
            } else { // Found the node to delete
                if node.left.is_some() && node.right.is_some() {
                    let min_node = find_min(&node.right).unwrap();
                    node.element = min_node.element;
                    node.right = delete(min_node.element, node.right);
                } else {
                    // Node with one child or no child
                    if node.left.is_none() {
                        node = node.right;
                    } else {
                        node = node.left;
                    }
                }
            }
            Some(node) // Return the updated tree
        }
    }
}

// Function to retrieve an element from a node
fn retrieve(node: &TreeNode) -> ElementType {
    node.element
}

fn main() {
    let mut t: SearchTree = make_empty();
    let mut j = 0;

    for i in 0..50 {
        j = (j + 7) % 50;
        t = insert(j, t);
    }

    for i in 0..50 {
        if let Some(p) = find(i, &t) {
            if retrieve(p) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        if let Some(p) = find(i, &t) {
            if retrieve(p) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if find(i, &t).is_some() {
            println!("Error at {}", i);
        }
    }

    if let (Some(min_node), Some(max_node)) = (find_min(&t), find_max(&t)) {
        println!("Min is {}, Max is {}", retrieve(min_node), retrieve(max_node));
    } else {
        println!("The tree is empty!");
    }
}