use std::io;
use std::str::FromStr;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(element: ElementType) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }

    fn make_empty(&mut self) {
        if let Some(left) = self.left.take() {
            left.make_empty();
        }
        if let Some(right) = self.right.take() {
            right.make_empty();
        }
    }

    fn find(&self, x: ElementType) -> Option<&Self> {
        if self.element == x {
            Some(self)
        } else if x < self.element {
            self.left.as_ref().and_then(|left| left.find(x))
        } else {
            self.right.as_ref().and_then(|right| right.find(x))
        }
    }

    fn find_min(&self) -> Option<&Self> {
        match self.left.as_ref() {
            Some(left) => left.find_min(),
            None => Some(self),
        }
    }

    fn find_max(&self) -> Option<&Self> {
        self.right.as_ref().and_then(|right| {
            right.find_max().or_else(|| Some(right))
        })
    }

    fn insert(&mut self, x: ElementType) {
        if x < self.element {
            if let Some(left) = self.left.as_mut() {
                left.insert(x);
            } else {
                self.left = Some(Box::new(TreeNode::new(x)));
            }
        } else {
            if let Some(right) = self.right.as_mut() {
                right.insert(x);
            } else {
                self.right = Some(Box::new(TreeNode::new(x)));
            }
        }
    }

    fn delete(&mut self, x: ElementType) -> bool {
        if self.element == x {
            if self.left.is_none() {
                if let Some(right) = self.right.take() {
                    *self = *right;
                } else {
                    return false;
                }
            } else if self.right.is_none() {
                if let Some(left) = self.left.take() {
                    *self = *left;
                } else {
                    return false;
                }
            } else {
                if let Some(left) = self.left.take() {
                    let mut min_right = self.right.take().unwrap();
                    min_right.delete(left.element);
                    self.element = left.element;
                    self.left = Some(left);
                    self.right = Some(min_right);
                }
            }
            true
        } else if x < self.element {
            if let Some(left) = self.left.as_mut() {
                left.delete(x);
            }
        } else {
            if let Some(right) = self.right.as_mut() {
                right.delete(x);
            }
        }
        false
    }

    fn delete_even_numbers(&mut self) {
        if let Some(left) = self.left.as_mut() {
            left.delete_even_numbers();
        }
        if let Some(right) = self.right.as_mut() {
            right.delete_even_numbers();
        }
        if self.element % 2 == 0 {
            self.delete(self.element);
        }
    }

    fn print_in_order(&self) {
        if let Some(left) = self.left.as_ref() {
            left.print_in_order();
        }
        println!("{}", self.element);
        if let Some(right) = self.right.as_ref() {
            right.print_in_order();
        }
    }
}

fn main() {
    let mut tree: Option<Box<TreeNode>> = None;
    let mut line = String::new();
    if io::stdin().read_line(&mut line).unwrap() > 0 {
        for token in line.trim().split_whitespace() {
            if let Ok(x) = token.parse::<i32>() {
                if tree.is_none() {
                    tree = Some(Box::new(TreeNode::new(x)));
                } else {
                    tree.as_mut().unwrap().insert(x);
                }
            }
        }
    }

    if let Some(tree) = tree.as_mut() {
        println!("Tree after insertion:");
        tree.print_in_order();

        tree.delete_even_numbers();
        println!("Tree after deleting even numbers:");
        tree.print_in_order();
    } else {
        println!("Tree is empty");
    }
}