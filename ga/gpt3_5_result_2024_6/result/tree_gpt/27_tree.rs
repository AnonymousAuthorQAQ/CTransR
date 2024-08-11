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
        None => None,
        Some(ref tree) => {
            if x < tree.element {
                find(x, tree.left.clone())
            } else if x > tree.element {
                find(x, tree.right.clone())
            } else {
                Some(Box::new(TreeNode {
                    element: tree.element,
                    left: tree.left.clone(),
                    right: tree.right.clone(),
                }))
            }
        }
    }
}

fn find_min(t: SearchTree) -> Position {
    match t {
        None => None,
        Some(ref tree) => {
            if let Some(ref mut left) = tree.left {
                find_min(Some(left.clone()))
            } else {
                Some(Box::new(TreeNode {
                    element: tree.element,
                    left: tree.left.clone(),
                    right: tree.right.clone(),
                }))
            }
        }
    }
}

fn find_max(t: SearchTree) -> Position {
    match t {
        None => None,
        Some(ref mut tree) => {
            let mut current = tree;
            while let Some(ref mut right) = current.right {
                current = right;
            }
            Some(Box::new(TreeNode {
                element: current.element,
                left: current.left.clone(),
                right: current.right.clone(),
            }))
        }
    }
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
                tree.left = insert(x, tree.left);
            } else if x > tree.element {
                tree.right = insert(x, tree.right);
            }
            Some(tree)
        }
    }
}

fn delete(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => None,
        Some(mut tree) => {
            if x < tree.element {
                tree.left = delete(x, tree.left);
            } else if x > tree.element {
                tree.right = delete(x, tree.right);
            } else {
                if let (Some(left), Some(right)) = (tree.left.take(), tree.right.take()) {
                    let min_right = find_min(Some(right));
                    if let Some(mut min) = min_right {
                        tree.element = min.element;
                        tree.right = delete(min.element, Some(right));
                    }
                } else {
                    if let Some(left) = tree.left {
                        return Some(left);
                    } else if let Some(right) = tree.right {
                        return Some(right);
                    } else {
                        return None;
                    }
                }
            }
            Some(tree)
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
        let p = find(i, t.clone());
        if p.is_none() || retrieve(&p) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        let p = find(i, t.clone());
        if p.is_none() || retrieve(&p) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        let p = find(i, t.clone());
        if p.is_some() {
            println!("Error at {}", i);
        }
    }

    let min = find_min(t.clone()).unwrap();
    let max = find_max(t.clone()).unwrap();
    println!("Min is {}, Max is {}", retrieve(&min), retrieve(&max));
}