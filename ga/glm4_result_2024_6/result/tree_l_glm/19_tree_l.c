use std::io;
use std::collections::Vec;
use std::cell::RefCell;
use std::rc::Rc;

type SearchTree = Rc<RefCell<TreeNode>>;
type Position = Rc<RefCell<TreeNode>>;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<SearchTree>,
    right: Option<SearchTree>,
}

impl TreeNode {
    fn new(x: i32) -> Self {
        TreeNode {
            element: x,
            left: None,
            right: None,
        }
    }
}

fn make_empty(tree: &mut Option<SearchTree>) {
    if let Some(node) = tree.take() {
        make_empty(&mut node.borrow_mut().left);
        make_empty(&mut node.borrow_mut().right);
    }
}

fn find(x: i32, tree: &SearchTree) -> Option<Position> {
    let node = tree.borrow();
    if node.element == x {
        return Some(tree.clone());
    }
    if x < node.element {
        find(x, node.left.as_ref().unwrap_or(&Rc::new(RefCell::new(TreeNode::new(0)))))
    } else {
        find(x, node.right.as_ref().unwrap_or(&Rc::new(RefCell::new(TreeNode::new(0)))))
    }
}

fn find_min(tree: &SearchTree) -> Option<Position> {
    let node = tree.borrow();
    if node.left.is_none() {
        return Some(tree.clone());
    }
    find_min(node.left.as_ref().unwrap())
}

fn find_max(tree: &SearchTree) -> Option<Position> {
    let node = tree.borrow();
    if node.right.is_none() {
        return Some(tree.clone());
    }
    find_max(node.right.as_ref().unwrap())
}

fn insert(x: i32, tree: &mut Option<SearchTree>) {
    if tree.is_none() {
        *tree = Some(Rc::new(RefCell::new(TreeNode::new(x))));
    } else {
        let node = tree.as_ref().unwrap().borrow_mut();
        if x < node.element {
            insert(x, &mut node.left);
        } else if x > node.element {
            insert(x, &mut node.right);
        }
    }
}

fn delete(x: i32, tree: &mut Option<SearchTree>) {
    if let Some(node) = tree.take() {
        if node.borrow().element == x {
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                return;
            } else if node.borrow().left.is_none() {
                *tree = node.borrow().right.clone();
            } else if node.borrow().right.is_none() {
                *tree = node.borrow().left.clone();
            } else {
                let min_node = find_min(node.borrow().right.as_ref().unwrap()).unwrap();
                node.borrow_mut().element = min_node.borrow().element;
                delete(min_node.borrow().element, &mut node.borrow_mut().right);
            }
        } else if x < node.borrow().element {
            delete(x, &mut node.borrow_mut().left);
        } else {
            delete(x, &mut node.borrow_mut().right);
        }
        *tree = Some(node);
    }
}

fn retrieve(p: &Position) -> i32 {
    p.borrow().element
}

fn print_in_order(tree: &SearchTree) {
    if let Some(node) = tree {
        print_in_order(&node.borrow().left);
        println!("{}", node.borrow().element);
        print_in_order(&node.borrow().right);
    }
}

fn delete_even_numbers(tree: &mut Option<SearchTree>) {
    if let Some(node) = tree {
        delete_even_numbers(&mut node.borrow_mut().left);
        delete_even_numbers(&mut node.borrow_mut().right);
        if node.borrow().element % 2 == 0 {
            delete(node.borrow().element, tree);
        }
    }
}

fn main() {
    let mut tree: Option<SearchTree> = None;
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let nums: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
    for num in nums {
        insert(num, &mut tree);
    }

    print_in_order(&tree);

    delete_even_numbers(&mut tree);

    println!("Tree after deleting even numbers:");
    print_in_order(&tree);

    if let Some(node) = &tree {
        println!("Min is {}, Max is {}", retrieve(find_min(node).unwrap()), retrieve(find_max(node).unwrap()));
    } else {
        println!("Tree is empty");
    }

    make_empty(&mut tree);
}