use std::io::{self, BufRead};

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        node.left = make_empty(node.left);
        node.right = make_empty(node.right);
    }
    None
}

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

fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    let mut current = t;
    while let Some(node) = current {
        if let Some(left_node) = &node.left {
            current = Some(left_node);
        } else {
            return Some(node);
        }
    }
    None
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut current = t;
    while let Some(node) = current {
        if let Some(right_node) = &node.right {
            current = Some(right_node);
        } else {
            return Some(node);
        }
    }
    None
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
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

fn delete(x: i32, t: SearchTree) -> SearchTree {
    // Implementation of delete function is omitted for brevity
    t
}

fn retrieve(p: &TreeNode) -> i32 {
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
        Some(mut node) => {
            node.left = delete_even_numbers(node.left);
            node.right = delete_even_numbers(node.right);

            if node.element % 2 == 0 {
                node = delete(node.element, Some(node));
            }

            Some(node)
        }
        None => None,
    }
}

fn main() {
    let mut t: SearchTree = None;
    let stdin = io::stdin();
    let mut has_input = false;

    if let Ok(input) = stdin.lock().lines().next() {
        if let Some(line) = input {
            for token in line.split_whitespace() {
                if let Ok(num) = token.parse::<i32>() {
                    t = insert(num, t);
                    has_input = true;
                }
            }
        }
    }

    if !has_input || t.is_none() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    print_in_order(&t);

    t = delete_even_numbers(t);

    println!("Tree after deleting even numbers:");
    print_in_order(&t);

    if let Some(tree) = &t {
        println!("Min is {}, Max is {}", retrieve(find_min(&t).unwrap()), retrieve(find_max(&t).unwrap()));
    } else {
        println!("Tree is empty");
    }

    t = make_empty(t);
}