use std::io::{self, BufRead};
use std::cmp::Ordering;

struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;
type Position = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        node.left = make_empty(node.left);
        node.right = make_empty(node.right);
    }
    t
}

fn find(x: i32, t: &SearchTree) -> Position {
    match t {
        None => None,
        Some(node) => match x.cmp(&node.element) {
            Ordering::Less => find(x, &node.left),
            Ordering::Greater => find(x, &node.right),
            Ordering::Equal => Some(Box::new(*node)),
        }
    }
}

fn find_min(t: &SearchTree) -> Position {
    match t {
        None => None,
        Some(mut node) => {
            let mut current = &mut *node;
            while let Some(left) = &current.left {
                current = &mut **left;
            }
            Some(Box::new(*current))
        }
    }
}

fn find_max(t: &SearchTree) -> Position {
    match t {
        None => None,
        Some(mut node) => {
            let mut current = &mut *node;
            while let Some(right) = &current.right {
                current = &mut **right;
            }
            Some(Box::new(*current))
        }
    }
}

fn insert(x: i32, mut t: SearchTree) -> SearchTree {
    match t {
        None => {
            t = Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            }));
        }
        Some(node) => match x.cmp(&node.element) {
            Ordering::Less => {
                node.left = insert(x, node.left);
            }
            Ordering::Greater => {
                node.right = insert(x, node.right);
            }
            Ordering::Equal => (),
        }
    }
    t
}

fn delete(x: i32, mut t: SearchTree) -> SearchTree {
    if let Some(mut node) = t.take() {
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
                    let mut tmp_cell = find_min(&node.right);
                    node.element = tmp_cell.as_ref().unwrap().element;
                    node.right = delete(node.element, node.right);
                }
            }
        }
        t = Some(node);
    }
    t
}

fn retrieve(p: &Position) -> i32 {
    p.as_ref().unwrap().element
}

fn print_in_order(t: &SearchTree) {
    match t {
        None => (),
        Some(node) => {
            print_in_order(&node.left);
            println!("{}", node.element);
            print_in_order(&node.right);
        }
    }
}

fn delete_even_numbers(mut t: SearchTree) -> SearchTree {
    if let Some(mut node) = t.take() {
        node.left = delete_even_numbers(node.left);
        node.right = delete_even_numbers(node.right);

        if node.element % 2 == 0 {
            t = delete(node.element, Some(node)); // Re-assign t after deleting the even number
        } else {
            t = Some(node);
        }
    }
    t
}

fn main() {
    let stdin = io::stdin();
    let mut t: SearchTree = None;
    let mut has_input = false;

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

    t = delete_even_numbers(t);

    println!("Tree after deleting even numbers:");
    print_in_order(&t);

    if let Some(node) = t {
        let min = find_min(&Some(node)).unwrap();
        let max = find_max(&Some(node)).unwrap();
        println!("Min is {}, Max is {}", retrieve(&min), retrieve(&max));
    } else {
        println!("Tree is empty");
    }

    t = make_empty(t);
}