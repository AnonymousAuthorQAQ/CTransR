use std::io::{self, BufRead};
use std::cmp::Ordering;

struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
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
    let mut curr = t.as_ref();
    while let Some(node) = curr {
        if let Some(left_node) = &node.left {
            curr = Some(left_node);
        } else {
            return Some(node);
        }
    }
    None
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut curr = t.as_ref();
    while let Some(node) = curr {
        if let Some(right_node) = &node.right {
            curr = Some(right_node);
        } else {
            return Some(node);
        }
    }
    None
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
        Some(mut node) => {
            if x < node.element {
                node.left = insert(x, node.left.take());
            } else if x > node.element {
                node.right = insert(x, node.right.take());
            }
            Some(node)
        }
    }
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            match x.cmp(&node.element) {
                Ordering::Less => node.left = delete(x, node.left.take()),
                Ordering::Greater => node.right = delete(x, node.right.take()),
                Ordering::Equal => {
                    if node.left.is_some() && node.right.is_some() {
                        let tmp_cell = find_min(&node.right).unwrap();
                        node.element = tmp_cell.element;
                        node.right = delete(node.element, node.right.take());
                    } else {
                        let tmp_cell = node;
                        if node.left.is_none() {
                            return node.right.take();
                        } else {
                            return node.left.take();
                        }
                    }
                }
            }
            Some(node)
        }
        None => {
            println!("Element not found");
            None
        }
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
                node = delete(node.element, Some(node));
            }

            Some(node)
        }
        None => None
    }
}

fn main() {
    let mut t: SearchTree = None;
    let stdin = io::stdin();
    let mut has_input = false;

    if let Some(Ok(line)) = stdin.lock().lines().next() {
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse() {
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

    if let Some(min_node) = find_min(&t) {
        if let Some(max_node) = find_max(&t) {
            println!("Min is {}, Max is {}", retrieve(min_node), retrieve(max_node));
        } else {
            println!("Tree is empty");
        }
    }

    t = make_empty(t);
}