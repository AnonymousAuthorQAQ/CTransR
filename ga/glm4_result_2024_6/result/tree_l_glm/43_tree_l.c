use std::io::{self, BufRead};

type SearchTree = Option<Box<TreeNode>>;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: SearchTree,
    right: SearchTree,
}

impl TreeNode {
    fn new(element: i32) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }

    fn make_empty(&mut self) {
        if let Some(ref mut left) = self.left {
            left.make_empty();
        }
        if let Some(ref mut right) = self.right {
            right.make_empty();
        }
        *self = None;
    }

    fn find(&self, x: i32) -> Option<&Self> {
        match self {
            Some(ref node) => {
                if x < node.element {
                    node.left.find(x)
                } else if x > node.element {
                    node.right.find(x)
                } else {
                    Some(node)
                }
            }
            None => None,
        }
    }

    fn find_min(&self) -> Option<&Self> {
        match self {
            Some(ref node) => {
                if let Some(ref left) = node.left {
                    left.find_min()
                } else {
                    Some(node)
                }
            }
            None => None,
        }
    }

    fn find_max(&self) -> Option<&Self> {
        match self {
            Some(ref node) => {
                if let Some(ref right) = node.right {
                    right.find_max()
                } else {
                    Some(node)
                }
            }
            None => None,
        }
    }

    fn insert(&mut self, x: i32) {
        if let Some(ref mut node) = self {
            if x < node.element {
                node.left.insert(x);
            } else if x > node.element {
                node.right.insert(x);
            }
        } else {
            *self = Some(Box::new(TreeNode::new(x)));
        }
    }

    fn delete(&mut self, x: i32) {
        if let Some(ref mut node) = self {
            if x < node.element {
                node.left.delete(x);
            } else if x > node.element {
                node.right.delete(x);
            } else if let (Some(ref mut left), Some(ref mut right)) = (node.left.as_mut(), node.right.as_mut()) {
                let min = right.find_min().unwrap();
                node.element = min.element;
                right.delete(min.element);
            } else {
                *self = node.left.take().or(node.right.take());
            }
        }
    }

    fn retrieve(&self) -> Option<i32> {
        self.as_ref().map(|node| node.element)
    }

    fn print_in_order(&self) {
        if let Some(ref node) = self {
            node.left.print_in_order();
            println!("{}", node.element);
            node.right.print_in_order();
        }
    }

    fn delete_even_numbers(&mut self) {
        if let Some(ref mut node) = self {
            node.left = node.left.take().map(|left| {
                let mut left = left;
                left.delete_even_numbers();
                left
            });
            node.right = node.right.take().map(|right| {
                let mut right = right;
                right.delete_even_numbers();
                right
            });
            if node.element % 2 == 0 {
                *self = None;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut tree: SearchTree = None;

    for token in line.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            tree.insert(num);
        }
    }

    if let Some(ref tree) = tree {
        println!("Tree after insertion:");
        tree.print_in_order();
    } else {
        println!("Tree is empty");
    }

    tree.as_mut().map(|tree| tree.delete_even_numbers());

    if let Some(ref tree) = tree {
        println!("Tree after deleting even numbers:");
        tree.print_in_order();
    } else {
        println!("Tree is empty");
    }

    if let Some(ref tree) = tree {
        println!("Min is {}, Max is {}", tree.find_min().unwrap().element, tree.find_max().unwrap().element);
    } else {
        println!("Tree is empty");
    }

    tree.as_mut().map(|tree| tree.make_empty());
}