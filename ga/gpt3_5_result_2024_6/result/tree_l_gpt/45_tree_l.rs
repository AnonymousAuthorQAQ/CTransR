use std::io::{self, BufRead};
use std::string::String;
use std::str::FromStr;

struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(element: i32) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, val: i32) {
        if val < self.element {
            if let Some(ref mut left) = self.left {
                left.insert(val);
            } else {
                self.left = Some(Box::new(TreeNode::new(val)));
            }
        } else {
            if let Some(ref mut right) = self.right {
                right.insert(val);
            } else {
                self.right = Some(Box::new(TreeNode::new(val)));
            }
        }
    }

    fn delete_even_numbers(&mut self) {
        if let Some(ref mut left) = self.left {
            left.delete_even_numbers();
        }
        
        if let Some(ref mut right) = self.right {
            right.delete_even_numbers();
        }

        if self.element % 2 == 0 {
            // Delete the node with an even number
            *self = unsafe { std::ptr::read(Entity::null()) };
        }
    }

    fn find_min(&self) -> i32 {
        if let Some(ref node) = self.left {
            node.find_min()
        } else {
            self.element
        }
    }

    fn find_max(&self) -> i32 {
        if let Some(ref node) = self.right {
            node.find_max()
        } else {
            self.element
        }
    }

    fn print_in_order(&self) {
        if let Some(ref left) = self.left {
            left.print_in_order();
        }

        println!("{}", self.element);

        if let Some(ref right) = self.right {
            right.print_in_order();
        }
    }
}

fn make_empty(tree: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    None
}

fn main() {
    let mut tree = None;
    let mut has_input = false;

    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    for token in line.trim().split(' ') {
        if let Ok(num) = i32::from_str(token) {
            if let Some(ref mut root) = tree {
                root.insert(num);
            } else {
                tree = Some(Box::new(TreeNode::new(num)));
            }
            
            has_input = true;
        }
    }

    if !has_input || tree.is_none() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    if let Some(ref root) = tree {
        root.print_in_order();
    }

    if let Some(ref mut root) = tree {
        root.delete_even_numbers();

        println!("Tree after deleting even numbers:");
        root.print_in_order();

        let min = root.find_min();
        let max = root.find_max();
        println!("Min is {}, Max is {}", min, max);
    } else {
        println!("Tree is empty");
        return;
    }

    // Empties the tree before terminating
	tre = make_empty(tree);
}