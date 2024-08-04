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
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        node.left = make_empty(node.left);
        node.right = make_empty(node.right);
    }
    None
}

fn find(x: i32, t: &SearchTree) -> Option<&TreeNode> {
    match t {
        Some(node) => match x.cmp(&node.element) {
            Ordering::Less => find(x, &node.left),
            Ordering::Greater => find(x, &node.right),
            Ordering::Equal => Some(node),
        },
        None => None,
    }
}

fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    let mut current = t;
    while let Some(node) = current {
        if let Some(ref left_node) = node.left {
            current = Some(left_node);
        } else {
            return Some(node);
        }
    }
    None
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut current = t;
    while let Some(node) = current {
        if let Some(ref right_node) = node.right {
            current = Some(right_node);
        } else {
            return Some(node);
        }
    }
    None
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            match x.cmp(&node.element) {
                Ordering::Less => node.left = insert(x, node.left),
                Ordering::Greater => node.right = insert(x, node.right),
                Ordering::Equal => {} // do nothing for duplicates
            }
            Some(node)
        }
        None => Some(Box::new(TreeNode::new(x))),
    }
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            match x.cmp(&node.element) {
                Ordering::Less => node.left = delete(x, node.left),
                Ordering::Greater => node.right = delete(x, node.right),
                Ordering::Equal => {
                    if node.left.is_some() && node.right.is_some() {
                        let min_right = find_min(&node.right).unwrap().element;
                        node.element = min_right;
                        node.right = delete(min_right, node.right);
                    } else {
                        node = if node.left.is_some() {
                            node.left.take()
                        } else {
                            node.right.take()
                        };
                    }
                }
            }
            Some(node)
        }
        None => None,
    }
}

fn retrieve(p: &TreeNode) -> i32 {
    p.element
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
                node = delete(node.element, Some(node));
            }

            Some(node)
        }
        None => None,
    }
}

fn main() {
    let stdin = io::stdin();
    let mut tree: SearchTree = None;
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
        let min = find_min(&Some(node)).unwrap().element;
        let max = find_max(&Some(node)).unwrap().element;
        println!("Min is {}, Max is {}", min, max);
    } else {
        println!("Tree is empty");
    }

    tree = make_empty(tree);
}