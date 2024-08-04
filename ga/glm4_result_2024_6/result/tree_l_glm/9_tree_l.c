use std::io;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

type SearchTree = Option<Box<TreeNode>>;

#[derive(Debug, Eq, PartialEq)]
struct TreeNode {
    element: i32,
    left: SearchTree,
    right: SearchTree,
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

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.element.cmp(&self.element)
    }
}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TreeNode {
    fn insert(&mut self, element: i32) {
        match element.cmp(&self.element) {
            Ordering::Less => {
                self.left.get_or_insert_with(|| Box::new(TreeNode::new(element)));
            }
            Ordering::Greater => {
                self.right.get_or_insert_with(|| Box::new(TreeNode::new(element)));
            }
            Ordering::Equal => {
                // Do nothing if element already exists
            }
        }
    }

    fn delete(&mut self, element: i32) {
        if self.element == element {
            if self.left.is_none() {
                *self = self.right.take().unwrap();
            } else if self.right.is_none() {
                *self = self.left.take().unwrap();
            } else {
                let mut min_right = self.right.as_mut().unwrap();
                while min_right.left.is_some() {
                    min_right = min_right.left.as_mut().unwrap();
                }
                self.element = min_right.element;
                min_right.delete(min_right.element);
            }
        } else {
            if element < self.element {
                self.left.as_mut().map(|left| left.delete(element));
            } else {
                self.right.as_mut().map(|right| right.delete(element));
            }
        }
    }

    fn delete_even_numbers(&mut self) {
        if self.left.is_some() {
            self.left.as_mut().unwrap().delete_even_numbers();
        }
        if self.right.is_some() {
            self.right.as_mut().unwrap().delete_even_numbers();
        }
        if self.element % 2 == 0 {
            self.delete(self.element);
        }
    }

    fn print_in_order(&self) {
        if self.left.is_some() {
            self.left.as_ref().unwrap().print_in_order();
        }
        println!("{}", self.element);
        if self.right.is_some() {
            self.right.as_ref().unwrap().print_in_order();
        }
    }
}

fn main() {
    let mut tree: SearchTree = None;
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let elements: Vec<i32> = line
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    for element in elements {
        tree.get_or_insert_with(|| Box::new(TreeNode::new(element))).insert(element);
    }

    println!("Tree after insertion:");
    tree.as_ref().unwrap().print_in_order();

    tree.as_mut().unwrap().delete_even_numbers();

    println!("Tree after deleting even numbers:");
    tree.as_ref().unwrap().print_in_order();
}