use std::io::{self, Read};
use std::collections::binary_tree::Node;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut tree = Node::new(None);
    for num_str in input.trim().split_whitespace() {
        if let Ok(num) = num_str.parse::<i32>() {
            tree.insert(num);
        }
    }

    println!("Tree after insertion:");
    tree.print_in_order();

    tree.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    tree.print_in_order();

    if let Some(min) = tree.find_min() {
        println!("Min is {}", min);
    }

    if let Some(max) = tree.find_max() {
        println!("Max is {}", max);
    }
}

trait Tree {
    fn new(value: Option<i32>) -> Self;
    fn insert(&mut self, value: i32);
    fn delete(&mut self, value: i32);
    fn delete_even_numbers(&mut self);
    fn find(&self, value: i32) -> Option<i32>;
    fn find_min(&self) -> Option<i32>;
    fn find_max(&self) -> Option<i32>;
    fn print_in_order(&self);
}

impl Tree for Node<Option<i32>> {
    fn new(value: Option<i32>) -> Self {
        Node::new(value)
    }

    fn insert(&mut self, value: i32) {
        if *self.value() == None {
            *self.value() = Some(value);
        } else {
            if value < self.value().unwrap() {
                if self.left().is_none() {
                    self.set_left(Node::new(Some(value)));
                } else {
                    self.left().unwrap().insert(value);
                }
            } else {
                if self.right().is_none() {
                    self.set_right(Node::new(Some(value)));
                } else {
                    self.right().unwrap().insert(value);
                }
            }
        }
    }

    fn delete(&mut self, value: i32) {
        if self.value().is_none() {
            return;
        }

        if value < self.value().unwrap() {
            if let Some(left) = self.left() {
                left.delete(value);
            }
        } else if value > self.value().unwrap() {
            if let Some(right) = self.right() {
                right.delete(value);
            }
        } else {
            if self.left().is_none() {
                *self = self.right().unwrap_or(Node::new(None));
            } else if self.right().is_none() {
                *self = self.left().unwrap();
            } else {
                let mut min_right = self.right().unwrap();
                while min_right.left().is_some() {
                    min_right = min_right.left().unwrap();
                }
                self.value() = min_right.value();
                self.right().unwrap().delete(min_right.value().unwrap());
            }
        }
    }

    fn delete_even_numbers(&mut self) {
        if self.value().is_some() && self.value().unwrap() % 2 == 0 {
            self.delete(self.value().unwrap());
        }
        if self.left().is_some() {
            self.left().unwrap().delete_even_numbers();
        }
        if self.right().is_some() {
            self.right().unwrap().delete_even_numbers();
        }
    }

    fn find(&self, value: i32) -> Option<i32> {
        if self.value().is_none() {
            return None;
        }

        if value == self.value().unwrap() {
            return self.value();
        }

        if value < self.value().unwrap() {
            if let Some(left) = self.left() {
                return left.find(value);
            }
        } else {
            if let Some(right) = self.right() {
                return right.find(value);
            }
        }

        None
    }

    fn find_min(&self) -> Option<i32> {
        if self.left().is_none() {
            self.value()
        } else {
            self.left().unwrap().find_min()
        }
    }

    fn find_max(&self) -> Option<i32> {
        if self.right().is_none() {
            self.value()
        } else {
            self.right().unwrap().find_max()
        }
    }

    fn print_in_order(&self) {
        if self.left().is_some() {
            self.left().unwrap().print_in_order();
        }
        if self.value().is_some() {
            println!("{}", self.value().unwrap());
        }
        if self.right().is_some() {
            self.right().unwrap().print_in_order();
        }
    }
}