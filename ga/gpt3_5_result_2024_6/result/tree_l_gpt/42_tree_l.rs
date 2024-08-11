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
    t
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
    match t {
        Some(node) => {
            if let Some(left) = &node.left {
                find_min(&Some(left))
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    match t {
        Some(node) => {
            let mut current = node;
            while let Some(right) = &current.right {
                current = right;
            }
            Some(current)
        }
        None => None,
    }
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            match x.cmp(&node.element) {
                Ordering::Less => node.left = insert(x, node.left),
                Ordering::Greater => node.right = insert(x, node.right),
                Ordering::Equal => (),
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
                    if node.left.is_none() {
                        return node.right;
                    } else if node.right.is_none() {
                        return node.left;
                    }
                    let min_right = find_min(&node.right).unwrap().element;
                    node.element = min_right;
                    node.right = delete(min_right, node.right);
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
                return delete(node.element, Some(node));
            }
            Some(node)
        }
        None => None,
    }
}

fn main() {
    let mut t: SearchTree = None;
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    for &num in numbers.iter() {
        t = insert(num, t);
    }

    if let Some(tree) = &t {
        println!("Tree after insertion:");
        print_in_order(&t);

        t = delete_even_numbers(t);

        println!("Tree after deleting even numbers:");
        print_in_order(&t);

        if let Some(min_node) = find_min(&t) {
            if let Some(max_node) = find_max(&t) {
                println!("Min is {}, Max is {}", retrieve(min_node), retrieve(max_node));
            }
        }
    } else {
        println!("Tree is empty");
    }

    t = make_empty(t);
}