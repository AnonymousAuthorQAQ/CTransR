use std::io::{self, Read};
use std::collections::binary_tree::Node;
use std::collections::binary_tree::Tree;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut tree = Tree::new();
    for num_str in input.trim().split_whitespace() {
        if let Ok(num) = num_str.parse::<i32>() {
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
    }

    if let Some(max) = tree.find_max() {
        println!("Max is {}", max);
    }
}

struct Tree {
    root: Option<Node<i32>>,
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }

    fn insert(&mut self, value: i32) {
        self.root = Some(Self::insert_rec(&mut self.root, value));
    }

    fn insert_rec(node: &mut Option<Node<i32>>, value: i32) -> Node<i32> {
        match node {
            Some(n) => {
                if value < n.value {
                    n.left = Some(Self::insert_rec(&mut n.left, value));
                } else if value > n.value {
                    n.right = Some(Self::insert_rec(&mut n.right, value));
                }
                n
            }
            None => Node::new(value),
        }
    }

    fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even_numbers_rec(&mut self.root);
    }

    fn delete_even_numbers_rec(node: &mut Option<Node<i32>>) -> Option<Node<i32>> {
        if let Some(mut n) = node {
            n.left = Self::delete_even_numbers_rec(&mut n.left);
            n.right = Self::delete_even_numbers_rec(&mut n.right);

            if n.value % 2 == 0 {
                return Self::delete_node(&mut n);
            }

            Some(n)
        } else {
            None
        }
    }

    fn delete_node(node: &mut Node<i32>) -> Option<Node<i32>> {
        if node.left.is_none() {
            return node.right;
        } else if node.right.is_none() {
            return node.left;
        }

        let mut min_larger_node = node.right.as_mut().unwrap();
        while min_larger_node.left.is_some() {
            min_larger_node = min_larger_node.left.as_mut().unwrap();
        }

        node.value = min_larger_node.value;
        node.right = Self::delete_node(min_larger_node);

        Some(node)
    }

    fn find_min(&self) -> Option<i32> {
        Self::find_min_rec(&self.root)
    }

    fn find_min_rec(node: &Option<Node<i32>>) -> Option<i32> {
        node.as_ref().and_then(|n| {
            if n.left.is_none() {
                Some(n.value)
            } else {
                Self::find_min_rec(&n.left)
            }
        })
    }

    fn find_max(&self) -> Option<i32> {
        Self::find_max_rec(&self.root)
    }

    fn find_max_rec(node: &Option<Node<i32>>) -> Option<i32> {
        node.as_ref().and_then(|n| {
            if n.right.is_none() {
                Some(n.value)
            } else {
                Self::find_max_rec(&n.right)
            }
        })
    }

    fn print_in_order(&self) {
        Self::print_in_order_rec(&self.root);
    }

    fn print_in_order_rec(node: &Option<Node<i32>>) {
        if let Some(n) = node {
            Self::print_in_order_rec(&n.left);
            println!("{}", n.value);
            Self::print_in_order_rec(&n.right);
        }
    }
}

#[derive(Clone)]
struct Node<T> {
    value: T,
    left: Option<Node<T>>,
    right: Option<Node<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}