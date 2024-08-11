use std::io::{self, Read};

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

    fn make_empty(&mut self) {
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
        self.left.as_ref().and_then(|left| left.find_min()).or(Some(self))
    }

    fn find_max(&self) -> Option<&Self> {
        self.right.as_ref().and_then(|right| right.find_max()).or(Some(self))
    }

    fn insert(&mut self, x: i32) {
        if x < self.element {
            self.left.get_or_insert_with(|| Box::new(TreeNode::new(x)));
        } else if x > self.element {
            self.right.get_or_insert_with(|| Box::new(TreeNode::new(x)));
        }
    }

    fn delete(&mut self, x: i32) -> Option<Box<TreeNode>> {
        if self.element == x {
            if let Some(left) = self.left.take() {
                if let Some(right) = self.right.take() {
                    if let Some(min) = right.find_min() {
                        self.element = min.element;
                        self.left = Some(left);
                        self.right = right.delete(min.element);
                    } else {
                        self.element = right.element;
                        self.left = left;
                        self.right = right.left;
                    }
                } else {
                    return Some(left);
                }
            } else if let Some(right) = self.right.take() {
                return Some(right);
            } else {
                return None;
            }
        } else if x < self.element {
            self.left = self.left.take().and_then(|left| left.delete(x));
        } else {
            self.right = self.right.take().and_then(|right| right.delete(x));
        }

        Some(self.into())
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

    let mut root = Box::new(TreeNode::new(0));
    for token in input.trim().split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            root.insert(num);
        }
    }

    if root.left.is_none() && root.right.is_none() {
        println!("Tree is empty");
    } else {
        println!("Tree after insertion:");
        root.print_in_order();

        root.delete_even_numbers();

        println!("Tree after deleting even numbers:");
        root.print_in_order();

        if let Some(min) = root.find_min() {
            if let Some(max) = root.find_max() {
                println!("Min is {}, Max is {}", min.retrieve(), max.retrieve());
            }
        }
    }
}