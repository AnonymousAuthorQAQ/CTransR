use std::ptr;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(x: i32) -> Box<TreeNode> {
        Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        })
    }
}

fn make_empty(tree: &mut Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        make_empty(&mut node.left);
        make_empty(&mut node.right);
        tree.take();
    }
}

fn find(x: i32, tree: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    if let Some(node) = tree {
        if x < node.element {
            find(x, &node.left)
        } else if x > node.element {
            find(x, &node.right)
        } else {
            Some(node)
        }
    } else {
        None
    }
}

fn find_min(tree: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    if let Some(node) = tree {
        if node.left.is_none() {
            Some(node)
        } else {
            find_min(&node.left)
        }
    } else {
        None
    }
}

fn find_max(tree: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    if let Some(node) = tree {
        let mut current = node;
        while current.right.is_some() {
            current = current.right.as_ref().unwrap();
        }
        Some(current)
    } else {
        None
    }
}

fn insert(x: i32, tree: &mut Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        if x < node.element {
            insert(x, &mut node.left);
        } else if x > node.element {
            insert(x, &mut node.right);
        }
    } else {
        *tree = Some(TreeNode::new(x));
    }
}

fn delete(x: i32, tree: &mut Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        if x < node.element {
            delete(x, &mut node.left);
        } else if x > node.element {
            delete(x, &mut node.right);
        } else {
            if node.left.is_some() && node.right.is_some() {
                let min_right = find_min(&node.right).unwrap();
                node.element = min_right.element;
                delete(min_right.element, &mut node.right);
            } else if node.left.is_some() {
                *tree = node.left.take();
            } else if node.right.is_some() {
                *tree = node.right.take();
            } else {
                *tree = None;
            }
        }
    }
}

fn retrieve(node: &Box<TreeNode>) -> i32 {
    node.element
}

fn main() {
    let mut tree: Option<Box<TreeNode>> = None;

    for i in 0..50 {
        insert(i, &mut tree);
    }

    for i in 0..50 {
        if find(i, &tree).is_none() || retrieve(find(i, &tree).unwrap()) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        delete(i, &mut tree);
    }

    for i in (1..50).step_by(2) {
        if find(i, &tree).is_none() || retrieve(find(i, &tree).unwrap()) != i {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if find(i, &tree).is_some() {
            println!("Error at {}", i);
        }
    }

    if let Some(min) = find_min(&tree) {
        println!("Min is {}, Max is {}", retrieve(min), retrieve(find_max(&tree).unwrap()));
    } else {
        println!("Tree is empty");
    }
}