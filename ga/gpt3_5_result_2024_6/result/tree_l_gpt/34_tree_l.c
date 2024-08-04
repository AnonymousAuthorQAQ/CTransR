use std::io::{self, BufRead};
use std::str::FromStr;

struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(tree: SearchTree) -> SearchTree {
    if let Some(mut node) = tree {
        node.left = make_empty(node.left);
        node.right = make_empty(node.right);
    }
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
    tree.as_ref().and_then(|node| {
        if let Some(left) = &node.left {
            find_min(&Some(left))
        } else {
            Some(node)
        }
    })
}

fn find_max(tree: &SearchTree) -> Option<&TreeNode> {
    let mut current = tree.as_ref();
    while let Some(node) = current {
        if let Some(right) = &node.right {
            current = Some(right);
        } else {
            return Some(node);
        }
    }
    None
}

fn insert(x: i32, mut tree: SearchTree) -> SearchTree {
    match tree {
        Some(mut node) => {
            if x < node.element {
                node.left = insert(x, node.left);
            } else if x > node.element {
                node.right = insert(x, node.right);
            }
            Some(node)
        }
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
    }
}

fn delete(x: i32, mut tree: SearchTree) -> SearchTree {
    if let Some(mut node) = tree.take() {
        if x < node.element {
            node.left = delete(x, node.left);
            Some(node)
        } else if x > node.element {
            node.right = delete(x, node.right);
            Some(node)
        } else if node.left.is_some() && node.right.is_some() {
            let min_right = find_min(&node.right).unwrap().element;
            node.element = min_right;
            node.right = delete(min_right, node.right);
            Some(node)
        } else {
            if node.left.is_none() {
                node = node.right.take().unwrap();
            } else {
                node = node.left.take().unwrap();
            }
            Some(node)
        }
    } else {
        None
    }
}

fn retrieve(node: &TreeNode) -> i32 {
    node.element
}

fn print_in_order(tree: &SearchTree) {
    if let Some(node) = tree {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn delete_even_numbers(mut tree: SearchTree) -> SearchTree {
    if let Some(mut node) = tree.take() {
        node.left = delete_even_numbers(node.left);
        node.right = delete_even_numbers(node.right);
        
        if node.element % 2 == 0 {
            tree = delete(node.element, Some(node));
        } else {
            tree = Some(node);
        }
    }
    tree
}

fn main() {
    let mut tree: SearchTree = None;
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    for token in input.split_whitespace() {
        if let Ok(num) = i32::from_str(token) {
            tree = insert(num, tree);
        }
    }

    if let Some(tree_ref) = &tree {
        println!("Tree after insertion:");
        print_in_order(&tree);

        tree = delete_even_numbers(tree);

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