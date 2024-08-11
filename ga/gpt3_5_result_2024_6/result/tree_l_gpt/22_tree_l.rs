use std::io::{self, BufRead};

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
        Some(node) => {
            if x == node.element {
                Some(node)
            } else if x < node.element {
                find(x, &node.left)
            } else {
                find(x, &node.right)
            }
        }
        None => None,
    }
}

fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    let mut current = t;
    while let Some(node) = current {
        if let Some(ref left) = node.left {
            current = Some(left);
        } else {
            return Some(node);
        }
    }
    None
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut current = t;
    while let Some(node) = current {
        if let Some(ref right) = node.right {
            current = Some(right);
        } else {
            return Some(node);
        }
    }
    None
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            if x < node.element {
                node.left = insert(x, node.left);
            } else if x > node.element {
                node.right = insert(x, node.right);
            }
            Some(node)
        }
        None => Some(Box::new(TreeNode::new(x))),
    }
}

fn delete(x: i32, t: &mut SearchTree) {
    if let Some(node) = t.take() {
        if x < node.element {
            delete(x, &mut node.left);
            *t = Some(node);
        } else if x > node.element {
            delete(x, &mut node.right);
            *t = Some(node);
        } else if node.left.is_some() && node.right.is_some() {
            let min_right = find_min(&node.right).unwrap().element;
            node.element = min_right;
            delete(min_right, &mut node.right);
            *t = Some(node);
        } else {
            *t = if node.left.is_none() {
                node.right
            } else {
                node.left
            };
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

fn delete_even_numbers(t: &mut SearchTree) {
    if let Some(node) = t.take() {
        delete_even_numbers(&mut node.left);
        delete_even_numbers(&mut node.right);
        if node.element % 2 == 0 {
            delete(node.element, t);
        } else {
            *t = Some(node);
        }
    }
}

fn main() {
    let mut t: SearchTree = None;
    let stdin = io::stdin();
    let mut has_input = false;

    if let Some(line) = stdin.lock().lines().next() {
        let line = line.unwrap();
        for token in line.split_whitespace() {
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

    delete_even_numbers(&mut t);

    println!("Tree after deleting even numbers:");
    print_in_order(&t);

    if let Some(node) = t {
        println!("Min is {}, Max is {}", retrieve(find_min(&t).unwrap()), retrieve(find_max(&t).unwrap()));
    } else {
        println!("Tree is empty");
    }

    make_empty(t);
}