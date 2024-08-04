use std::io::{self, Write};
use std::ops::Drop;

#[derive(Debug)]
struct TreeNode {
    element: i32,
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

    fn insert(&mut self, x: i32) {
        self.root = Self::insert_node(self.root.take(), x);
    }

    fn insert_node(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_node(n.left, x);
                } else if x > n.element {
                    n.right = Self::insert_node(n.right, x);
                }
                Some(n)
            }
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
        }
    }

    fn delete_even_numbers(&mut self) {
        self.root = Self::delete_even(self.root.take());
    }

    fn delete_even(node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                n.left = Self::delete_even(n.left);
                n.right = Self::delete_even(n.right);

                if n.element % 2 == 0 {
                    return Self::delete_node(n);
                }
                Some(n)
            }
            None => None,
        }
    }

    fn delete_node(n: Box<TreeNode>) -> Option<Box<TreeNode>> {
        if n.left.is_none() {
            return n.right;
        } else if n.right.is_none() {
            return n.left;
        }

        // Node with two children: get the inorder successor (smallest in the right subtree)
        let mut successor = n.right;
        while let Some(ref mut s) = successor {
            if s.left.is_none() {
                break;
            }
            successor = s.left;
        }

        let successor_node = successor.take()?;
        Some(Box::new(TreeNode {
            element: successor_node.element,
            left: n.left,
            right: Self::delete_node(successor_node),
        }))
    }

    fn print_in_order(&self) {
        Self::print_in_order_node(&self.root);
    }

    fn print_in_order_node(node: &Option<Box<TreeNode>>) {
        if let Some(ref n) = node {
            Self::print_in_order_node(&n.left);
            println!("{}", n.element);
            Self::print_in_order_node(&n.right);
        }
    }

    fn find_min(&self) -> Option<i32> {
        Self::find_min_node(&self.root)
    }

    fn find_min_node(node: &Option<Box<TreeNode>>) -> Option<i32> {
        match node {
            Some(ref n) => {
                if n.left.is_none() {
                    Some(n.element)
                } else {
                    Self::find_min_node(&n.left)
                }
            }
            None => None,
        }
    }

    fn find_max(&self) -> Option<i32> {
        Self::find_max_node(&self.root)
    }

    fn find_max_node(node: &Option<Box<TreeNode>>) -> Option<i32> {
        match node {
            Some(ref n) => {
                if n.right.is_none() {
                    Some(n.element)
                } else {
                    Self::find_max_node(&n.right)
                }
            }
            None => None,
        }
    }
}

fn main() {
    let mut tree = SearchTree::new();
    let mut input = String::new();

    print!("Enter space-separated integers: ");
    io::stdout().flush().unwrap();
    
    if let Ok(_) = io::stdin().read_line(&mut input) {
        let nums: Vec<&str> = input.split_whitespace().collect();
        
        for num_str in nums {
            if let Ok(num) = num_str.parse::<i32>() {
                tree.insert(num);
            } else {
                eprintln!("Skipping invalid input: {}", num_str);
            }
        }
    }

    println!("Tree after insertion:");
    tree.print_in_order();

    // Delete all even numbers
    tree.delete_even_numbers();
    println!("Tree after deleting even numbers:");
    tree.print_in_order();

    if let Some(min) = tree.find_min() {
        if let Some(max) = tree.find_max() {
            println!("Min is {}, Max is {}", min, max);
        }
    } else {
        println!("Tree is empty");
    }
}