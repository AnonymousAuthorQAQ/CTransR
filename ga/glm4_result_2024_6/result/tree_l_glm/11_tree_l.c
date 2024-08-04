use std::io::{self, BufRead};

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

fn make_empty(tree: &mut Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        make_empty(&mut node.left);
        make_empty(&mut node.right);
        *tree = None;
    }
}

fn find(x: i32, tree: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
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

fn find_min(tree: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
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

fn find_max(tree: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
    if let Some(node) = tree {
        if node.right.is_none() {
            Some(node)
        } else {
            find_max(&node.right)
        }
    } else {
        None
    }
}

fn insert(x: i32, tree: &mut Option<Box<TreeNode>>) {
    if let None = tree {
        *tree = Some(Box::new(TreeNode::new(x)));
    } else if x < tree.as_ref().unwrap().element {
        insert(x, &mut tree.as_mut().unwrap().left);
    } else if x > tree.as_ref().unwrap().element {
        insert(x, &mut tree.as_mut().unwrap().right);
    }
}

fn delete(x: i32, tree: &mut Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        if x < node.element {
            delete(x, &mut node.left);
        } else if x > node.element {
            delete(x, &mut node.right);
        } else {
            if let (Some(left), Some(right)) = (&node.left, &node.right) {
                let mut min_node = find_min(&right).unwrap();
                node.element = min_node.element;
                delete(min_node.element, &mut right);
            } else {
                *tree = node.left.or(node.right);
            }
        }
    }
}

fn retrieve(node: &TreeNode) -> i32 {
    node.element
}

fn print_in_order(tree: &Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn delete_even_numbers(tree: &mut Option<Box<TreeNode>>) {
    if let Some(node) = tree {
        delete_even_numbers(&mut node.left);
        delete_even_numbers(&mut node.right);
        if node.element % 2 == 0 {
            delete(node.element, tree);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().and_then(|line| line.ok());
    let mut tree: Option<Box<TreeNode>> = None;

    if let Some(line) = line {
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                insert(num, &mut tree);
            }
        }
    }

    print_in_order(&tree);
    delete_even_numbers(&mut tree);
    print_in_order(&tree);

    if let Some(node) = &tree {
        println!("Min is {}, Max is {}", retrieve(find_min(&tree).unwrap()), retrieve(find_max(&tree).unwrap()));
    } else {
        println!("Tree is empty");
    }

    make_empty(&mut tree);
}