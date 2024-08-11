use std::io::{self, BufRead};
use std::collections::VecDeque;

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

    fn make_empty(&mut self) {
        self.root = None;
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
            None => {
                Some(Box::new(TreeNode {
                    element: x,
                    left: None,
                    right: None,
                }))
            }
        }
    }

    fn delete(&mut self, x: i32) {
        self.root = Self::delete_node(self.root.take(), x);
    }

    fn delete_node(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete_node(n.left, x);
                    Some(n)
                } else if x > n.element {
                    n.right = Self::delete_node(n.right, x);
                    Some(n)
                } else {
                    // Node with one child or no child
                    if n.left.is_none() {
                        return n.right;
                    } else if n.right.is_none() {
                        return n.left;
                    }
                    // Node with two children
                    let min_node = Self::find_min(n.right.as_ref().unwrap());
                    n.element = min_node.element;
                    n.right = Self::delete_node(n.right, min_node.element);
                    Some(n)
                }
            }
            None => None,
        }
    }

    fn find_min(node: &TreeNode) -> &TreeNode {
        if let Some(ref left) = node.left {
            Self::find_min(left)
        } else {
            node
        }
    }

    fn find_max(node: &TreeNode) -> &TreeNode {
        let mut current = node;
        while let Some(ref right) = current.right {
            current = right;
        }
        current
    }

    fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even_node(self.root.take());
    }

    fn delete_even_node(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                n.left = Self::delete_even_node(n.left);
                n.right = Self::delete_even_node(n.right);

                if n.element % 2 == 0 {
                    return Self::delete_node(Some(n), n.element);
                }
                Some(n)
            }
            None => None,
        }
    }

    fn print_in_order(&self) {
        Self::in_order(self.root.as_ref());
    }

    fn in_order(node: Option<&Box<TreeNode>>) {
        if let Some(n) = node {
            Self::in_order(n.left.as_ref());
            println!("{}", n.element);
            Self::in_order(n.right.as_ref());
        }
    }

    fn retrieve_min(&self) -> Option<i32> {
        self.root.as_ref().map(|node| Self::find_min(node).element)
    }

    fn retrieve_max(&self) -> Option<i32> {
        self.root.as_ref().map(|node| Self::find_max(node).element)
    }
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    let mut tree = SearchTree::new();

    for token in line.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            tree.insert(num);
        }
    }

    if tree.root.is_none() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    tree.print_in_order();

    // Delete all even numbers
    tree.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    tree.print_in_order();

    match (tree.retrieve_min(), tree.retrieve_max()) {
        (Some(min), Some(max)) => {
            println!("Min is {}, Max is {}", min, max);
        }
        _ => {
            println!("Tree is empty");
        }
    }

    tree.make_empty();
}