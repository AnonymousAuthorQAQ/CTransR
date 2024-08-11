use std::io::{self, BufRead};
use std::ptr;

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
    fn new() -> SearchTree {
        SearchTree { root: None }
    }

    fn insert(&mut self, x: i32) {
        self.root = Self::insert_node(self.root.take(), x);
    }

    fn insert_node(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_node(n.left, x);
                } else if x > n.element {
                    n.right = Self::insert_node(n.right, x);
                }
                Some(n)
            }
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
        }
    }

    fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even_numbers_node(self.root.take());
    }

    fn delete_even_numbers_node(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                n.left = Self::delete_even_numbers_node(n.left);
                n.right = Self::delete_even_numbers_node(n.right);
                if n.element % 2 == 0 {
                    return Self::delete_node(n);
                }
                Some(n)
            }
            None => None,
        }
    }

    fn delete_node(node: Box<TreeNode>) -> Option<Box<TreeNode>> {
        if node.left.is_none() {
            return node.right;
        } else if node.right.is_none() {
            return node.left;
        }

        let min_node = SearchTree::find_min(&node.right);
        let new_root = Box::new(TreeNode {
            element: min_node.element,
            left: node.left,
            right: Self::delete_node(min_node),
        });
        Some(new_root)
    }

    fn find_min(node: &Option<Box<TreeNode>>) -> &Box<TreeNode> {
        match node {
            Some(n) => {
                if n.left.is_none() {
                    n
                } else {
                    Self::find_min(&n.left)
                }
            }
            None => panic!("Trying to find min in an empty tree"),
        }
    }

    fn find_max(node: &Option<Box<TreeNode>>) -> &Box<TreeNode> {
        match node {
            Some(n) => {
                if n.right.is_none() {
                    n
                } else {
                    Self::find_max(&n.right)
                }
            }
            None => panic!("Trying to find max in an empty tree"),
        }
    }

    fn print_in_order(node: &Option<Box<TreeNode>>) {
        if let Some(ref n) = node {
            Self::print_in_order(&n.left);
            println!("{}", n.element);
            Self::print_in_order(&n.right);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut tree = SearchTree::new();

    println!("Enter integers (space-separated):");
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    for &num in &numbers {
        tree.insert(num);
    }

    println!("Tree after insertion:");
    SearchTree::print_in_order(&tree.root);

    // Delete all even numbers
    tree.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    SearchTree::print_in_order(&tree.root);

    if let Some(ref root) = tree.root {
        let min = SearchTree::find_min(&tree.root).element;
        let max = SearchTree::find_max(&tree.root).element;
        println!("Min is {}, Max is {}", min, max);
    } else {
        println!("Tree is empty");
    }
}