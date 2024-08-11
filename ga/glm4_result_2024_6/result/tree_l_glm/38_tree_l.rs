use std::io::{self, Read};
use std::collections::binary_heap::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
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
}

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.element.cmp(&self.element)
    }
}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreeNode")
            .field("element", &self.element)
            .field("left", &self.left.as_ref().map(|x| x.element))
            .field("right", &self.right.as_ref().map(|x| x.element))
            .finish()
    }
}

fn make_empty<T>(tree: &mut Option<T>) where T: Default {
    *tree = None;
}

fn find(x: i32, tree: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    match tree {
        Some(node) => {
            if x < node.element {
                find(x, &node.left)
            } else if x > node.element {
                find(x, &node.right)
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_min(tree: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    match tree {
        Some(node) => {
            if let Some(left) = &node.left {
                find_min(&Some(left.clone()))
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_max(tree: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    match tree {
        Some(node) => {
            if let Some(right) = &node.right {
                find_max(&Some(right.clone()))
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn insert(x: i32, tree: &mut Option<Box<TreeNode>>) {
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

fn delete(x: i32, tree: &mut Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        if x < node.element {
            delete(x, &mut node.left);
        } else if x > node.element {
            delete(x, &mut node.right);
        } else {
            if let Some(left) = node.left.take() {
                if let Some(right) = node.right.take() {
                    *tree = Some(Box::new(TreeNode {
                        element: find_min(&Some(right)).unwrap().element,
                        left: Some(left),
                        right: find_min(&Some(right)).unwrap().right,
                    }));
                } else {
                    *tree = Some(left);
                }
            } else if let Some(right) = node.right.take() {
                *tree = Some(right);
            } else {
                *tree = None;
            }
        }
    }
}

fn retrieve(node: &Box<TreeNode>) -> i32 {
    node.element
}

fn print_in_order(tree: &Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn delete_even_numbers(tree: &mut Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        delete_even_numbers(&mut node.left);
        delete_even_numbers(&mut node.right);
        if node.element % 2 == 0 {
            delete(node.element, tree);
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tree: Option<Box<TreeNode>> = None;

    for token in input.split_whitespace() {
        if let Ok(x) = token.parse::<i32>() {
            insert(x, &mut tree);
        }
    }

    print_in_order(&tree);
    println!("Tree after deleting even numbers:");
    delete_even_numbers(&mut tree);
    print_in_order(&tree);

    if let Some(node) = &tree {
        println!("Min is {}, Max is {}", retrieve(find_min(&tree).unwrap()), retrieve(find_max(&tree).unwrap()));
    } else {
        println!("Tree is empty");
    }

    make_empty(&mut tree);
}