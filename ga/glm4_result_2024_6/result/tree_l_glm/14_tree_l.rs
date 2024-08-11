use std::io::{self, BufRead};

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
            self.left.get_or_insert_with(|| Box::new(TreeNode::new(x)));
        } else {
            self.right.get_or_insert_with(|| Box::new(TreeNode::new(x)));
        }
    }

    fn delete(&mut self, x: i32) {
        if let Some(left) = self.left.as_mut() {
            if x < self.element {
                left.delete(x);
            }
        }
        if let Some(right) = self.right.as_mut() {
            if x > self.element {
                right.delete(x);
            }
        }
        if x == self.element {
            if self.left.is_none() {
                if let Some(right) = self.right.take() {
                    *self = *right;
                } else {
                    self.make_empty();
                }
            } else if self.right.is_none() {
                if let Some(left) = self.left.take() {
                    *self = *left;
                } else {
                    self.make_empty();
                }
            } else {
                if let Some(min) = self.right.as_ref().and_then(|right| right.find_min()) {
                    self.element = min.element;
                    self.right.as_mut().unwrap().delete(min.element);
                }
            }
        }
    }

    fn retrieve(&self) -> i32 {
        self.element
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

    fn delete_even_numbers(&mut self) {
        if let Some(left) = self.left.as_mut() {
            left.delete_even_numbers();
        }
        if let Some(right) = self.right.as_mut() {
            right.delete_even_numbers();
        }
        if self.element % 2 == 0 {
            self.delete(self.element);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().and_then(|line| line.ok());
    let mut tree = None;

    if let Some(line) = line {
        let mut tokens = line.split_whitespace();
        while let Some(token) = tokens.next() {
            if let Ok(num) = token.parse::<i32>() {
                if tree.is_none() {
                    tree = Some(Box::new(TreeNode::new(num)));
                } else {
                    tree.as_mut().unwrap().insert(num);
                }
            }
        }
    }

    if let Some(tree) = tree.as_mut() {
        println!("Tree after insertion:");
        tree.print_in_order();

        tree.delete_even_numbers();
        println!("Tree after deleting even numbers:");
        tree.print_in_order();

        if let Some(min) = tree.find_min() {
            if let Some(max) = tree.find_max() {
                println!("Min is {}, Max is {}", min.retrieve(), max.retrieve());
            }
        }
    } else {
        println!("Tree is empty");
    }
}