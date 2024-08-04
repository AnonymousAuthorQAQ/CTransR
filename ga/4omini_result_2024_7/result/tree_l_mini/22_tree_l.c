use std::io::{self, BufRead};
use std::cmp::Ord;

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
                    n.left = Self::insert_node(n.left.take(), x);
                } else if x > n.element {
                    n.right = Self::insert_node(n.right.take(), x);
                }
                Some(n)
            }
        }
    }

    fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even(self.root.take());
    }

    fn delete_even(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        if let Some(mut n) = node {
            n.left = Self::delete_even(n.left);
            n.right = Self::delete_even(n.right);

            if n.element % 2 == 0 {
                return Self::delete_node(n);
            }
            Some(n)
        } else {
            None
        }
    }

    fn delete_node(node: Box<TreeNode>) -> Option<Box<TreeNode>> {
        if node.left.is_none() {
            return node.right;
        } else if node.right.is_none() {
            return node.left;
        } else {
            let min_node = Self::find_min(node.right.as_ref().unwrap());
            let mut new_node = Box::new(TreeNode {
                element: min_node.element,
                left: node.left,
                right: Self::delete_node(node.right),
            });
            Some(new_node)
        }
    }

    fn find_min(node: &Box<TreeNode>) -> &Box<TreeNode> {
        if let Some(ref left) = node.left {
            Self::find_min(left)
        } else {
            node
        }
    }

    fn find_max(node: &Box<TreeNode>) -> &Box<TreeNode> {
        if let Some(ref right) = node.right {
            Self::find_max(right)
        } else {
            node
        }
    }

    fn print_in_order(&self) {
        Self::print_in_order_node(&self.root);
    }

    fn print_in_order_node(node: &Option<Box<TreeNode>>) {
        if let Some(ref n) = node {
            Self::print_in_order_node(&n.left);
            println!("{}", n.element);
            Self::print_in_order_node(&n.right);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut tree = SearchTree::new();
    
    println!("Enter integers (space-separated):");
    let line = stdin.lock().lines().next().unwrap().unwrap();
    
    for token in line.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            tree.insert(num);
        }
    }

    println!("Tree after insertion:");
    tree.print_in_order();

    // Delete all even numbers
    tree.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    tree.print_in_order();

    if let Some(root) = &tree.root {
        println!(
            "Min is {}, Max is {}",
            SearchTree::find_min(root).element,
            SearchTree::find_max(root).element
        );
    } else {
        println!("Tree is empty");
    }
}