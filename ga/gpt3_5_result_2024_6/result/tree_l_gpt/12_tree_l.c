use std::io::{self, BufRead};
use std::cmp::Ordering;
use std::mem;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Link,
    right: Link,
}

type Link = Option<Box<TreeNode>>;

fn make_empty(t: SearchTree) -> SearchTree {
    if let Some(mut node) = t {
        node.left = make_empty(node.left);
        node.right = make_empty(node.right);
    }
    None
}

fn find(x: i32, t: &SearchTree) -> Option<&TreeNode> {
    match t {
        Some(node) => match x.cmp(&node.element) {
            Ordering::Less => find(x, &node.left),
            Ordering::Greater => find(x, &node.right),
            Ordering::Equal => Some(node),
        },
        None => None,
    }
}

fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    match t {
        Some(node) => {
            if let Some(ref left) = node.left {
                find_min(&node.left)
            } else {
                Some(node)
            }
        }
        None => None,
    }
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    let mut current = t;
    while let Some(node) = current {
        match node.right {
            Some(_) => current = &node.right,
            None => return Some(node),
        }
    }
    None
}

fn insert(x: i32, t: SearchTree) -> SearchTree {
    match t {
        Some(mut node) => {
            match x.cmp(&node.element) {
                Ordering::Less => node.left = insert(x, node.left),
                Ordering::Greater => node.right = insert(x, node.right),
                Ordering::Equal => {} // do nothing for now
            }
            Some(node)
        }
        None => Some(Box::new(TreeNode { element: x, left: None, right: None })),
    }
}

fn delete(x: i32, t: &mut SearchTree) {
    if let Some(node) = t.take() {
        match x.cmp(&node.element) {
            Ordering::Less => delete(x, &mut node.left),
            Ordering::Greater => delete(x, &mut node.right),
            Ordering::Equal => {
                *t = if node.left.is_none() {
                    node.right
                } else if node.right.is_none() {
                    node.left
                } else {
                    let replacement = find_min(&node.right).unwrap().element;
                    delete(replacement, &mut node.right);
                    Some(Box::new(TreeNode {
                        element: replacement,
                        left: node.left,
                        right: node.right,
                    }))
                }
            }
        }
    }
}

fn retrieve(p: Option<&TreeNode>) -> i32 {
    p.unwrap().element
}

fn print_inorder(t: &SearchTree) {
    if let Some(node) = t {
        print_inorder(&node.left);
        println!("{}", node.element);
        print_inorder(&node.right);
    }
}

fn delete_even_numbers(t: &mut SearchTree) {
    if let Some(mut node) = t.take() {
        delete_even_numbers(&mut node.left);
        delete_even_numbers(&mut node.right);

        if node.element % 2 == 0 {
            delete(node.element, t);
        } else {
            *t = Some(node);
        }
    }
}

type SearchTree = Link;

fn main() {
    let mut t: SearchTree = None;
    let stdin = io::stdin();
    let mut has_input = false;

    if let Some(Ok(input)) = stdin.lock().lines().next() {
        for token in input.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                t = insert(num, t);
                has_input = true;
            }
        }
    }

    if !has_input || t.is_none() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    print_inorder(&t);

    delete_even_numbers(&mut t);

    println!("Tree after deleting even numbers:");
    print_inorder(&t);

    if let Some(node) = t {
        println!("Min is {}, Max is {}", retrieve(find_min(&t).unwrap()), retrieve(find_max(&t).unwrap()));
    } else {
        println!("Tree is empty");
    }

    t = make_empty(t);
}