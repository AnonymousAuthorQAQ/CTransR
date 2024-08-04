use std::ptr;

type ElementType = i32;

#[derive(Debug)]
struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        node.left = make_empty(node.left);
        node.right = make_empty(node.right);
    }
    None
}

fn find(x: ElementType, t: &SearchTree) -> Option<&TreeNode> {
    match t {
        Some(node) => {
            if x < node.element {
                find(x, &node.left)
            } else if x > node.element {
                find(x, &node.right)
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    match t {
        Some(node) => {
            if let Some(left_node) = &node.left {
                find_min(&node.left)
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut node = t.as_ref();
    while let Some(curr) = node {
        if let Some(right_node) = &curr.right {
            node = Some(right_node);
        } else {
            break;
        }
    }
    node.map(|x| x.as_ref())
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => {
            let new_node = TreeNode {
                element: x,
                left: None,
                right: None,
            };
            Some(Box::new(new_node))
        }
        Some(mut node) => {
            if x < node.element {
                node.left = insert(x, node.left);
            } else if x > node.element {
                node.right = insert(x, node.right);
            }
            Some(node)
        }
    }
}

fn delete(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => None,
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left);
            } else if x > node.element {
                node.right = delete(x, node.right);
            } else if node.left.is_some() && node.right.is_some() {
                let tmp_cell = find_min(&node.right).unwrap();
                node.element = tmp_cell.element;
                node.right = delete(node.element, node.right);
            } else {
                let tmp_cell = node;
                if node.left.is_none() {
                    node = node.right.unwrap();
                } else {
                    node = node.left.unwrap();
                }
                drop(tmp_cell);
            }
            Some(node)
        }
    }
}

fn retrieve(p: &TreeNode) -> ElementType {
    p.element
}

fn main() {
    let mut t = make_empty(None);
    let mut j = 0;

    for i in 0..50 {
        j = (j + 7) % 50;
        t = insert(j, t);
    }

    for i in 0..50 {
        if let Some(p) = find(i, &t) {
            if retrieve(p) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        if let Some(p) = find(i, &t) {
            if retrieve(p) != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if find(i, &t).is_some() {
            println!("Error at {}", i);
        }
    }

    if let Some(min_node) = find_min(&t) {
        if let Some(max_node) = find_max(&t) {
            println!("Min is {}, Max is {}", retrieve(min_node), retrieve(max_node));
        }
    }
}