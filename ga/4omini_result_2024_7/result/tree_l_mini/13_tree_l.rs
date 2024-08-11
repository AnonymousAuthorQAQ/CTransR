use std::collections::VecDeque;
use std::io::{self, BufRead};

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

fn insert(x: i32, tree: SearchTree) -> SearchTree {
    match tree {
        None => Some(Box::new(TreeNode { element: x, left: None, right: None })),
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

fn delete_even_numbers(tree: SearchTree) -> SearchTree {
    match tree {
        None => None,
        Some(node) => {
            let left = delete_even_numbers(node.left);
            let right = delete_even_numbers(node.right);

            if node.element % 2 == 0 {
                return merge_trees(left, right);
            }
            Some(Box::new(TreeNode {
                element: node.element,
                left,
                right,
            }))
        }
    }
}

fn merge_trees(left: SearchTree, right: SearchTree) -> SearchTree {
    // Helper function to merge two subtrees
    match (left, right) {
        (None, None) => None,
        (Some(node), None) => Some(node),
        (None, Some(node)) => Some(node),
        (Some(left_node), Some(right_node)) => {
            let mut merged = left_node;
            merged.right = Some(right_node);
            Some(merged)
        }
    }
}

fn print_in_order(tree: &SearchTree) {
    if let Some(ref node) = tree {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn find_min(tree: &SearchTree) -> Option<i32> {
    let mut current = tree;
    while let Some(ref node) = current {
        current = &node.left;
    }
    current.map(|node| node.element)
}

fn find_max(tree: &SearchTree) -> Option<i32> {
    let mut current = tree;
    while let Some(ref node) = current {
        current = &node.right;
    }
    current.map(|node| node.element)
}

fn main() {
    let mut tree = make_empty();
    
    // Read input line from stdin
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Read a line of space-separated integers
    let mut has_input = false;
    
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                tree = insert(num, tree);
                has_input = true;
            }
        }
        break;  // Only read the first line
    }

    if !has_input || tree.is_none() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    print_in_order(&tree);

    // Delete all even numbers
    tree = delete_even_numbers(tree);

    println!("Tree after deleting even numbers:");
    print_in_order(&tree);

    // Find and print Min and Max
    if let Some(min) = find_min(&tree) {
        if let Some(max) = find_max(&tree) {
            println!("Min is {}, Max is {}", min, max);
        }
    } else {
        println!("Tree is empty");
    }
}