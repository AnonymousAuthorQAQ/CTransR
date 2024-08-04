use std::io::{self, BufRead};
use std::collections::VecDeque;

type ElementType = i32;

#[derive(Debug)]
struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

struct SearchTree {
    root: Option<Box<TreeNode>>,
}

impl SearchTree {
    fn new() -> Self {
        SearchTree { root: None }
    }

    fn insert(&mut self, x: ElementType) {
        self.root = Self::insert_node(self.root.take(), x);
    }

    fn insert_node(node: Option<Box<TreeNode>>, x: ElementType) -> Option<Box<TreeNode>> {
        match node {
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_node(n.left.take(), x);
                } else if x > n.element {
                    n.right = Self::insert_node(n.right.take(), x);
                }
                Some(n)
            }
        }
    }

    fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even_nodes(self.root.take());
    }

    fn delete_even_nodes(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        if let Some(n) = node {
            let new_left = Self::delete_even_nodes(n.left);
            let new_right = Self::delete_even_nodes(n.right);
            if n.element % 2 == 0 {
                new_right
            } else {
                Some(Box::new(TreeNode {
                    element: n.element,
                    left: new_left,
                    right: new_right,
                }))
            }
        } else {
            None
        }
    }

    fn print_in_order(&self) {
        Self::in_order_traversal(&self.root);
    }

    fn in_order_traversal(node: &Option<Box<TreeNode>>) {
        if let Some(n) = node {
            Self::in_order_traversal(&n.left);
            println!("{}", n.element);
            Self::in_order_traversal(&n.right);
        }
    }

    fn find_min(&self) -> Option<ElementType> {
        self.root.as_ref().and_then(|node| Self::min_value(node))
    }

    fn min_value(node: &Box<TreeNode>) -> Option<ElementType> {
        if let Some(ref left) = node.left {
            Self::min_value(left)
        } else {
            Some(node.element)
        }
    }

    fn find_max(&self) -> Option<ElementType> {
        self.root.as_ref().and_then(|node| Self::max_value(node))
    }

    fn max_value(node: &Box<TreeNode>) -> Option<ElementType> {
        if let Some(ref right) = node.right {
            Self::max_value(right)
        } else {
            Some(node.element)
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut tree = SearchTree::new();

    for token in line.split_whitespace() {
        if let Ok(num) = token.parse::<ElementType>() {
            tree.insert(num);
        }
    }

    if tree.root.is_none() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    tree.print_in_order();

    // Delete all even numbers
    tree.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    tree.print_in_order();

    // Print min and max
    if let Some(min) = tree.find_min() {
        if let Some(max) = tree.find_max() {
            println!("Min is {}, Max is {}", min, max);
        }
    } else {
        println!("Tree is empty");
    }
}