use std::io;
use std::io::prelude::*;
use std::collections::binary_tree::Node;
use std::collections::binary_tree::Tree;

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let numbers: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();

    let mut tree = Tree::new();
    for num in numbers {
        tree.insert(num);
    }

    println!("Tree after insertion:");
    tree.print_in_order();

    tree.delete_even_numbers();

    println!("Tree after deleting even numbers:");
    tree.print_in_order();

    if let Some((min, max)) = tree.min_max() {
        println!("Min is {}, Max is {}", min, max);
    } else {
        println!("Tree is empty");
    }
}

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(element: i32) -> TreeNode {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(tree: &mut SearchTree) {
    if let Some(node) = tree {
        make_empty(&mut node.left);
        make_empty(&mut node.right);
        tree.take();
    }
}

fn find(x: i32, tree: &SearchTree) -> Option<&TreeNode> {
    tree.as_ref().and_then(|node| {
        if x < node.element {
            find(x, &node.left)
        } else if x > node.element {
            find(x, &node.right)
        } else {
            Some(node)
        }
    })
}

fn find_min(tree: &SearchTree) -> Option<&TreeNode> {
    tree.as_ref().and_then(|node| {
        if let Some(left) = &node.left {
            find_min(&Some(left.clone()))
        } else {
            Some(node)
        }
    })
}

fn find_max(tree: &SearchTree) -> Option<&TreeNode> {
    tree.as_ref().and_then(|node| {
        if let Some(right) = &node.right {
            find_max(&Some(right.clone()))
        } else {
            Some(node)
        }
    })
}

fn insert(x: i32, tree: &mut SearchTree) {
    if let Some(node) = tree {
        if x < node.element {
            insert(x, &mut node.left);
        } else if x > node.element {
            insert(x, &mut node.right);
        }
    } else {
        *tree = Some(Box::new(TreeNode::new(x)));
    }
}

fn delete(x: i32, tree: &mut SearchTree) {
    if let Some(node) = tree {
        if x < node.element {
            delete(x, &mut node.left);
        } else if x > node.element {
            delete(x, &mut node.right);
        } else if let (Some(left), Some(right)) = (&node.left, &node.right) {
            let min = find_min(&Some(right.clone())).unwrap();
            node.element = min.element;
            delete(min.element, &mut node.right);
        } else {
            let node = tree.take();
            if let Some(left) = node.unwrap().left {
                *tree = Some(left);
            } else if let Some(right) = node.unwrap().right {
                *tree = Some(right);
            }
        }
    }
}

fn retrieve(node: &TreeNode) -> i32 {
    node.element
}

fn print_in_order(tree: &SearchTree) {
    if let Some(node) = tree {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn delete_even_numbers(tree: &mut SearchTree) {
    if let Some(node) = tree {
        delete_even_numbers(&mut node.left);
        delete_even_numbers(&mut node.right);
        if node.element % 2 == 0 {
            delete(node.element, tree);
        }
    }
}

impl Tree {
    fn new() -> Tree {
        None
    }

    fn insert(&mut self, element: i32) {
        insert(element, self);
    }

    fn delete(&mut self, element: i32) {
        delete(element, self);
    }

    fn find(&self, element: i32) -> Option<&TreeNode> {
        find(element, self)
    }

    fn find_min(&self) -> Option<&TreeNode> {
        find_min(self)
    }

    fn find_max(&self) -> Option<&TreeNode> {
        find_max(self)
    }

    fn print_in_order(&self) {
        print_in_order(self);
    }

    fn delete_even_numbers(&mut self) {
        delete_even_numbers(self);
    }

    fn min_max(&self) -> Option<(i32, i32)> {
        self.find_min().and_then(|min| {
            self.find_max().map(|max| (min.element, max.element))
        })
    }
}