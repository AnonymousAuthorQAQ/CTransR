use std::io::{self, Read};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq)]
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

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.element.cmp(&other.element)
    }
}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.element == other.element
    }
}

fn make_empty() -> Option<Box<TreeNode>> {
    None
}

fn find(x: i32, tree: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    if let Some(ref node) = tree {
        match x.cmp(&node.element) {
            Ordering::Less => find(x, &node.left),
            Ordering::Greater => find(x, &node.right),
            Ordering::Equal => Some(node),
        }
    } else {
        None
    }
}

fn find_min(tree: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    if let Some(node) = tree {
        if node.left.is_none() {
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
        if node.right.is_none() {
            Some(node)
        } else {
            find_max(&node.right)
        }
    } else {
        None
    }
}

fn insert(x: i32, tree: &mut Option<Box<TreeNode>>) {
    if tree.is_none() {
        *tree = Some(Box::new(TreeNode::new(x)));
    } else {
        let mut node = tree.as_mut().unwrap();
        match x.cmp(&node.element) {
            Ordering::Less => insert(x, &mut node.left),
            Ordering::Greater => insert(x, &mut node.right),
            Ordering::Equal => (),
        }
    }
}

fn delete(x: i32, tree: &mut Option<Box<TreeNode>>) {
    if let Some(mut node) = tree.take() {
        match x.cmp(&node.element) {
            Ordering::Less => {
                let left = delete(x, &mut node.left);
                node.left = left;
                *tree = Some(node);
            }
            Ordering::Greater => {
                let right = delete(x, &mut node.right);
                node.right = right;
                *tree = Some(node);
            }
            Ordering::Equal => {
                if node.left.is_none() {
                    *tree = node.right;
                } else if node.right.is_none() {
                    *tree = node.left;
                } else {
                    let mut min_right = node.right.take().unwrap();
                    node.element = find_min(&Some(min_right)).unwrap().element;
                    let right = delete(node.element, &mut min_right);
                    node.right = right;
                    *tree = Some(node);
                }
            }
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tree: Option<Box<TreeNode>> = None;

    for token in input.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            insert(num, &mut tree);
        }
    }

    print_in_order(&tree);
    delete_even_numbers(&mut tree);
    print_in_order(&tree);

    if let Some(node) = tree {
        println!("Min is {}, Max is {}", retrieve(&node), retrieve(find_max(&tree).unwrap()));
    } else {
        println!("Tree is empty");
    }
}