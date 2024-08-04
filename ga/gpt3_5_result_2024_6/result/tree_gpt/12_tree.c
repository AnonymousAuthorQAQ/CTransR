use std::ptr;
use std::mem;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>
}

type SearchTree = Option<Box<TreeNode>>;

impl TreeNode {
    fn new(element: ElementType) -> Self {
        TreeNode {
            element,
            left: None,
            right: None
        }
    }
}

fn make_empty(tree: SearchTree) -> SearchTree {
    if let Some(mut t) = tree {
        t.left = make_empty(t.left);
        t.right = make_empty(t.right);
        // Drop the current tree and set it to None
        drop(t);
        None
    } else {
        None
    }
}

fn find(x: ElementType, tree: &SearchTree) -> Option<&TreeNode> {
    match tree {
        Some(node) => {
            if x < node.element {
                find(x, &node.left)
            } else if x > node.element {
                find(x, &node.right)
            } else {
                Some(node)
            }
        }
        None => None
    }
}

fn find_min(tree: &SearchTree) -> Option<&TreeNode> {
    match tree {
        Some(node) => {
            if let Some(mut current) = tree {
                while let Some(next) = &current.left {
                    current = next;
                }
                Some(current)
            } else {
                Some(node)
            }
        }
        None => None
    }
}

fn find_max(tree: &SearchTree) -> Option<&TreeNode> {
    if let Some(curr) = tree {
        let mut current = curr;
        while let Some(next) = &current.right {
            current = next;
        }
        Some(current)
    } else {
        None
    }
}

fn insert(x: ElementType, tree: SearchTree) -> SearchTree {
    match tree {
        Some(mut node) => {
            if x < node.element {
                node.left = insert(x, node.left);
                
            } else if x > node.element {
                node.right = insert(x, node.right);
            }
            Some(node)
        }
        None => {
            Some(Box::new(TreeNode::new(x)))
        }
    }
}

fn delete(x: ElementType, tree: &mut SearchTree) {
    if let Some(node) = tree.as_mut() {
        if x < node.element {
            delete(x, &mut node.left);
        } else if x > node.element {
            delete(x, &mut node.right);
        } else {
            if node.left.is_none() {
                let right = node.right.take();
                let _ = mem::replace(tree, right);
            } else if node.right.is_none() {
                let left = node.left.take();
                let _ = mem::replace(tree, left);
            } else {
                let mut tmp_cell = find_min(&mut node.right).unwrap().element;
                node.element = tmp_cell;

                delete(tmp_cell, &mut node.right);
            }
        }
    }
}

fn retrieve(p: &TreeNode) -> ElementType {
    p.element
}

fn main() {
    let mut t: SearchTree = None;
    
    for i in 0..50 {
        let j = (i + 7) % 50;
        t = insert(j, t);
    }

    for i in 0..50 {
        let p = find(i, &t);
        if p.is_none() || retrieve(p.unwrap()) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        delete(i, &mut t);
    }

    for i in (1..50).step_by(2) {
        let p = find(i, &t);
        if p.is_none() || retrieve(p.unwrap()) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        let p = find(i, &t);
        if p.is_some() {
            println!("Error at {}", i);
        }
    }

    println!("Min is {}, Max is {}", retrieve(find_min(&t).unwrap()), 
                                  retrieve(find_max(&t).unwrap()));
}