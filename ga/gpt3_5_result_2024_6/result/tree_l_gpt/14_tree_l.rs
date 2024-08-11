use std::io::{self, BufRead};

struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;
type Position = Option<Box<TreeNode>>;

impl TreeNode {
    fn new(value: i32) -> TreeNode {
        TreeNode {
            element: value,
            left: None,
            right: None,
        }
    }
}

fn make_empty(t: SearchTree) -> SearchTree {
    if let mut tree = t {
        tree.left = make_empty(tree.left);
        tree.right = make_empty(tree.right);
    }
    None
}

fn find(x: i32, t: SearchTree) -> Position {
    match t {
        Some(ref tree) if x < tree.element => find(x, tree.left.clone()),
        Some(ref tree) if x > tree.element => find(x, tree.right.clone()),
        _ => t,
    }
}

fn find_min(t: SearchTree) -> Position {
    match t {
        Some(ref mut tree) => {
            let mut current = tree;
            while let Some(ref mut left_node) = current.left {
                current = left_node;
            }
            Some(Box::new(TreeNode::new(current.element))
        }
        None => None,
    }
}

fn find_max(t: SearchTree) -> Position {
    match t {
        Some(ref mut tree) => {
            let mut current = tree;
            while let Some(ref mut right_node) = current.right {
                current = right_node;
            }
            Some(Box::new(TreeNode::new(current.element))
        }
        None => None,
    }
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => Some(Box::new(TreeNode::new(x))),
        Some(mut tree) => {
            if x < tree.element {
                tree.left = insert(x, tree.left);
            } else if x > tree.element {
                tree.right = insert(x, tree.right);
            }
            Some(tree)
        }
    }
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    if let Some(mut tree) = t {
        if x < tree.element {
            tree.left = delete(x, tree.left);
        } else if x > tree.element {
            tree.right = delete(x, tree.right);
        } else if let (l, r) = (tree.left.take(), tree.right.take()) {
            match (l, find_min(r)) {
                (_, None) => return r,
                (Some(left), Some(min)) => {
                    tree.element = min.element;
                    tree.right =Some(min);
                    tree.left =Some(delete(min.element,r));
                }
                _ => (),
            }
        }
        Some(tree)
    } else {
        None
    }
}

fn retrieve(p: &Position) -> i32 {
    p.as_ref().unwrap().element
}

fn print_in_order(t: &SearchTree) {
    if let Some(ref tree) = t {
        print_in_order(&tree.left);
        println!("{}", tree.element);
        print_in_order(&tree.right);
    }
}

fn delete_even_numbers(t: &mut SearchTree) {
    if let Some(ref mut tree) = t {
        delete_even_numbers(&mut tree.left);
        delete_even_numbers(&mut tree.right);

        if tree.element % 2 == 0 {
            *t = delete(tree.element, t.clone());
        }
    }
}

fn main() {
    let mut t: SearchTree = None;
    let stdin = io::stdin();
    let mut line = String::new();

    if let Ok(_) = stdin.lock().read_line(&mut line) {
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                t = insert(num, t);
            }
        }
    }

    if t.is_none() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    print_in_order(&t);

    delete_even_number(&mut t);

    println!("Tree after deleting even numbers:");
    print_in_order(&t);

    if let Some(ref tree) = t {
        let min = retrieve(&find_min(t).unwrap());
        let max = retrieve(&find_max(t).unwrap());
        println!("Min is {}, Max is {}", min, max);
    } else {
        println!("Tree is empty");
    }

    t = make_empty(t);
}