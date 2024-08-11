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

fn make_empty(t: SearchTree) -> SearchTree {
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
    let mut current = t.as_ref();
    while let Some(node) = current {
        match node.left {
            Some(_) => current = node.left.as_ref(),
            None => return Some(node),
        }
    }
    None
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut current = t.as_ref();
    while let Some(node) = current {
        match node.right {
            Some(_) => current = node.right.as_ref(),
            None => return Some(node),
        }
    }
    None
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            match x.cmp(&node.element) {
                Ordering::Less => node.left = insert(x, node.left.take()),
                Ordering::Greater => node.right = insert(x, node.right.take()),
                _ => {}
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
                Ordering::Less => node.left = delete(x, node.left.take()),
                Ordering::Greater => node.right = delete(x, node.right.take()),
                Ordering::Equal => {
                    if node.left.is_none() {
                        return node.right.take();
                    } else if node.right.is_none() {
                        return node.left.take();
                    } else {
                        let mut tmp_node = find_min(&node.right).unwrap().clone();
                        node.element = tmp_node.element;
                        node.right = delete(node.element, node.right.take());
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
            node.left = delete_even_numbers(node.left.take());
            node.right = delete_even_numbers(node.right.take());
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
    let stdin = io::stdin();
    let mut has_input = false;

    if let Ok(input) = stdin.lock().lines().next() {
        if let Ok(line) = input {
            for token in line.split_whitespace() {
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

    t = delete_even_numbers(t);

    println!("Tree after deleting even numbers:");
    print_in_order(&t);

    if let Some(tree) = &t {
        if let Some(min_node) = find_min(tree) {
            if let Some(max_node) = find_max(tree) {
                println!("Min is {}, Max is {}", retrieve(min_node), retrieve(max_node));
            }
        }
    } else {
        println!("Tree is empty");
    }

    t = make_empty(t);
}