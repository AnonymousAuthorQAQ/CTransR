use std::io;
use std::collections::VecDeque;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(element: ElementType) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
}

fn make_empty(tree: &mut Option<Box<TreeNode>>) {
    if let Some(mut node) = tree.take() {
        make_empty(&mut node.left);
        make_empty(&mut node.right);
    }
}

fn find(x: ElementType, tree: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
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

fn find_min(tree: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
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

fn find_max(tree: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
    match tree {
        Some(node) => {
            if node.right.is_none() {
                Some(node)
            } else {
                find_max(&node.right)
            }
        }
        None => None,
    }
}

fn insert(x: ElementType, tree: &mut Option<Box<TreeNode>>) {
    if tree.is_none() {
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

fn delete(x: ElementType, tree: &mut Option<Box<TreeNode>>) {
    if let Some(mut node) = tree.take() {
        if x < node.element {
            delete(x, &mut node.left);
            *tree = Some(node);
        } else if x > node.element {
            delete(x, &mut node.right);
            *tree = Some(node);
        } else if node.left.is_none() {
            *tree = node.right;
        } else if node.right.is_none() {
            *tree = node.left;
        } else {
            let mut right_min = node.right.take().unwrap();
            node.element = right_min.element;
            delete(right_min.element, &mut right_min.right);
            node.right = Some(right_min);
            *tree = Some(node);
        }
    }
}

fn retrieve(node: &TreeNode) -> ElementType {
    node.element
}

fn print_in_order(tree: &Option<Box<TreeNode>>) {
    let mut stack = VecDeque::new();
    let mut current = tree;

    while current.is_some() || !stack.is_empty() {
        while let Some(node) = current {
            stack.push_back(node);
            current = &node.left;
        }
        if let Some(node) = stack.pop_back() {
            println!("{}", node.element);
            current = &node.right;
        }
    }
}

fn delete_even_numbers(tree: &mut Option<Box<TreeNode>>) {
    if let Some(mut node) = tree.take() {
        delete_even_numbers(&mut node.left);
        delete_even_numbers(&mut node.right);
        if node.element % 2 == 0 {
            delete(node.element, tree);
        } else {
            *tree = Some(node);
        }
    }
}

fn main() {
    let mut tree: Option<Box<TreeNode>> = None;
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    for token in input.trim().split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            insert(num, &mut tree);
        }
    }

    if let Some(ref tree) = tree {
        println!("Tree after insertion:");
        print_in_order(&tree);
    } else {
        println!("Tree is empty");
    }

    delete_even_numbers(&mut tree);

    if let Some(ref tree) = tree {
        println!("Tree after deleting even numbers:");
        print_in_order(&tree);
    } else {
        println!("Tree is empty");
    }
}