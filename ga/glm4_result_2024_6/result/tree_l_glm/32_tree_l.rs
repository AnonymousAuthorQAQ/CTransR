use std::io;
use std::collections::VecDeque;

type ElementType = i32;
type SearchTree = Option<Box<TreeNode>>;

struct TreeNode {
    element: ElementType,
    left: SearchTree,
    right: SearchTree,
}

impl TreeNode {
    fn new(element: ElementType) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
}

impl SearchTree {
    fn make_empty(&mut self) {
        *self = None;
    }

    fn find(&self, x: ElementType) -> &Option<Box<TreeNode>> {
        match self {
            Some(node) => {
                if x < node.element {
                    node.left.find(x)
                } else if x > node.element {
                    node.right.find(x)
                } else {
                    self
                }
            },
            None => None,
        }
    }

    fn find_min(&self) -> &Option<Box<TreeNode>> {
        match self {
            Some(node) => {
                if node.left.is_some() {
                    node.left.find_min()
                } else {
                    self
                }
            },
            None => None,
        }
    }

    fn find_max(&self) -> &Option<Box<TreeNode>> {
        match self {
            Some(node) => {
                if node.right.is_some() {
                    node.right.find_max()
                } else {
                    self
                }
            },
            None => None,
        }
    }

    fn insert(&mut self, x: ElementType) {
        if self.is_none() {
            *self = Some(Box::new(TreeNode::new(x)));
        } else if x < self.as_ref().unwrap().element {
            self.as_mut().unwrap().left.insert(x);
        } else if x > self.as_ref().unwrap().element {
            self.as_mut().unwrap().right.insert(x);
        }
    }

    fn delete(&mut self, x: ElementType) {
        if self.is_none() {
            return;
        }
        match (self.as_mut().unwrap().left.find(x), self.as_mut().unwrap().right.find(x)) {
            (Some(_), Some(_)) => {
                if let Some(min) = self.as_mut().unwrap().right.find_min() {
                    self.as_mut().unwrap().element = min.as_ref().unwrap().element;
                    self.as_mut().unwrap().right.delete(min.as_ref().unwrap().element);
                }
            },
            (Some(_), None) => {
                *self = self.take().unwrap().left;
            },
            (None, Some(_)) => {
                *self = self.take().unwrap().right;
            },
            (None, None) => {
                self.make_empty();
            },
        }
    }

    fn delete_even_numbers(&mut self) {
        if self.is_none() {
            return;
        }
        self.as_mut().unwrap().left.delete_even_numbers();
        self.as_mut().unwrap().right.delete_even_numbers();
        if self.as_ref().unwrap().element % 2 == 0 {
            self.delete(self.as_ref().unwrap().element);
        }
    }

    fn print_in_order(&self) {
        if self.is_none() {
            return;
        }
        self.as_ref().unwrap().left.print_in_order();
        println!("{}", self.as_ref().unwrap().element);
        self.as_ref().unwrap().right.print_in_order();
    }
}

fn main() {
    let mut input = VecDeque::new();
    let mut line = String::new();
    if io::stdin().read_line(&mut line).unwrap() > 0 {
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                input.push_back(num);
            }
        }
    }

    let mut tree: SearchTree = None;
    for num in input {
        tree.insert(num);
    }

    println!("Tree after insertion:");
    tree.print_in_order();

    tree.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    tree.print_in_order();

    if let Some(min) = tree.find_min() {
        println!("Min is {}, Max is {}", min.as_ref().unwrap().element, tree.find_max().unwrap().as_ref().unwrap().element);
    } else {
        println!("Tree is empty");
    }
}