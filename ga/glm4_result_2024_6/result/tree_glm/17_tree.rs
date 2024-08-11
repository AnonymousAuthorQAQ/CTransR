use std::ptr;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
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

fn make_empty<T>(tree: &mut Option<T>) where T: TreeNode {
    if let Some(ref mut node) = tree {
        make_empty(&mut node.left);
        make_empty(&mut node.right);
        *tree = None;
    }
}

fn find<T>(x: i32, tree: &Option<T>) -> Option<&T> where T: TreeNode {
    if let Some(ref node) = tree {
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

fn find_min<T>(tree: &Option<T>) -> Option<&T> where T: TreeNode {
    if let Some(ref node) = tree {
        if node.left.is_none() {
            Some(node)
        } else {
            find_min(&node.left)
        }
    } else {
        None
    }
}

fn find_max<T>(tree: &Option<T>) -> Option<&T> where T: TreeNode {
    if let Some(ref mut node) = tree {
        while let Some(ref right) = node.right {
            node = right;
        }
        Some(node)
    } else {
        None
    }
}

fn insert(x: i32, tree: &mut Option<Box<TreeNode>>) {
    if tree.is_none() {
        *tree = Some(Box::new(TreeNode::new(x)));
    } else {
        if x < tree.as_ref().unwrap().element {
            insert(x, &mut tree.as_mut().unwrap().left);
        } else if x > tree.as_ref().unwrap().element {
            insert(x, &mut tree.as_mut().unwrap().right);
        }
    }
}

fn delete<T>(x: i32, tree: &mut Option<T>) where T: TreeNode {
    if let Some(ref mut node) = tree {
        if x < node.element {
            delete(x, &mut node.left);
        } else if x > node.element {
            delete(x, &mut node.right);
        } else {
            if node.left.is_some() && node.right.is_some() {
                let min_right = find_min(&node.right).unwrap();
                node.element = min_right.element;
                delete(min_right.element, &mut node.right);
            } else {
                let mut node = Box::new(TreeNode::new(0));
                std::mem::swap(&mut node, tree);
                if node.left.is_none() {
                    *tree = node.right;
                } else if node.right.is_none() {
                    *tree = node.left;
                }
            }
        }
    }
}

fn retrieve<T>(p: &T) -> i32 where T: TreeNode {
    p.element
}

fn main() {
    let mut tree: Option<Box<TreeNode>> = None;

    for i in 0..50 {
        insert(i, &mut tree);
    }

    for i in 0..50 {
        if let Some(p) = find(i, &tree) {
            assert_eq!(retrieve(p), i);
        } else {
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
            if let Some(p) = find(i, &tree) {
                assert_eq!(retrieve(p), i);
            } else {
                println!("Error at {}", i);
            }
        } else {
            if find(i, &tree).is_some() {
                println!("Error at {}", i);
            }
        }
    }

    println!("Min is {}, Max is {}", retrieve(find_min(&tree).unwrap()), retrieve(find_max(&tree).unwrap()));
}