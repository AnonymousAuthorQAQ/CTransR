use std::io::{self, BufRead};
use std::str::FromStr;

type Tree = Option<Box<Node>>;

struct Node {
    value: i32,
    left: Tree,
    right: Tree,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

impl Tree {
    fn make_empty(&mut self) {
        *self = None;
    }

    fn insert(&mut self, value: i32) {
        match self {
            None => {
                *self = Some(Box::new(Node::new(value)));
            }
            Some(node) => {
                if value < node.value {
                    node.left.insert(value);
                } else if value > node.value {
                    node.right.insert(value);
                }
            }
        }
    }

    fn delete(&mut self, value: i32) {
        if let Some(node) = self {
            if value < node.value {
                node.left.delete(value);
            } else if value > node.value {
                node.right.delete(value);
            } else {
                if let Some(left) = node.left.take() {
                    if let Some(right) = node.right.take() {
                        let mut min_right = right;
                        while let Some(min_left) = min_right.left {
                            min_right = min_left;
                        }
                        *self = Some(min_right);
                        self.as_mut().unwrap().left = left;
                        self.as_mut().unwrap().right = right;
                    } else {
                        *self = node.left;
                    }
                } else if let Some(right) = node.right.take() {
                    *self = node.right;
                } else {
                    *self = None;
                }
            }
        }
    }

    fn delete_even_numbers(&mut self) {
        if let Some(node) = self {
            node.left.delete_even_numbers();
            node.right.delete_even_numbers();
            if node.value % 2 == 0 {
                self.delete(node.value);
            }
        }
    }

    fn print_in_order(&self) {
        if let Some(node) = self {
            node.left.print_in_order();
            println!("{}", node.value);
            node.right.print_in_order();
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let numbers: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();

    let mut tree: Tree = None;
    for number in numbers {
        tree.insert(number);
    }

    println!("Tree after insertion:");
    tree.print_in_order();

    tree.delete_even_numbers();
    println!("Tree after deleting even numbers:");
    tree.print_in_order();

    if let Some(node) = tree {
        println!("Min is {}, Max is {}", node.left.as_ref().map_or(node.value, |n| n.value), node.right.as_ref().map_or(node.value, |n| n.value));
    } else {
        println!("Tree is empty");
    }
}