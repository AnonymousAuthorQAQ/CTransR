use std::io::{self, BufRead};
use std::cmp::Ordering;

struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;
type Position = Option<Box<TreeNode>>;

impl TreeNode {
    fn new(element: i32) -> TreeNode {
        TreeNode { element, left: None, right: None }
    }
}

fn make_empty(t: SearchTree) -> SearchTree {
    drop(t);
    None
}

fn find(x: i32, t: SearchTree) -> Position {
    match t {
        Some(ref node) => {
            match x.cmp(&node.element) {
                Ordering::Less => find(x, node.left.clone()),
                Ordering::Greater => find(x, node.right.clone()),
                Ordering::Equal => t,
            }
        }
        None => None,
    }
}

fn find_min(t: SearchTree) -> Position {
    match t {
        Some(ref node) => {
            if let Some(ref left_node) = node.left {
                find_min(Some(left_node.clone()))
            } else {
                Some(Box::new(node.clone()))
            }
        }
        None => None,
    }
}

fn find_max(t: SearchTree) -> Position {
    match t {
        Some(ref node) => {
            let mut current = node.clone();
            while let Some(ref right_node) = current.right {
                current = right_node.clone();
            }
            Some(Box::new(current.clone()))
        }
        None => None,
    }
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => Some(Box::new(TreeNode::new(x))),
        Some(mut node) => {
            match x.cmp(&node.element) {
                Ordering::Less => node.left = insert(x, node.left.take()),
                Ordering::Greater => node.right = insert(x, node.right.take()),
                Ordering::Equal => (),
            }
            Some(node)
        }
    }
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        if x < node.element {
            node.left = delete(x, node.left.take());
        } else if x > node.element {
            node.right = delete(x, node.right.take());
        } else if node.left.is_some() && node.right.is_some() {
            let min = find_min(node.right.take());
            if let Some(min_node) = min {
                node.element = min_node.element;
                node.right = delete(node.element, node.right.take());
            }
        } else {
            node = if node.left.is_none() { node.right.take() } else { node.left.take() };
        }
        Some(node)
    } else {
        None
    }
}

fn retrieve(p: &Position) -> i32 {
    p.as_ref().unwrap().element
}

fn print_in_order(t: &SearchTree) {
    match t {
        Some(ref node) => {
            print_in_order(&node.left);
            println!("{}", node.element);
            print_in_order(&node.right);
        }
        None => (),
    }
}

fn delete_even_numbers(t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            node.left = delete_even_numbers(node.left.take());
            node.right = delete_even_numbers(node.right.take());
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
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    for token in input.split_whitespace() {
        match token.parse::<i32>() {
            Ok(num) => {
                t = insert(num, t);
            }
            Err(_) => (),
        }
    }

    if let Some(ref t) = t {
        println!("Tree after insertion:");
        print_in_order(t);

        t = delete_even_numbers(t.clone());
        
        println!("Tree after deleting even numbers:");
        print_in_order(&t);

        if let Some(min_node) = find_min(t.clone()) {
            let min = retrieve(&Some(min_node));
            if let Some(max_node) = find_max(t) {
                let max = retrieve(&Some(max_node));
                println!("Min is {}, Max is {}", min, max);
            }
        }

        t = make_empty(t);
    } else {
        println!("Tree is empty");
    }
}