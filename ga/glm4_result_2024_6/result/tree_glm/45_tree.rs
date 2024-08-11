use std::cmp::Ordering;
use std::collections::binary_search_tree::{Node, SearchTree};

fn main() {
    let mut tree: SearchTree<i32> = SearchTree::new();
    let mut j: i32 = 0;

    for i in 0..50 {
        j = (j + 7) % 50;
        tree.insert(j);
    }

    for i in 0..50 {
        if !tree.contains(&i) {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        tree.remove(&i);
    }

    for i in (1..50).step_by(2) {
        if !tree.contains(&i) {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if tree.contains(&i) {
            println!("Error at {}", i);
        }
    }

    println!("Min is {}, Max is {}", tree.min().unwrap(), tree.max().unwrap());
}

struct SearchTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> SearchTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: T) {
        self.root = Some(Self::insert_rec(self.root.take(), value));
    }

    fn insert_rec(mut node: Option<Box<Node<T>>>, value: T) -> Box<Node<T>> {
        match node {
            Some(n) => {
                match value.cmp(&n.element) {
                    Ordering::Less => {
                        n.left = Self::insert_rec(n.left.take(), value);
                    }
                    Ordering::Greater => {
                        n.right = Self::insert_rec(n.right.take(), value);
                    }
                    Ordering::Equal => {
                        return node;
                    }
                }
                Box::new(n)
            }
            None => Box::new(Node { element: value, left: None, right: None }),
        }
    }

    pub fn remove(&mut self, value: &T) -> bool {
        self.root = Self::remove_rec(self.root.take(), value);
        true
    }

    fn remove_rec(mut node: Option<Box<Node<T>>>, value: &T) -> Option<Box<Node<T>>> {
        match node {
            Some(n) => {
                match value.cmp(&n.element) {
                    Ordering::Less => {
                        n.left = Self::remove_rec(n.left.take(), value);
                    }
                    Ordering::Greater => {
                        n.right = Self::remove_rec(n.right.take(), value);
                    }
                    Ordering::Equal => {
                        return Self::delete_node(node);
                    }
                }
                Some(Box::new(n))
            }
            None => None,
        }
    }

    fn delete_node(node: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        match node {
            Some(n) => {
                if n.left.is_none() {
                    n.right
                } else if n.right.is_none() {
                    n.left
                } else {
                    let mut temp = n.right.take();
                    let mut min_larger_node = temp.unwrap();
                    while min_larger_node.left.is_some() {
                        min_larger_node = min_larger_node.left.take().unwrap();
                    }
                    n.element = min_larger_node.element;
                    n.right = Self::remove_rec(temp, &min_larger_node.element);
                    Some(Box::new(n))
                }
            }
            None => None,
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        Self::contains_rec(self.root.as_ref(), value)
    }

    fn contains_rec(node: Option<&Box<Node<T>>>, value: &T) -> bool {
        match node {
            Some(n) => match value.cmp(&n.element) {
                Ordering::Less => Self::contains_rec(n.left.as_ref(), value),
                Ordering::Greater => Self::contains_rec(n.right.as_ref(), value),
                Ordering::Equal => true,
            },
            None => false,
        }
    }

    pub fn min(&self) -> Option<T> {
        Self::min_rec(self.root.as_ref())
    }

    fn min_rec(node: Option<&Box<Node<T>>>) -> Option<T> {
        match node {
            Some(n) => {
                if n.left.is_none() {
                    Some(n.element)
                } else {
                    Self::min_rec(n.left.as_ref())
                }
            }
            None => None,
        }
    }

    pub fn max(&self) -> Option<T> {
        Self::max_rec(self.root.as_ref())
    }

    fn max_rec(node: Option<&Box<Node<T>>>) -> Option<T> {
        match node {
            Some(n) => {
                if n.right.is_none() {
                    Some(n.element)
                } else {
                    Self::max_rec(n.right.as_ref())
                }
            }
            None => None,
        }
    }
}

#[derive(Clone)]
struct Node<T> {
    element: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}