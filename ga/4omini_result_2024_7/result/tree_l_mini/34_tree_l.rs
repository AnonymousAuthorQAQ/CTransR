use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    // In Rust, unless particular complexities arise we're handling ownership -- just letting it go drops it.
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
                Some(node)
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
                    // Node with two children
                    let min_node = find_min(&node.right).unwrap().clone();
                    node.element = min_node.element;
                    node.right = delete(node.element, node.right);
                    Some(node)
                } else {
                    // Node with one or no child.
                    if node.left.is_none() {
                        Some(node.right).map(|n| *n)
                    } else {
                        Some(node.left).map(|n| *n)
                    }
                }
            }
        }
        None => None,
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

fn print_in_order(t: &SearchTree) {
    if let Some(node) = t {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut tree = make_empty(None);

    for token in input.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            tree = insert(num, tree);
        }
    }

    println!("Tree after insertion:");
    print_in_order(&tree);

    tree = delete_even_numbers(tree);
    println!("Tree after deleting even numbers:");
    print_in_order(&tree);

    if let Some(min_node) = find_min(&tree) {
        // Displays minimum and maximum only if a non-empty tree is present.
        if let Some(max_node) = find_max(&tree) {
            println!("Min is {}, Max is {}", min_node.element, max_node.element);
        }
    } else {
        println!("Tree is empty");
    }
}