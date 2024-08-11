use std::ptr;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(element: i32) -> Box<TreeNode> {
        Box::new(TreeNode {
            element,
            left: None,
            right: None,
        })
    }
}

fn make_empty(tree: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    if let Some(mut tree) = tree {
        tree.left = make_empty(tree.left);
        tree.right = make_empty(tree.right);
    }
    None
}

fn find(x: i32, tree: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    if let Some(ref tree) = tree {
        if x < tree.element {
            find(x, &tree.left)
        } else if x > tree.element {
            find(x, &tree.right)
        } else {
            Some(tree)
        }
    } else {
        None
    }
}

fn find_min(tree: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    if let Some(ref tree) = tree {
        if let None = tree.left {
            Some(tree)
        } else {
            find_min(&tree.left)
        }
    } else {
        None
    }
}

fn find_max(tree: &Option<Box<TreeNode>>) -> Option<&Box<TreeNode>> {
    if let Some(ref tree) = tree {
        if let None = tree.right {
            Some(tree)
        } else {
            find_max(&tree.right)
        }
    } else {
        None
    }
}

fn insert(x: i32, tree: &mut Option<Box<TreeNode>>) {
    if let None = tree {
        *tree = Some(TreeNode::new(x));
    } else {
        let mut tree = tree.as_mut().unwrap();
        if x < tree.element {
            insert(x, &mut tree.left);
        } else if x > tree.element {
            insert(x, &mut tree.right);
        }
    }
}

fn delete(x: i32, tree: &mut Option<Box<TreeNode>>) {
    if let Some(mut tree) = tree.take() {
        if x < tree.element {
            delete(x, &mut tree.left);
            *tree.left = None;
        } else if x > tree.element {
            delete(x, &mut tree.right);
            *tree.right = None;
        } else {
            if let Some(mut right) = tree.right.take() {
                *tree = *right;
            } else if let Some(mut left) = tree.left.take() {
                *tree = *left;
            } else {
                return;
            }
        }
        *tree.left = make_empty(tree.left);
        *tree.right = make_empty(tree.right);
        *tree = Some(tree);
    }
}

fn retrieve(tree: &Option<Box<TreeNode>>) -> i32 {
    tree.as_ref().unwrap().element
}

fn main() {
    let mut t: Option<Box<TreeNode>> = None;
    for i in 0..50 {
        insert(i, &mut t);
    }
    for i in 0..50 {
        if let None = find(i, &t) || retrieve(find(i, &t).unwrap()) != i {
            println!("Error at {}", i);
        }
    }
    for i in 0..50 {
        if i % 2 == 0 {
            delete(i, &mut t);
        }
    }
    for i in 1..50 {
        if i % 2 == 1 {
            if let None = find(i, &t) || retrieve(find(i, &t).unwrap()) != i {
                println!("Error at {}", i);
            }
        } else {
            if let Some(_) = find(i, &t) {
                println!("Error at {}", i);
            }
        }
    }
    println!("Min is {}, Max is {}", retrieve(find_min(&t).unwrap()), retrieve(find_max(&t).unwrap()));
}