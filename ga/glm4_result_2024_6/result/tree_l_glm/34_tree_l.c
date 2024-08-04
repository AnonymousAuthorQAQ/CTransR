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

fn make_empty(tree: SearchTree) -> SearchTree {
    match tree {
        Some(node) => {
            let mut left = make_empty(node.left);
            let mut right = make_empty(node.right);
            left.take();
            right.take();
            None
        }
        None => None,
    }
}

fn find(x: i32, tree: &SearchTree) -> Option<&TreeNode> {
    match tree {
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

fn find_min(tree: &SearchTree) -> Option<&TreeNode> {
    match tree {
        Some(node) => {
            if node.left.is_none() {
                Some(node)
            } else {
                find_min(&node.left)
            }
        }
        None => None,
    }
}

fn find_max(tree: &SearchTree) -> Option<&TreeNode> {
    match tree {
        Some(node) => {
            let mut current = node;
            while current.right.is_some() {
                current = current.right.as_ref().unwrap();
            }
            Some(current)
        }
        None => None,
    }
}

fn insert(x: i32, tree: &mut SearchTree) {
    match tree {
        Some(node) => {
            if x < node.element {
                insert(x, &mut node.left);
            } else if x > node.element {
                insert(x, &mut node.right);
            }
        }
        None => {
            *tree = Some(Box::new(TreeNode::new(x)));
        }
    }
}

fn delete(x: i32, tree: &mut SearchTree) {
    match tree {
        Some(node) => {
            if x < node.element {
                delete(x, &mut node.left);
            } else if x > node.element {
                delete(x, &mut node.right);
            } else if node.left.is_some() && node.right.is_some() {
                let min_right = find_min(&node.right).unwrap();
                node.element = min_right.element;
                delete(min_right.element, &mut node.right);
            } else {
                let mut left = node.left.take();
                let mut right = node.right.take();
                if left.is_none() {
                    *tree = right;
                } else if right.is_none() {
                    *tree = left;
                }
            }
        }
        None => {
            eprintln!("Element not found");
        }
    }
}

fn retrieve(node: &TreeNode) -> i32 {
    node.element
}

fn print_in_order(tree: &SearchTree) {
    match tree {
        Some(node) => {
            print_in_order(&node.left);
            println!("{}", node.element);
            print_in_order(&node.right);
        }
        None => {}
    }
}

fn delete_even_numbers(tree: &mut SearchTree) {
    if let Some(node) = tree {
        delete_even_numbers(&mut node.left);
        delete_even_numbers(&mut node.right);
        if node.element % 2 == 0 {
            delete(node.element, tree);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().and_then(|result| result.ok());
    let mut tree: SearchTree = None;

    if let Some(line) = line {
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                insert(num, &mut tree);
            }
        }
    }

    print!("Tree after insertion:\n");
    print_in_order(&tree);
    println!("");

    delete_even_numbers(&mut tree);

    print!("Tree after deleting even numbers:\n");
    print_in_order(&tree);
    println!("");

    if let Some(node) = find_min(&tree) {
        println!("Min is {}, Max is {}", retrieve(node), retrieve(find_max(&tree).unwrap()));
    } else {
        println!("Tree is empty");
    }
}