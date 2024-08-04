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

fn find(x: i32, t: SearchTree) -> Option<&TreeNode> {
    match t {
        Some(node) => {
            if x < node.element {
                find(x, node.left.as_ref())
            } else if x > node.element {
                find(x, node.right.as_ref())
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_min(t: SearchTree) -> Option<&TreeNode> {
    t.as_ref().and_then(|node| {
        node.left.as_ref().map_or(Some(node), |left| find_min(Some(left)))
    })
}

fn find_max(t: SearchTree) -> Option<&TreeNode> {
    let mut node = t.as_ref();
    while let Some(n) = node {
        if let Some(r) = n.right.as_ref() {
            node = Some(r);
        } else {
            break;
        }
    }
    node
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            if x < node.element {
                node.left = insert(x, node.left.take());
            } else if x > node.element {
                node.right = insert(x, node.right.take());
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
            node.left = delete(x, node.left.take());
        } else if x > node.element {
            node.right = delete(x, node.right.take());
        } else {
            let tmp_cell = if node.left.is_some() && node.right.is_some() {
                let min_right = find_min(node.right.as_ref()).unwrap().element;
                node.element = min_right;
                delete(min_right, node.right.take());
                None
            } else {
                Some(node)
            };
            return tmp_cell
        }
        return Some(node);
    }
    None
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
        return Some(node);
    }
    None
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

    if let Some(t) = t {
        let min = find_min(Some(&t)).unwrap().element;
        let max = find_max(Some(&t)).unwrap().element;
        println!("Min is {}, Max is {}", min, max);
    } else {
        println!("Tree is empty");
    }

    t = make_empty(t);
}