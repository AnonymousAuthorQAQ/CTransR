use std::cmp::Ord;
use std::io::{self, BufRead};
use std::ptr;
use std::vec::Vec;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

struct SearchTree {
    root: Option<Box<TreeNode>>,
}

impl SearchTree {
    fn new() -> Self {
        SearchTree { root: None }
    }

    fn insert(&mut self, x: i32) {
        self.root = Self::insert_node(self.root.take(), x);
    }

    fn insert_node(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_node(n.left, x);
                } else if x > n.element {
                    n.right = Self::insert_node(n.right, x);
                }
                Some(n)
            }
        }
    }

    fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even_node(self.root.take());
    }

    fn delete_even_node(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            None => None,
            Some(mut n) => {
                n.left = Self::delete_even_node(n.left);
                n.right = Self::delete_even_node(n.right);

                if n.element % 2 == 0 {
                    return Self::join(n.left, n.right);
                }
                Some(n)
            }
        }
    }

    fn join(left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match (left, right) {
            (None, None) => None,
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (Some(l), Some(r)) => {
                let min_node = Self::find_min(&r);
                let new_root = min_node.clone();
                let new_right = Self::delete_node(r, new_root.element);
                Some(Box::new(TreeNode {
                    element: new_root.element,
                    left: Some(l),
                    right: new_right,
                }))
            }
        }
    }

    fn find_min(node: &Option<Box<TreeNode>>) -> &Box<TreeNode> {
        match node {
            Some(n) if n.left.is_some() => Self::find_min(&n.left),
            Some(n) => n,
            None => panic!("Tree is empty"),
        }
    }

    fn delete_node(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            None => return None,
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete_node(n.left, x);
                } else if x > n.element {
                    n.right = Self::delete_node(n.right, x);
                } else {
                    // Node with the value x found
                    if n.left.is_none() {
                        return n.right;
                    } else if n.right.is_none() {
                        return n.left;
                    } else {
                        let min_node = Self::find_min(&n.right);
                        n.element = min_node.element;
                        n.right = Self::delete_node(n.right, min_node.element);
                    }
                }
                Some(n)
            }
        }
    }

    fn print_in_order(&self) {
        Self::in_order(&self.root);
    }

    fn in_order(node: &Option<Box<TreeNode>>) {
        if let Some(n) = node {
            Self::in_order(&n.left);
            println!("{}", n.element);
            Self::in_order(&n.right);
        }
    }

    fn find_min(&self) -> Option<i32> {
        self.root.as_ref().map(|node| Self::find_min(&Some(node.clone())).element)
    }

    fn find_max(&self) -> Option<i32> {
        self.root.as_ref().map(|node| Self::find_max(&Some(node.clone())).element)
    }

    fn find_max(node: &Option<Box<TreeNode>>) -> &Box<TreeNode> {
        match node {
            Some(n) if n.right.is_some() => Self::find_max(&n.right),
            Some(n) => n,
            None => panic!("Tree is empty"),
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let nums: Vec<i32> = input
        .split_whitespace()
        .filter_map(|token| token.parse::<i32>().ok())
        .collect();

    let mut tree = SearchTree::new();

    for &num in &nums {
        tree.insert(num);
    }

    if nums.is_empty() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    tree.print_in_order();

    // Delete all even numbers
    tree.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    tree.print_in_order();

    match (tree.find_min(), tree.find_max()) {
        (Some(min), Some(max)) => {
            println!("Min is {}, Max is {}", min, max);
        }
        _ => {
            println!("Tree is empty");
        }
    }
}