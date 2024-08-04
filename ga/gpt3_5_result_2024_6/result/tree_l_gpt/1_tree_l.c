use std::io::{self, BufRead};
use std::cmp::Ordering;

struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(element: i32) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
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
            match x.cmp(&node.element) {
                Ordering::Less => find(x, &node.left),
                Ordering::Greater => find(x, &node.right),
                Ordering::Equal => Some(node),
            }
        }
        None => None,
    }
}

fn find_min(tree: &SearchTree) -> Option<&TreeNode> {
    tree.as_ref().and_then(|node| node.left.as_ref().map_or(Some(node), |left| find_min(&Some(left))))
}

fn find_max(tree: &SearchTree) -> Option<&TreeNode> {
    let mut current = tree;
    while let Some(node) = current {
        if let Some(right) = &node.right {
            current = &Some(right);
        } else {
            return Some(node);
        }
    }
    None
}

fn insert(x: i32, mut tree: SearchTree) -> SearchTree {
    match tree {
        Some(mut node) => {
            match x.cmp(&node.element) {
                Ordering::Less => {
                    node.left = insert(x, node.left);
                }
                Ordering::Greater => {
                    node.right = insert(x, node.right);
                }
                Ordering::Equal => {}
            }
            Some(node)
        }
        None => Some(Box::new(TreeNode::new(x))),
    }
}

fn delete(x: i32, mut tree: SearchTree) -> SearchTree {
    if let Some(mut node) = tree.take() {
        match x.cmp(&node.element) {
            Ordering::Less => node.left = delete(x, node.left),
            Ordering::Greater => node.right = delete(x, node.right),
            Ordering::Equal => {
                if node.left.is_none() {
                    return node.right;
                } else if node.right.is_none() {
                    return node.left;
                } else {
                    let min_right = find_min(&node.right).unwrap().element;
                    node.element = min_right;
                    node.right = delete(min_right, node.right);
                }
            }
        }
        Some(node)
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
            return delete(node.element, Some(node));
        }

        Some(node)
    } else {
        None
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().lock().read_line(&mut input_line).unwrap();
    let mut tree: SearchTree = None;

    for token in input_line.trim().split(' ') {
        if let Ok(num) = token.parse::<i32>() {
            tree = insert(num, tree);
        }
    }

    if let Some(ref t) = tree {
        println!("Tree after insertion:");
        print_in_order(t);

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