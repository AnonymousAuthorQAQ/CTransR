use std::ptr;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    if let Some(mut tree) = t {
        tree.left = make_empty(tree.left);
        tree.right = make_empty(tree.right);
    }
    None
}

fn find(x: ElementType, t: SearchTree) -> Option<&TreeNode> {
    match t {
        Some(ref tree) => {
            if x < tree.element {
                find(x, tree.left.as_ref())
            } else if x > tree.element {
                find(x, tree.right.as_ref())
            } else {
                Some(tree)
            }
        }
        None => None,
    }
}

fn find_min(t: SearchTree) -> Option<&TreeNode> {
    let mut current = t.as_ref();
    if let Some(mut tree) = current {
        while let Some(ref mut left) = tree.left {
            current = tree.left.as_ref();
        }
    }
    current
}

fn find_max(t: SearchTree) -> Option<&TreeNode> {
    let mut current = t.as_ref();
    if let Some(mut tree) = current {
        while let Some(ref mut right) = tree.right {
            current = tree.right.as_ref();
        }
    }
    current
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
        None => {
            let new_tree = TreeNode {
                element: x,
                left: None,
                right: None,
            };
            Some(Box::new(new_tree))
        }
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
                let tmp_cell;
                match (tree.left.as_ref(), tree.right.as_ref()) {
                    (Some(_), Some(_)) => {
                        tmp_cell = find_min(tree.right.as_ref()).unwrap().element;
                        tree.element = tmp_cell;
                        tree.right = delete(tmp_cell, tree.right);
                    }
                    _ => {
                        tmp_cell = tree;
                        if tree.left.is_none() {
                            return tree.right;
                        } else if tree.right.is_none() {
                            return tree.left;
                        }
                        drop(tmp_cell);
                        None
                    }
                }
            }
            Some(tree)
        }
        None => None,
    }
}

fn retrieve(p: &TreeNode) -> ElementType {
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
        if let Some(p) = find(i, t) {
            if retrieve(p) != i {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        if let Some(p) = find(i, t) {
            if retrieve(p) != i {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        if let Some(_) = find(i, t) {
            println!("Error at {}", i);
        }
    }

    if let Some(min) = find_min(t) {
        if let Some(max) = find_max(t) {
            println!("Min is {}, Max is {}", retrieve(min), retrieve(max));
        }
    }
}