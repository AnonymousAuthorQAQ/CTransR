use std::ptr;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;
type Position = Option<&Box<TreeNode>>;

fn make_empty(tree: SearchTree) -> SearchTree {
    if let Some(mut node) = tree {
        node.left = make_empty(node.left);
        node.right = make_empty(node.right);
    }
    None
}

fn find(x: ElementType, tree: SearchTree) -> Position {
    match tree {
        None => None,
        Some(node) => {
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

fn find_min(tree: Position) -> Position {
    let mut current = tree;
    while let Some(ref node) = current.unwrap().left.as_ref() {
        current = Some(node);
    }
    current
}

fn find_max(tree: Position) -> Position {
    let mut current = tree;
    while let Some(ref node) = current.unwrap().right.as_ref() {
        current = Some(node);
    }
    current
}

fn insert(x: ElementType, mut tree: SearchTree) -> SearchTree {
    if let Some(ref mut node) = tree {
        if x < node.element {
            node.left = insert(x, node.left.take());
        } else if x > node.element {
            node.right = insert(x, node.right.take());
        }
        tree
    } else {
        Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        }))
    }
}

fn delete(x: ElementType, mut tree: SearchTree) -> SearchTree {
    if let Some(mut node) = tree.take() {
        if x < node.element {
            node.left = delete(x, node.left.take());
        } else if x > node.element {
            node.right = delete(x, node.right.take());
        } else {
            if node.left.is_some() && node.right.is_some() {
                let min_node = find_min(node.right.as_ref());
                node.element = min_node.unwrap().element;
                node.right = delete(node.element, node.right.take());
            } else {
                let temp = node;
                if node.left.is_none() {
                    tree = temp.right.take();
                } else if node.right.is_none() {
                    tree = temp.left.take();
                }
            }
        }
        Some(node)
    } else {
        None
    }
}

fn retrieve(position: Position) -> ElementType {
    position.unwrap().element
}

fn main() {
    let mut tree: SearchTree = None;
    let mut j = 0;

    tree = make_empty(tree);
    for i in 0..50 {
        j = (j + 7) % 50;
        tree = insert(j, tree);
    }

    for i in 0..50 {
        let p = find(i, tree.as_ref());
        if p.is_none() || retrieve(p) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        tree = delete(i, tree);
    }

    for i in (1..50).step_by(2) {
        let p = find(i, tree.as_ref());
        if p.is_none() || retrieve(p) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if find(i, tree.as_ref()).is_some() {
            println!("Error at {}", i);
        }
    }

    println!("Min is {}, Max is {}", retrieve(find_min(tree.as_ref())), retrieve(find_max(tree.as_ref())));
}