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
    t
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

fn find_min(t: &SearchTree) -> &TreeNode {
    let mut current = t.as_ref().unwrap();
    while let Some(left) = &current.left {
        current = left;
    }
    current
}

fn find_max(t: &SearchTree) -> &TreeNode {
    let mut current = t.as_ref().unwrap();
    while let Some(right) = &current.right {
        current = right;
    }
    current
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

fn delete_even_numbers(mut t: SearchTree) -> SearchTree {
    if let Some(mut node) = t.take() {
        node.left = delete_even_numbers(node.left);
        node.right = delete_even_numbers(node.right);
        
        if node.element % 2 == 0 {
            t = delete(node.element, t);
        }
    }
    t
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left);
            } else if x > node.element {
                node.right = delete(x, node.right);
            } else if node.left.is_some() && node.right.is_some() {
                let min_right = find_min(&node.right).element;
                node.element = min_right;
                node.right = delete(min_right, node.right);
            } else {
                t = if node.left.is_none() {
                    node.right
                } else {
                    node.left
                };
            }
            Some(node)
        }
        None => None,
    }
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

fn main() {
    let mut t = None;
    let mut has_input = false;

    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    for token in input.split(" ") {
        if let Ok(num) = token.parse::<i32>() {
            t = insert(num, t);
            has_input = true;
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
        let min = retrieve(find_min(&t));
        let max = retrieve(find_max(&t));
        println!("Min is {}, Max is {}", min, max);
    } else {
        println!("Tree is empty");
    }

    t = make_empty(t);
}