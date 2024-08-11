use std::io::{self, BufRead};
use std::ptr::null_mut;

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

fn insert(value: i32, tree: SearchTree) -> SearchTree {
    match tree {
        Some(mut node) => {
            if value < node.element {
                node.left = insert(value, node.left);
                Some(node)
            } else if value > node.element {
                node.right = insert(value, node.right);
                Some(node)
            } else {
                Some(node)  // Value already in the tree, no duplicates
            }
        }
        None => Some(Box::new(TreeNode {
            element: value,
            left: None,
            right: None,
        })),
    }
}

fn delete(value: i32, tree: SearchTree) -> SearchTree {
    match tree {
        Some(mut node) => {
            if value < node.element {
                node.left = delete(value, node.left);
                Some(node)
            } else if value > node.element {
                node.right = delete(value, node.right);
                Some(node)
            } else {
                // Node to be deleted has been found
                if node.left.is_none() {
                    return node.right; // Replace with right child
                } else if node.right.is_none() {
                    return node.left; // Replace with left child
                }
                // Node with two children: get the inorder successor (smallest in the right subtree)
                let min_node = find_min(node.right.as_ref().unwrap());
                node.element = min_node.element; // Copy the inorder successor's value
                node.right = delete(min_node.element, node.right); // Delete the inorder successor
                Some(node)
            }
        }
        None => {
            eprintln!("Error: Element not found");
            None
        }
    }
}

fn find_min(tree: &TreeNode) -> &TreeNode {
    if let Some(ref left) = tree.left {
        find_min(left)
    } else {
        tree
    }
}

fn find_max(tree: &TreeNode) -> &TreeNode {
    let mut current = tree;
    while let Some(ref right) = current.right {
        current = right;
    }
    current
}

fn print_in_order(tree: &SearchTree) {
    if let Some(ref node) = tree {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn delete_even_numbers(tree: SearchTree) -> SearchTree {
    match tree {
        Some(mut node) => {
            node.left = delete_even_numbers(node.left);
            node.right = delete_even_numbers(node.right);
            if node.element % 2 == 0 {
                // Delete this node if it's even
                delete(node.element, Some(node))
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn main() {
    let mut tree = make_empty();
    println!("Enter integers separated by spaces:");
    
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let numbers: Vec<i32> = line.split_whitespace()
                                  .filter_map(|s| s.parse().ok())
                                  .collect();

    for num in numbers {
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

    // Find min and max if the tree is not empty
    if let Some(ref tree) = tree {
        println!("Min is {}, Max is {}", find_min(tree).element, find_max(tree).element);
    } else {
        println!("Tree is empty");
    }
}