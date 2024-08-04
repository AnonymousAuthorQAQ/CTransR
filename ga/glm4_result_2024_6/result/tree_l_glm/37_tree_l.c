use std::io;
use std::collections::Vec;

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

fn make_empty(tree: &mut Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        make_empty(&mut node.left);
        make_empty(&mut node.right);
        tree.take();
    }
}

fn find(x: i32, tree: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
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

fn find_min(tree: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
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

fn find_max(tree: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    if let Some(node) = tree {
        if let None = node.right {
            Some(node)
        } else {
            find_max(&node.right)
        }
    } else {
        None
    }
}

fn insert(x: i32, tree: &mut Option<Box<TreeNode>>) {
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

fn delete(x: i32, tree: &mut Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        if x < node.element {
            delete(x, &mut node.left);
        } else if x > node.element {
            delete(x, &mut node.right);
        } else if let Some(left) = node.left.take() {
            if let Some(right) = node.right.take() {
                *tree = Some(left);
                insert(node.element, tree);
                insert(right.element, tree);
            } else {
                *tree = Some(left);
            }
        } else if let Some(right) = node.right.take() {
            *tree = Some(right);
        } else {
            *tree = None;
        }
    }
}

fn retrieve(node: &Box<TreeNode>) -> i32 {
    node.element
}

fn print_in_order(tree: &Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn delete_even_numbers(tree: &mut Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        delete_even_numbers(&mut node.left);
        delete_even_numbers(&mut node.right);
        if node.element % 2 == 0 {
            delete(node.element, tree);
        }
    }
}

fn main() {
    let mut tree: Option<Box<TreeNode>> = None;

    println!("Enter space-separated integers:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let tokens: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();

    for token in tokens {
        insert(token, &mut tree);
    }

    println!("Tree after insertion:");
    print_in_order(&tree);

    delete_even_numbers(&mut tree);

    println!("Tree after deleting even numbers:");
    print_in_order(&tree);

    if let Some(node) = tree {
        println!("Min is {}, Max is {}", retrieve(node.left.as_ref().unwrap()), retrieve(node.right.as_ref().unwrap()));
    } else {
        println!("Tree is empty");
    }

    make_empty(&mut tree);
}