use std::io::{self, BufRead};

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    t.map(|mut root| {
        root.left = make_empty(root.left.take());
        root.right = make_empty(root.right.take());
        root
    })
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

fn find_min(t: &SearchTree) -> &TreeNode {
    let mut current = t.as_ref().unwrap();
    while let Some(ref left) = current.left {
        current = left;
    }
    current
}

fn find_max(t: &SearchTree) -> &TreeNode {
    let mut current = t.as_ref().unwrap();
    while let Some(ref right) = current.right {
        current = right;
    }
    current
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => {
            Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            }))
        }
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
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left);
            } else if x > node.element {
                node.right = delete(x, node.right);
            } else if node.left.is_some() && node.right.is_some() {
                let min_right = find_min(&node.right);
                node.element = min_right.element;
                node.right = delete(node.element, node.right);
            } else {
                if node.left.is_none() {
                    return node.right.take();
                } else if node.right.is_none() {
                    return node.left.take();
                }
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
    t.map(|mut node| {
        node.left = delete_even_numbers(node.left.take());
        node.right = delete_even_numbers(node.right.take());

        if node.element % 2 == 0 {
            node = delete(node.element, Some(node)).unwrap();
        }

        node
    })
}

fn main() {
    let stdin = io::stdin();
    let mut tree: SearchTree = None;

    if let Some(Ok(line)) = stdin.lock().lines().next() {
        for token in line.trim().split_whitespace() {
            if let Ok(num) = token.parse() {
                tree = insert(num, tree);
            }
        }
    }

    if let Some(tree_ref) = &tree {
        println!("Tree after insertion:");
        print_in_order(tree_ref);

        tree = delete_even_numbers(tree);

        println!("Tree after deleting even numbers:");
        print_in_order(&tree);

        println!("Min is {}, Max is {}", retrieve(find_min(&tree)), retrieve(find_max(&tree)));
    } else {
        println!("Tree is empty");
    }

    tree = make_empty(tree);
}