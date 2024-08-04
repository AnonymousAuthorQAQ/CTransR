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
    match tree {
        Some(node) => {
            if let Some(left) = &node.left {
                find_min(&node.left)
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_max(tree: &SearchTree) -> Option<&TreeNode> {
    let mut current = tree;
    while let Some(node) = current {
        if let Some(right) = &node.right {
            current = &node.right;
        } else {
            return Some(node);
        }
    }
    None
}

fn insert(x: i32, tree: SearchTree) -> SearchTree {
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

fn delete(x: i32, tree: SearchTree) -> SearchTree {
    match tree {
        Some(mut node) => {
            match x.cmp(&node.element) {
                Ordering::Less => {
                    node.left = delete(x, node.left);
                }
                Ordering::Greater => {
                    node.right = delete(x, node.right);
                }
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
        }
        None => None,
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

fn delete_even_numbers(tree: SearchTree) -> SearchTree {
    match tree {
        Some(mut node) => {
            node.left = delete_even_numbers(node.left);
            node.right = delete_even_numbers(node.right);
            if node.element % 2 == 0 {
                node = delete(node.element, node);
            }
            Some(node)
        }
        None => None,
    }
}

fn main() {
    let mut tree: SearchTree = None;
    let stdin = io::stdin();
    let mut has_input = false;

    if let Some(Ok(line)) = stdin.lock().lines().next() {
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                tree = insert(num, tree);
                has_input = true;
            }
        }
    }

    if !has_input || tree.is_none() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    print_in_order(&tree);

    // Delete all even numbers
    tree = delete_even_numbers(tree);

    println!("Tree after deleting even numbers:");
    print_in_order(&tree);

    if let Some(node) = tree {
        let min = retrieve(find_min(&tree).unwrap());
        let max = retrieve(find_max(&tree).unwrap());
        println!("Min is {}, Max is {}", min, max);
    } else {
        println!("Tree is empty");
    }

    tree = make_empty(tree);
}