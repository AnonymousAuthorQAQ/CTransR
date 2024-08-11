use std::io::{self, BufRead};
use std::ptr;
use std::mem;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(tree: SearchTree) -> SearchTree {
    None
}

fn find(x: i32, tree: &SearchTree) -> Option<&TreeNode> {
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

fn find_min(tree: &SearchTree) -> Option<&TreeNode> {
    match tree {
        Some(node) => {
            let mut current = node;
            while let Some(ref left) = current.left {
                current = left;
            }
            Some(current)
        }
        None => None,
    }
}

fn find_max(tree: &SearchTree) -> Option<&TreeNode> {
    let mut current = tree;
    while let Some(ref node) = current {
        current = &node.right;
    }
    current.as_deref()
}

fn insert(x: i32, tree: SearchTree) -> SearchTree {
    match tree {
        Some(mut node) => {
            if x < node.element {
                node.left = insert(x, node.left);
            } else if x > node.element {
                node.right = insert(x, node.right);
            }
            Some(node)
        }
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
    }
}

fn delete(x: i32, tree: SearchTree) -> SearchTree {
    match tree {
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left);
                Some(node)
            } else if x > node.element {
                node.right = delete(x, node.right);
                Some(node)
            } else if node.left.is_some() && node.right.is_some() {
                let min_node = find_min(&node.right).unwrap();
                node.element = min_node.element;
                node.right = delete(node.element, node.right);
                Some(node)
            } else {
                if node.left.is_none() {
                    node.right
                } else {
                    node.left
                }
            }
        }
        None => {
            println!("Element not found");
            None
        }
    }
}

fn retrieve(node: &Option<Box<TreeNode>>) -> i32 {
    match node {
        Some(n) => n.element,
        None => 0,
    }
}

fn print_in_order(tree: &SearchTree) {
    if let Some(node) = tree {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn delete_even_numbers(tree: SearchTree) -> SearchTree {
    match tree {
        Some(mut node) => {
            node.left = delete_even_numbers(node.left);
            node.right = delete_even_numbers(node.right);
            
            if node.element % 2 == 0 {
                return delete(node.element, Some(node));
            }
            Some(node)
        }
        None => None,
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut tree: SearchTree = make_empty(None);

    for line in reader.lines() {
        if let Ok(line) = line {
            for token in line.split_whitespace() {
                if let Ok(num) = token.parse::<i32>() {
                    tree = insert(num, tree);
                }
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

    if let Some(t) = tree {
        println!(
            "Min is {:?}, Max is {:?}",
            retrieve(find_min(&Some(t.clone()))),
            retrieve(find_max(&Some(t.clone())))
        );
    } else {
        println!("Tree is empty");
    }
}