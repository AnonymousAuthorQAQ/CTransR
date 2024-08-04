use std::io::{self, BufRead};
use std::ptr;
use std::rc::Rc;
use std::cell::RefCell;

type ElementType = i32;

#[derive(Debug)]
struct TreeNode {
    element: ElementType,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

type SearchTree = Option<Rc<RefCell<TreeNode>>>;

fn make_empty() -> SearchTree {
    None
}

fn find(x: ElementType, t: &SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    match t {
        Some(node) => {
            let node_ref = node.borrow();
            if x < node_ref.element {
                find(x, &node_ref.left)
            } else if x > node_ref.element {
                find(x, &node_ref.right)
            } else {
                Some(Rc::clone(node))
            }
        }
        None => None,
    }
}

fn find_min(t: &SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    match t {
        Some(node) => {
            let mut current = Rc::clone(node);
            while let Some(ref left) = current.borrow().left {
                current = Rc::clone(left);
            }
            Some(current)
        }
        None => None,
    }
}

fn find_max(t: &SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    match t {
        Some(node) => {
            let mut current = Rc::clone(node);
            while let Some(ref right) = current.borrow().right {
                current = Rc::clone(right);
            }
            Some(current)
        }
        None => None,
    }
}

fn insert(x: ElementType, t: &mut SearchTree) {
    match t {
        Some(node) => {
            let mut node_ref = node.borrow_mut();
            if x < node_ref.element {
                insert(x, &mut node_ref.left);
            } else if x > node_ref.element {
                insert(x, &mut node_ref.right);
            }
        }
        None => {
            *t = Some(Rc::new(RefCell::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })));
        }
    }
}

fn delete(x: ElementType, t: &mut SearchTree) {
    if let Some(node) = t {
        let mut node_ref = node.borrow_mut();
        if x < node_ref.element {
            delete(x, &mut node_ref.left);
        } else if x > node_ref.element {
            delete(x, &mut node_ref.right);
        } else {
            // Found the node to delete
            let left = node_ref.left.take();
            let right = node_ref.right.take();

            if left.is_none() {
                *t = right; // Replace with right child
            } else if right.is_none() {
                *t = left; // Replace with left child
            } else {
                // Node has two children: get the minimum from the right subtree
                if let Some(min_node) = find_min(&node_ref.right) {
                    node_ref.element = min_node.borrow().element;
                    delete(node_ref.element, &mut node_ref.right); // Delete the min node
                }
                *t = Some(node.clone()); // Keep the current node
            }
        }
    }
}

fn delete_even_numbers(t: &mut SearchTree) {
    if let Some(node) = t {
        let mut node_ref = node.borrow_mut();
        delete_even_numbers(&mut node_ref.left);
        delete_even_numbers(&mut node_ref.right);

        if node_ref.element % 2 == 0 {
            *t = None; // Delete this node by replacing it with None
            if let Some(left) = node_ref.left.take() {
                *t = Some(left); // Take left child
            } else if let Some(right) = node_ref.right.take() {
                *t = Some(right); // Take right child
            }
        }
    }
}

fn print_in_order(t: &SearchTree) {
    if let Some(node) = t {
        let node_ref = node.borrow();
        print_in_order(&node_ref.left);
        println!("{}", node_ref.element);
        print_in_order(&node_ref.right);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut tree = make_empty();

    println!("Enter space-separated integers (end with enter):");

    let input = stdin.lock().lines().next().unwrap().unwrap();
    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|token| token.parse().ok())
        .collect();

    for num in numbers {
        insert(num, &mut tree);
    }

    println!("Tree after insertion:");
    print_in_order(&tree);

    // Delete all even numbers
    delete_even_numbers(&mut tree);

    println!("Tree after deleting even numbers:");
    print_in_order(&tree);

    if let Some(min_node) = find_min(&tree) {
        if let Some(max_node) = find_max(&tree) {
            println!(
                "Min is {}, Max is {}",
                min_node.borrow().element,
                max_node.borrow().element
            );
        } else {
            println!("Tree is empty after deletion");
        }
    } else {
        println!("Tree is empty");
    }
}