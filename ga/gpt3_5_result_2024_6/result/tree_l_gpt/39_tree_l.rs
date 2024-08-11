use std::io::{self, BufRead};

struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(element: i32) -> Self {
        TreeNode { element, left: None, right: None }
    }
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
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

fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    match t {
        Some(node) => {
            if let Some(ref left) = node.left {
                find_min(&node.left)
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    match t {
        Some(node) => {
            let mut current = node;
            while let Some(ref right) = current.right {
                current = right;
            }
            Some(current)
        }
        None => None,
    }
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
        None => Some(Box::new(TreeNode::new(x))),
    }
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left);
            } else if x > node.element {
                node.right = delete(x, node.right);
            } else if node.left.is_some() && node.right.is_some() {
                let min_right = find_min(&node.right).unwrap().element;
                node.element = min_right;
                node.right = delete(min_right, node.right);
            } else {
                return if node.left.is_none() {
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
    let mut has_input = false;

    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    for token in input.split_whitespace() {
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

    // Delete all even numbers
    t = delete_even_numbers(t);

    println!("Tree after deleting even numbers:");
    print_in_order(&t);

    if let Some(tree) = &t {
        let min = retrieve(find_min(&t).unwrap());
        let max = retrieve(find_max(&t).unwrap());
        println!("Min is {}, Max is {}", min, max);
    } else {
        println!("Tree is empty");
    }

    t = make_empty(t);
}