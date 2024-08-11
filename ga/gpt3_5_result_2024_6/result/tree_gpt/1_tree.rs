use std::ptr;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;
type Position = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    if let Some(mut tree) = t {
        tree.left = make_empty(tree.left);
        tree.right = make_empty(tree.right);
    }
    t
}

fn find(x: ElementType, t: SearchTree) -> Position {
    match t {
        Some(tree) => {
            if x < tree.element {
                find(x, tree.left)
            } else if x > tree.element {
                find(x, tree.right)
            } else {
                Some(tree)
            }
        }
        None => None,
    }
}

fn find_min(t: SearchTree) -> Position {
    match t {
        Some(mut tree) => {
            if let Some(mut left_tree) = tree.left {
                while let Some(left) = left_tree.left {
                    left_tree = left;
                }
                Some(left_tree)
            } else {
                Some(tree)
            }
        }
        None => None,
    }
}

fn find_max(t: SearchTree) -> Position {
    match t {
        Some(mut tree) => {
            if let Some(mut right_tree) = tree.right {
                while let Some(right) = right_tree.right {
                    right_tree = right;
                }
                Some(right_tree)
            } else {
                Some(tree)
            }
        }
        None => None,
    }
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
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
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
                if let Some(mut right) = tree.right {
                    let mut tmp_cell = find_min(tree.right);
                    tree.element = tmp_cell.as_ref().unwrap().element;
                }
                let tmp_cell = tree;
                if tree.left.is_none() {
                    tree = tree.right.take();
                } else if tree.right.is_none() {
                    tree = tree.left.take();
                }
                drop(tmp_cell);
            }
            Some(tree)
        }
        None => None,
    }
}

fn retrieve(p: Position) -> ElementType {
    p.unwrap().element
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
            if retrieve(Some(p)) != i {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        if let Some(p) = find(i, t) {
            if retrieve(Some(p)) != i {
                println!("Error at {}", i);
            }
        }
    }
    for i in (0..50).step_by(2) {
        if let Some(p) = find(i, t) {
            println!("Error at {}", i);
        }
    }

    if let Some(min) = find_min(t) {
        if let Some(max) = find_max(t) {
            println!("Min is {}, Max is {}", retrieve(Some(min)), retrieve(Some(max));
        }
    }
}