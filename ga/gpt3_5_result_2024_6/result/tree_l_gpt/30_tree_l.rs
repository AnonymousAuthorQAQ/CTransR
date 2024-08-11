use std::io::{self, BufRead};
use std::cmp::Ordering;
use std::ptr;

struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    t
}

fn find(x: i32, t: &SearchTree) -> Option<&TreeNode> {
    match t {
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

fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    let mut current = t;
    while let Some(node) = current {
        match &node.left {
            Some(_) => current = &node.left,
            None => return Some(node),
        }
    }
    None
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut current = t;
    while let Some(node) = current {
        match &node.right {
            Some(_) => current = &node.right,
            None => return Some(node),
        }
    }
    None
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
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
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
    }
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
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

fn retrieve(p: &TreeNode) -> i32 {
    p.element
}

fn print_in_order(t: &SearchTree) {
    match t {
        Some(node) => {
            print_in_order(&node.left);
            println!("{}", node.element);
            print_in_order(&node.right);
        }
        None => {}
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
    let mut has_input = false;

    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    for token in input.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            t = insert(num, t);
            has_input = true;
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

    if let Some(node) = t.as_ref() {
        println!("Min is {}, Max is {}", retrieve(find_min(&t).unwrap()), retrieve(find_max(&t).unwrap()));
    } else {
        println!("Tree is empty");
    }

    t = make_empty(t);
}