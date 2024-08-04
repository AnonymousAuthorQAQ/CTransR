use std::ptr;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(tree: SearchTree) -> SearchTree {
    if let Some(mut t) = tree {
        t.left = make_empty(t.left);
        t.right = make_empty(t.right);
        t = None;
    }
    tree
}

fn find(x: i32, tree: &SearchTree) -> Option<&TreeNode> {
    match tree {
        Some(node) if x < node.element => find(x, &node.left),
        Some(node) if x > node.element => find(x, &node.right),
        _ => tree.as_ref().map(|node| &**node),
    }
}

fn find_min(tree: &SearchTree) -> Option<&TreeNode> {
    match tree {
        Some(node) if node.left.is_none() => tree.as_ref().map(|node| &**node),
        Some(node) => find_min(&node.left),
        _ => None,
    }
}

fn find_max(tree: &SearchTree) -> Option<&TreeNode> {
    let mut current_tree = tree;
    while let Some(ref node) = current_tree {
        if node.right.is_none() {
            return Some(&**node);
        }
        current_tree = &node.right;
    }
    None
}

fn insert(x: i32, tree: SearchTree) -> SearchTree {
    match tree {
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
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

fn delete(x: i32, tree: SearchTree) -> SearchTree {
    match tree {
        Some(mut node) if x < node.element => {
            node.left = delete(x, node.left);
            Some(node)
        }
        Some(mut node) if x > node.element => {
            node.right = delete(x, node.right);
            Some(node)
        }
        Some(mut node) if node.left.is_some() && node.right.is_some() => {
            let mut tmp_cell = find_min(&node.right).unwrap().element;
            node.element = tmp_cell;
            node.right = delete(tmp_cell, node.right);
            Some(node)
        }
        Some(mut node) => {
            let tmp_cell = node;
            if node.left.is_none() {
                return node.right;
            } else if node.right.is_none() {
                return node.left;
            }
            ptr::drop_in_place(tmp_cell);
            None
        }
        None => None,
    }
}

fn retrieve(p: &TreeNode) -> i32 {
    p.element
}

fn main() {
    let mut t: SearchTree = None;
    let mut j = 0;

    t = make_empty(t);
    for i in 0..50 {
        j = (j + 7) % 50;
        t = insert(j, t);
    }

    for i in 0..50 {
        if let Some(p) = find(i, &t) {
            if retrieve(p) != i {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        if let Some(p) = find(i, &t) {
            if retrieve(p) != i {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        if let Some(_) = find(i, &t) {
            println!("Error at {}", i);
        }
    }

    match (find_min(&t), find_max(&t)) {
        (Some(min), Some(max)) => println!("Min is {}, Max is {}", retrieve(min), retrieve(max)),
        _ => println!("No elements in tree"),
    }
}