use std::io::{self, BufRead};
use std::ptr;
use std::vec::Vec;

type ElementType = i32;

#[derive(Debug)]
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

struct SearchTree {
    root: Option<Box<TreeNode>>,
}

impl SearchTree {
    fn new() -> Self {
        SearchTree { root: None }
    }

    fn insert(&mut self, x: ElementType) {
        self.root = Self::insert_node(self.root.take(), x);
    }

    fn insert_node(node: Option<Box<TreeNode>>, x: ElementType) -> Option<Box<TreeNode>> {
        match node {
            None => Some(Box::new(TreeNode::new(x))),
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
        let mut node = node?;
        node.left = Self::delete_even_node(node.left);
        node.right = Self::delete_even_node(node.right);

        if node.element % 2 == 0 {
            return SearchTree::delete_node(node);
        }

        Some(node)
    }

    fn delete_node(node: Box<TreeNode>) -> Option<Box<TreeNode>> {
        let elem = node.element;
        match (node.left, node.right) {
            (None, None) => None,  // No children
            (Some(left), None) => Some(left),  // Only left child
            (None, Some(right)) => Some(right),  // Only right child
            (Some(left), Some(right)) => {
                let min_node = Self::find_min_node(right);
                let mut new_right = right;
                new_right = Self::delete_node_by_value(new_right, min_node.element);
                let mut new_node = Box::new(TreeNode {
                    element: min_node.element,
                    left: Some(left),
                    right: new_right,
                });
                Some(new_node)
            }
        }
    }

    fn delete_node_by_value(node: Option<Box<TreeNode>>, x: ElementType) -> Option<Box<TreeNode>> {
        match node {
            None => return None,
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete_node_by_value(n.left, x);
                } else if x > n.element {
                    n.right = Self::delete_node_by_value(n.right, x);
                } else {
                    return SearchTree::delete_node(n);
                }
                Some(n)
            }
        }
    }

    fn find_min_node(node: Option<Box<TreeNode>>) -> Box<TreeNode> {
        let mut current = node.unwrap();
        while let Some(left) = current.left {
            current = left;
        }
        current
    }

    fn find_max_node(node: Option<&Box<TreeNode>>) -> Option<ElementType> {
        let mut current = node?;
        while let Some(right) = &current.right {
            current = right;
        }
        Some(current.element)
    }

    fn print_in_order(&self) {
        Self::print_in_order_node(&self.root);
    }

    fn print_in_order_node(node: &Option<Box<TreeNode>>) {
        if let Some(n) = node {
            Self::print_in_order_node(&n.left);
            println!("{}", n.element);
            Self::print_in_order_node(&n.right);
        }
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut tree = SearchTree::new();

    println!("Enter numbers (separated by spaces):");
    let line = stdin.lock().lines().next().unwrap().unwrap();
    for token in line.split_whitespace() {
        if let Ok(num) = token.parse::<ElementType>() {
            tree.insert(num);
        }
    }

    if tree.is_empty() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    tree.print_in_order();

    // Delete all even numbers
    tree.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    tree.print_in_order();

    if let Some(min) = tree.root.as_ref().and_then(|n| SearchTree::find_min_node(Some(n.clone())).element) {
        if let Some(max) = tree.root.as_ref().and_then(|n| SearchTree::find_max_node(Some(n))) {
            println!("Min is {}, Max is {}", min, max);
        }
    } else {
        println!("Tree is empty");
    }
}