use std::io::{self, BufRead};

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
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
            if let Some(left) = &node.left {
                find_min(&Some(left))
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut result = t;
    while let Some(node) = result {
        if let Some(right) = &node.right {
            result = &Some(right);
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
    if let Some(mut node) = t {
        if x < node.element {
            node.left = delete(x, node.left);
            return Some(node);
        } else if x > node.element {
            node.right = delete(x, node.right);
            return Some(node);
        } else if node.left.is_some() && node.right.is_some() {
            let min_right = find_min(&node.right).unwrap().element;
            node.element = min_right;
            node.right = delete(min_right, node.right);
            return Some(node);
        } else {
            if node.left.is_none() {
                return node.right;
            } else if node.right.is_none() {
                return node.left;
            }
        }
    }
    t
}

fn retrieve(p: &TreeNode) -> i32 {
    p.element
}

fn print_in_order(t: &SearchTree) {
    match t {
        Some(node) => {
            print_in_order(&node.left);
            println!("{}", node.element);
            print_in_order(&node.right);
        }
        None => {}
    }
}

fn delete_even_numbers(t: &mut SearchTree) {
    if let Some(node) = t {
        delete_even_numbers(&mut node.left);
        delete_even_numbers(&mut node.right);
        
        if node.element % 2 == 0 {
            *t = delete(node.element, t);
        }
    }
}

fn main() {
    let mut t: SearchTree = None;
    let stdin = io::stdin();
    
    let mut has_input = false;

    if let Some(input) = stdin.lock().lines().next() {
        if let Ok(line) = input {
            for token in line.trim().split_whitespace() {
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

    delete_even_numbers(&mut t);

    println!("Tree after deleting even numbers:");
    print_in_order(&t);

    if let Some(tree) = &t {
        println!("Min is {}, Max is {}", retrieve(find_min(tree).unwrap()), retrieve(find_max(tree).unwrap()));
    } else {
        println!("Tree is empty");
    }

    t = make_empty(t);
}