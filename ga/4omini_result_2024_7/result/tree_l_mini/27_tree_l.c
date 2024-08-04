use std::io::{self, BufRead};
use std::ptr;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty() -> SearchTree {
    None
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
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

fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => None, // Not found
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left);
                Some(node)
            } else if x > node.element {
                node.right = delete(x, node.right);
                Some(node)
            } else {
                // Found the node to delete
                if node.left.is_some() && node.right.is_some() {
                    let min_node = find_min(node.right.as_ref().unwrap());
                    node.element = min_node.element;
                    node.right = delete(min_node.element, node.right);
                    Some(node)
                } else {
                    if node.left.is_none() {
                        Some(node.right.take().unwrap_or_else(|| Box::new(TreeNode { element: 0, left: None, right: None })))
                    } else {
                        Some(node.left.take().unwrap())
                    }
                }
            }
        }
    }
}

fn find_min(t: &TreeNode) -> &TreeNode {
    if let Some(ref left) = t.left {
        find_min(left)
    } else {
        t
    }
}

fn find_max(t: &TreeNode) -> &TreeNode {
    if let Some(ref right) = t.right {
        find_max(right)
    } else {
        t
    }
}

fn delete_even_numbers(t: SearchTree) -> SearchTree {
    match t {
        None => None,
        Some(mut node) => {
            node.left = delete_even_numbers(node.left);
            node.right = delete_even_numbers(node.right);
            if node.element % 2 == 0 {
                return delete(node.element, Some(node));
            }
            Some(node)
        }
    }
}

fn print_in_order(t: &SearchTree) {
    if let Some(ref node) = *t {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut tree = make_empty();

    for line in handle.lines() {
        let line = line.unwrap();
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
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

    if let Some(ref t) = tree {
        println!(
            "Min is {}, Max is {}",
            find_min(t).element,
            find_max(t).element
        );
    } else {
        println!("Tree is empty");
    }
}