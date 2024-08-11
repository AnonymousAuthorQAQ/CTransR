use std::io::{self, BufRead};
use std::collections::binary_tree::Node;
use std::collections::binary_tree::Tree;

fn main() {
    let mut tree = Tree::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                tree.insert(num);
            }
        }
    }

    println!("Tree after insertion:");
    tree.print_in_order();

    // Delete all even numbers
    tree.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    tree.print_in_order();

    if !tree.is_empty() {
        println!("Min is {}, Max is {}", tree.find_min().unwrap(), tree.find_max().unwrap());
    } else {
        println!("Tree is empty");
    }
}

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

impl Drop for TreeNode {
    fn drop(&mut self) {
        if let Some(left) = self.left.take() {
            drop(left);
        }
        if let Some(right) = self.right.take() {
            drop(right);
        }
    }
}

#[derive(Debug)]
struct Tree {
    root: Option<Box<TreeNode>>,
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }

    fn insert(&mut self, element: i32) {
        self.root = Some(Tree::insert_recursive(self.root.take(), element));
    }

    fn insert_recursive(mut node: Option<Box<TreeNode>>, element: i32) -> Box<TreeNode> {
        match node {
            None => Box::new(TreeNode::new(element)),
            Some(mut n) => {
                if element < n.element {
                    n.left = Tree::insert_recursive(n.left.take(), element);
                } else {
                    n.right = Tree::insert_recursive(n.right.take(), element);
                }
                n
            }
        }
    }

    fn delete_even_numbers(&mut self) {
        self.root = Tree::delete_even_numbers_recursive(self.root.take());
    }

    fn delete_even_numbers_recursive(mut node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        if let Some(mut n) = node {
            n.left = Tree::delete_even_numbers_recursive(n.left.take());
            n.right = Tree::delete_even_numbers_recursive(n.right.take());
            if n.element % 2 == 0 {
                None
            } else {
                Some(n)
            }
        } else {
            None
        }
    }

    fn print_in_order(&self) {
        if let Some(root) = &self.root {
            Tree::print_in_order_recursive(root);
        }
    }

    fn print_in_order_recursive(node: &Box<TreeNode>) {
        if let Some(left) = &node.left {
            Tree::print_in_order_recursive(left);
        }
        println!("{}", node.element);
        if let Some(right) = &node.right {
            Tree::print_in_order_recursive(right);
        }
    }

    fn find_min(&self) -> Option<i32> {
        Tree::find_min_recursive(&self.root)
    }

    fn find_min_recursive(node: &Option<Box<TreeNode>>) -> Option<i32> {
        node.as_ref().map(|n| {
            if let Some(left) = &n.left {
                Tree::find_min_recursive(Some(left))
            } else {
                Some(n.element)
            }
        }).flatten()
    }

    fn find_max(&self) -> Option<i32> {
        Tree::find_max_recursive(&self.root)
    }

    fn find_max_recursive(node: &Option<Box<TreeNode>>) -> Option<i32> {
        node.as_ref().map(|n| {
            if let Some(right) = &n.right {
                Tree::find_max_recursive(Some(right))
            } else {
                Some(n.element)
            }
        }).flatten()
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}