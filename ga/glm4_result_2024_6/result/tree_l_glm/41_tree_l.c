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
        } else if let Some(left) = &self.left {
            left.find(x)
        } else if let Some(right) = &self.right {
            right.find(x)
        } else {
            None
        }
    }

    fn find_min(&self) -> Option<&Self> {
        if let Some(left) = &self.left {
            left.find_min()
        } else {
            Some(self)
        }
    }

    fn find_max(&self) -> Option<&Self> {
        if let Some(right) = &self.right {
            right.find_max()
        } else {
            Some(self)
        }
    }

    fn insert(&mut self, x: i32) {
        if x < self.element {
            if let None = self.left {
                self.left = Some(Box::new(TreeNode::new(x)));
            } else {
                self.left.as_mut().unwrap().insert(x);
            }
        } else if x > self.element {
            if let None = self.right {
                self.right = Some(Box::new(TreeNode::new(x)));
            } else {
                self.right.as_mut().unwrap().insert(x);
            }
        }
    }

    fn delete(&mut self, x: i32) -> Option<Self> {
        if self.element == x {
            if let Some(left) = self.left.take() {
                if let Some(right) = self.right.take() {
                    let min = right.find_min().unwrap();
                    self.element = min.element;
                    self.left = right.delete(min.element);
                } else {
                    *self = *left;
                }
            } else if let Some(right) = self.right.take() {
                *self = *right;
            } else {
                return None;
            }
            Some(TreeNode::new(self.element))
        } else if let Some(left) = &mut self.left {
            if let Some(node) = left.delete(x) {
                self.left = Some(node);
                return Some(TreeNode::new(self.element));
            }
        } else if let Some(right) = &mut self.right {
            if let Some(node) = right.delete(x) {
                self.right = Some(node);
                return Some(TreeNode::new(self.element));
            }
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
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut tree: Option<Box<TreeNode>> = None;

    for token in line.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            if tree.is_none() {
                tree = Some(Box::new(TreeNode::new(num)));
            } else {
                tree.as_mut().unwrap().insert(num);
            }
        }
    }

    if let Some(tree) = tree {
        println!("Tree after insertion:");
        tree.print_in_order();

        tree.delete_even_numbers();
        println!("Tree after deleting even numbers:");
        tree.print_in_order();

        if let Some(min) = tree.find_min() {
            if let Some(max) = tree.find_max() {
                println!("Min is {}, Max is {}", min.retrieve(), max.retrieve());
            } else {
                println!("Tree is empty");
            }
        } else {
            println!("Tree is empty");
        }
    } else {
        println!("Tree is empty");
    }
}