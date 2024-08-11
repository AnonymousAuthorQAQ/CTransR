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

fn make_empty(t: &mut Option<Box<TreeNode>>) {
    if let Some(node) = t {
        make_empty(&mut node.left);
        make_empty(&mut node.right);
        t.take();
    }
}

fn find(x: i32, t: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    t.as_ref().and_then(|node| {
        if x < node.element {
            find(x, &node.left)
        } else if x > node.element {
            find(x, &node.right)
        } else {
            Some(node)
        }
    })
}

fn find_min(t: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    t.as_ref().and_then(|node| {
        if let Some(min) = find_min(&node.left) {
            Some(min)
        } else {
            Some(node)
        }
    })
}

fn find_max(t: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    t.as_ref().and_then(|node| {
        if let Some(max) = find_max(&node.right) {
            Some(max)
        } else {
            Some(node)
        }
    })
}

fn insert(x: i32, t: &mut Option<Box<TreeNode>>) {
    if let None = t {
        *t = Some(Box::new(TreeNode::new(x)));
    } else {
        if x < t.as_ref().unwrap().element {
            insert(x, &mut t.as_mut().unwrap().left);
        } else if x > t.as_ref().unwrap().element {
            insert(x, &mut t.as_mut().unwrap().right);
        }
    }
}

fn delete(x: i32, t: &mut Option<Box<TreeNode>>) {
    if let Some(node) = t {
        if x < node.element {
            delete(x, &mut node.left);
        } else if x > node.element {
            delete(x, &mut node.right);
        } else if node.left.is_some() && node.right.is_some() {
            if let Some(min) = find_min(&node.right) {
                node.element = min.element;
                delete(min.element, &mut node.right);
            }
        } else {
            *t = node.left.or(node.right);
        }
    }
}

fn retrieve(p: &Box<TreeNode>) -> i32 {
    p.element
}

fn print_in_order(t: &Option<Box<TreeNode>>) {
    if let Some(node) = t {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn delete_even_numbers(t: &mut Option<Box<TreeNode>>) {
    if let Some(node) = t {
        delete_even_numbers(&mut node.left);
        delete_even_numbers(&mut node.right);
        if node.element % 2 == 0 {
            delete(node.element, t);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().and_then(|line| line.ok());
    let mut tree = None;

    if let Some(line) = line {
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                insert(num, &mut tree);
            }
        }
    }

    if tree.is_none() {
        println!("Tree is empty");
    } else {
        println!("Tree after insertion:");
        print_in_order(&tree);
        delete_even_numbers(&mut tree);
        println!("Tree after deleting even numbers:");
        print_in_order(&tree);
        if let Some(node) = find_min(&tree) {
            println!("Min is {}, Max is {}", retrieve(node), retrieve(find_max(&tree).unwrap()));
        } else {
            println!("Tree is empty");
        }
    }

    make_empty(&mut tree);
}