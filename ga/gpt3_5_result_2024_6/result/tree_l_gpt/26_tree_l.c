use std::io::{self, BufRead};

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;
type Position = SearchTree;

fn make_empty(t: &mut SearchTree) {
    *t = None;
}

fn find(x: ElementType, t: &SearchTree) -> Option<&Position> {
    match t {
        None => None,
        Some(node) => {
            if x < node.element {
                find(x, &node.left)
            } else if x > node.element {
                find(x, &node.right)
            } else {
                Some(node)
            }
        }
    }
}

fn find_min(t: &SearchTree) -> Option<&Position> {
    match t {
        None => None,
        Some(node) => {
            if let Some(mut n) = &node.left {
                find_min(&Some(n))
            } else {
                Some(node)
            }
        }
    }
}

fn find_max(t: &SearchTree) -> &Position {
    let mut temp = t.as_ref();
    while let Some(node) = temp {
        temp = node.right.as_ref();
    }
    return temp.unwrap();
}

fn insert(x: ElementType, t: &mut SearchTree) {
    match t {
        None => {
            *t = Some(Box::new(TreeNode{
                element: x,
                left: None,
                right: None,
            }));
        }
        Some(node) => {
            if x < node.element {
                insert(x, &mut node.left);
            } else if x > node.element {
                insert(x, &mut node.right);
            }
        }
    }
}

fn delete(x: ElementType, t: &mut SearchTree) {
    let mut tmp_cell = None;
    match t {
        None => eprintln!("Element not found"),
        Some(node) => {
            if x < node.element {
                delete(x, &mut node.left);
            } else if x > node.element {
                delete(x, &mut node.right);
            } else if let (Some(_), Some(mut right)) = (&mut node.left, &mut node.right) {
                *t = Some(Box::new(TreeNode{
                    element: find_min(&Some(right)).unwrap().element,
                    left: node.left.take(),
                    right: node.right.take(),
                }));
                delete(find_min(&Some(right)).unwrap().element, &mut right);
            } else {
                tmp_cell = Some(*node);
                if node.left.is_none() {
                    *t = node.right.take();
                } else if node.right.is_none() {
                    *t = node.left.take();
                }
            }
        }
    }
}

fn retrieve(p: &Position) -> ElementType {
    p.as_ref().unwrap().element
}

fn print_in_order(t: &SearchTree) {
    match t {
        None => (),
        Some(node) => {
            print_in_order(&node.left);
            println!("{}", node.element);
            print_in_order(&node.right);
        }
    }
}

fn delete_even_numbers(t: &mut SearchTree) {
    if t.is_none() {
        return;
    }

    delete_even_numbers(&mut t.as_mut().unwrap().left);
    delete_even_numbers(&mut t.as_mut().unwrap().right);

    if t.as_ref().unwrap().element % 2 == 0 {
        delete(t.as_ref().unwrap().element, t);
    }
}

fn main() {
    let mut t: SearchTree = None;
    let stdin = io::stdin();
    let mut has_input = false;

    if let Ok(line) = stdin.lock().lines().next() {
        if let Ok(input) = line {
            for token in input.split(" ") {
                if let Ok(num) = token.parse::<ElementType>() {
                    insert(num, &mut t);
                    has_input = true;
                }
            }
        }
    }

    if !has_input || t.is_none() {
        println!("Tree is empty");
        return;
    }

    println!("Tree after insertion:");
    print_in_order(&t);

    delete_even_numbers(&mut t);

    println!("Tree after deleting even numbers:");
    print_in_order(&t);

    if let Some(min_node) = find_min(&t) {
        if let Some(max_node) = find_max(&t) {
            println!("Min is {}, Max is {}", retrieve(min_node), retrieve(max_node));
        }
    } else {
        println!("Tree is empty");
    }

    make_empty(&mut t);
}