use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    if let Some(mut tree) = t {
        tree.left = make_empty(tree.left.take());
        tree.right = make_empty(tree.right.take());
    }
    t
}

fn find(x: i32, t: &SearchTree) -> Option<&TreeNode> {
    match t {
        None => None,
        Some(node) => {
            match x.cmp(&node.element) {
                Ordering::Less => find(x, &node.left),
                Ordering::Greater => find(x, &node.right),
                _ => Some(node),
            }
        }
    }
}

fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    t.as_ref().and_then(|node| {
        if let Some(left) = &node.left {
            find_min(&Some(left))
        } else {
            Some(node)
        }
    })
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut current = t.as_ref();
    while let Some(node) = current {
        if let Some(right) = &node.right {
            current = Some(right);
        } else {
            break;
        }
    }
    current
}

fn insert(x: i32, mut t: SearchTree) -> SearchTree {
    if let Some(mut tree) = t {
        if x < tree.element {
            tree.left = Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            }));
        } else if x > tree.element {
            tree.right = Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            }));
        }
        t = Some(tree);
    } else {
        t = Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        }));
    }
    t
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    let mut new_tree = t;
    if let Some(mut t_node) = new_tree.take() {
        match x.cmp(&t_node.element) {
            Ordering::Less => {
                t_node.left = delete(x, t_node.left.take());
            },
            Ordering::Greater => {
                t_node.right = delete(x, t_node.right.take());
            },
            _ => {
                if t_node.left.is_none() {
                    new_tree = t_node.right.take();
                } else if t_node.right.is_none() {
                    new_tree = t_node.left.take();
                } else {
                    let mut tmp_cell = find_min(&t_node.right);
                    t_node.element = tmp_cell.unwrap().element;
                    t_node.right = delete(t_node.element, t_node.right.take());
                    new_tree = Some(t_node);
                }
            },
        }
    }

    new_tree
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
    if let Some(node) = t.as_mut() {
        node.left = delete_even_numbers(node.left.take());
        node.right = delete_even_numbers(node.right.take());
        if node.element % 2 == 0 {
            t = delete(node.element, t);
        }
    }
    t
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut t: SearchTree = None;

    for token in input.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
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

    match (find_min(&t), find_max(&t)) {
        (Some(min_node), Some(max_node)) => {
            println!("Min is {}, Max is {}", retrieve(&min_node), retrieve(&max_node));
        },
        _ => println!("Tree is empty"),
    }

    t = make_empty(t);
}