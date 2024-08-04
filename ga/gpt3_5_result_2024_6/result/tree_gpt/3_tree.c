use std::ptr;

struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;
type Position = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        node.left = make_empty(node.left);
        node.right = make_empty(node.right);
    }
    t
}

fn find(x: i32, t: SearchTree) -> Position {
    match t {
        None => None,
        Some(node) => {
            if x < node.element {
                find(x, node.left.clone())
            } else if x > node.element {
                find(x, node.right.clone())
            } else {
                Some(node)
            }
        }
    }
}

fn find_min(t: SearchTree) -> Position {
    match t {
        None => None,
        Some(node) => {
            if let Some(mut left) = node.left.clone() {
                find_min(Some(left))
            } else {
                Some(node)
            }
        }
    }
}

fn find_max(t: SearchTree) -> Position {
    match t {
        None => None,
        Some(mut node) => {
            while let Some(right) = node.right.clone() {
                node = right;
            }
            Some(node)
        }
    }
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })),
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
 
fn delete(x: i32, t: SearchTree) -> SearchTree {
    match t {
        None => panic!("Element not found"),
        Some(mut node) => {
            if x < node.element {
                node.left = delete(x, node.left.take());
            } else if x > node.element {
                node.right = delete(x, node.right.take());
            } else {
                if let Some(mut right) = node.right.take() {
                    let min = find_min(Some(right.clone())).unwrap();
                    node.element = min.element;
                    node.right = delete(node.element, node.right.take());
                } else {
                    let tmp_node = node;
                    if node.left.is_none() {
                        node = node.right.take().unwrap();
                    } else if node.right.is_none() {
                        node = node.left.take().unwrap();
                    }
                    drop(tmp_node);
                }
            }
            Some(node)
        }
    }
}

fn retrieve(p: Position) -> i32 {
    p.unwrap().element
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
        let p = find(i, t.clone());
        if p.is_none() || retrieve(p) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        t = delete(i, t);
    }

    for i in (1..50).step_by(2) {
        let p = find(i, t.clone());
        if p.is_none() || retrieve(p) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if find(i, t.clone()).is_some() {
            println!("Error at {}", i);
        }
    }

    println!(
        "Min is {}, Max is {}",
        retrieve(find_min(t.clone()).unwrap()),
        retrieve(find_max(t.clone()).unwrap())
    );
}