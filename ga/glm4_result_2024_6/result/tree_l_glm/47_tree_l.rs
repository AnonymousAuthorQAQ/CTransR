use std::io::{self, Read};

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

    fn make_empty(&mut self) {
        if let Some(ref mut left) = self.left {
            left.make_empty();
        }
        if let Some(ref mut right) = self.right {
            right.make_empty();
        }
        self.left = None;
        self.right = None;
    }

    fn find(&self, x: i32) -> Option<&Self> {
        if self.element == x {
            Some(self)
        } else if x < self.element {
            self.left.as_ref().and_then(|left| left.find(x))
        } else {
            self.right.as_ref().and_then(|right| right.find(x))
        }
    }

    fn find_min(&self) -> Option<&Self> {
        self.left.as_ref().map_or(self, |left| left.find_min())
    }

    fn find_max(&self) -> Option<&Self> {
        self.right.as_ref().and_then(|right| {
            if let Some(node) = right.find_max() {
                Some(node)
            } else {
                Some(self)
            }
        })
    }

    fn insert(&mut self, x: i32) {
        if x < self.element {
            self.left.get_or_insert_with(|| Box::new(TreeNode::new(x)));
        } else if x > self.element {
            self.right.get_or_insert_with(|| Box::new(TreeNode::new(x)));
        }
    }

    fn delete(&mut self, x: i32) -> bool {
        if let Some(mut node) = self.left.take() {
            if node.element < x {
                self.left = Some(node);
                if let Some(right) = node.right.take() {
                    node.right = Some(right);
                }
                if let Some(mut right) = self.right.take() {
                    node.right = Some(right);
                    return node.delete(x);
                }
            } else {
                self.left = node.delete(x);
            }
        } else if let Some(mut node) = self.right.take() {
            if node.element > x {
                self.right = Some(node);
                if let Some(left) = node.left.take() {
                    node.left = Some(left);
                }
                if let Some(mut left) = self.left.take() {
                    node.left = Some(left);
                    return node.delete(x);
                }
            } else {
                self.right = node.delete(x);
            }
        }
        false
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut root: Option<Box<TreeNode>> = None;
    for token in input.split_whitespace() {
        if let Ok(x) = token.parse::<i32>() {
            let mut node = root.take().unwrap_or_else(|| Box::new(TreeNode::new(x)));
            node.insert(x);
            root = Some(node);
        }
    }

    if let Some(mut node) = root {
        node.delete_even_numbers();
        println!("Tree after deleting even numbers:");
        node.print_in_order();
    } else {
        println!("Tree is empty");
    }
}

impl TreeNode {
    fn delete_even_numbers(&mut self) {
        if let Some(mut left) = self.left.take() {
            left.delete_even_numbers();
            self.left = Some(left);
        }
        if let Some(mut right) = self.right.take() {
            right.delete_even_numbers();
            self.right = Some(right);
        }
        if self.element % 2 == 0 {
            self.delete(self.element);
        }
    }

    fn print_in_order(&self) {
        if let Some(left) = self.left.as_ref() {
            left.print_in_order();
        }
        println!("{}", self.element);
        if let Some(right) = self.right.as_ref() {
            right.print_in_order();
        }
    }
}