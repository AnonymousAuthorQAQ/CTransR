use std::io::{self, Read};
use std::collections::Vec;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(x: i32) -> Self {
        TreeNode {
            element: x,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, x: i32) {
        match x.cmp(&self.element) {
            std::cmp::Ordering::Less => {
                if self.left.is_none() {
                    self.left = Some(Box::new(TreeNode::new(x)));
                } else {
                    self.left.as_mut().unwrap().insert(x);
                }
            }
            std::cmp::Ordering::Greater => {
                if self.right.is_none() {
                    self.right = Some(Box::new(TreeNode::new(x)));
                } else {
                    self.right.as_mut().unwrap().insert(x);
                }
            }
            std::cmp::Ordering::Equal => {
                // Do nothing if the element already exists
            }
        }
    }

    fn delete(&mut self, x: i32) -> bool {
        match x.cmp(&self.element) {
            std::cmp::Ordering::Less => {
                if let Some(left) = self.left.as_mut() {
                    left.delete(x);
                }
            }
            std::cmp::Ordering::Greater => {
                if let Some(right) = self.right.as_mut() {
                    right.delete(x);
                }
            }
            std::cmp::Ordering::Equal => {
                if self.left.is_none() {
                    return true;
                } else if self.right.is_none() {
                    return true;
                } else {
                    let mut temp = self.left.as_mut().unwrap();
                    while temp.right.is_some() {
                        temp = temp.right.as_mut().unwrap();
                    }
                    self.element = temp.element;
                    temp.delete(temp.element);
                }
            }
        }
        false
    }

    fn find_min(&self) -> Option<i32> {
        match self.left {
            Some(ref left) => left.find_min(),
            None => Some(self.element),
        }
    }

    fn find_max(&self) -> Option<i32> {
        match self.right {
            Some(ref right) => right.find_max(),
            None => Some(self.element),
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

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read line");
    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut root = TreeNode::new(numbers[0]);
    for num in &numbers[1..] {
        root.insert(*num);
    }

    println!("Tree after insertion:");
    root.print_in_order();

    root.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    root.print_in_order();

    if let Some(min) = root.find_min() {
        println!("Min is {}", min);
    }
    if let Some(max) = root.find_max() {
        println!("Max is {}", max);
    }
}