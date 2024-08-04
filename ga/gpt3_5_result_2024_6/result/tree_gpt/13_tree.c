use std::ptr;

type ElementType = i32;

#[derive(Debug)]
struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;
type Position = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    if let Some(mut tree) = t {
        tree.left = make_empty(tree.left.take());
        tree.right = make_empty(tree.right.take());
    }
    t
}

fn find(x: ElementType, t: SearchTree) -> Position {
    match t {
        None => return None,
        Some(ref tree) => {
            if x < tree.element {
                return find(x, tree.left.clone());
            } else if x > tree.element {
                return find(x, tree.right.clone());
            } else {
                return Some(Box::new(tree.clone()));
            }
        }
    }
}

fn find_min(t: SearchTree) -> Position {
    let mut current = t;
    while let Some(ref mut tree) = current {
        if let Some(ref left_tree) = tree.left {
            current = Some(left_tree.clone());
        } else {
            return current;
        }
    }
    None
}

fn find_max(t: SearchTree) -> Position {
    let mut current = t;
    while let Some(ref mut tree) = current {
        if let Some(ref right_tree) = tree.right {
            current = Some(right_tree.clone());
        } else {
            return current;
        }
    }
    None
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => return Some(Box::new(TreeNode {
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
            return Some(tree);
        }
    }
}

fn delete(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => return None,
        Some(mut tree) => {
            if x < tree.element {
                tree.left = delete(x, tree.left.take());
            } else if x > tree.element {
                tree.right = delete(x, tree.right.take());
            } else if tree.left.is_some() && tree.right.is_some() {
                let tmp_cell = find_min(tree.right.take());
                if let Some(tmp) = tmp_cell {
                    tree.element = tmp.element;
                    tree.right = delete(tmp.element, tree.right.take());
                }
            } else {
                let tmp_cell = tree;
                if tree.left.is_none() {
                    tree = tree.right.take().unwrap();
                } else if tree.right.is_none() {
                    tree = tree.left.take().unwrap();
                }
            }
            return Some(tree);
        }
    }
}

fn retrieve(p: &Position) -> ElementType {
    p.as_ref().unwrap().element
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
        if let Some(p) = find(i, t.clone()) {
            if retrieve(&Some(p)) != i {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        if let Some(p) = find(i, t.clone()) {
            if retrieve(&Some(p)) != i {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        if let Some(_) = find(i, t.clone()) {
            println!("Error at {}", i);
        }
    }

    let min_val = find_min(t);
    let max_val = find_max(t);

    if let Some(min_node) = min_val {
        println!("Min is {}, Max is {}", retrieve(&min_val), retrieve(&max_val));
    }
}