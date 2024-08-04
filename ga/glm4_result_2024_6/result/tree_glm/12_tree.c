use std::cmp::Ordering;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(x: i32) -> Self {
        TreeNode {
            element: x,
            left: None,
            right: None,
        }
    }
}

fn make_empty(tree: &mut Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        make_empty(&mut node.left);
        make_empty(&mut node.right);
        tree.take();
    }
}

fn find(x: i32, tree: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
    if let Some(node) = tree {
        match x.cmp(&node.element) {
            Ordering::Less => find(x, &node.left),
            Ordering::Greater => find(x, &node.right),
            Ordering::Equal => Some(node),
        }
    } else {
        None
    }
}

fn find_min(tree: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
    if let Some(node) = tree {
        if let None = node.left {
            return Some(node);
        }
        find_min(&node.left)
    } else {
        None
    }
}

fn find_max(tree: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
    if let Some(node) = tree {
        if let None = node.right {
            return Some(node);
        }
        find_max(&node.right)
    } else {
        None
    }
}

fn insert(x: i32, tree: &mut Option<Box<TreeNode>>) {
    if let None = tree {
        *tree = Some(Box::new(TreeNode::new(x)));
    } else {
        match x.cmp(&tree.as_ref().unwrap().element) {
            Ordering::Less => insert(x, &mut tree.as_mut().unwrap().left),
            Ordering::Greater => insert(x, &mut tree.as_mut().unwrap().right),
            Ordering::Equal => (),
        }
    }
}

fn delete(x: i32, tree: &mut Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        match x.cmp(&node.element) {
            Ordering::Less => delete(x, &mut node.left),
            Ordering::Greater => delete(x, &mut node.right),
            Ordering::Equal => {
                if let (Some(left), Some(right)) = (&node.left, &node.right) {
                    let min_node = find_min(&Some(right)).unwrap();
                    node.element = min_node.element;
                    delete(min_node.element, &mut node.right);
                } else if let Some(child) = node.left.as_ref() {
                    *tree = node.left.take();
                } else if let Some(child) = node.right.as_ref() {
                    *tree = node.right.take();
                } else {
                    *tree = None;
                }
            }
        }
    } else {
        println!("Element not found");
    }
}

fn retrieve(p: &TreeNode) -> i32 {
    p.element
}

fn main() {
    let mut tree: Option<Box<TreeNode>> = None;
    for i in 0..50 {
        insert(i, &mut tree);
    }

    for i in 0..50 {
        if let None = find(i, &tree) {
            println!("Error at {}", i);
        }
    }

    for i in 0..50 {
        if i % 2 == 0 {
            delete(i, &mut tree);
        }
    }

    for i in 1..50 {
        if i % 2 == 1 {
            if let None = find(i, &tree) {
                println!("Error at {}", i);
            }
        } else {
            if let Some(_) = find(i, &tree) {
                println!("Error at {}", i);
            }
        }
    }

    if let Some(min) = find_min(&tree) {
        println!("Min is {}", retrieve(min));
    }
    if let Some(max) = find_max(&tree) {
        println!("Max is {}", retrieve(max));
    }
}