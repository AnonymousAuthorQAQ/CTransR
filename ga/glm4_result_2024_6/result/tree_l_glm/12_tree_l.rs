use std::io::{self, Read};
use std::str::FromStr;

type SearchTree = Option<Box<TreeNode>>;

#[derive(Debug)]
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
        tree.take();
    }
}

fn find(x: i32, tree: &SearchTree) -> Option<&TreeNode> {
    if let Some(node) = tree {
        if x < node.element {
            find(x, &node.left)
        } else if x > node.element {
            find(x, &node.right)
        } else {
            Some(node)
        }
    } else {
        None
    }
}

fn find_min(tree: &SearchTree) -> Option<&TreeNode> {
    if let Some(node) = tree {
        if let None = node.left {
            Some(node)
        } else {
            find_min(&node.left)
        }
    } else {
        None
    }
}

fn find_max(tree: &SearchTree) -> Option<&TreeNode> {
    if let Some(node) = tree {
        let mut current = node;
        while let Some(right) = &current.right {
            current = right;
        }
        Some(current)
    } else {
        None
    }
}

fn insert(x: i32, tree: &mut SearchTree) {
    if let None = tree {
        *tree = Some(Box::new(TreeNode::new(x)));
    } else {
        if x < tree.as_ref().unwrap().element {
            insert(x, &mut tree.as_mut().unwrap().left);
        } else if x > tree.as_ref().unwrap().element {
            insert(x, &mut tree.as_mut().unwrap().right);
        }
    }
}

fn delete(x: i32, tree: &mut SearchTree) {
    if let Some(mut node) = tree.take() {
        if x < node.element {
            delete(x, &mut node.left);
            *tree = Some(node);
        } else if x > node.element {
            delete(x, &mut node.right);
            *tree = Some(node);
        } else if let Some(right) = node.right.take() {
            if let Some(min) = find_min(&right) {
                node.element = min.element;
                delete(min.element, &mut right);
                node.right = Some(right);
            } else {
                node.left = right.left;
                node.right = right.right;
            }
            *tree = Some(node);
        } else if let Some(left) = node.left.take() {
            *tree = Some(left);
        }
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut tree: SearchTree = None;
    for &num in &tokens {
        insert(num, &mut tree);
    }

    if let Some(node) = tree {
        println!("Tree after insertion:");
        print_in_order(&node);
    } else {
        println!("Tree is empty");
    }

    delete_even_numbers(&mut tree);

    if let Some(node) = tree {
        println!("Tree after deleting even numbers:");
        print_in_order(&node);
        println!("Min is {}, Max is {}", retrieve(find_min(&tree).unwrap()), retrieve(find_max(&tree).unwrap()));
    } else {
        println!("Tree is empty");
    }

    make_empty(&mut tree);
}