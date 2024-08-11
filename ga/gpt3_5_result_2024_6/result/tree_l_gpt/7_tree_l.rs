use std::io::{self, BufRead};

struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(element: i32) -> TreeNode {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    if let mut tree = t {
        tree.left = make_empty(tree.left.take());
        tree.right = make_empty(tree.right.take());
    }
    t
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => Some(Box::new(TreeNode::new(x))),
        Some(mut tree) => {
            if x < tree.element {
                tree.left = insert(x, tree.left.take());
            } else if x > tree.element {
                tree.right = insert(x, tree.right.take());
            }
            Some(tree)
        },
    }
}

fn print_in_order(t: &SearchTree) {
    if let Some(ref tree) = *t {
        print_in_order(&tree.left);
        println!("{}", tree.element);
        print_in_order(&tree.right);
    }
}

fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    t.as_ref().and_then(|tree| {
        find_min(&tree.left).or(Some(tree))
    })
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut current = t;
    while let Some(ref tree) = **current {
        current = &tree.right;
    }
    current.as_ref().map(|tree| &**tree)
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => t,
        Some(mut tree) => {
            if x < tree.element {
                tree.left = delete(x, tree.left.take());
            } else if x > tree.element {
                tree.right = delete(x, tree.right.take());
            } else if let (Some(left), Some(right)) = (tree.left.take(), tree.right.take()) {
                let min_node = find_min(&tree.right).unwrap().element;
                tree.element = min_node;
                tree.right = delete(min_node, Some(right));
            } else {
                tree = tree.left.take().or(tree.right.take());
            }
            Some(tree)
        }
    }
}

fn delete_even_numbers(t: SearchTree) -> SearchTree {
    match t {
        None => None,
        Some(mut tree) => {
            tree.left = delete_even_numbers(tree.left.take());
            tree.right = delete_even_numbers(tree.right.take());

            if tree.element % 2 == 0 {
                tree = delete(tree.element, Some(tree));
            }
            Some(tree)
        }
    }
}

fn main() {
    let mut t: SearchTree = None;
    let mut line = String::new();
    if let Ok(_) = io::stdin().lock().read_line(&mut line) {
        for token in line.trim().split(' ') {
            if let Ok(num) = token.parse::<i32>() {
                t = insert(num, t);
            }
        }
    }

    if let Some(ref t) = t {
        println!("Tree after insertion:");
        print_in_order(&t);

        t = delete_even_numbers(t);

        println!("Tree after deleting even numbers:");
        print_in_order(&t);

        if let Some(min_node) = find_min(&t) {
            if let Some(max_node) = find_max(&t) {
                println!("Min is {}, Max is {}", min_node.element, max_node.element);
            }
        }
    } else {
        println!("Tree is empty");
    }

    t = make_empty(t);
}