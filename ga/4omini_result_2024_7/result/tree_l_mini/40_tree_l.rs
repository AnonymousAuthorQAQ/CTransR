use std::io::{self, BufRead};
use std::ptr;
use std::boxed::Box;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty<T>(tree: T) -> SearchTree {
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
        if node.right.is_none() {
            return Some(node);
        }
        current = &node.right;
    }
    None
}

fn insert(x: i32, tree: SearchTree) -> SearchTree {
    match tree {
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
        Some(mut node) if x < node.element => {
            node.left = insert(x, node.left);
            Some(node)
        }
        Some(mut node) if x > node.element => {
            node.right = insert(x, node.right);
            Some(node)
        }
        _ => tree,
    }
}

fn delete(x: i32, tree: SearchTree) -> SearchTree {
    match tree {
        None => {
            println!("Element not found");
            None
        }
        Some(mut node) if x < node.element => {
            node.left = delete(x, node.left);
            Some(node)
        }
        Some(mut node) if x > node.element => {
            node.right = delete(x, node.right);
            Some(node)
        }
        Some(mut node) => {
            if node.left.is_some() && node.right.is_some() {
                let min_node = find_min(&node.right).unwrap();
                node.element = min_node.element;
                node.right = delete(node.element, node.right);
                Some(node)
            } else if node.left.is_none() {
                Some(node.right.take().unwrap())
            } else {
                Some(node.left.take().unwrap())
            }
        }
    }
}

fn delete_even_numbers(tree: SearchTree) -> SearchTree {
    match tree {
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

fn print_in_order(tree: &SearchTree) {
    if let Some(node) = tree {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().expect("Failed to read line").unwrap();
    let mut tree: SearchTree = None;

    for token in line.split_whitespace() {
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

    let min_node = find_min(&tree);
    let max_node = find_max(&tree);

    if let Some(min) = min_node {
        if let Some(max) = max_node {
            println!("Min is {}, Max is {}", min.element, max.element);
        }
    } else {
        println!("Tree is empty");
    }
}