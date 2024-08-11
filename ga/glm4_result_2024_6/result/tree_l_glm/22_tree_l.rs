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
        let mut node = *node;
        node.left = make_empty(node.left);
        node.right = make_empty(node.right);
        drop(node);
    }
    None
}

fn find(x: i32, t: SearchTree) -> Option<*const TreeNode> {
    if let Some(node) = t {
        if x < node.element {
            find(x, node.left)
        } else if x > node.element {
            find(x, node.right)
        } else {
            Some(&*node)
        }
    } else {
        None
    }
}

fn find_min(t: SearchTree) -> Option<*const TreeNode> {
    if let Some(node) = t {
        if let None = node.left {
            return Some(&*node);
        }
        find_min(node.left)
    } else {
        None
    }
}

fn find_max(t: SearchTree) -> Option<*const TreeNode> {
    if let Some(node) = t {
        if let None = node.right {
            return Some(&*node);
        }
        find_max(node.right)
    } else {
        None
    }
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    if let None = t {
        return Some(Box::new(TreeNode::new(x)));
    }
    let mut node = t.unwrap();
    if x < node.element {
        node.left = insert(x, node.left);
    } else if x > node.element {
        node.right = insert(x, node.right);
    }
    Some(node)
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        if x < node.element {
            node.left = delete(x, node.left);
        } else if x > node.element {
            node.right = delete(x, node.right);
        } else {
            if let Some(left) = node.left.take() {
                if let Some(right) = node.right.take() {
                    if let Some(min) = find_min(right) {
                        node.element = min.element;
                        node.left = left;
                        node.right = delete(min.element, right);
                    } else {
                        node = left;
                    }
                } else {
                    node = left;
                }
            } else if let Some(right) = node.right.take() {
                node = right;
            } else {
                return None;
            }
        }
        Some(node)
    } else {
        None
    }
}

fn retrieve(p: *const TreeNode) -> i32 {
    unsafe { (*p).element }
}

fn print_in_order(t: SearchTree) {
    if let Some(node) = t {
        print_in_order(node.left);
        println!("{}", node.element);
        print_in_order(node.right);
    }
}

fn delete_even_numbers(t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        node.left = delete_even_numbers(node.left);
        node.right = delete_even_numbers(node.right);
        if node.element % 2 == 0 {
            node = delete(node.element, Some(node)).unwrap();
        }
        Some(node)
    } else {
        None
    }
}

fn main() {
    let stdin = io::stdin();
    let mut t: SearchTree = None;
    let mut has_input = false;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                t = insert(num, t);
                has_input = true;
            }
        }
    }

    if has_input {
        println!("Tree after insertion:");
        print_in_order(t);

        t = delete_even_numbers(t);

        println!("Tree after deleting even numbers:");
        print_in_order(t);

        if let Some(min) = find_min(t) {
            println!("Min is {}, Max is {}", retrieve(min), retrieve(find_max(t).unwrap()));
        }
    } else {
        println!("Tree is empty");
    }
}