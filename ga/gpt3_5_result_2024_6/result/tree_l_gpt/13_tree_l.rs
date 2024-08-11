use std::io::{self, BufRead};

// Define a TreeNode struct
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

// Define an alias for the tree
type SearchTree = Option<Box<TreeNode>>;

// Function to create an empty tree
fn make_empty(t: SearchTree) -> SearchTree {
    t
}

// Function to find a value in the tree
fn find(x: i32, t: &SearchTree) -> Option<&TreeNode> {
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

// Function to find the minimum value in the tree
fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    match t {
        Some(node) => {
            if let Some(left) = &node.left {
                find_min(&node.left)
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

// Function to find the maximum value in the tree
fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut node = t;
    while let Some(n) = node {
        if let Some(right) = &n.right {
            node = &n.right;
        } else {
            return Some(n);
        }
    }
    None
}

// Function to insert a value into the tree
fn insert(x: i32, t: SearchTree) -> SearchTree {
    Some(Box::new(TreeNode {
        element: x,
        left: None,
        right: None,
    }))
}
// Function to delete a value from the tree
fn delete(x: i32, t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        if x < node.element {
            node.left = delete(x, node.left);
        } else if x > node.element {
            node.right = delete(x, node.right);
        } else {
            if node.left.is_none() {
                return node.right.take();
            } else if node.right.is_none() {
                return node.left.take();
            }
            let min_right = find_min(&node.right).unwrap().element;
            node.element = min_right;
            node.right = delete(min_right, node.right);
        }
        Some(node)
    } else {
        None
    }
}

// Function to retrieve the value of a node
fn retrieve(p: &TreeNode) -> i32 {
    p.element
}

// Function to print the tree in-order
fn print_in_order(t: &SearchTree) {
    if let Some(node) = t {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

// Function to delete all even numbers from the tree
fn delete_even_numbers(t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        node.left = delete_even_numbers(node.left);
        node.right = delete_even_numbers(node.right);

        if node.element % 2 == 0 {
            return delete(node.element, Some(node));
        }
        Some(node)
    } else {
        None
    }
}

fn main() {
    let mut t: SearchTree = None;
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    for token in input.split_whitespace() {
        if let Ok(num) = token.parse() {
            t = insert(num, t);
        }
    }

    if t.is_none() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    print_in_order(&t);

    t = delete_even_numbers(t);

    println!("Tree after deleting even numbers:");
    print_in_order(&t);

    if let Some(min_node) = find_min(&t) {
        let min = retrieve(min_node);
        if let Some(max_node) = find_max(&t) {
            let max = retrieve(max_node);
            println!("Min is {}, Max is {}", min, max);
        }
    } else {
        println!("Tree is empty");
    }

    t = make_empty(t);
}