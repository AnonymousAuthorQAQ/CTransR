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
        make_empty(tree.left.take());
        make_empty(tree.right.take());
    }
    None
}

fn find(x: ElementType, t: SearchTree) -> Position {
    match t {
        None => None,
        Some(tree) => {
            if x < tree.element {
                find(x, tree.left.as_ref().map(|node| &**node))
            } else if x > tree.element {
                find(x, tree.right.as_ref().map(|node| &**node))
            } else {
                Some(tree.as_mut())
            }
        }
    }
}

fn find_min(t: SearchTree) -> Position {
    match t {
        Some(mut tree) => {
            if let Some(mut left_node) = tree.left.take() {
                let result = find_min(Some(left_node.as_mut()));
                tree.left = Some(left_node);
                result
            } else {
                Some(tree.as_mut())
            }
        }
        None => None,
    }
}

fn find_max(t: SearchTree) -> Position {
    let mut current = t;
    loop {
        match current {
            Some(mut tree) => {
                if let Some(right_node) = &tree.right {
                    current = Some(right_node.as_ref());
                } else {
                    return Some(tree.as_mut());
                }
            }
            None => return None,
        }
    }
}

fn insert(x: ElementType, mut t: SearchTree) -> SearchTree {
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
                tree.left = insert(x, tree.left.take());
            } else if x > tree.element {
                tree.right = insert(x, tree.right.take());
            }
            Some(tree)
        }
    }
}

fn delete(x: ElementType, mut t: SearchTree) -> SearchTree {
    if let Some(mut tree) = t.take() {
        if x < tree.element {
            tree.left = delete(x, tree.left.take());
            Some(tree)
        } else if x > tree.element {
            tree.right = delete(x, tree.right.take());
            Some(tree)
        } else {
            if let Some(right_node) = tree.right.take() {
                let min = find_min(Some(right_node.as_mut()));
                if let Some(next) = min {
                    tree.element = next.element;
                    tree.right = delete(next.element, tree.right.take());
                }
            } else {
                t = tree.left.take();
            }
            Some(tree)
        }
    } else {
        None
    }
}

fn retrieve(p: Position) -> ElementType {
    unsafe {
        assert!(!p.is_none());
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
        let p = find(i, t);
        if p.is_none() || retrieve(p) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        let p = find(i, t);
        if p.is_none() || retrieve(p) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if find(i, t).is_some() {
            println!("Error at {}", i);
        }
    }

    let min = find_min(t);
    let max = find_max(t);

    if let Some(p) = min {
        println!("Min is {}, Max is {}", retrieve(p), retrieve(find_max(t).unwrap()));
    } else {
        println!("Invalid tree");
    }
}