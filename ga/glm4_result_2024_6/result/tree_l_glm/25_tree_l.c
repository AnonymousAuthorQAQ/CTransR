use std::io::{self, BufRead};
use std::collections::binary_tree::{Node, BinaryTree};

fn main() {
    let mut tree = BinaryTree::new();
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let numbers: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();

    for &num in &numbers {
        tree.insert(num);
    }

    println!("Tree after insertion:");
    tree.print_inorder();

    tree.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    tree.print_inorder();

    if let Some((min, max)) = tree.min_max() {
        println!("Min is {}, Max is {}", min, max);
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

type BinaryTree = Option<Box<Node>>;

impl BinaryTree {
    fn new() -> BinaryTree {
        None
    }

    fn insert(&mut self, value: i32) {
        if let Some(ref mut node) = *self {
            if value < node.value {
                node.left.insert(value);
            } else if value > node.value {
                node.right.insert(value);
            }
        } else {
            *self = Some(Box::new(Node::new(value)));
        }
    }

    fn delete_even_numbers(&mut self) {
        if let Some(mut node) = self.take() {
            *self = Some(Self::delete_even_numbers_rec(node));
        }
    }

    fn delete_even_numbers_rec(mut node: Box<Node>) -> Box<Node> {
        node.left = node.left.take().map(Self::delete_even_numbers_rec);
        node.right = node.right.take().map(Self::delete_even_numbers_rec);

        if node.value % 2 == 0 {
            if let Some(right) = node.right.take() {
                return right;
            } else if let Some(left) = node.left.take() {
                return left;
            }
        }

        node
    }

    fn print_inorder(&self) {
        if let Some(node) = self {
            node.left.print_inorder();
            println!("{}", node.value);
            node.right.print_inorder();
        }
    }

    fn min_max(&self) -> Option<(i32, i32)> {
        if let Some(node) = self {
            let (min, max) = node.min_max();
            Some((min, max))
        } else {
            None
        }
    }
}

impl Node {
    fn min_max(&self) -> (i32, i32) {
        let mut min = self.value;
        let mut max = self.value;

        if let Some(ref left) = self.left {
            let (left_min, left_max) = left.min_max();
            min = std::cmp::min(min, left_min);
            max = std::cmp::max(max, left_max);
        }

        if let Some(ref right) = self.right {
            let (right_min, right_max) = right.min_max();
            min = std::cmp::min(min, right_min);
            max = std::cmp::max(max, right_max);
        }

        (min, max)
    }
}