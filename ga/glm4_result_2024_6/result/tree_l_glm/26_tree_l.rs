use std::io::{self, BufRead};

type SearchTree = Option<Box<TreeNode>>;

struct TreeNode {
    element: i32,
    left: SearchTree,
    right: SearchTree,
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

fn make_empty(tree: &mut SearchTree) {
    if let Some(node) = tree {
        make_empty(&mut node.left);
        make_empty(&mut node.right);
        *tree = None;
    }
}

fn find(x: i32, tree: &SearchTree) -> Option<&TreeNode> {
    match tree {
        None => None,
        Some(node) => {
            if x < node.element {
                find(x, &node.left)
            } else if x > node.element {
                find(x, &node.right)
            } else {
                Some(node)
            }
        }
    }
}

fn find_min(tree: &SearchTree) -> Option<&TreeNode> {
    match tree {
        None => None,
        Some(node) => {
            if let None = node.left {
                Some(node)
            } else {
                find_min(&node.left)
            }
        }
    }
}

fn find_max(tree: &SearchTree) -> Option<&TreeNode> {
    let mut current = tree;
    while let Some(node) = current {
        if let None = node.right {
            return Some(node);
        }
        current = &node.right;
    }
    None
}

fn insert(x: i32, tree: &mut SearchTree) {
    if let None = tree {
        *tree = Some(Box::new(TreeNode::new(x)));
    } else {
        let node = tree.as_mut().unwrap();
        if x < node.element {
            insert(x, &mut node.left);
        } else if x > node.element {
            insert(x, &mut node.right);
        }
    }
}

fn delete(x: i32, tree: &mut SearchTree) {
    if let Some(mut node) = tree.take() {
        match (node.left.as_ref(), node.right.as_ref()) {
            (None, None) => {
                if node.element == x {
                    return;
                }
            }
            (None, Some(_)) | (Some(_), None) => {
                if node.element == x {
                    *tree = node.left.or(node.right);
                    return;
                }
            }
            (Some(_), Some(_)) => {
                if node.element == x {
                    if let Some(min) = find_min(&node.right) {
                        node.element = min.element;
                        delete(min.element, &mut node.right);
                    }
                    return;
                }
            }
        }
        *tree = Some(node);
    }
}

fn retrieve(node: &TreeNode) -> i32 {
    node.element
}

fn print_in_order(tree: &SearchTree) {
    if let Some(node) = tree {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
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
    let mut tree: SearchTree = None;
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().and_then(|result| result.ok());
    if let Some(line) = line {
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                insert(num, &mut tree);
            }
        }
    }

    print!("Tree after insertion:\n");
    print_in_order(&tree);
    delete_even_numbers(&mut tree);
    print!("Tree after deleting even numbers:\n");
    print_in_order(&tree);

    if let Some(node) = find_min(&tree) {
        println!("Min is {}, Max is {}", retrieve(node), retrieve(find_max(&tree).unwrap()));
    } else {
        println!("Tree is empty");
    }

    make_empty(&mut tree);
}