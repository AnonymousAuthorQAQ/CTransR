use std::ptr;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Link,
    right: Link,
}

type Link = Option<Box<TreeNode>>;
type SearchTree = Link;

fn make_empty(tree: SearchTree) -> SearchTree {
    match tree {
        Some(mut node) => {
            node.left = make_empty(node.left);
            node.right = make_empty(node.right);
            None
        }
        None => None,
    }
}

fn find(x: ElementType, tree: SearchTree) -> SearchTree {
    match tree {
        Some(node) if x == node.element => Some(node),
        Some(node) if x < node.element => find(x, node.left),
        Some(node) => find(x, node.right),
        None => None,
    }
}

fn find_min(tree: SearchTree) -> SearchTree {
    match tree {
        Some(node) => {
            if let Some(ref left) = node.left {
                find_min(Some(left.clone()))
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_max(tree: SearchTree) -> SearchTree {
    let mut current = tree;
    while let Some(ref node) = current {
        if let Some(ref right) = node.right {
            current = Some(right.clone());
        } else {
            return Some(node)
        }
    }
    None
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
            let new_node = TreeNode {
                element: x,
                left: None,
                right: None,
            };
            Some(Box::new(new_node))
        }
    }
}

fn delete(x: ElementType, tree: SearchTree) -> SearchTree {
    match tree {
       Some(mut node) if x < node.element => {
            node.left = delete(x, node.left);
            Some(node)
        }
        Some(mut node) if x > node.element => {
            node.right = delete(x, node.right);
            Some(node)
        }
        Some(node) => {
            let temp = find_min(node.right.clone());
            if let Some(tmp_node) = temp {
                node.element = tmp_node.element;
                node.right = delete(tmp_node.element, node.right);
            } else {
                let node = match node {
                    Some(node) => node,
                    None => return None,
                };
                if node.left.is_none() {
                    return node.right;
                } else {
                    return node.left;
                }
            }
            Some(node)
        }
        None => None,
    }
}

fn retrieve(node: &TreeNode) -> ElementType {
    node.element
}

fn main() {
    let mut tree: SearchTree = None;

    for i in (0..50).map(|x| (x * 7) % 50) {
        tree = insert(i as ElementType, tree);
    }

    for i in 0..50 {
        if let Some(node) = find(i, tree.clone()) {
            if retrieve(&*node) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in 0..50 {
        if i % 2 == 0 {
            tree = delete(i, tree.clone());
        }
    }

    for i in 1..50 {
        if let Some(node) = find(i, tree.clone()) {
            if retrieve(&*node) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if find(i, tree.clone()).is_some() {
            println!("Error at {}", i);
        }
    }

    if let Some(min) = find_min(tree.clone()) {
        if let Some(max) = find_max(tree.clone()) {
            println!("Min is {}, Max is {}", retrieve(&*min), retrieve(&*max));
        }
    }
}