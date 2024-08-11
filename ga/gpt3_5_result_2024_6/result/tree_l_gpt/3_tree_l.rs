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

fn make_empty(mut t: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    t.map(|mut node| {
        node.left = make_empty(node.left.take());
        node.right = make_empty(node.right.take());
        drop(node);
        None
    })
}

fn find(x: i32, t: Option<&Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    match t {
        Some(node) => match x.cmp(&node.element) {
            Ordering::Less => find(x, node.left.as_ref()),
            Ordering::Greater => find(x, node.right.as_ref()),
            Ordering::Equal => Some(node),
        },
        None => None,
    }
}

fn find_min(t: Option<&Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    t.and_then(|mut node| {
        let mut current = node;
        while let Some(left) = &current.left {
            current = left;
        }
        Some(current)
    })
}

fn find_max(t: Option<&Box<TreeNode>) -> Option<&Box<TreeNode>> {
    t.and_then(|mut node| {
        let mut current = node;
        while let Some(right) = &current.right {
            current = right;
        }
        Some(current)
    })
}

fn insert(x: i32, mut t: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    if let Some(mut node) = t.take() {
        match x.cmp(&node.element) {
            Ordering::Less => node.left = insert(x, node.left),
            Ordering::Greater => node.right = insert(x, node.right),
            Ordering::Equal => {}
        }
        Some(node)
    } else {
        Some(Box::new(TreeNode::new(x)))
    }
}

fn delete(x: i32, mut t: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    if let Some(mut node) = t.take() {
        match x.cmp(&node.element) {
            Ordering::Less => node.left = delete(x, node.left),
            Ordering::Greater => node.right = delete(x, node.right),
            Ordering::Equal => {
                if node.left.is_none() {
                    return node.right.take();
                } else if node.right.is_none() {
                    return node.left.take();
                } else {
                    let tmp_cell = find_min(node.right.as_ref()).unwrap().clone();
                    node.element = tmp_cell.element;
                    node.right = delete(node.element, node.right);
                }
            }
        }
        Some(node)
    } else {
        None
    }
}

fn retrieve(p: Option<&Box<TreeNode>>) -> i32 {
    p.map_or(0, |node| node.element)
}

fn print_in_order(t: Option<&Box<TreeNode>>) {
    if let Some(node) = t {
        print_in_order(node.left.as_ref());
        println!("{}", node.element);
        print_in_order(node.right.as_ref());
    }
}

fn delete_even_numbers(mut t: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    if let Some(mut node) = t.take() {
        node.left = delete_even_numbers(node.left);
        node.right = delete_even_numbers(node.right);

        if node.element % 2 == 0 {
            t = delete(node.element, t);
        }
        t
    } else {
        None
    }
}

fn main() {
    let mut t: Option<Box<TreeNode>> = None;
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut has_input = false;

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
    print_in_order(t.as_ref());

    t = delete_even_numbers(t);

    println!("Tree after deleting even numbers:");
    print_in_order(t.as_ref());

    if let Some(node) = t {
        let min = retrieve(find_min(Some(&node)));
        let max = retrieve(find_max(Some(&node)));
        println!("Min is {}, Max is {}", min, max);
    } else {
        println!("Tree is empty");
    }

    t = make_empty(t);
}