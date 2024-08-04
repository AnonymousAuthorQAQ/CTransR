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
    t.as_ref().and_then(|node| {
        if let Some(left) = &node.left {
            find_min(&Some(left))
        } else {
            Some(node)
        }
    })
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    t.as_ref().map(|node| {
        let mut current = node;
        while let Some(right) = &current.right {
            current = right;
        }
        current
    })
}

fn insert(x: i32, mut t: SearchTree) -> SearchTree {
    if let Some(node) = &mut t {
        if x < node.element {
            node.left = insert(x, node.left.take());
        } else if x > node.element {
            node.right = insert(x, node.right.take());
        }
        t
    } else {
        Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        }))
    }
}

fn delete(x: i32, mut t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        if x < node.element {
            node.left = delete(x, node.left.take());
        } else if x > node.element {
            node.right = delete(x, node.right.take());
        } else if node.left.is_some() && node.right.is_some() {
            let min_right = find_min(&node.right).unwrap().element;
            node.element = min_right;
            node.right = delete(min_right, node.right.take());
        } else {
            t = if node.left.is_none() { node.right.take() } else { node.left.take() };
        }
    }
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

fn delete_even_numbers(mut t: SearchTree) -> SearchTree {
    if let Some(mut node) = t.take() {
        node.left = delete_even_numbers(node.left.take());
        node.right = delete_even_numbers(node.right.take());

        if node.element % 2 == 0 {
            t = delete(node.element, Some(node));
        } else {
            t = Some(node);
        }
    }
    t
}

fn main() {
    let stdin = io::stdin();
    let mut tree = None;
    let mut has_input = false;

    if let Ok(mut line) = String::new().read_line(&stdin) {
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                tree = insert(num, tree);
                has_input = true;
            }
        }
    }

    if !has_input || tree.is_none() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    print_in_order(&tree);

    // Delete all even numbers
    tree = delete_even_numbers(tree);

    println!("Tree after deleting even numbers:");
    print_in_order(&tree);

    if let Some(t) = &tree {
        println!("Min is {}, Max is {}", retrieve(find_min(t).unwrap()), retrieve(find_max(t).unwrap()));
    } else {
        println!("Tree is empty");
    }

    tree = make_empty(tree);
}