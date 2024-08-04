use std::ptr;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;
type Position = Option<Box<TreeNode>>;

fn make_empty(mut t: SearchTree) -> SearchTree {
    t = None;
    t
}

fn find(x: ElementType, t: &SearchTree) -> Position {
    match t {
        None => None,
        Some(ref node) => {
            if x < node.element {
                find(x, &node.left)
            } else if x > node.element {
                find(x, &node.right)
            } else {
                Some(Box::new(TreeNode {
                    element: node.element,
                    left: node.left.clone(),
                    right: node.right.clone(),
                }))
            }
        }
    }
}

fn find_min(t: &SearchTree) -> Position {
    match t {
        None => None,
        Some(ref node) => {
            if node.left.is_none() {
                Some(Box::new(TreeNode {
                    element: node.element,
                    left: node.left.clone(),
                    right: node.right.clone(),
                }))
            } else {
                find_min(&node.left)
            }
        }
    }
}

fn find_max(t: &SearchTree) -> Position {
    match t {
        None => None,
        Some(ref node) => {
            if node.right.is_none() {
                Some(Box::new(TreeNode {
                    element: node.element,
                    left: node.left.clone(),
                    right: node.right.clone(),
                }))
            } else {
                find_max(&node.right)
            }
        }
    }
}

fn insert(x: ElementType, mut t: SearchTree) -> SearchTree {
    match t {
        None => {
            t = Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            }));
        },
        Some(ref mut node) => {
            if x < node.element {
                node.left = insert(x, node.left.take());
            } else if x > node.element {
                node.right = insert(x, node.right.take());
            }
        }
    }
    t
}

fn delete(x: ElementType, mut t: SearchTree) -> SearchTree {
    if let Some(ref mut node) = t {
        if x < node.element {
            node.left = delete(x, node.left.take());
        } else if x > node.element {
            node.right = delete(x, node.right.take());
        } else {
            if node.left.is_none() {
                return node.right.take();
            } else if node.right.is_none() {
                return node.left.take();
            }
            let mut tmp_cell = find_min(&node.right).unwrap();
            node.element = tmp_cell.element;
            node.right = delete(node.element, node.right.take());
        }
    }
    t
}

fn retrieve(p: &Position) -> ElementType {
    p.as_ref().unwrap().element
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
        if let Some(p) = find(i, &t) {
            if retrieve(&Some(p)) != i {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).filter(|x| x % 2 == 0) {
        t = delete(i, t);
    }

    for i in (1..50).filter(|x| x % 2 != 0) {
        if let Some(p) = find(i, &t) {
            if retrieve(&Some(p)) != i {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).filter(|x| x % 2 == 0) {
        if find(i, &t).is_some() {
            println!("Error at {}", i);
        }
    }

    if let Some(min) = find_min(&t) {
        if let Some(max) = find_max(&t) {
            println!("Min is {}, Max is {}", retrieve(&Some(min)), retrieve(&Some(max)));
        }
    }
}