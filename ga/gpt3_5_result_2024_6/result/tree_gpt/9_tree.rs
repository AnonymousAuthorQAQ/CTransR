use std::ptr;

#[derive(Debug)]
struct TreeNode {
    element: i32,
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

fn find(x: i32, t: SearchTree) -> Position {
    match t {
        None => None,
        Some(ref tree) => {
            if x < tree.element {
                find(x, tree.left.as_ref().map(|x| x.clone()))
            } else if x > tree.element {
                find(x, tree.right.as_ref().map(|x| x.clone()))
            } else {
                t
            }
        }
    }
}

fn find_min(t: SearchTree) -> Position {
    match t {
        None => None,
        Some(ref tree) => {
            if tree.left.is_none() {
                t
            } else {
                find_min(tree.left.as_ref().map(|x| x.clone()))
            }
        }
    }
}

fn find_max(t: SearchTree) -> Position {
    match t {
        None => None,
        Some(mut tree) => {
            while let Some(ref mut right) = tree.right {
                tree = right.clone();
            }
            Some(tree)
        }
    }
}

fn insert(x: i32, mut t: SearchTree) -> SearchTree {
    if let Some(mut tree) = t {
        if x < tree.element {
            tree.left = insert(x, tree.left);
        } else if x > tree.element {
            tree.right = insert(x, tree.right);
        }
        Some(tree)
    } else {
        t = Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        }));
        t
    }
}

fn delete(x: i32, mut t: SearchTree) -> SearchTree {
    if let Some(mut tree) = t.take() {
        if x < tree.element {
            tree.left = delete(x, tree.left);
            Some(tree)
        } else if x > tree.element {
            tree.right = delete(x, tree.right);
            Some(tree)
        } else if let (left, right) = (tree.left.take(), tree.right.take()) {
            let tmp_cell = find_min(right.as_ref().map(|x| x.clone()));
            if let Some(r_min) = tmp_cell {
                tree.element = r_min.element;
                tree.right = delete(tree.element, Some(r_min));
                Some(tree)
            } else {
                if let Some(left_tree) = left {
                    t = Some(left_tree);
                } else if let Some(right_tree) = right {
                    t = Some(right_tree);
                } 
                None
            }
        } else {
            if let Some(left_tree) = left {
                t = Some(left_tree);
            } else if let Some(right_tree) = right {
                t = Some(right_tree);
            } 
            None
        }
    } else {
        None
    }
}

fn retrieve(p: Position) -> i32 {
    if let Some(tree) = p {
        tree.element
    } else {
        0
    }
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
        let p = find(i, t);
        if let Some(pos) = p {
            if retrieve(Some(pos.clone())) != i {
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
        let p = find(i, t);
        if let Some(pos) = p {
            if retrieve(Some(pos.clone())) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        let p = find(i, t);
        if p.is_some() {
            println!("Error at {}", i);
        }
    }

    let min_val = find_min(t);
    let max_val = find_max(t);
    match (min_val, max_val) {
        (Some(min), Some(max)) => {
            println!("Min is {}, Max is {}", retrieve(min), retrieve(max));
        }
        _ => {}
    }
}