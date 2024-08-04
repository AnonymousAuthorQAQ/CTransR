use std::io::{self, BufRead};

struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    if let Some(mut t) = t {
        t.left = make_empty(t.left);
        t.right = make_empty(t.right);
    }
    None
}

fn find(x: i32, t: &SearchTree) -> Option<&TreeNode> {
    match t {
        None => None,
        Some(node) => {
            if x < node.element {
                find(x, &node.left)
            } else if x > node.element {
                find(x, &node.right)
            } else {
                Some(node)
            }
        }
    }
}

fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    match t {
        Some(node) => {
            if let Some(ref mut left) = node.left {
                find_min(&left)
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut t = t;
    while let Some(ref node) = *t {
        t = &node.right;
    }
    t.as_ref()
}

fn insert(x: i32, mut t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        if x < node.element {
            node.left = insert(x, node.left);
        } else if x > node.element {
            node.right = insert(x, node.right);
        }
        Some(node)
    } else {
        Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        }))
    }
}

fn delete(x: i32, mut t: SearchTree) -> SearchTree {
    if let Some(mut node) = t.take() {
        let mut new_node = node;

        if x < new_node.element {
            new_node.left = delete(x, new_node.left);
        } else if x > new_node.element {
            new_node.right = delete(x, new_node.right);
        } else if let (Some(left), right) = (new_node.left.take(), new_node.right.take()) {
            let min = find_min(&right).unwrap().element;
            new_node.element = min;
            new_node.right = delete(min, Some(right));
            return Some(new_node);
        } else {
            return Some(left.or(right).unwrap());
        }

        Some(new_node)
    } else {
        t
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

fn delete_even_numbers(mut t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        node.left = delete_even_numbers(node.left);
        node.right = delete_even_numbers(node.right);

        if node.element % 2 == 0 {
            t = delete(node.element, t);
        }
    }
    t
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();

    let mut t: SearchTree = None;
    let has_input = line.trim().split_whitespace().any(|token| {
        if let Ok(num) = token.parse() {
            t = insert(num, t);
            true
        } else {
            false
        }
    });

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
        println!("Min is {}, Max is {}", retrieve(find_min(&t).unwrap()), retrieve(find_max(&t).unwrap()));
    } else {
        println!("Tree is empty");
    }

    t = make_empty(t);
}