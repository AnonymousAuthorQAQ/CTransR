use std::io::{self, BufRead};
use std::str::FromStr;

struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn make_empty(tree: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    tree.map(|mut node| {
        node.left = make_empty(node.left.take());
        node.right = make_empty(node.right.take());
        node
    })
}

fn find(x: i32, tree: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
    match tree {
        Some(node) => {
            if x < node.element {
                find(x, &node.left)
            } else if x > node.element {
                find(x, &node.right)
            } else {
                tree.as_ref().map(|value| &**value)
            }
        },
        None => None
    }
}

fn find_min(tree: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
    tree.as_ref().and_then(|node| {
        if let Some(ref left) = node.left {
            find_min(&Some(left))
        } else {
            Some(node)
        }
    })
}

fn find_max(tree: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
    let mut current = tree;
    while let Some(node) = current {
        current = &node.right;
    }
    current.as_ref()
}

fn insert(x: i32, mut tree: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    if let Some(mut node) = tree.take() {
        if x < node.element {
            node.left = insert(x, node.left);
        } else if x > node.element {
            node.right = insert(x, node.right);
        }
        Some(node)

    } else {
        Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        }))
    }
}

fn delete(x: i32, mut tree: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    if let Some(mut node) = tree.take() {
        if x < node.element {
            node.left = delete(x, node.left);
        } else if x > node.element {
            node.right = delete(x, node.right);
        } else if node.left.is_some() && node.right.is_some() {
            let min_right = find_min(&node.right).unwrap().element;
            node.element = min_right;
            node.right = delete(min_right, node.right);
        } else {
            node = if node.left.is_none() {
                node.right.take()
            } else {
                node.left.take()
            };
        }
        Some(node)

    } else {
        None
    }
}

fn retrieve(node: &TreeNode) -> i32 {
    node.element
}

fn print_in_order(tree: &Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn delete_even_numbers(mut tree: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    if let Some(mut node) = tree.take() {
        node.left = delete_even_numbers(node.left);
        node.right = delete_even_numbers(node.right);

        if node.element % 2 == 0 {
            tree = delete(node.element, Some(node));
        }

        tree
    } else {
        None
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    
    let mut tree: Option<Box<TreeNode>> = None;

    for token in input.split_whitespace() {
        if let Ok(num) = i32::from_str(token) {
            tree = insert(num, tree);
        }
    }

    if let Some(ref node) = tree {
        println!("Tree after insertion:");
        print_in_order(&Some(node.clone()));

        tree = delete_even_numbers(tree.clone());

        println!("Tree after deleting even numbers:");
        print_in_order(&tree);

        if let Some(min_node) = find_min(&tree) {
            if let Some(max_node) = find_max(&tree) {
                println!("Min is {}, Max is {}", retrieve(min_node), retrieve(max_node));
            }
        }
    } else {
        println!("Tree is empty");
    }

    tree = make_empty(tree);
}