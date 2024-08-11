use std::ptr;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;
type Position = Option<*mut TreeNode>;

fn make_empty(t: SearchTree) -> SearchTree {
    if let Some(mut tree) = t {
        tree.left = make_empty(tree.left);
        tree.right = make_empty(tree.right);
    }
    None
}

fn find(x: ElementType, t: SearchTree) -> Position {
    match t {
        None => None,
        Some(tree) => {
            if x < tree.element {
                find(x, tree.left.clone())
            } else if x > tree.element {
                find(x, tree.right.clone())
            } else {
                Some(tree.as_mut())
            }
        }
    }
}

fn find_min(t: SearchTree) -> Position {
    match t {
        None => None,
        Some(mut tree) => {
            if let Some(mut left) = tree.left.take() {
                find_min(Some(left))
            } else {
                Some(tree.as_mut())
            }
        }
    }
}

fn find_max(t: SearchTree) -> Position {
    match t {
        None => None,
        Some(mut tree) => {
            let mut current_tree = tree.as_mut();
            while let Some(mut right) = current_tree.right.take() {
                current_tree = right.as_mut();
            }
            Some(current_tree)
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
        None => None,
        Some(mut tree) => {
            if x < tree.element {
                tree.left = delete(x, tree.left.take());
            } else if x > tree.element {
                tree.right = delete(x, tree.right.take());
            } else {
                if let (left, right) = (tree.left.take(), tree.right.take()) {
                    let tmp_cell = find_min(Some(right.clone())).unwrap().clone();
                    tree.element = tmp_cell.element;
                    tree.right = delete(tmp_cell.element, Some(right));
                } else {
                    let tmp_cell = tree;
                    let left = tmp_cell.left;
                    let right = tmp_cell.right;
                    ptr::drop_in_place(tmp_cell);
                    return if let Some(left) = left {
                        Some(left)
                    } else {
                        right
                    };
                }
            }
            Some(tree)
        }
    }
}

fn retrieve(p: Position) -> ElementType {
    unsafe {
        (*p.unwrap()).element
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
        if let Some(p) = find(i, t.clone()) {
            if retrieve(Some(p)) != i {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        if let Some(p) = find(i, t.clone()) {
            if retrieve(Some(p)) != i {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        if let Some(p) = find(i, t.clone()) {
            println!("Error at {}", i);
        }
    }

    if let (Some(min_node), Some(max_node)) = (find_min(t.clone()), find_max(t.clone())) {
        println!("Min is {}, Max is {}", retrieve(min_node), retrieve(max_node));
    }
}