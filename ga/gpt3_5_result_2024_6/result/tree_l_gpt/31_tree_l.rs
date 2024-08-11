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
    if let Some(node) = t {
        make_empty(node.left);
        make_empty(node.right);
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
        if let Some(left_node) = &node.left {
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
        if let Some(right_node) = &node.right {
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
                Ordering::Equal => (),
            }
            Some(node)
        }
        None => Some(Box::new(TreeNode::new(x))),
    }
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    let mut new_tree = t;
    if let Some(mut node) = new_tree.take() {
        match x.cmp(&node.element) {
            Ordering::Less => node.left = delete(x, node.left),
            Ordering::Greater => node.right = delete(x, node.right),
            Ordering::Equal => {
                new_tree = if node.left.is_none() {
                    node.right.take()
                } else if node.right.is_none() {
                    node.left.take()
                } else {
                    let min_node = find_min(&node.right).unwrap();
                    node.element = min_node.element;
                    node.right = delete(min_node.element, node.right);
                    Some(node)
                }
            }
        }
    }
    new_tree
}

fn retrieve(p: Option<&TreeNode>) -> i32 {
    p.unwrap().element
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
        },
        None => None,
    }
}

fn main() {
    let mut t: SearchTree = None;
    let stdin = io::stdin();
    let mut line = String::new();

    if stdin.lock().read_line(&mut line).unwrap() > 0 {
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                t = insert(num, t);
            }
        }
    }

    if let Some(tree) = &t {
        println!("Tree after insertion:");
        print_in_order(tree);

        t = delete_even_numbers(t);

        println!("Tree after deleting even numbers:");
        print_in_order(&t);

        match find_min(&t) {
            Some(min) => {
                match find_max(&t) {
                    Some(max) => {
                        println!("Min is {}, Max is {}", retrieve(Some(min)), retrieve(Some(max));
                    },
                    None => println!("Max not found"),
                }
            },
            None => println!("Min not found"),
        }
    } else {
        println!("Tree is empty");
    }

    t = make_empty(t);
}