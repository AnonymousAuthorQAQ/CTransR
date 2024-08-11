use std::io::{self, BufRead};

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
        Some(node) => {
            if x < node.element {
                find(x, &node.left)
            } else if x > node.element {
                find(x, &node.right)
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    let mut node = t.as_ref();
    while let Some(n) = node {
        if let Some(ref left_node) = n.left {
            node = Some(left_node);
        } else {
            return Some(n);
        }
    }
    None
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut node = t.as_ref();
    while let Some(n) = node {
        if let Some(ref right_node) = n.right {
            node = Some(right_node);
        } else {
            return Some(n);
        }
    }
    None
}

fn insert(x: i32, mut t: SearchTree) -> SearchTree {
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

fn delete(x: i32, mut t: SearchTree) -> SearchTree {
    if let Some(ref mut node) = t {
        if x < node.element {
            node.left = delete(x, node.left.take());
        } else if x > node.element {
            node.right = delete(x, node.right.take());
        } else if node.left.is_some() && node.right.is_some() {
            let min_right = find_min(&node.right).unwrap().element;
            node.element = min_right;
            node.right = delete(min_right, node.right.take());
        } else {
            *t = node.left.take().or(node.right.take());
        }
    }
    t
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

fn delete_even_numbers(mut t: SearchTree) -> SearchTree {
    if let Some(node) = t.as_mut() {
        node.left = delete_even_numbers(node.left.take());
        node.right = delete_even_numbers(node.right.take());

        if node.element % 2 == 0 {
            t = delete(node.element, t);
        }
    }
    t
}

fn main() {
    let stdin = io::stdin();
    let mut tree: SearchTree = None;

    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();

    for token in line.trim().split(" ") {
        if let Ok(num) = token.parse::<i32>() {
            tree = insert(num, tree);
        }
    }

    if let Some(ref t) = tree {
        println!("Tree after insertion:");
        print_in_order(&tree);

        tree = delete_even_numbers(tree);

        println!("Tree after deleting even numbers:");
        print_in_order(&tree);

        if let Some(min_node) = find_min(&tree) {
            if let Some(max_node) = find_max(&tree) {
                println!("Min is {}, Max is {}", retrieve(min_node), retrieve(max_node));
            }
        }
    } else {
        println!("Tree is empty");
    }

    tree = make_empty(tree);
}