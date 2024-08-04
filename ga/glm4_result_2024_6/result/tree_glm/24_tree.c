use std::ptr::null_mut;
use std::cmp::Ordering;

type ElementType = i32;
type SearchTree = Option<Box<TreeNode>>;
type Position = *mut TreeNode;

struct TreeNode {
    element: ElementType,
    left: SearchTree,
    right: SearchTree,
}

impl TreeNode {
    fn new(x: ElementType) -> Box<TreeNode> {
        Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })
    }
}

fn make_empty(tree: &mut SearchTree) {
    if let Some(node) = tree {
        make_empty(&mut node.left);
        make_empty(&mut node.right);
        *tree = None;
    }
}

fn find(x: ElementType, tree: &SearchTree) -> Position {
    match tree {
        Some(node) => {
            match x.cmp(&node.element) {
                Ordering::Less => find(x, &node.left),
                Ordering::Greater => find(x, &node.right),
                Ordering::Equal => node as Position,
            }
        },
        None => null_mut(),
    }
}

fn find_min(tree: &SearchTree) -> Position {
    match tree {
        Some(node) => {
            if node.left.is_none() {
                node as Position
            } else {
                find_min(&node.left)
            }
        },
        None => null_mut(),
    }
}

fn find_max(tree: &SearchTree) -> Position {
    let mut node = tree;
    while let Some(current) = node {
        if current.right.is_none() {
            return current as Position;
        }
        node = &current.right;
    }
    null_mut()
}

fn insert(x: ElementType, tree: &mut SearchTree) {
    if tree.is_none() {
        *tree = Some(TreeNode::new(x));
    } else {
        match x.cmp(&tree.as_ref().unwrap().element) {
            Ordering::Less => insert(x, &mut tree.as_mut().unwrap().left),
            Ordering::Greater => insert(x, &mut tree.as_mut().unwrap().right),
            Ordering::Equal => return,
        }
    }
}

fn delete(x: ElementType, tree: &mut SearchTree) {
    if let Some(mut node) = tree.take() {
        match x.cmp(&node.element) {
            Ordering::Less => {
                let left = node.left.take();
                *tree = Some(node);
                delete(x, &mut left);
                tree.as_mut().unwrap().left = left;
            },
            Ordering::Greater => {
                let right = node.right.take();
                *tree = Some(node);
                delete(x, &mut right);
                tree.as_mut().unwrap().right = right;
            },
            Ordering::Equal => {
                if node.left.is_none() {
                    *tree = node.right;
                } else if node.right.is_none() {
                    *tree = node.left;
                } else {
                    let mut right = node.right.take();
                    *tree = Some(right.as_mut().unwrap());
                    let min = find_min(&right);
                    node.element = unsafe { (*min).element };
                    delete(node.element, &mut right);
                    tree.as_mut().unwrap().right = right;
                }
            },
        }
    }
}

fn retrieve(p: Position) -> ElementType {
    unsafe { (*p).element }
}

fn main() {
    let mut t: SearchTree = None;
    let mut i: i32 = 0;
    let mut j: i32 = 0;

    for i in 0..50 {
        j = (j + 7) % 50;
        insert(j, &mut t);
    }

    for i in 0..50 {
        let p = find(i, &t);
        if p.is_null() || retrieve(p) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        delete(i, &mut t);
    }

    for i in (1..50).step_by(2) {
        let p = find(i, &t);
        if p.is_null() || retrieve(p) != i {
            println!("Error at {}", i);
        }
    }

    for i in 0..50 {
        let p = find(i, &t);
        if !p.is_null() {
            println!("Error at {}", i);
        }
    }

    println!("Min is {}, Max is {}", retrieve(find_min(&t)), retrieve(find_max(&t)));
}