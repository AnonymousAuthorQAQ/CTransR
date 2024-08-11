use std::cmp::Ordering;
use std::io::{self, BufRead};
use std::ptr;

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
    let mut current = t.as_ref().unwrap();
    let mut node = current;
    while let Some(left) = &node.left {
        current = left;
        node = current;
    }
    Some(node)
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut current = t.as_ref().unwrap();
    let mut node = current;
    while let Some(right) = &node.right {
        current = right;
        node = current;
    }
    Some(node)
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
    let stdin = io::stdin();
    let mut t: SearchTree = None;
    let mut has_input = false;

    println!("Enter space-separated numbers to build the tree:");

    for line in stdin.lock().lines() {
        if let Ok(input) = line {
            for token in input.split_whitespace() {
                if let Ok(num) = token.parse::<i32>() {
                    t = insert(num, t);
                    has_input = true;
                }
            }
        }
    }

    if !has_input || t.is_none() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    print_in_order(&t);

    // Delete all even numbers
    t = delete_even_numbers(t);

    println!("Tree after deleting even numbers:");
    print_in_order(&t);

    if let Some(tree) = &t {
        let min = find_min(&t).unwrap().element;
        let max = find_max(&t).unwrap().element;
        println!("Min is {}, Max is {}", min, max);
    } else {
        println!("Tree is empty");
    }

    t = make_empty(t);
}