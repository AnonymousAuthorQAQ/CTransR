use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

impl TreeNode {
    fn new(element: i32) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i32) {
        if value < self.element {
            if let Some(ref mut left) = self.left {
                left.insert(value);
            } else {
                self.left = Some(Box::new(TreeNode::new(value)));
            }
        } else if value > self.element {
            if let Some(ref mut right) = self.right {
                right.insert(value);
            } else {
                self.right = Some(Box::new(TreeNode::new(value)));
            }
        }
    }

    fn delete_even_numbers(&mut self) {
        if let Some(ref mut left) = self.left {
            left.delete_even_numbers();
        }
        if let Some(ref mut right) = self.right {
            right.delete_even_numbers();
        }
        if self.element % 2 == 0 {
            *self = TreeNode::delete(self);
        }
    }

    fn delete(node: &TreeNode) -> TreeNode {
        if node.left.is_none() {
            return node.right.as_ref().map_or(TreeNode::new(0), |right| (*right).clone());
        }
        if node.right.is_none() {
            return node.left.as_ref().map_or(TreeNode::new(0), |left| (*left).clone());
        }

        let mut tmp_cell = TreeNode::find_min(node.right.as_ref().unwrap().as_ref());
        let mut new_node = node.clone();
        new_node.element = tmp_cell.element;
        new_node.right = Some(Box::new(TreeNode::delete(tmp_cell)));
        new_node
    }

    fn find_min(node: &TreeNode) -> &TreeNode {
        if let Some(ref left) = node.left {
            TreeNode::find_min(left)
        } else {
            node
        }
    }

    fn print_in_order(&self) {
        if let Some(ref left) = self.left {
            left.print_in_order();
        }
        println!("{}", self.element);
        if let Some(ref right) = self.right {
            right.print_in_order();
        }
    }

    fn print_min_max(&self) {
        let min = TreeNode::find_min(self).element;
        let max = TreeNode::find_max(self).element;
        println!("Min is {}, Max is {}", min, max);
    }

    fn find_max(node: &TreeNode) -> &TreeNode {
        let mut current = node;
        while let Some(ref right) = current.right {
            current = right;
        }
        current
    }
}

fn main() {
    let mut root: SearchTree = None;
    let stdin = io::stdin();
    let mut tree_values = Vec::new();

    for line in stdin.lock().lines() {
        if let Ok(input) = line {
            let tokens: Vec<&str> = input.split_whitespace().collect();
            for token in tokens {
                if let Ok(num) = i32::from_str(token) {
                    tree_values.push(num);
                }
            }
        }
    }

    for &value in tree_values.iter() {
        if let Some(ref mut node) = root {
            node.insert(value);
        } else {
            root = Some(Box::new(TreeNode::new(value)));
        }
    }

    if let Some(ref node) = root {
        println!("Tree after insertion:");
        node.print_in_order();
        node.delete_even_numbers();
        println!("Tree after deleting even numbers:");
        node.print_in_order();
        node.print_min_max();
    } else {
        println!("Tree is empty");
    }
}