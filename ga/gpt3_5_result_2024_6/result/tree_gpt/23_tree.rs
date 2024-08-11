use std::ptr;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

impl TreeNode {
    fn new(element: ElementType) -> Self {
        TreeNode { element, left: None, right: None }
    }
}

fn make_empty(t: SearchTree) -> SearchTree {
    if let Some(mut tree) = t {
        tree.left = make_empty(tree.left);
        tree.right = make_empty(tree.right);
        return None;
    }
    
    return None;
}

fn find(x: ElementType, t: &SearchTree) -> Option<&TreeNode> {
    match t {
        Some(ref tree) => {
            if x < tree.element {
                find(x, &tree.left)
            } else if x > tree.element {
                find(x, &tree.right)
            } else {
                Some(&**tree)
            }
        }
        None => None
    }
}

fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    match t {
        Some(ref tree) => {
            if let Some(ref mut left) = tree.left {
                find_min(&left)
            } else {
                Some(&**tree)
            }
        }
        None => None
    }
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut node = t;

    while let Some(ref tree) = node {
        if let Some(ref right) = tree.right {
            node = &right;
        } else {
            break;
        }
    }

    node.as_ref().map(|node| &**node)
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        Some(mut tree) => {
            if x < tree.element {
                tree.left = insert(x, tree.left);
            } else if x > tree.element {
                tree.right = insert(x, tree.right);
            }
            Some(tree)
        }
        None => Some(Box::new(TreeNode::new(x)))
    }
}

fn delete(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        Some(mut tree) => {
            if x < tree.element {
                tree.left = delete(x, tree.left);
            } else if x > tree.element {
                tree.right = delete(x, tree.right);
            } else {
                let tmp_cell = if let Some(min_right) = find_min(&tree.right) {
                    min_right.element
                } else {
                    tree.element
                };
                tree.element = tmp_cell;
                tree.right = delete(tmp_cell, tree.right);
            }
            Some(tree)
        }
        None => None
    }
}

fn retrieve(p: &TreeNode) -> ElementType {
    p.element
}

fn main() {
    let mut t: SearchTree = None;
    
    for i in 0..50 {
        t = insert((i * 7) % 50, t);
    }
    
    for i in 0..50 {
        if let Some(p) = find(i, &t) {
            if retrieve(p) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
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
        } else {
            println!("Error at {}", i);
        }
    }
    
    for i in (0..50).step_by(2) {
        if find(i, &t).is_some() {
            println!("Error at {}", i);
        }
    }

    let min_value = find_min(&t).map(|node| retrieve(node));
    let max_value = find_max(&t).map(|node| retrieve(node));
    
    match (min_value, max_value) {
        (Some(min), Some(max)) => println!("Min is {}, Max is {}", min, max),
        _ => ()
    }
}