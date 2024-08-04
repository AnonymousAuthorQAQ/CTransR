use std::ptr::NonNull;

type SearchTree = Option<Box<TreeNode>>;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: SearchTree,
    right: SearchTree,
}

impl TreeNode {
    fn new(element: i32) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
}

fn make_empty(tree: &mut SearchTree) {
    if let Some(node) = tree {
        make_empty(&mut node.left);
        make_empty(&mut node.right);
        *tree = None;
    }
}

fn find(x: i32, tree: &SearchTree) -> Option<NonNull<TreeNode>> {
    let mut current = tree.as_ref()?;
    loop {
        if x < current.element {
            current = current.left.as_ref()?;
        } else if x > current.element {
            current = current.right.as_ref()?;
        } else {
            return Some(NonNull::from(current));
        }
    }
}

fn find_min(tree: &SearchTree) -> Option<NonNull<TreeNode>> {
    let mut current = tree.as_ref()?;
    while let Some(left) = current.left.as_ref() {
        current = left;
    }
    Some(NonNull::from(current))
}

fn find_max(tree: &SearchTree) -> Option<NonNull<TreeNode>> {
    let mut current = tree.as_ref()?;
    while let Some(right) = current.right.as_ref() {
        current = right;
    }
    Some(NonNull::from(current))
}

fn insert(x: i32, tree: &mut SearchTree) {
    if let None = tree {
        *tree = Some(Box::new(TreeNode::new(x)));
    } else {
        let mut current = tree.as_mut().unwrap();
        while x != current.element {
            if x < current.element {
                if let None = current.left {
                    current.left = Some(Box::new(TreeNode::new(x)));
                    return;
                }
                current = current.left.as_mut().unwrap();
            } else {
                if let None = current.right {
                    current.right = Some(Box::new(TreeNode::new(x)));
                    return;
                }
                current = current.right.as_mut().unwrap();
            }
        }
    }
}

fn delete(x: i32, tree: &mut SearchTree) {
    if let Some(mut node) = tree.take() {
        if x < node.element {
            node.left = delete(x, &mut node.left);
        } else if x > node.element {
            node.right = delete(x, &mut node.right);
        } else {
            if let Some(mut right) = node.right.take() {
                if let Some(mut left) = node.left.take() {
                    node.element = find_min(&Some(right)).unwrap().as_ref().element;
                    node.right = Some(right);
                    node.left = left;
                } else {
                    node = right;
                }
            } else {
                node = node.left.unwrap_or(node);
            }
        }
        *tree = Some(node);
    }
}

fn retrieve(p: NonNull<TreeNode>) -> i32 {
    p.as_ref().element
}

fn main() {
    let mut tree: SearchTree = None;
    for i in 0..50 {
        insert(i, &mut tree);
    }

    for i in 0..50 {
        if let None = find(i, &tree) {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).filter(|&x| x % 2 == 0) {
        delete(i, &mut tree);
    }

    for i in (1..50).filter(|&x| x % 2 != 0) {
        if let None = find(i, &tree) {
            println!("Error at {}", i);
        }
    }

    for i in 0..50 {
        if i % 2 == 0 && find(i, &tree).is_some() {
            println!("Error at {}", i);
        }
    }

    println!(
        "Min is {}, Max is {}",
        retrieve(find_min(&tree).unwrap()),
        retrieve(find_max(&tree).unwrap())
    );
}