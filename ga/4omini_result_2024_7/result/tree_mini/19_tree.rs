#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty() -> SearchTree {
    None
}

fn find(x: i32, tree: &SearchTree) -> Option<&TreeNode> {
    match tree {
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

fn find_min(tree: &SearchTree) -> Option<&TreeNode> {
    match tree {
        Some(node) => {
            if node.left.is_some() {
                find_min(&node.left)
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_max(tree: &SearchTree) -> Option<&TreeNode> {
    match tree {
        Some(node) => {
            let mut current = node;
            while current.right.is_some() {
                if let Some(right_node) = &current.right {
                    current = right_node;
                }
            }
            Some(current)
        }
        None => None,
    }
}

fn insert(x: i32, tree: SearchTree) -> SearchTree {
    if tree.is_none() {
        Some(Box::new(TreeNode {
            element: x,
            left: None,
            right: None,
        }))
    } else {
        let mut new_tree = tree;
        if let Some(node) = new_tree.as_mut() {
            if x < node.element {
                node.left = insert(x, node.left.take());
            } else if x > node.element {
                node.right = insert(x, node.right.take());
            }
        }
        new_tree
    }
}

fn delete(x: i32, tree: SearchTree) -> SearchTree {
    if let Some(mut node) = tree {
        if x < node.element {
            node.left = delete(x, node.left.take());
            Some(node)
        } else if x > node.element {
            node.right = delete(x, node.right.take());
            Some(node)
        } else {
            if node.left.is_some() && node.right.is_some() {
                let min_node = find_min(&node.right);
                if let Some(min_node) = min_node {
                    node.element = min_node.element;
                    node.right = delete(min_node.element, node.right.take());
                }
            } else {
                if node.left.is_none() {
                    return node.right;
                } else {
                    return node.left;
                }
            }
            Some(node)
        }
    } else {
        panic!("Element not found");
    }
}

fn retrieve(position: &TreeNode) -> i32 {
    position.element
}

fn main() {
    let mut tree: SearchTree = make_empty();

    let mut j = 0;
    for i in 0..50 {
        j = (j + 7) % 50;
        tree = insert(j, tree);
    }

    for i in 0..50 {
        match find(i, &tree) {
            Some(p) => {
                if retrieve(p) != i {
                    println!("Error at {}", i);
                }
            }
            None => println!("Error at {}", i),
        }
    }

    for i in (0..50).step_by(2) {
        tree = delete(i, tree);
    }

    for i in (1..50).step_by(2) {
        match find(i, &tree) {
            Some(p) => {
                if retrieve(p) != i {
                    println!("Error at {}", i);
                }
            }
            None => println!("Error at {}", i),
        }
    }

    for i in (0..50).step_by(2) {
        if find(i, &tree).is_some() {
            println!("Error at {}", i);
        }
    }

    if let Some(min_node) = find_min(&tree) {
        if let Some(max_node) = find_max(&tree) {
            println!("Min is {}, Max is {}", retrieve(min_node), retrieve(max_node));
        }
    }
}