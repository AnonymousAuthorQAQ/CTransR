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
    Some(Box::new(TreeNode {
        element: 0,
        left: None,
        right: None,
    }))
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
                t
            }
        }
    }
}

fn find_min(t: SearchTree) -> Position {
    match t.clone() {
        None => None,
        Some(ref tree) => {
            if tree.left.is_none() {
                t
            } else {
                find_min(tree.left.clone())
            }
        }
    }
}

fn find_max(t: SearchTree) -> Position {
    let mut curr_t = t.clone();
    while let Some(ref mut tree) = curr_t {
        if tree.right.is_none() {
            return curr_t;
        }
        curr_t = tree.right.clone();
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
        None => panic!("Element not found"),
        Some(mut tree) => {
            if x < tree.element {
                tree.left = delete(x, tree.left);
            } else if x > tree.element {
                tree.right = delete(x, tree.right);
            } else {
                if let (Some(left), Some(right)) = (tree.left.clone(), tree.right.clone()) {
                    let min = find_min(tree.right.clone());
                    tree.element = min.unwrap().element;
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
            }
            Some(tree)
        }
    }
}

fn retrieve(p: Position) -> ElementType {
    p.unwrap().element
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
        if p.is_none() || retrieve(p) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t.clone());
    }

    for i in (1..50).step_by(2) {
        let p = find(i, t.clone());
        if p.is_none() || retrieve(p) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        let p = find(i, t.clone());
        if p.is_some() {
            println!("Error at {}", i);
        }
    }

    println!("Min is {}, Max is {}", retrieve(find_min(t.clone()).unwrap()), retrieve(find_max(t.clone()).unwrap()));
}