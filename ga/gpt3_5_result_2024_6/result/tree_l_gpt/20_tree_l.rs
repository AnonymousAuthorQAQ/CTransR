use std::io::{self, BufRead};
use std::cmp::Ordering;

struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(element: i32) -> TreeNode {
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

fn insert(x: i32, tree: SearchTree) -> SearchTree {
    match tree {
        None => Some(Box::new(TreeNode::new(x))),
        Some(mut node) => {
            match x.cmp(&node.element) {
                Ordering::Less => node.left = insert(x, node.left),
                Ordering::Greater => node.right = insert(x, node.right),
                Ordering::Equal => (),
            }
            Some(node)
        }
    }
}

fn delete(x: i32, tree: SearchTree) -> SearchTree {
    if let Some(mut node) = tree {
        match x.cmp(&node.element) {
            Ordering::Less => node.left = delete(x, node.left),
            Ordering::Greater => node.right = delete(x, node.right),
            Ordering::Equal => {
                if node.left.is_none() {
                    return node.right;
                } else if node.right.is_none() {
                    return node.left;
                } else {
                    let mut min_right = find_min(&mut node.right);
                    node.element = min_right.element;
                    node.right = delete(min_right.element, node.right);
                }
            }
        }
        Some(node)
    } else {
        None
    }
}

fn find_min(tree: &mut SearchTree) -> &mut TreeNode {
    match tree {
        Some(node) => {
            if let Some(ref mut left) = node.left {
                find_min(&mut node.left)
            } else {
                node
            }
        }
        None => panic!("Tree is empty"),
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
        Some(mut node) => {
            node.left = delete_even_numbers(node.left);
            node.right = delete_even_numbers(node.right);

            if node.element % 2 == 0 {
                return delete(node.element, Some(node));
            }

            Some(node)
        }
        None => None
    }
}

fn main() {
    let mut tree: SearchTree = None;

    let stdin = io::stdin();
    let mut input = String::new();
    
    stdin.lock().read_line(&mut input).unwrap();
    let tokens: Vec<&str> = input.trim().split(" ").collect();

    for token in tokens {
        if let Ok(num) = token.parse::<i32>() {
            tree = insert(num, tree);
        }
    }

    if let Some(ref tree) = tree {
        println!("Tree after insertion:");
        print_in_order(tree);

        tree = delete_even_numbers(tree);

        println!("Tree after deleting even numbers:");
        print_in_order(&tree);

        if let Some(min) = find_min(&mut tree) {
            if let Some(max) = find_max(&mut tree) {
                println!("Min is {}, Max is {}", min.element, max.element);
            }
        }
    } else {
        println!("Tree is empty");
    }

    tree = make_empty(tree);
}