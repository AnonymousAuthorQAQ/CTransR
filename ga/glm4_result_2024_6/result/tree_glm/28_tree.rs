use std::ptr;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(tree: SearchTree) -> SearchTree {
    if let Some(mut node) = tree {
        node.left = make_empty(node.left);
        node.right = make_empty(node.right);
        None
    } else {
        None
    }
}

fn find(x: ElementType, tree: &SearchTree) -> Option<&TreeNode> {
    let mut current = tree.as_ref()?;
    while current.is_some() {
        if x < current.as_ref().unwrap().element {
            current = current.as_ref().unwrap().left.as_ref();
        } else if x > current.as_ref().unwrap().element {
            current = current.as_ref().unwrap().right.as_ref();
        } else {
            return current;
        }
    }
    None
}

fn find_min(tree: &SearchTree) -> Option<&TreeNode> {
    let mut current = tree.as_ref()?;
    while let Some(node) = current {
        if node.left.is_none() {
            return Some(node);
        }
        current = node.left.as_ref();
    }
    None
}

fn find_max(tree: &SearchTree) -> Option<&TreeNode> {
    let mut current = tree.as_ref()?;
    while let Some(node) = current {
        if node.right.is_none() {
            return Some(node);
        }
        current = node.right.as_ref();
    }
    None
}

fn insert(x: ElementType, tree: SearchTree) -> SearchTree {
    if let None = tree {
        return Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        }));
    }

    let mut current = tree;
    loop {
        match current.as_mut() {
            Some(node) => {
                if x < node.element {
                    current = node.left.insert(x);
                } else if x > node.element {
                    current = node.right.insert(x);
                } else {
                    return current;
                }
            }
            None => return Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
        }
    }
}

fn delete(x: ElementType, tree: SearchTree) -> SearchTree {
    if let Some(mut node) = tree {
        if x < node.element {
            node.left = delete(x, node.left);
        } else if x > node.element {
            node.right = delete(x, node.right);
        } else {
            if let Some(mut right) = node.right.take() {
                node.element = find_min(&right).unwrap().element;
                node.right = delete(node.element, right);
            } else {
                return node.left;
            }
        }
        Some(node)
    } else {
        None
    }
}

fn retrieve(p: &TreeNode) -> ElementType {
    p.element
}

fn main() {
    let mut t: SearchTree = None;
    for i in 0..50 {
        t = insert(i, t);
    }
    for i in 0..50 {
        if let None = find(i, &t) {
            println!("Error at {}", i);
        }
    }
    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }
    for i in (1..50).step_by(2) {
        if let None = find(i, &t) {
            println!("Error at {}", i);
        }
    }
    for i in (0..50).step_by(2) {
        if let Some(_) = find(i, &t) {
            println!("Error at {}", i);
        }
    }
    println!("Min is {}, Max is {}", retrieve(find_min(&t).unwrap()), retrieve(find_max(&t).unwrap()));
}