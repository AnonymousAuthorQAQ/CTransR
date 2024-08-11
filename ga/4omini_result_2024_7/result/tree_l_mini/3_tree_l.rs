use std::io;
use std::ptr;
use std::rc::Rc;
use std::cell::RefCell;

type ElementType = i32;

#[derive(Clone)]
struct TreeNode {
    element: ElementType,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

type SearchTree = Option<Rc<RefCell<TreeNode>>>;

fn make_empty() -> SearchTree {
    None
}

fn insert(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => {
            let new_node = Rc::new(RefCell::new(TreeNode {
                element: x,
                left: None,
                right: None,
            }));
            Some(new_node)
        }
        Some(node) => {
            if x < node.borrow().element {
                node.borrow_mut().left = insert(x, node.borrow().left.clone());
            } else if x > node.borrow().element {
                node.borrow_mut().right = insert(x, node.borrow().right.clone());
            }
            Some(node) // Return the unchanged node pointer
        }
    }
}

fn find_min(t: SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    match t {
        None => None,
        Some(node) => {
            if node.borrow().left.is_none() {
                Some(node)
            } else {
                find_min(node.borrow().left.clone())
            }
        }
    }
}

fn find_max(t: SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    let mut current = t;
    while let Some(node) = current {
        if node.borrow().right.is_none() {
            return Some(node);
        }
        current = node.borrow().right.clone();
    }
    None
}

fn delete(x: ElementType, t: SearchTree) -> SearchTree {
    match t {
        None => None, // Element not found
        Some(node) => {
            if x < node.borrow().element {
                let left = delete(x, node.borrow().left.clone());
                node.borrow_mut().left = left;
                Some(node)
            } else if x > node.borrow().element {
                let right = delete(x, node.borrow().right.clone());
                node.borrow_mut().right = right;
                Some(node)
            } else { // Found node to delete
                if node.borrow().left.is_some() && node.borrow().right.is_some() {
                    // Two children
                    let min_node = find_min(node.borrow().right.clone()).unwrap();
                    let min_value = min_node.borrow().element;
                    node.borrow_mut().element = min_value;
                    node.borrow_mut().right = delete(min_value, node.borrow().right.clone());
                    Some(node)
                } else {
                    // One or no child
                    let child = if node.borrow().left.is_some() {
                        node.borrow().left.clone()
                    } else {
                        node.borrow().right.clone()
                    };
                    child
                }
            }
        }
    }
}

fn delete_even_numbers(t: SearchTree) -> SearchTree {
    if t.is_none() {
        return None;
    }

    let left = delete_even_numbers(t.as_ref().unwrap().borrow().left.clone());
    let right = delete_even_numbers(t.as_ref().unwrap().borrow().right.clone());

    let current_node = t.unwrap();
    if current_node.borrow().element % 2 == 0 {
        return delete(current_node.borrow().element, if left.is_none() && right.is_none() { None } else { Some(current_node.clone()) });
    }

    current_node.borrow_mut().left = left;
    current_node.borrow_mut().right = right;
    Some(current_node)
}

fn print_in_order(t: SearchTree) {
    if let Some(node) = t {
        print_in_order(node.borrow().left.clone());
        println!("{}", node.borrow().element);
        print_in_order(node.borrow().right.clone());
    }
}

fn main() {
    let mut tree = make_empty();

    let mut input = String::new();
    println!("Enter integers separated by spaces:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<ElementType> = input
        .split_whitespace()
        .filter_map(|token| token.parse::<ElementType>().ok())
        .collect();

    for num in numbers {
        tree = insert(num, tree);
    }

    if tree.is_none() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    print_in_order(tree.clone());

    // Delete all even numbers
    tree = delete_even_numbers(tree);

    println!("Tree after deleting even numbers:");
    print_in_order(tree.clone());

    if let (Some(min), Some(max)) = (find_min(tree.clone()), find_max(tree.clone())) {
        println!("Min is {}, Max is {}", min.borrow().element, max.borrow().element);
    } else {
        println!("Tree is empty");
    }
}