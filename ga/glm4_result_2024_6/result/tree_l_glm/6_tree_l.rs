use std::io::{self, BufRead};
use std::collections::binary_tree::Node;
use std::collections::binary_tree::Tree;

fn main() {
    let mut tree = Tree::new();
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
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

pub struct Tree {
    root: Option<Node<i32>>,
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn insert(&mut self, value: i32) {
        self.root = Some(Tree::insert_recursive(self.root.take(), value));
    }

    fn insert_recursive(root: Option<Node<i32>>, value: i32) -> Node<i32> {
        match root {
            Some(node) => {
                if value < node.value {
                    Node {
                        value,
                        left: Tree::insert_recursive(node.left, value),
                        right: node.right,
                    }
                } else {
                    Node {
                        value,
                        left: node.left,
                        right: Tree::insert_recursive(node.right, value),
                    }
                }
            }
            None => Node {
                value,
                left: None,
                right: None,
            },
        }
    }

    pub fn delete(&mut self, value: i32) {
        self.root = Tree::delete_recursive(self.root.take(), value);
    }

    fn delete_recursive(root: Option<Node<i32>>, value: i32) -> Option<Node<i32>> {
        match root {
            Some(node) => {
                if value < node.value {
                    Some(Node {
                        value: node.value,
                        left: Tree::delete_recursive(node.left, value),
                        right: node.right,
                    })
                } else if value > node.value {
                    Some(Node {
                        value: node.value,
                        left: node.left,
                        right: Tree::delete_recursive(node.right, value),
                    })
                } else {
                    if node.left.is_none() {
                        node.right
                    } else if node.right.is_none() {
                        node.left
                    } else {
                        let mut min_node = node.right.as_ref().unwrap().left.as_ref().unwrap();
                        while min_node.left.is_some() {
                            min_node = min_node.left.as_ref().unwrap();
                        }
                        Some(Node {
                            value: min_node.value,
                            left: node.left,
                            right: Tree::delete_recursive(node.right, min_node.value),
                        })
                    }
                }
            }
            None => None,
        }
    }

    pub fn delete_even_numbers(&mut self) {
        self.root = Tree::delete_even_numbers_recursive(self.root.take());
    }

    fn delete_even_numbers_recursive(root: Option<Node<i32>>) -> Option<Node<i32>> {
        match root {
            Some(node) => {
                let left = Tree::delete_even_numbers_recursive(node.left);
                let right = Tree::delete_even_numbers_recursive(node.right);
                if node.value % 2 == 0 {
                    if let Some(right) = right {
                        Some(right)
                    } else {
                        left
                    }
                } else {
                    Some(Node {
                        value: node.value,
                        left,
                        right,
                    })
                }
            }
            None => None,
        }
    }

    pub fn find_min(&self) -> Option<i32> {
        Tree::find_min_recursive(self.root.as_ref())
    }

    fn find_min_recursive(node: Option<&Node<i32>>) -> Option<i32> {
        match node {
            Some(node) => {
                if node.left.is_none() {
                    Some(node.value)
                } else {
                    Tree::find_min_recursive(node.left.as_ref())
                }
            }
            None => None,
        }
    }

    pub fn find_max(&self) -> Option<i32> {
        Tree::find_max_recursive(self.root.as_ref())
    }

    fn find_max_recursive(node: Option<&Node<i32>>) -> Option<i32> {
        match node {
            Some(node) => {
                if node.right.is_none() {
                    Some(node.value)
                } else {
                    Tree::find_max_recursive(node.right.as_ref())
                }
            }
            None => None,
        }
    }

    pub fn print_in_order(&self) {
        Tree::print_in_order_recursive(self.root.as_ref());
        println!();
    }

    fn print_in_order_recursive(node: Option<&Node<i32>>) {
        if let Some(node) = node {
            Tree::print_in_order_recursive(node.left.as_ref());
            println!("{}", node.value);
            Tree::print_in_order_recursive(node.right.as_ref());
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub left: Option<Node<T>>,
    pub right: Option<Node<T>>,
}

impl<T: Ord> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}