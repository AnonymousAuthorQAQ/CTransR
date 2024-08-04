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
        tree.left = make_empty(tree.left.take());
        tree.right = make_empty(tree.right.take());
    }
    t
}

fn find(x: ElementType, t: &SearchTree) -> Option<&TreeNode> {
    match t {
        Some(tree) => {
            if x < tree.element {
                find(x, &tree.left)
            } else if x > tree.element {
                find(x, &tree.right)
            } else {
                Some(tree)
            }
        }
        None => None,
    }
}

fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    match t {
        Some(tree) => {
            if let Some(ref mut left) = &tree.left {
                find_min(&Some(left))
            } else {
                Some(tree)
            }
        }
        None => None,
    }
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut tree = t;
    while let Some(cur) = tree {
        if let Some(ref right_tree) = &cur.right {
            tree = &Some(right_tree);
        } else {
            return Some(cur);
        }
    }
    None
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
        Some(mut tree) => {
            if x < tree.element {
                tree.left = insert(x, tree.left.take());
            } else if x > tree.element {
                tree.right = insert(x, tree.right.take());
            }
            Some(tree)
        }
    }
}

fn delete(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => panic!("Element not found"),
        Some(mut tree) => {
            if x < tree.element {
                tree.left = delete(x, tree.left.take());
            } else if x > tree.element {
                tree.right = delete(x, tree.right.take());
            } else if tree.left.is_some() && tree.right.is_some() {
                let tmp_cell = find_min(&tree.right);
                if let Some(tmp) = tmp_cell {
                    tree.element = tmp.element;
                    tree.right = delete(tmp.element, tree.right.take());
                }
            } else {
                let tmp_cell = tree;
                if tree.left.is_none() {
                    tree = tree.right.take();
                } else {
                    tree = tree.left.take();
                }
                drop(tmp_cell);
            }
            Some(tree)
        }
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

    if let Some(min) = find_min(&t) {
        if let Some(max) = find_max(&t) {
            println!("Min is {}, Max is {}", retrieve(min), retrieve(max));
        }
    }
}