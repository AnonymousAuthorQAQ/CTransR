use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty() -> SearchTree {
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
    match t {
        Some(node) => {
            if node.right.is_none() {
                Some(node)
            } else {
                find_max(&node.right)
            }
        }
        None => None,
    }
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
                Some(node) // Ignore duplicates
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
                } else {
                    // One or no children
                    if node.left.is_none() {
                        return node.right;
                    } else {
                        return node.left;
                    }
                }
                Some(node)
            }
        }
        None => None, // Element not found
    }
}

fn delete_even_numbers(t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            node.left = delete_even_numbers(node.left);
            node.right = delete_even_numbers(node.right);

            if node.element % 2 == 0 {
                return delete(node.element, Some(node));
            }

            Some(node)
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

fn main() {
    let stdin = io::stdin();
    let mut tree = make_empty();

    println!("Enter integers separated by spaces (Ctrl+D to finish):");
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let numbers: Vec<i32> = line
        .split_whitespace()
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

    if let (Some(min_node), Some(max_node)) = (find_min(&tree), find_max(&tree)) {
        println!("Min is {}, Max is {}", min_node.element, max_node.element);
    } else {
        println!("Tree is empty");
    }
}