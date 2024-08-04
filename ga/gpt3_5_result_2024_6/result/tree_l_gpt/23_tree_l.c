use std::ptr;

struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

impl TreeNode {
    fn new(element: i32) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
}

fn make_empty(t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        node.left = make_empty(node.left);
        node.right = make_empty(node.right);
    }
    None
}

fn find(x: i32, t: &SearchTree) -> Option<&TreeNode> {
    if let Some(node) = t {
        if x < node.element {
            find(x, &node.left)
        } else if x > node.element {
            find(x, &node.right)
        } else {
            Some(node)
        }
    } else {
        None
    }
}

fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    let mut t = t.as_ref();
    while let Some(node) = t {
        if let Some(ref left) = node.left {
            t = Some(left);
        } else {
            return Some(node);
        }
    }
    None
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut t = t.as_ref();
    while let Some(node) = t {
        if let Some(ref right) = node.right {
            t = Some(right);
        } else {
            return Some(node);
        }
    }
    None
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => Some(Box::new(TreeNode::new(x))),
        Some(mut node) => {
            if x < node.element {
                node.left = insert(x, node.left);
            } else if x > node.element {
                node.right = insert(x, node.right);
            }
            Some(node)
        }
    }
}

fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => None,
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left);
            } else if x > node.element {
                node.right = delete(x, node.right);
            } else if node.left.is_some() && node.right.is_some() {
                let min_right = find_min(&node.right);
                if let Some(min_node) = min_right {
                    node.element = min_node.element;
                    node.right = delete(node.element, node.right);
                }
            } else {
                let temp = node;
                if temp.left.is_none() {
                    return temp.right;
                } else {
                    return temp.left;
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
    match t {
        Some(mut node) => {
            node.left = delete_even_numbers(node.left);
            node.right = delete_even_numbers(node.right);

            if node.element % 2 == 0 {
                return delete(node.element, Some(node));
            }
            Some(node)
        }
        None => None,
    }
}

fn main() {
    let mut t: SearchTree = None;
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("Failed to read input");
    let nums: Vec<i32> = line.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if nums.is_empty() {
        println!("Tree is empty");
        return;
    }

    for &num in &nums {
        t = insert(num, t);
    }

    println!("Tree after insertion:");
    print_in_order(&t);

    t = delete_even_numbers(t);

    println!("Tree after deleting even numbers:");
    print_in_order(&t);

    if let Some(node) = t {
        if let Some(min_node) = find_min(&t) {
            if let Some(max_node) = find_max(&t) {
                println!("Min is {}, Max is {}", retrieve(min_node), retrieve(max_node));
            }
        }
    } else {
        println!("Tree is empty");
    }

    t = make_empty(t);
}