use std::io::{self, Write};
use std::ptr;

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

struct SearchTree {
    root: Option<Box<TreeNode>>,
}

impl SearchTree {
    fn new() -> Self {
        SearchTree { root: None }
    }

    fn in_order(&self) {
        Self::print_in_order(&self.root);
    }

    fn print_in_order(node: &Option<Box<TreeNode>>) {
        if let Some(ref n) = node {
            Self::print_in_order(&n.left);
            println!("{}", n.element);
            Self::print_in_order(&n.right);
        }
    }

    fn insert(&mut self, element: i32) {
        self.root = Self::insert_node(self.root.take(), element);
    }

    fn insert_node(node: Option<Box<TreeNode>>, element: i32) -> Option<Box<TreeNode>> {
        match node {
            None => Some(Box::new(TreeNode::new(element))),
            Some(mut n) => {
                if element < n.element {
                    n.left = Self::insert_node(n.left, element);
                } else if element > n.element {
                    n.right = Self::insert_node(n.right, element);
                }
                Some(n)
            }
        }
    }

    fn find_min(&self) -> Option<i32> {
        self.root.as_ref().and_then(|n| Self::find_min_node(n))
    }

    fn find_min_node(node: &Box<TreeNode>) -> Option<i32> {
        if let Some(ref left) = node.left {
            return Self::find_min_node(left);
        }
        Some(node.element)
    }

    fn find_max(&self) -> Option<i32> {
        self.root.as_ref().and_then(|n| Self::find_max_node(n))
    }

    fn find_max_node(node: &Box<TreeNode>) -> Option<i32> {
        if let Some(ref right) = node.right {
            return Self::find_max_node(right);
        }
        Some(node.element)
    }

    fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even_node(self.root.take());
    }

    fn delete_even_node(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        let mut node = node?;
        node.left = Self::delete_even_node(node.left);
        node.right = Self::delete_even_node(node.right);

        if node.element % 2 == 0 {
            return Self::delete_node(node);
        }

        Some(node)
    }

    fn delete_node(node: Box<TreeNode>) -> Option<Box<TreeNode>> {
        let mut new_node = None;
        
        match (node.left, node.right) {
            (None, None) => {} // Leaf node gets removed.
            (Some(left), None) => new_node = Some(left),
            (None, Some(right)) => new_node = Some(right),
            (Some(left), Some(right)) => {
                let min_value = Self::find_min_node(&right)?;
                new_node = Self::delete_node_map(right, min_value);
                new_node = Some(Box::new(TreeNode {
                    element: min_value,
                    left: Some(left),
                    right: new_node,
                }));
            }
        }

        new_node
    }

    fn delete_node_map(node: Box<TreeNode>, value: i32) -> Option<Box<TreeNode>> {
        if node.element == value {
            return None; // Just remove the node with matched value.
        }

        let left = if let Some(left_child) = node.left {
            Self::delete_node_map(left_child, value).map(|n| Some(n))
        } else {
            None
        };

        let right = if let Some(right_child) = node.right {
            Self::delete_node_map(right_child, value).map(|n| Some(n))
        } else {
            None
        };

        Some(Box::new(TreeNode {
            element: node.element,
            left,
            right,
        }))
    }
}

fn main() {
    let mut tree = SearchTree::new();
    let mut input = String::new();

    print!("Please enter numbers separated by spaces: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    input.split_whitespace().for_each(|token| {
        if let Ok(num) = token.parse::<i32>() {
            tree.insert(num);
        }
    });

    println!("Tree after insertion:");
    tree.in_order();

    tree.delete_even_numbers();
    println!("Tree after deleting even numbers:");
    tree.in_order();

    match tree.find_min() {
        Some(min) => print!("Min is: {}\n", min),
        None => print!("Tree is empty\n"),
    }

    match tree.find_max() {
        Some(max) => print!("Max is: {}\n", max),
        None => print!("Tree is empty\n"),
    }
}