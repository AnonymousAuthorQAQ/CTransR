use std::ptr;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type Position = Option<Box<TreeNode>>;
type SearchTree = Option<Box<TreeNode>>;

fn fatal_error(msg: &str) {
    eprintln!("{}", msg);
    std::process::exit(1);
}

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
        Some(ref tree) => {
            if x < tree.element {
                find(x, tree.left.clone())
            } else if x > tree.element {
                find(x, tree.right.clone())
            } else {
                Some(Box::new(*tree))
            }
        }
    }
}

fn find_min(t: SearchTree) -> Position {
    if let Some(ref mut tree) = t {
        if let Some(ref mut left_tree) = tree.left {
            find_min(Some(left_tree.clone()))
        } else {
            Some(Box::new(*tree))
        }
    } else {
        None
    }
}

fn find_max(t: SearchTree) -> Position {
    if let Some(mut tree) = t {
        while let Some(next_tree) = tree.right.clone() {
            tree = next_tree;
        }
        Some(Box::new(*tree))
    } else {
        None
    }
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => {
            Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            }))
        }
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
        None => fatal_error("Element not found"),
        Some(mut tree) => {
            if x < tree.element {
                tree.left = delete(x, tree.left);
            } else if x > tree.element {
                tree.right = delete(x, tree.right);
            } else if tree.left.is_some() && tree.right.is_some() {
                let tmp_cell = find_min(tree.right.clone());
                tree.element = tmp_cell.unwrap().element;
                tree.right = delete(tree.element, tree.right);
            } else {
                let tmp_cell = tree;
                if tree.left.is_none() {
                    return tree.right;
                } else if tree.right.is_none() {
                    return tree.left;
                }
                drop(tmp_cell);
            }
            Some(tree)
        }
    }
}

fn retrieve(p: &Position) -> ElementType {
    p.as_ref().unwrap().element
}

fn main() {
    let mut t = make_empty(None);
    let mut j = 0;
    
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
        t = delete(i, t.clone());
    }
    
    for i in (1..50).step_by(2) {
        let p = find(i, t.clone());
        if p.is_none() || retrieve(&p) != i {
            println!("Error at {}", i);
        }
    }
    
    for i in (0..50).step_by(2) {
        if find(i, t.clone()).is_some() {
            println!("Error at {}", i);
        }
    }

    let min_val = retrieve(&find_min(t.clone()).unwrap());
    let max_val = retrieve(&find_max(t.clone()).unwrap());

    println!("Min is {}, Max is {}", min_val, max_val);
}