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

    // Insert an element 
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

    // Delete an element
    fn delete(&mut self, x: i32) {
        self.root = Self::delete_node(self.root.take(), x);
    }

    fn delete_node(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            None => None,
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete_node(n.left, x);
                } else if x > n.element {
                    n.right = Self::delete_node(n.right, x);
                } else {
                    // Node with one child or no child
                    if n.left.is_none() {
                        return n.right;
                    } else if n.right.is_none() {
                        return n.left;
                    }

                    // Node with two children: get the inorder successor
                    if let Some(min_node) = Self::find_min(&n.right) {
                        n.element = min_node.element;
                        n.right = Self::delete_node(n.right, min_node.element);
                    }
                }
                Some(n)
            }
        }
    }

    // Find the minimum value in the tree
    fn find_min(node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            Some(n) => {
                let mut current = n;
                while let Some(ref left) = current.left {
                    current = left;
                }
                Some(current)
            },
            None => None,
        }
    }

    // Find the maximum value in the tree
    fn find_max(node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        let mut current = node;
        while current.is_some() {
            current = &current.as_ref().unwrap().right;
        }
        current.and_then(|n| Some(n.as_ref()))
    }

    // Print the tree in inorder
    fn print_in_order(&self) {
        Self::in_order_traversal(&self.root);
    }

    fn in_order_traversal(node: &Option<Box<TreeNode>>) {
        if let Some(n) = node {
            Self::in_order_traversal(&n.left);
            println!("{}", n.element);
            Self::in_order_traversal(&n.right);
        }
    }

    // Function to delete even numbers from the tree
    fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even_nodes(self.root.take());
    }

    fn delete_even_nodes(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        if let Some(mut n) = node {
            n.left = Self::delete_even_nodes(n.left);
            n.right = Self::delete_even_nodes(n.right);

            if n.element % 2 == 0 {
                return Self::delete_node(Some(n), n.element);
            }
            Some(n)
        } else {
            None
        }
    }
}

fn main() {
    let mut tree = SearchTree::new();

    // Read input from stdin
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    for token in input.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            tree.insert(num);
        }
    }

    // Print the initial tree
    println!("Tree after insertion:");
    tree.print_in_order();

    // Delete all even numbers
    tree.delete_even_numbers();

    // Print the tree after deleting even numbers
    println!("Tree after deleting even numbers:");
    tree.print_in_order();

    // Print the min and max values
    if let Some(min) = SearchTree::find_min(&tree.root) {
        if let Some(max) = SearchTree::find_max(&tree.root) {
            println!("Min is {}, Max is {}", min.element, max.element);
        } else {
            println!("Tree is empty after deletions.");
        }
    } else {
        println!("Tree is empty after deletions.");
    }
}