use std::io;
use std::io::prelude::*;
use std::collections::binary_tree::Node;
use std::collections::binary_tree::BinaryTree;
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().expect("Failed to read line").expect("No input");

    let tokens: Vec<&str> = line.split_whitespace().collect();
    let mut tree: BinaryTree<i32> = BinaryTree::new();

    for token in tokens {
        if let Ok(num) = i32::from_str(token) {
            tree.insert(num);
        }
    }

    println!("Tree after insertion:");
    tree.print_in_order();

    tree.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    tree.print_in_order();

    if let Some(min) = tree.find_min() {
        println!("Min is {}", min);
    } else {
        println!("Tree is empty");
    }

    if let Some(max) = tree.find_max() {
        println!("Max is {}", max);
    }
}

trait BinaryTree<T> {
    fn new() -> Self;
    fn insert(&mut self, value: T);
    fn delete(&mut self, value: T);
    fn delete_even_numbers(&mut self);
    fn find(&self, value: T) -> Option<&Node<T>>;
    fn find_min(&self) -> Option<T>;
    fn find_max(&self) -> Option<T>;
    fn print_in_order(&self);
}

impl<T: Ord> BinaryTree<T> for BinaryTree<T> {
    fn new() -> Self {
        BinaryTree { root: None }
    }

    fn insert(&mut self, value: T) {
        self.root = Some(Node::new(value, self.root.take()));
    }

    fn delete(&mut self, value: T) {
        self.root = Some(Node::delete(&value, self.root.take()));
    }

    fn delete_even_numbers(&mut self) {
        self.root = Some(Node::delete_even_numbers(self.root.take()));
    }

    fn find(&self, value: T) -> Option<&Node<T>> {
        Node::find(&value, self.root)
    }

    fn find_min(&self) -> Option<T> {
        Node::find_min(self.root)
    }

    fn find_max(&self) -> Option<T> {
        Node::find_max(self.root)
    }

    fn print_in_order(&self) {
        Node::print_in_order(self.root);
    }
}

struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn new(value: T, child: Option<Box<Node<T>>>) -> Box<Node<T>> {
        Box::new(Node {
            value,
            left: child,
            right: None,
        })
    }

    fn delete(&self, value: &T, mut node: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        if node.is_none() {
            return None;
        }

        let mut current = node.as_mut();
        while let Some(n) = current {
            if &n.value == value {
                return Node::delete_node(n);
            }

            if value < &n.value {
                current = n.left.as_mut();
            } else {
                current = n.right.as_mut();
            }
        }

        node
    }

    fn delete_even_numbers(&mut self) {
        self.left = Node::delete_even_numbers(self.left.take());
        self.right = Node::delete_even_numbers(self.right.take());

        if self.value % 2 == 0 {
            self.delete(&self.value);
        }
    }

    fn delete_node(&mut self) -> Option<Box<Node<T>>> {
        if self.left.is_none() {
            return self.right.take();
        }

        if self.right.is_none() {
            return self.left.take();
        }

        let mut temp = self.left.take();
        let mut current = temp.as_mut();

        while current.as_ref().unwrap().right.is_some() {
            current = current.unwrap().right.as_mut();
        }

        let mut temp_node = current.unwrap();
        temp_node.right = self.right.take();
        temp_node.left = temp.take();
        Some(temp_node)
    }

    fn find(&self, value: &T) -> Option<&Node<T>> {
        if &self.value == value {
            return Some(self);
        }

        if value < &self.value {
            return self.left.as_ref().and_then(|n| n.find(value));
        }

        self.right.as_ref().and_then(|n| n.find(value))
    }

    fn find_min(&self) -> Option<T> {
        if self.left.is_none() {
            return Some(self.value);
        }

        self.left.as_ref().and_then(|n| n.find_min())
    }

    fn find_max(&self) -> Option<T> {
        if self.right.is_none() {
            return Some(self.value);
        }

        self.right.as_ref().and_then(|n| n.find_max())
    }

    fn print_in_order(&self) {
        if let Some(left) = &self.left {
            left.print_in_order();
        }
        println!("{}", self.value);
        if let Some(right) = &self.right {
            right.print_in_order();
        }
    }
}