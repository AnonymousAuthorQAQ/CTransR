use std::io::{self, BufRead};

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
    match t {
        Some(node) => {
            if let Some(mut left) = &node.left {
                find_min(&node.left)
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut current = t;
    while let Some(node) = current {
        if let Some(mut right) = &node.right {
            current = &node.right;
        } else {
            return Some(node);
        }
    }
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
                node.left = insert(x, node.left.take());
            } else if x > node.element {
                node.right = insert(x, node.right.take());
            }
            Some(node)
        }
    }
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => {
            println!("Element not found");
            None
        }
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left.take());
            } else if x > node.element {
                node.right = delete(x, node.right.take());
            } else if node.left.is_some() && node.right.is_some() {
                let min_right = find_min(&node.right).unwrap().element;
                node.element = min_right;
                node.right = delete(min_right, node.right.take());
            } else {
                let tmp_node = node;
                if tmp_node.left.is_none() {
                    node = tmp_node.right.take();
                } else if tmp_node.right.is_none() {
                    node = tmp_node.left.take();
                }
            }
            Some(node)
        }
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

fn delete_even_numbers(t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        node.left = delete_even_numbers(node.left.take());
        node.right = delete_even_numbers(node.right.take());

        if node.element % 2 == 0 {
            return delete(node.element, Some(node));
        }
    }
    t
}

fn main() {
    let mut t: SearchTree = None;
    let stdin = io::stdin();
    let mut has_input = false;

    if let Some(line) = stdin.lock().lines().next() {
        if let Ok(nums) = line {
            for token in nums.split_whitespace() {
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

    // Delete all even numbers
    t = delete_even_numbers(t);

    println!("Tree after deleting even numbers:");
    print_in_order(&t);

    if let Some(tree) = &t {
        if let Some(min) = find_min(&t) {
            if let Some(max) = find_max(&t) {
                println!("Min is {}, Max is {}", retrieve(min), retrieve(max));
            }
        }
    } else {
        println!("Tree is empty");
    }

    t = make_empty(t);
}