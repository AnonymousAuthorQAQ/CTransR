use std::io::{self, BufRead};

struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;
type Position = Option<Box<TreeNode>>;

impl TreeNode {
    fn new(element: i32) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
}

fn make_empty(tree: SearchTree) -> SearchTree {
    if let Some(mut t) = tree {
        t.left = make_empty(t.left);
        t.right = make_empty(t.right);
    }
    None
}

fn find(x: i32, tree: SearchTree) -> Position {
    match tree {
        Some(ref t) => {
            if x < t.element {
                find(x, t.left.clone())
            } else if x > t.element {
                find(x, t.right.clone())
            } else {
                Some(Box::new(t.clone()))
            }
        },
        None => None,
    }
}

fn find_min(tree: SearchTree) -> Position {
    match tree {
        Some(ref t) => {
            if t.left.is_none() {
                Some(Box::new(t.clone()))
            } else {
                find_min(t.left.clone())
            }
        },
        None => None,
    }
}

fn find_max(tree: SearchTree) -> Position {
    match tree {
        Some(ref mut t) => {
            let mut temp = t.clone();
            while let Some(ref mut right) = temp.right {
                temp = right.clone();
            }
            Some(Box::new(temp))
        },
        None => None,
    }
}

fn insert(x: i32, tree: SearchTree) -> SearchTree {
    match tree {
        Some(mut t) => {
            if x < t.element {
                t.left = insert(x, t.left);
            } else if x > t.element {
                t.right = insert(x, t.right);
            }
            Some(t)
        },
        None => Some(Box::new(TreeNode::new(x))),
    }
}

fn delete(x: i32, tree: SearchTree) -> SearchTree {
    match tree {
        Some(mut t) => {
            if x < t.element {
                t.left = delete(x, t.left);
            } else if x > t.element {
                t.right = delete(x, t.right);
            } else if t.left.is_some() && t.right.is_some() {
                let tmp_cell = find_min(t.right.clone());
                t.element = tmp_cell.unwrap().element;
                t.right = delete(t.element, t.right);
            } else {
                let tmp_cell = t.clone();
                if t.left.is_none() {
                    t = t.right.unwrap();
                } else if t.right.is_none() {
                    t = t.left.unwrap();
                }
                return None;
            }
            Some(t)
        },
        None => None,
    }
}

fn retrieve(p: Position) -> i32 {
    p.unwrap().element
}

fn print_in_order(tree: SearchTree) {
    match tree {
        Some(t) => {
            print_in_order(t.left.clone());
            println!("{}", t.element);
            print_in_order(t.right.clone());
        },
        None => return,
    }
}

fn delete_even_numbers(tree: SearchTree) -> SearchTree {
    match tree {
        Some(mut t) => {
            t.left = delete_even_numbers(t.left);
            t.right = delete_even_numbers(t.right);

            if t.element % 2 == 0 {
                t = delete(t.element, t);
            }
            
            Some(t)
        },
        None => None,
    }
}

fn main() {
    let mut tree: SearchTree = None;
    let mut has_input = false;

    let stdin = io::stdin();
    let input = stdin.lock();
    for line in input.lines() {
        if let Ok(line) = line {
            for token in line.trim().split_whitespace() {
                if let Ok(num) = token.parse::<i32>() {
                    tree = insert(num, tree);
                    has_input = true;
                }
            }
        }
    }

    if !has_input || tree.is_none() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    print_in_order(tree.clone());

    // Delete all even numbers
    tree = delete_even_numbers(tree.clone());

    println!("Tree after deleting even numbers:");
    print_in_order(tree.clone());

    match tree {
        Some(t) => {
            if let Some(min) = retrieve(find_min(tree.clone())) {
                if let Some(max) = retrieve(find_max(tree.clone())) {
                    println!("Min is {}, Max is {}", min, max);
                }
            }
        },
        None => println!("Tree is empty"),
    }

    tree = make_empty(tree);
}