use std::io::{self, BufRead};

type SearchTree = Option<Box<TreeNode>>;

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

fn find(x: i32, tree: &SearchTree) -> Option<&TreeNode> {
    tree.as_ref().and_then(|node| {
        if x < node.element {
            find(x, &node.left)
        } else if x > node.element {
            find(x, &node.right)
        } else {
            Some(node)
        }
    })
}

fn find_min(tree: &SearchTree) -> Option<&TreeNode> {
    tree.as_ref().and_then(|node| {
        if let Some(min_node) = find_min(&node.left) {
            Some(min_node)
        } else {
            Some(node)
        }
    })
}

fn find_max(tree: &SearchTree) -> Option<&TreeNode> {
    tree.as_ref().and_then(|node| {
        if let Some(max_node) = find_max(&node.right) {
            Some(max_node)
        } else {
            Some(node)
        }
    })
}

fn insert(x: i32, tree: &mut SearchTree) {
    if let Some(node) = tree {
        if x < node.element {
            insert(x, &mut node.left);
        } else if x > node.element {
            insert(x, &mut node.right);
        }
    } else {
        *tree = Some(Box::new(TreeNode::new(x)));
    }
}

fn delete(x: i32, tree: &mut SearchTree) {
    if let Some(node) = tree {
        if x < node.element {
            delete(x, &mut node.left);
        } else if x > node.element {
            delete(x, &mut node.right);
        } else {
            if let (Some(left), Some(right)) = (&node.left, &node.right) {
                let min_node = find_min(&node.right).unwrap();
                node.element = min_node.element;
                delete(min_node.element, &mut node.right);
            } else {
                *tree = node.left.or(node.right);
            }
        }
    }
}

fn retrieve(node: &TreeNode) -> i32 {
    node.element
}

fn print_in_order(tree: &SearchTree) {
    if let Some(node) = tree {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn delete_even_numbers(tree: &mut SearchTree) {
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
    let mut tree: SearchTree = None;

    if let Some(line) = line {
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                insert(num, &mut tree);
            }
        }
    }

    if let Some(ref tree) = tree {
        println!("Tree after insertion:");
        print_in_order(tree);
    } else {
        println!("Tree is empty");
    }

    delete_even_numbers(&mut tree);

    if let Some(ref tree) = tree {
        println!("Tree after deleting even numbers:");
        print_in_order(tree);
        println!("Min is {}, Max is {}", retrieve(find_min(&tree).unwrap()), retrieve(find_max(&tree).unwrap()));
    } else {
        println!("Tree is empty");
    }

    make_empty(&mut tree);
}