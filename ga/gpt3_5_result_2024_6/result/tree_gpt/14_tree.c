use std::ptr;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;
type Position = SearchTree;

fn make_empty(t: SearchTree) -> SearchTree {
    t
}

fn find(x: ElementType, t: SearchTree) -> Option<&Position> {
    match t {
        None => None,
        Some(ref node) => {
            if x < node.element {
                find(x, node.left.as_ref())
            } else if x > node.element {
                find(x, node.right.as_ref())
            } else {
                Some(node)
            }
        }
    }
}

fn find_min(t: SearchTree) -> Option<&Position> {
    t.as_ref().and_then(|node| {
        if let Some(ref left) = node.left {
            find_min(Some(left))
        } else {
            Some(node)
        }
    })
}

fn find_max(t: SearchTree) -> Option<&Position> {
    t.as_ref().map(|node| {
        let mut current = node;
        while let Some(ref right) = current.right {
            current = right;
        }
        current
    })
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
        Some(mut node) => {
            if x < node.element {
                node.left = insert(x, node.left.take());
            } else if x > node.element {
                node.right = insert(x, node.right.take());
            }
            Some(node)
        }
    }
}

fn delete(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => panic!("Element not found"),
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left.take());
            } else if x > node.element {
                node.right = delete(x, node.right.take());
            } else if node.left.is_some() && node.right.is_some() {
                let min_right = find_min(node.right.take()).unwrap().as_ref().unwrap().element;
                node.element = min_right;
                node.right = delete(min_right, node.right.take());
            } else {
                let tmp_cell = node;
                *node = if node.left.is_none() {
                    node.right.take().unwrap()
                } else {
                    node.left.take().unwrap()
                };
                ptr::drop_in_place(tmp_cell);
            }
            Some(node)
        }
    }
}

fn retrieve(p: &Position) -> ElementType {
    p.as_ref().unwrap().element
}

fn main() {
    let mut t: SearchTree = None;
    let mut j = 0;

    for i in 0..50 {
        j = (j + 7) % 50;
        t = insert(j, t);
    }

    for i in 0..50 {
        let p = find(i, t);
        if p.is_none() || retrieve(&p.unwrap()) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        let p = find(i, t);
        if p.is_none() || retrieve(&p.unwrap()) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if find(i, t).is_some() {
            println!("Error at {}", i);
        }
    }

    match (find_min(t), find_max(t)) {
        (Some(min), Some(max)) => {
            println!("Min is {}, Max is {}", retrieve(min), retrieve(max));
        }
        _ => {}
    }
}