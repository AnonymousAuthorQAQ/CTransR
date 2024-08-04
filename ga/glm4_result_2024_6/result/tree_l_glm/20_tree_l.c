use std::io::{self, Read};
use std::collections::binary_tree::Node;

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

impl TreeNode {
    fn make_empty(&mut self) {
        if let Some(left) = self.left.take() {
            left.make_empty();
        }
        if let Some(right) = self.right.take() {
            right.make_empty();
        }
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
        self.left.as_ref().and_then(|left| left.find_min()).or(Some(self))
    }

    fn find_max(&self) -> Option<&Self> {
        self.right.as_ref().and_then(|right| right.find_max()).or(Some(self))
    }

    fn insert(&mut self, x: i32) {
        if x < self.element {
            if self.left.is_none() {
                self.left = Some(Box::new(TreeNode::new(x)));
            } else {
                self.left.as_mut().unwrap().insert(x);
            }
        } else {
            if self.right.is_none() {
                self.right = Some(Box::new(TreeNode::new(x)));
            } else {
                self.right.as_mut().unwrap().insert(x);
            }
        }
    }

    fn delete(&mut self, x: i32) -> Option<Box<TreeNode>> {
        if x < self.element {
            self.left = self.left.take().and_then(|left| {
                let node = left.delete(x);
                if node.is_none() {
                    None
                } else {
                    Some(node)
                }
            });
        } else if x > self.element {
            self.right = self.right.take().and_then(|right| {
                let node = right.delete(x);
                if node.is_none() {
                    None
                } else {
                    Some(node)
                }
            });
        } else {
            if self.left.is_none() {
                return Some(Box::new(TreeNode {
                    element: self.element,
                    left: None,
                    right: self.right.take(),
                }));
            } else if self.right.is_none() {
                return Some(Box::new(TreeNode {
                    element: self.element,
                    left: self.left.take(),
                    right: None,
                }));
            }

            let mut right_min = self.right.as_mut().unwrap();
            while right_min.left.is_some() {
                right_min = right_min.left.as_mut().unwrap();
            }

            self.element = right_min.element;
            self.right = right_min.delete(right_min.element);
        }

        None
    }

    fn retrieve(&self) -> i32 {
        self.element
    }

    fn print_in_order(&self) {
        if let Some(left) = &self.left {
            left.print_in_order();
        }
        println!("{}", self.element);
        if let Some(right) = &self.right {
            right.print_in_order();
        }
    }

    fn delete_even_numbers(&mut self) {
        if let Some(left) = self.left.take() {
            left.delete_even_numbers();
            self.left = Some(left);
        }
        if let Some(right) = self.right.take() {
            right.delete_even_numbers();
            self.right = Some(right);
        }
        if self.element % 2 == 0 {
            self.delete(self.element);
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut tree: Option<Box<TreeNode>> = None;

    for token in input.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            let mut node = Box::new(TreeNode::new(num));
            if let Some(mut root) = tree.take() {
                root.insert(num);
                node = root;
            }
            tree = Some(node);
        }
    }

    if let Some(mut tree) = tree {
        println!("Tree after insertion:");
        tree.print_in_order();

        tree.delete_even_numbers();
        println!("Tree after deleting even numbers:");
        tree.print_in_order();

        if let Some(min) = tree.find_min() {
            println!("Min is {}, Max is {}", min.retrieve(), tree.find_max().unwrap().retrieve());
        } else {
            println!("Tree is empty");
        }
    } else {
        println!("Tree is empty");
    }
}