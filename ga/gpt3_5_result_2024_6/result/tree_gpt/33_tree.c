use std::ptr;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type Position = Option<Box<TreeNode>>;
type SearchTree = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    match t {
        Some(mut tree) => {
            tree.left = make_empty(tree.left.take());
            tree.right = make_empty(tree.right.take());
            None
        }
        None => None,
    }
}

fn find(x: ElementType, t: SearchTree) -> Option<&Box<TreeNode>> {
    match t {
        Some(ref tree) if x < tree.element => find(x, tree.left.as_ref()),
        Some(ref tree) if x > tree.element => find(x, tree.right.as_ref()),
        _ => t.as_ref(),
    }
}

fn find_min(t: SearchTree) -> Option<&Box<TreeNode>> {
    match t.as_ref() {
        Some(tree) => tree.left.as_ref().map_or(t.as_ref(), |left| find_min(Some(left))),
        _ => None,
    }
}

fn find_max(t: SearchTree) -> Option<&Box<TreeNode>> {
    match t.as_ref() {
        Some(tree) => tree.right.as_ref().map_or(t.as_ref(), |right| find_max(Some(right))),
        _ => None,
    }
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        Some(mut tree) => {
            if x < tree.element {
                tree.left = insert(x, tree.left.take());
            } else if x > tree.element {
                tree.right = insert(x, tree.right.take());
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
                tree.left = delete(x, tree.left.take());
            } else if x > tree.element {
                tree.right = delete(x, tree.right.take());
            } else {
                let tmp = match tree.right {
                    Some(ref mut right) => {
                        if right.left.is_none() {
                            let tmp = tree.element;
                            tree.element = right.element;
                            Some(Box::new(TreeNode {
                                element: tmp,
                                left: None,
                                right: delete(tmp, tree.right.take()),
                            }))
                        } else {
                            let min_node = find_min(tree.right.take());
                            if let Some(node) = min_node {
                                tree.element = node.element;
                            }
                            tree.right = delete(tree.element, tree.right.take());
                            None
                        }
                    }
                    None => tree.left.take(),
                };
                return tmp;
            }
            Some(tree)
        }
        None => None,
    }
}

fn retrieve(p: &Box<TreeNode>) -> ElementType {
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

    if let Some(min_node) = find_min(t) {
        if let Some(max_node) = find_max(t) {
            println!("Min is {}, Max is {}", retrieve(min_node), retrieve(max_node));
        }
    }
}