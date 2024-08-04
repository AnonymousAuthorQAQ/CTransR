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

    fn insert(&mut self, element: i32) {
        if element < self.element {
            self.left.get_or_insert_with(|| Box::new(TreeNode::new(element)));
        } else if element > self.element {
            self.right.get_or_insert_with(|| Box::new(TreeNode::new(element)));
        }
    }

    fn delete(&mut self, element: i32) -> bool {
        if self.element == element {
            if self.left.is_none() && self.right.is_none() {
                return true;
            } else if self.left.is_none() {
                *self = *self.right.take().unwrap();
            } else if self.right.is_none() {
                *self = *self.left.take().unwrap();
            } else {
                let mut min_right = self.right.as_mut().unwrap();
                while min_right.left.is_some() {
                    min_right = min_right.left.as_mut().unwrap();
                }
                self.element = min_right.element;
                self.right.as_mut().unwrap().delete(min_right.element);
            }
            true
        } else if element < self.element {
            self.left.as_mut().map(|left| left.delete(element)).unwrap_or(false)
        } else {
            self.right.as_mut().map(|right| right.delete(element)).unwrap_or(false)
        }
    }

    fn find_min(&self) -> Option<&Self> {
        self.left.as_ref().and_then(|left| left.find_min()).or(Some(self))
    }

    fn find_max(&self) -> Option<&Self> {
        self.right.as_ref().and_then(|right| right.find_max()).or(Some(self))
    }

    fn delete_even_numbers(&mut self) {
        if self.element % 2 == 0 {
            self.delete(self.element);
        }
        self.left.as_mut().map(|left| left.delete_even_numbers());
        self.right.as_mut().map(|right| right.delete_even_numbers());
    }

    fn print_in_order(&self) {
        self.left.as_ref().map(|left| left.print_in_order());
        println!("{}", self.element);
        self.right.as_ref().map(|right| right.print_in_order());
    }
}

fn main() {
    let stdin = io::stdin();
    let mut tree: Option<Box<TreeNode>> = None;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        for token in line.split_whitespace() {
            if let Ok(element) = token.parse::<i32>() {
                if tree.is_none() {
                    tree = Some(Box::new(TreeNode::new(element)));
                } else {
                    tree.as_mut().unwrap().insert(element);
                }
            }
        }
    }

    if let Some(ref mut tree) = tree {
        println!("Tree after insertion:");
        tree.print_in_order();

        tree.delete_even_numbers();

        println!("Tree after deleting even numbers:");
        tree.print_in_order();

        if let Some(min) = tree.find_min() {
            println!("Min is {}, Max is {}", min.element, tree.find_max().unwrap().element);
        }
    } else {
        println!("Tree is empty");
    }
}