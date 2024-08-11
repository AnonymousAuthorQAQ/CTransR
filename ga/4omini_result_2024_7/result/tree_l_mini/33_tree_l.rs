use std::io::{self, BufRead};
use std::ptr; // For null pointer
use std::rc::Rc;
use std::cell::RefCell;

type ElementType = i32;

#[derive(Debug)]
struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    None
}

fn find(x: ElementType, t: &SearchTree) -> Option<&TreeNode> {
    match t {
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

fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    match t {
        Some(node) => {
            if node.left.is_none() {
                Some(node)
            } else {
                find_min(&node.left)
            }
        }
        None => None,
    }
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut current = t;
    while let Some(node) = current {
        if node.right.is_none() {
            return Some(node);
        }
        current = &node.right;
    }
    None
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => {
            Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            }))
        }
        Some(mut node) => {
            if x < node.element {
                node.left = insert(x, node.left);
            } else if x > node.element {
                node.right = insert(x, node.right);
            }
            Some(node)
        }
    }
}

fn delete(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => {
            println!("Element not found");
            None
        }
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left);
                Some(node)
            } else if x > node.element {
                node.right = delete(x, node.right);
                Some(node)
            } else {
                // Node found
                match (node.left, node.right) {
                    (None, None) => None, // No children
                    (Some(_), None) => node.left, // Only left child
                    (None, Some(_)) => node.right, // Only right child
                    (Some(left), Some(right)) => {
                        // Two children
                        let min_node = find_min(&node.right).unwrap().element;
                        node.element = min_node;
                        node.right = delete(min_node, node.right);
                        Some(node)
                    }
                }
            }
        }
    }
}

fn retrieve(p: &TreeNode) -> ElementType {
    p.element
}

fn print_in_order(t: &SearchTree) {
    if let Some(node) = t {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn delete_even_numbers(t: SearchTree) -> SearchTree {
    match t {
        None => None,
        Some(mut node) => {
            node.left = delete_even_numbers(node.left);
            node.right = delete_even_numbers(node.right);
            if node.element % 2 == 0 {
                delete(node.element, Some(node)) // deleting current node
            } else {
                Some(node)
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut tree: SearchTree = None;

    for line in handle.lines() {
        let line = line.unwrap();
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<ElementType>() {
                tree = insert(num, tree);
            }
        }
    }

    if tree.is_none() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    print_in_order(&tree);

    // Delete all even numbers
    tree = delete_even_numbers(tree);

    println!("Tree after deleting even numbers:");
    print_in_order(&tree);

    if let Some(min_node) = find_min(&tree) {
        if let Some(max_node) = find_max(&tree) {
            println!("Min is {}, Max is {}", retrieve(min_node), retrieve(max_node));
        }
    } else {
        println!("Tree is empty");
    }

    // Rust automatically handles memory cleanup, no need for explicit free
}