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
    match tree {
        Some(node) => {
            let mut current = node;
            while let Some(ref right_node) = current.right {
                current = right_node;
            }
            Some(current)
        }
        None => None,
    }
}

fn insert(x: i32, tree: SearchTree) -> SearchTree {
    match tree {
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
            Some(node)
        }
    }
}

fn delete(x: i32, tree: SearchTree) -> SearchTree {
    match tree {
        None => None,
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left);
                Some(node)
            } else if x > node.element {
                node.right = delete(x, node.right);
                Some(node)
            } else {
                // Node with two children
                if node.left.is_some() && node.right.is_some() {
                    if let Some(min_node) = find_min(&node.right) {
                        node.element = min_node.element;
                        node.right = delete(min_node.element, node.right);
                    }
                    Some(node)
                } else {
                    // Node with one or no child
                    if node.left.is_none() {
                        Some(node.right.take())
                    } else {
                        Some(node.left.take())
                    }
                }
            }
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

fn delete_even_numbers(tree: SearchTree) -> SearchTree {
    match tree {
        None => None,
        Some(mut node) => {
            node.left = delete_even_numbers(node.left);
            node.right = delete_even_numbers(node.right);

            if node.element % 2 == 0 {
                delete(node.element, Some(node))
            } else {
                Some(node)
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut tree = make_empty();

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

    if let (Some(min_node), Some(max_node)) = (find_min(&tree), find_max(&tree)) {
        println!("Min is {}, Max is {}", min_node.element, max_node.element);
    } else {
        println!("Tree is empty");
    }
}