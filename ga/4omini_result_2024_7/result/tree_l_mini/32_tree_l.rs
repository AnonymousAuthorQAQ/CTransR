use std::io::{self, BufRead};
use std::cmp::Ord;

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

fn find(x: i32, t: &SearchTree) -> Option<&TreeNode> {
    match t {
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

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            if x < node.element {
                node.left = insert(x, node.left);
                Some(node)
            } else if x > node.element {
                node.right = insert(x, node.right);
                Some(node)
            } else {
                Some(node) // No duplicates allowed, return the node unchanged
            }
        }
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
    }
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left);
                Some(node)
            } else if x > node.element {
                node.right = delete(x, node.right);
                Some(node)
            } else {
                // Node found
                if node.left.is_some() && node.right.is_some() {
                    // Two children
                    let min_node = find_min(&node.right).unwrap();
                    node.element = min_node.element;
                    node.right = delete(node.element, node.right);
                    Some(node)
                } else {
                    // One or no children
                    let temp = if node.left.is_none() {
                        node.right
                    } else {
                        node.left
                    };
                    temp
                }
            }
        }
        None => None,
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

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut tree: SearchTree = None;

    // Read input
    for line in handle.lines() {
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

    if let Some(tree) = &tree {
        let min = find_min(&tree).unwrap();
        let max = find_max(&tree).unwrap();
        println!("Min is {}, Max is {}", min.element, max.element);
    } else {
        println!("Tree is empty");
    }
}