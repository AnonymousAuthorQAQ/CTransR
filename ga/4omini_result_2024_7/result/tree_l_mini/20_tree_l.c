use std::io::{self, BufRead};
use std::ptr::null_mut;

type ElementType = i32;

#[derive(Debug)]
struct TreeNode {
    element: ElementType,
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

    fn insert(&mut self, x: ElementType) {
        self.root = Self::insert_node(self.root.take(), x);
    }

    fn insert_node(node: Option<Box<TreeNode>>, x: ElementType) -> Option<Box<TreeNode>> {
        if let Some(mut n) = node {
            if x < n.element {
                n.left = Self::insert_node(n.left, x);
            } else if x > n.element {
                n.right = Self::insert_node(n.right, x);
            }
            Some(n)
        } else {
            Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            }))
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

    fn delete_node(mut node: Box<TreeNode>) -> Option<Box<TreeNode>> {
        match (node.left.take(), node.right.take()) {
            (None, None) => None, // No children
            (Some(left), None) => Some(left), // Only left child
            (None, Some(right)) => Some(right), // Only right child
            (Some(left), Some(right)) => {
                let min = Self::find_min(right);
                node.element = min.element;
                node.right = Self::delete_node(right);
                Some(node)
            }
        }
    }

    fn find_min(node: Box<TreeNode>) -> Box<TreeNode> {
        let mut current = node;
        while let Some(left) = current.left {
            current = left;
        }
        current
    }

    fn find_max(node: &Option<Box<TreeNode>>) -> Option<ElementType> {
        let mut current = node;
        while let Some(ref n) = current {
            current = &n.right;
        }
        current.as_ref().map(|n| n.element)
    }

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
}

fn main() {
    let mut tree = SearchTree::new();
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input = reader.lines().next().unwrap().unwrap();
    let numbers: Vec<&str> = input.split_whitespace().collect();

    let mut has_input = false;

    for token in numbers {
        if let Ok(num) = token.parse::<ElementType>() {
            tree.insert(num);
            has_input = true;
        }
    }

    if !has_input {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    tree.print_in_order();

    // Delete all even numbers
    tree.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    tree.print_in_order();

    // Find min and max
    if let Some(min) = tree.root.as_ref().map(SearchTree::find_min) {
        println!("Min is {}", min.element);
    } else {
        println!("Tree is empty after deletion");
    }

    if let Some(max) = SearchTree::find_max(&tree.root) {
        println!("Max is {}", max);
    } else {
        println!("Tree is empty after deletion");
    }
}