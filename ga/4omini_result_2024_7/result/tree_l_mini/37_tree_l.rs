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
    fn new() -> Self {
        SearchTree { root: None }
    }

    fn insert(&mut self, x: i32) {
        self.root = Self::insert_rec(self.root.take(), x);
    }

    fn insert_rec(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_rec(n.left, x);
                } else if x > n.element {
                    n.right = Self::insert_rec(n.right, x);
                }
                Some(n)
            }
        }
    }

    fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even_rec(self.root.take());
    }

    fn delete_even_rec(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            None => None,
            Some(mut n) => {
                n.left = Self::delete_even_rec(n.left);
                n.right = Self::delete_even_rec(n.right);
                if n.element % 2 == 0 {
                    return Self::delete_node(n);
                }
                Some(n)
            }
        }
    }

    fn delete_node(node: Box<TreeNode>) -> Option<Box<TreeNode>> {
        match (node.left, node.right) {
            (None, None) => None,
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (Some(left), Some(right)) => {
                let min_node = Self::find_min(right.as_ref());
                let new_node = TreeNode {
                    element: min_node.element,
                    left: left,
                    right: Self::delete_node(right),
                };
                Some(Box::new(new_node))
            }
        }
    }

    fn find_min(node: &Box<TreeNode>) -> &TreeNode {
        if let Some(ref left) = node.left {
            Self::find_min(left)
        } else {
            node
        }
    }

    fn print_in_order(&self) {
        Self::print_in_order_rec(&self.root);
    }

    fn print_in_order_rec(node: &Option<Box<TreeNode>>) {
        if let Some(ref n) = node {
            Self::print_in_order_rec(&n.left);
            println!("{}", n.element);
            Self::print_in_order_rec(&n.right);
        }
    }

    fn find_min(&self) -> Option<i32> {
        Self::find_min_rec(&self.root).map(|node| node.element)
    }

    fn find_min_rec(node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            Some(n) => Some(Self::find_min(n)),
            None => None,
        }
    }

    fn find_max(&self) -> Option<i32> {
        Self::find_max_rec(&self.root).map(|node| node.element)
    }

    fn find_max_rec(node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            Some(n) => {
                if n.right.is_some() {
                    Self::find_max_rec(&n.right)
                } else {
                    Some(n)
                }
            }
            None => None,
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut tree = SearchTree::new();

    for line in handle.lines() {
        let line = line.unwrap();
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                tree.insert(num);
            }
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

    if let (Some(min), Some(max)) = (tree.find_min(), tree.find_max()) {
        println!("Min is {}, Max is {}", min, max);
    } else {
        println!("Tree is empty");
    }
}