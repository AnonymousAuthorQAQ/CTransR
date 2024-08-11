use std::io::{self, BufRead};
use std::collections::binary_search_tree::{Node, Tree};

fn main() {
    let mut tree = Tree::new();

    let stdin = io::stdin();
    let line = stdin.lock().lines().next().expect("Failed to read line").expect("Failed to read line");
    let tokens: Vec<&str> = line.split_whitespace().collect();
    for token in tokens {
        if let Ok(num) = token.parse::<i32>() {
            tree.insert(num);
        }
    }

    println!("Tree after insertion:");
    tree.print_in_order();

    tree.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    tree.print_in_order();

    if !tree.is_empty() {
        println!("Min is {}, Max is {}", tree.find_min().unwrap(), tree.find_max().unwrap());
    } else {
        println!("Tree is empty");
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

type Tree = Option<Box<Node>>;

impl Tree {
    fn new() -> Tree {
        None
    }

    fn insert(&mut self, value: i32) {
        match self {
            Some(node) => {
                if value < node.value {
                    node.left.insert(value);
                } else if value > node.value {
                    node.right.insert(value);
                }
            }
            None => {
                *self = Some(Box::new(Node::new(value)));
            }
        }
    }

    fn delete_even_numbers(&mut self) {
        if let Some(node) = self {
            node.left = node.left.take().map(|left| {
                left.delete_even_numbers();
                left
            });
            node.right = node.right.take().map(|right| {
                right.delete_even_numbers();
                right
            });

            if node.value % 2 == 0 {
                if node.left.is_none() {
                    *self = node.right.take();
                } else if node.right.is_none() {
                    *self = node.left.take();
                } else {
                    let mut right = node.right.take().unwrap();
                    *self = node.left.take();
                    self.insert(right.value);
                }
            }
        }
    }

    fn find_min(&self) -> Option<i32> {
        match self {
            Some(node) => {
                if node.left.is_none() {
                    Some(node.value)
                } else {
                    node.left.find_min()
                }
            }
            None => None,
        }
    }

    fn find_max(&self) -> Option<i32> {
        match self {
            Some(node) => {
                if node.right.is_none() {
                    Some(node.value)
                } else {
                    node.right.find_max()
                }
            }
            None => None,
        }
    }

    fn print_in_order(&self) {
        match self {
            Some(node) => {
                node.left.print_in_order();
                println!("{}", node.value);
                node.right.print_in_order();
            }
            None => {}
        }
    }

    fn is_empty(&self) -> bool {
        self.is_none()
    }
}