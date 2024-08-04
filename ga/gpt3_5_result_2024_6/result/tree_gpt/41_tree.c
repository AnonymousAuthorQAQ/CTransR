use std::mem;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;
type Position = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    t
}

fn find(x: ElementType, t: SearchTree) -> Position {
    match t {
        Some(ref node) if x < node.element => find(x, node.left.as_ref().map(|n| n.clone())),
        Some(ref node) if x > node.element => find(x, node.right.as_ref().map(|n| n.clone())),
        _ => t,
    }
}

fn find_min(t: SearchTree) -> Position {
    match t {
        Some(ref node) => node.left.clone().map_or(t, |n| find_min(Some(n))),
        _ => t,
    }
}

fn find_max(t: SearchTree) -> Position {
    t.as_ref().map(|mut node| {
        while let Some(ref child) = node.right {
            node = child;
        }
        Some(node.clone())
    }).flatten()
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
        Some(mut node) if x < node.element => {
            node.left = insert(x, node.left.take());
            Some(node)
        },
        Some(mut node) if x > node.element => {
            node.right = insert(x, node.right.take());
            Some(node)
        },
        _ => t // element already in tree
    }
}

fn delete(x: ElementType, t: SearchTree) -> SearchTree {
    // Implement Deletion logic here
    unimplemented!()
}

fn retrieve(p: Position) -> ElementType {
    // Return the element in the position p
    if let Some(node) = p {
        node.element
    } else {
        panic!("Element not found")
    }
}

fn main() {
    let mut t = make_empty(None);
    
    // Inserting numbers into the tree
    for i in (0..50).step_by(7) {
        t = insert(i, t);
    }

    // Searching for elements in the tree
    for i in 0..50 {
        let p = find(i, t);
        if p.is_none() || retrieve(p) != i {
            println!("Error at {}", i);
        }
    }

    // Deleting even numbers from the tree
    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    // Checking if odd numbers are still present in the tree
    for i in (1..50).step_by(2) {
        let p = find(i, t);
        if p.is_none() || retrieve(p) != i {
            println!("Error at {}", i);
        }
    }

    // Checking minimum and maximum elements
    if let Some(p) = find_min(t) {
        println!("Min is {}", retrieve(Some(p)));
    }
    if let Some(p) = find_max(t) {
        println!("Max is {}", retrieve(Some(p)));
    }
}