use std::io;
use std::str::Split;

type SearchTree = Option<Box<TreeNode>>;
type Position = Option<Box<TreeNode>>;

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

fn make_empty(tree: SearchTree) -> SearchTree {
    if let Some(mut node) = tree {
        node.left = make_empty(node.left);
        node.right = make_empty(node.right);
        if node.left.is_none() && node.right.is_none() {
            let _ = Box::from(node);
        }
    }
    None
}

fn find(x: i32, tree: &SearchTree) -> Position {
    if let Some(node) = tree {
        if x < node.element {
            find(x, &node.left)
        } else if x > node.element {
            find(x, &node.right)
        } else {
            Some(node.clone())
        }
    } else {
        None
    }
}

fn find_min(tree: &SearchTree) -> Position {
    if let Some(node) = tree {
        if let None = node.left {
            return Some(node.clone());
        }
        find_min(&node.left)
    } else {
        None
    }
}

fn find_max(tree: &SearchTree) -> Position {
    if let Some(mut node) = tree {
        while let Some(right) = node.right {
            node = right;
        }
        return Some(node.clone());
    }
    None
}

fn insert(x: i32, tree: &mut SearchTree) {
    if let None = tree {
        *tree = Some(Box::new(TreeNode::new(x)));
    } else {
        if x < tree.as_ref().unwrap().element {
            insert(x, &mut tree.as_mut().unwrap().left);
        } else if x > tree.as_ref().unwrap().element {
            insert(x, &mut tree.as_mut().unwrap().right);
        }
    }
}

fn delete(x: i32, tree: &mut SearchTree) {
    if let Some(mut node) = tree {
        if x < node.element {
            delete(x, &mut node.left);
        } else if x > node.element {
            delete(x, &mut node.right);
        } else {
            if let Some(mut right) = node.right.take() {
                if let None = node.left {
                    *tree = Some(right);
                } else {
                    let mut min_right = find_min(&right);
                    if let Some(min_node) = min_right {
                        node.element = min_node.element;
                        delete(min_node.element, &mut right);
                        node.right = Some(right);
                    } else {
                        node.right = Some(right);
                    }
                }
            } else if let None = node.left {
                *tree = node.right;
            } else {
                *tree = node.left;
            }
        }
    }
}

fn retrieve(position: &Position) -> i32 {
    position.as_ref().unwrap().element
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
    let mut tree: SearchTree = None;

    loop {
        println!("Enter space-separated integers (or 'exit' to finish):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "exit" {
            break;
        }

        for token in input.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                insert(num, &mut tree);
            }
        }
    }

    if let Some(tree) = &tree {
        println!("Tree after insertion:");
        print_in_order(tree);
    }

    delete_even_numbers(&mut tree);

    if let Some(tree) = &tree {
        println!("Tree after deleting even numbers:");
        print_in_order(tree);
    }

    let min = find_min(&tree);
    let max = find_max(&tree);
    if let (Some(min), Some(max)) = (min, max) {
        println!("Min is {}, Max is {}", retrieve(&min), retrieve(&max));
    } else {
        println!("Tree is empty");
    }

    make_empty(tree);
}