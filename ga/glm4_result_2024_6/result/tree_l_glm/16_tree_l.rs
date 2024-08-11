use std::io::{self, Read};
use std::str::FromStr;
use std::cmp::Ordering;

type ElementType = i32;
type SearchTree = Option<Box<TreeNode>>;

struct TreeNode {
    element: ElementType,
    left: SearchTree,
    right: SearchTree,
}

impl TreeNode {
    fn new(element: ElementType) -> Self {
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
        tree.take();
    }
}

fn find(x: ElementType, tree: &SearchTree) -> Option<&TreeNode> {
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

fn find_min(tree: &SearchTree) -> Option<&TreeNode> {
    if let Some(node) = tree {
        if let None = node.left {
            return Some(node);
        }
        find_min(&node.left)
    } else {
        None
    }
}

fn find_max(tree: &SearchTree) -> Option<&TreeNode> {
    if let Some(node) = tree {
        if let None = node.right {
            return Some(node);
        }
        find_max(&node.right)
    } else {
        None
    }
}

fn insert(x: ElementType, tree: &mut SearchTree) {
    if let None = tree {
        *tree = Some(Box::new(TreeNode::new(x)));
    } else {
        match x.cmp(&tree.as_ref().unwrap().element) {
            Ordering::Less => insert(x, &mut tree.as_mut().unwrap().left),
            Ordering::Greater => insert(x, &mut tree.as_mut().unwrap().right),
            _ => {}
        }
    }
}

fn delete(x: ElementType, tree: &mut SearchTree) {
    if let Some(node) = tree {
        match x.cmp(&node.element) {
            Ordering::Less => delete(x, &mut node.left),
            Ordering::Greater => delete(x, &mut node.right),
            Ordering::Equal => {
                if let (Some(left), Some(right)) = (node.left.clone(), node.right.clone()) {
                    let min_node = find_min(&right).unwrap();
                    node.element = min_node.element;
                    delete(min_node.element, &mut node.right);
                } else {
                    *tree = node.left.or(node.right);
                }
            }
        }
    }
}

fn retrieve(p: &TreeNode) -> ElementType {
    p.element
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    for token in input.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            insert(num, &mut tree);
        }
    }
    if let None = tree {
        println!("Tree is empty");
        return;
    }
    println!("Tree after insertion:");
    print_in_order(&tree);
    delete_even_numbers(&mut tree);
    println!("Tree after deleting even numbers:");
    print_in_order(&tree);
    if let Some(node) = tree {
        println!("Min is {}, Max is {}", retrieve(find_min(&tree).unwrap()), retrieve(find_max(&tree).unwrap()));
    } else {
        println!("Tree is empty");
    }
}