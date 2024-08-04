use std::io::{self, Write};

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

impl TreeNode {
    fn new(element: i32) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
}

pub fn make_empty<T>(tree: T) -> Option<Box<TreeNode>> {
    tree // Rust's ownership system will free now unused memory automatically
}

pub fn find(x: i32, tree: &SearchTree) -> Option<&TreeNode> {
    match tree {
        Some(node) if x < node.element => find(x, &node.left),
        Some(node) if x > node.element => find(x, &node.right),
        Some(node) => Some(node),
        None => None,
    }
}

pub fn find_min(tree: &SearchTree) -> Option<&TreeNode> {
    match tree {
        Some(node) if node.left.is_none() => Some(node),
        Some(node) => find_min(&node.left),
        None => None,
    }
}

pub fn find_max(tree: &SearchTree) -> Option<&TreeNode> {
    match tree {
        Some(node) if node.right.is_none() => Some(node),
        Some(node) => find_max(&node.right),
        None => None,
    }
}

pub fn insert(x: i32, tree: &mut SearchTree) {
    match tree {
        Some(ref mut node) => {
            if x < node.element {
                insert(x, &mut node.left);
            } else if x > node.element {
                insert(x, &mut node.right);
            }
        }
        None => {
            *tree = Some(Box::new(TreeNode::new(x))); // create new node
        }
    }
}

pub fn delete(x: i32, tree: &mut SearchTree) -> bool {
    if let Some(ref mut node) = tree {
        if x < node.element {
            return delete(x, &mut node.left);
        } else if x > node.element {
            return delete(x, &mut node.right);
        } else {
            // Node to be deleted found
            match (node.left.as_ref(), node.right.as_ref()) {
                (None, None) => {
                    // Node has no children
                    *tree = None;
                }
                (Some(left), None) => {
                    // Node has only a left child
                    *tree = Some(left.clone());
                }
                (None, Some(right)) => {
                    // Node has only a right child
                    *tree = Some(right.clone());
                }
                (Some(left), Some(right)) => {
                    // Node has two children
                    if let Some(min_node) = find_min(&node.right) {
                        node.element = min_node.element;
                        delete(min_node.element, &mut node.right);
                    }
                }
            }
            return true;
        }
    }
    false // Return false if the node to delete wasn't found
}

pub fn print_in_order(tree: &SearchTree) {
    if let Some(node) = tree {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

pub fn delete_even_numbers(tree: &mut SearchTree) {
    if let Some(ref mut node) = tree {
        delete_even_numbers(&mut node.left);
        delete_even_numbers(&mut node.right);
        if node.element % 2 == 0 {
            delete(node.element, tree);
        }
    }
}

fn main() {
    let mut tree: SearchTree = None;
    let mut input = String::new();
    
    print!("Enter integers separated by spaces: ");
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    for token in input.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            insert(num, &mut tree);
        }
    }
    
    println!("Tree after insertion:");
    print_in_order(&tree);

    // Delete all even numbers
    delete_even_numbers(&mut tree);
    
    println!("Tree after deleting even numbers:");
    print_in_order(&tree);

    if let (Some(min), Some(max)) = (find_min(&tree), find_max(&tree)) {
        println!("Min is {}, Max is {}", min.element, max.element);
    } else {
        println!("Tree is empty");
    }
}