use std::io::{self, BufRead};
use std::mem;

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
    None
}

fn find(x: i32, t: SearchTree) -> Position {
    match t {
        None => None,
        Some(node) if x < node.element => find(x, node.left),
        Some(node) if x > node.element => find(x, node.right),
        _ => t,
    }
}

fn find_min(t: SearchTree) -> Position {
    match t {
        Some(mut node) => match node.left {
            Some(_) => find_min(node.left),
            None => Some(node),
        },
        _ => None,
    }
}

fn find_max(t: SearchTree) -> Position {
    let mut current = t;
    while let Some(mut node) = current {
        match node.right {
            Some(_) => current = node.right,
            None => return Some(node),
        }
    }
    None
}

fn insert(x: i32, mut t: SearchTree) -> SearchTree {
    if let Some(ref mut node) = t {
        if x < node.element {
            node.left = insert(x, node.left.take());
        } else if x > node.element {
            node.right = insert(x, node.right.take());
        }
        t
    } else {
        let new_node = TreeNode {
            element: x,
            left: None,
            right: None,
        };
        Some(Box::new(new_node))
    }
}

fn delete(x: i32, mut t: SearchTree) -> SearchTree {
    if let Some(mut node) = t.take() {
        if x < node.element {
            node.left = delete(x, node.left);
            return Some(node);
        } else if x > node.element {
            node.right = delete(x, node.right);
            return Some(node);
        }

        if node.left.is_some() && node.right.is_some() {
            let tmp_cell = find_min(node.right.take()).unwrap();
            node.element = tmp_cell.element;
            node.right = delete(node.element, node.right);
            return Some(node);
        }

        let tmp_cell = node;
        if node.left.is_none() {
            return node.right;
        } else {
            return node.left;
        }
    }
    t
}

fn retrieve(p: &Position) -> i32 {
    if let Some(node) = p {
        node.element
    } else {
        unreachable!()
    }
}

fn print_in_order(t: &SearchTree) {
    if let Some(node) = t {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn delete_even_numbers(mut t: SearchTree) -> SearchTree {
    if let Some(ref mut node) = t {
        node.left = delete_even_numbers(node.left.take());
        node.right = delete_even_numbers(node.right.take());

        if node.element % 2 == 0 {
            mem::replace(&mut t, delete(node.element, t));
        }
    }
    t
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    let mut has_input = false;

    let mut t: SearchTree = None;

    if let Ok(_) = stdin.lock().read_line(&mut line) {
        for token in line.trim().split(" ") {
            if let Ok(num) = token.parse::<i32>() {
                t = insert(num, t);
                has_input = true;
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
        let min = retrieve(&find_min(Some(node.clone())));
        let max = retrieve(&find_max(Some(node)));
        println!("Min is {}, Max is {}", min, max);
    } else {
        println!("Tree is empty");
    }

    t = make_empty(t);
}