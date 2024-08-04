use std::io::{self, BufRead};
use std::ptr;
use std::cell::RefCell;
use std::rc::Rc;

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
        None => None,
        Some(node) => {
            let node = node.borrow();
            if x < node.element {
                find(x, &node.left)
            } else if x > node.element {
                find(x, &node.right)
            } else {
                Some(Rc::clone(node))
            }
        }
    }
}

fn find_min(t: &SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    match t {
        None => None,
        Some(node) => {
            let left = &node.borrow().left;
            if left.is_none() {
                Some(Rc::clone(node))
            } else {
                find_min(left)
            }
        }
    }
}

fn find_max(t: &SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    let mut current = t;
    while let Some(node) = current {
        current = &node.borrow().right;
    }
    current.and_then(Rc::clone)
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => {
            Some(Rc::new(RefCell::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })))
        }
        Some(node) => {
            let mut node_ref = node.borrow_mut();
            if x < node_ref.element {
                node_ref.left = insert(x, node_ref.left.clone());
                Some(node)
            } else if x > node_ref.element {
                node_ref.right = insert(x, node_ref.right.clone());
                Some(node)
            } else {
                Some(node) // Do not insert duplicates
            }
        }
    }
}

fn delete(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => {
            println!("Element not found");
            None
        }
        Some(node) => {
            let mut node_ref = node.borrow_mut();
            if x < node_ref.element {
                node_ref.left = delete(x, node_ref.left.clone());
                Some(node)
            } else if x > node_ref.element {
                node_ref.right = delete(x, node_ref.right.clone());
                Some(node)
            } else {
                if node_ref.left.is_some() && node_ref.right.is_some() {
                    if let Some(min_node) = find_min(&node_ref.right) {
                        let min_value = min_node.borrow().element;
                        node_ref.element = min_value;
                        node_ref.right = delete(min_value, node_ref.right.clone());
                    }
                    Some(node)
                } else { // One child case
                    let tmp_node = Rc::clone(node);
                    if node_ref.left.is_none() { 
                        Some(node_ref.right.clone()) // Only right child
                    } else {
                        Some(node_ref.left.clone()) // Only left child
                    }
                }
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

fn delete_even_numbers(t: SearchTree) -> SearchTree {
    if let Some(node) = t {
        let mut node_ref = node.borrow_mut();
        node_ref.left = delete_even_numbers(node_ref.left.clone());
        node_ref.right = delete_even_numbers(node_ref.right.clone());

        if node_ref.element % 2 == 0 {
            delete(node_ref.element, Some(node.clone()))
        } else {
            Some(node)
        }
    } else {
        None
    }
}

fn main() {
    let mut t = make_empty();
    let stdin = io::stdin();
    let input_line: String = stdin.lock().lines().next().expect("No input").unwrap();

    let numbers: Vec<i32> = input_line
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();

    // Insert elements into the tree
    for num in numbers {
        t = insert(num, t);
    }

    if t.is_none() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    print_in_order(&t);

    // Delete all even numbers
    t = delete_even_numbers(t);

    println!("Tree after deleting even numbers:");
    print_in_order(&t);

    if let Some(t) = t {
        let min = find_min(&Some(Rc::clone(&t)));
        let max = find_max(&Some(Rc::clone(&t)));
        
        if let Some(min_node) = min {
            print!("Min is {}", min_node.borrow().element);
        }
        if let Some(max_node) = max {
            print!(", Max is {}", max_node.borrow().element);
        }
        println!();
    } else {
        println!("Tree is empty");
    }
}