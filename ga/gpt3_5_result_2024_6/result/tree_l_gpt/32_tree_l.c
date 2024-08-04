use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(element: i32) -> Self {
        TreeNode { element, left: None, right: None }
    }
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(tree: SearchTree) -> SearchTree {
    tree
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
    let mut node = tree.as_ref();
    while let Some(n) = node {
        if n.left.is_none() {
            return Some(n);
        }
        node = n.left.as_ref();
    }
    None
}

fn find_max(tree: &SearchTree) -> Option<&TreeNode> {
    let mut node = tree.as_ref();
    while let Some(n) = node {
        if n.right.is_none() {
            return Some(n);
        }
        node = n.right.as_ref();
    }
    None
}

fn insert(x: i32, mut tree: SearchTree) -> SearchTree {
    match tree {
        Some(mut node) => {
            match x.cmp(&node.element) {
                Ordering::Less => node.left = insert(x, node.left),
                Ordering::Greater => node.right = insert(x, node.right),
                Ordering::Equal => {}  // do nothing if element already exists
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
    let stdin = io::stdin();
    let input = stdin.lock().lines()
        .next().unwrap().unwrap();

    let mut tree: SearchTree = None;
    let mut has_input = false;

    for token in input.split_whitespace() {
        if let Ok(num) = token.parse() {
            tree = insert(num, tree);
            has_input = true;
        }
    }

    if !has_input || tree.is_none() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    print_in_order(&tree);

    tree = delete_even_numbers(tree);

    println!("Tree after deleting even numbers:");
    print_in_order(&tree);

    if let Some(node) = tree {
        if let Some(min) = find_min(&tree) {
            if let Some(max) = find_max(&tree) {
                println!("Min is {}, Max is {}", retrieve(min), retrieve(max));
            }
        }
    } else {
        println!("Tree is empty");
    }

    tree = make_empty(tree);
}