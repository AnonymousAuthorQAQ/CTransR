use std::io::{self, BufRead};

#[derive(Debug)]
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

    fn make_empty(&mut self) {
        self.left = None;
        self.right = None;
    }

    fn find(&self, x: i32) -> Option<&Self> {
        match self {
            None => None,
            Some(node) => {
                if x < node.element {
                    node.left.find(x)
                } else if x > node.element {
                    node.right.find(x)
                } else {
                    Some(node)
                }
            }
        }
    }

    fn find_min(&self) -> Option<&Self> {
        match self {
            None => None,
            Some(node) => {
                if node.left.is_none() {
                    Some(node)
                } else {
                    node.left.find_min()
                }
            }
        }
    }

    fn find_max(&self) -> Option<&Self> {
        match self {
            None => None,
            Some(node) => {
                if node.right.is_none() {
                    Some(node)
                } else {
                    node.right.find_max()
                }
            }
        }
    }

    fn insert(&mut self, x: i32) {
        match self {
            None => {
                *self = Some(Box::new(TreeNode::new(x)));
            }
            Some(node) => {
                if x < node.element {
                    node.left.insert(x);
                } else if x > node.element {
                    node.right.insert(x);
                }
            }
        }
    }

    fn delete(&mut self, x: i32) {
        match self {
            None => return,
            Some(node) => {
                if x < node.element {
                    node.left.delete(x);
                } else if x > node.element {
                    node.right.delete(x);
                } else {
                    if node.left.is_some() && node.right.is_some() {
                        let mut min_right = node.right.take().unwrap();
                        node.element = min_right.find_min().unwrap().element;
                        node.right = Some(min_right);
                    } else {
                        *self = node.left.or(node.right);
                    }
                }
            }
        }
    }

    fn delete_even_numbers(&mut self) {
        if let Some(node) = self {
            node.left = node.left.and_then(|left| {
                left.delete_even_numbers();
                if node.element % 2 == 0 {
                    None
                } else {
                    Some(left)
                }
            });
            node.right = node.right.and_then(|right| {
                right.delete_even_numbers();
                if node.element % 2 == 0 {
                    None
                } else {
                    Some(right)
                }
            });
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut tree = None;

    for token in line.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            tree.insert(num);
        }
    }

    if tree.is_none() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    print_in_order(&tree);

    tree.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    print_in_order(&tree);

    if let Some(min) = tree.find_min() {
        println!("Min is {}, Max is {}", min.element, tree.find_max().unwrap().element);
    } else {
        println!("Tree is empty");
    }
}

fn print_in_order(tree: &Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}