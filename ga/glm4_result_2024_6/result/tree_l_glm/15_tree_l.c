use std::io::{self, Read};
use std::collections::binary_search_tree::Node;
use std::collections::binary_search_tree::SearchTree;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut tree: SearchTree<i32> = SearchTree::new();

    for token in input.split_whitespace() {
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

trait TreeOperations<T> {
    fn new() -> Self;
    fn insert(&mut self, value: T);
    fn delete(&mut self, value: T);
    fn find(&self, value: T) -> Option<&Node<T>>;
    fn find_min(&self) -> Option<&Node<T>>;
    fn find_max(&self) -> Option<&Node<T>>;
    fn delete_even_numbers(&mut self);
    fn print_in_order(&self);
    fn is_empty(&self) -> bool;
}

impl<T: Ord + std::fmt::Display> TreeOperations<T> for SearchTree<T> {
    fn new() -> Self {
        Self::new()
    }

    fn insert(&mut self, value: T) {
        self.insert(value);
    }

    fn delete(&mut self, value: T) {
        self.delete(value);
    }

    fn find(&self, value: T) -> Option<&Node<T>> {
        self.find(&value)
    }

    fn find_min(&self) -> Option<&Node<T>> {
        self.find_min()
    }

    fn find_max(&self) -> Option<&Node<T>> {
        self.find_max()
    }

    fn delete_even_numbers(&mut self) {
        let mut nodes_to_delete = Vec::new();
        for node in self.iter() {
            if node.value % 2 == 0 {
                nodes_to_delete.push(node.value);
            }
        }
        for value in nodes_to_delete {
            self.delete(value);
        }
    }

    fn print_in_order(&self) {
        for node in self.iter() {
            println!("{}", node.value);
        }
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}