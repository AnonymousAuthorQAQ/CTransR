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
            std::cmp::Ordering::Equal => {}
        }
    }

    fn find(&self, x: i32) -> Option<&TreeNode> {
        match x.cmp(&self.element) {
            std::cmp::Ordering::Less => self.left.as_ref().and_then(|left| left.find(x)),
            std::cmp::Ordering::Greater => self.right.as_ref().and_then(|right| right.find(x)),
            std::cmp::Ordering::Equal => Some(self),
        }
    }

    fn find_min(&self) -> Option<&TreeNode> {
        match self.left.as_ref() {
            Some(left) => left.find_min(),
            None => Some(self),
        }
    }

    fn find_max(&self) -> Option<&TreeNode> {
        match self.right.as_ref() {
            Some(right) => right.find_max(),
            None => Some(self),
        }
    }

    fn delete(&mut self, x: i32) -> Option<TreeNode> {
        match x.cmp(&self.element) {
            std::cmp::Ordering::Less => {
                if let Some(left) = self.left.as_mut() {
                    left.delete(x)
                } else {
                    None
                }
            }
            std::cmp::Ordering::Greater => {
                if let Some(right) = self.right.as_mut() {
                    right.delete(x)
                } else {
                    None
                }
            }
            std::cmp::Ordering::Equal => {
                if self.left.is_none() {
                    if self.right.is_none() {
                        None
                    } else {
                        Some(*self.right.take().unwrap())
                    }
                } else if self.right.is_none() {
                    Some(*self.left.take().unwrap())
                } else {
                    let mut right_min = self.right.as_mut().unwrap();
                    while right_min.left.is_some() {
                        right_min = right_min.left.as_mut().unwrap();
                    }
                    self.element = right_min.element;
                    right_min.delete(right_min.element)
                }
            }
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
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().and_then(|result| result.ok());
    let mut tree = TreeNode::new(0);

    if let Some(line) = line {
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                tree.insert(num);
            }
        }
    }

    println!("Tree after insertion:");
    tree.print_in_order();

    tree.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    tree.print_in_order();
}