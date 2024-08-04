use std::io::{self, BufRead};
use std::ptr::null_mut;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    // In Rust, the tree will be automatically deallocated when it goes out of scope
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
    match t {
        None => None,
        Some(node) => {
            let mut current = node;
            while let Some(ref right_node) = current.right {
                current = right_node;
            }
            Some(current)
        }
    }
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
            Some(node) // Return the updated node
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
                Some(node)
            } else if x > node.element {
                node.right = delete(x, node.right);
                Some(node)
            } else if node.left.is_some() && node.right.is_some() {
                let min_node = find_min(&node.right).unwrap();
                node.element = min_node.element;
                node.right = delete(min_node.element, node.right);
                Some(node)
            } else {
                // One child or no child
                if node.left.is_none() {
                    Some(node.right.take()) // Take the right child
                } else {
                    Some(node.left.take()) // Take the left child
                }
            }
        }
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

fn print_in_order(t: &SearchTree) {
    if let Some(node) = t {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut tree: SearchTree = None;

    for line in reader.lines() {
        let line = line.unwrap();
        for token in line.split_whitespace() {
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

    if let Some(t) = &tree {
        let min_value = find_min(&tree).map(|n| n.element);
        let max_value = find_max(&tree).map(|n| n.element);
        match (min_value, max_value) {
            (Some(min), Some(max)) => {
                println!("Min is {}, Max is {}", min, max);
            }
            _ => {
                println!("Tree is empty");
            }
        }
    } else {
        println!("Tree is empty");
    }
}