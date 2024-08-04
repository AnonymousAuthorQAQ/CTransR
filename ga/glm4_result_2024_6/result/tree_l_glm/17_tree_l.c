use std::io::{self, BufRead};
use std::collections::binary_tree::Node;

fn make_empty<T>(tree: &mut Option<Node<T>>) {
    if let Some(node) = tree {
        make_empty(&mut node.left);
        make_empty(&mut node.right);
        *tree = None;
    }
}

fn find<T>(x: T, tree: &Option<Node<T>>) -> Option<&Node<T>>
where
    T: Ord + Copy,
{
    if let Some(node) = tree {
        if x < node.element {
            find(x, &node.left)
        } else if x > node.element {
            find(x, &node.right)
        } else {
            Some(node)
        }
    } else {
        None
    }
}

fn find_min<T>(tree: &Option<Node<T>>) -> Option<&Node<T>>
where
    T: Ord + Copy,
{
    if let Some(node) = tree {
        if node.left.is_none() {
            Some(node)
        } else {
            find_min(&node.left)
        }
    } else {
        None
    }
}

fn find_max<T>(tree: &Option<Node<T>>) -> Option<&Node<T>>
where
    T: Ord + Copy,
{
    if let Some(node) = tree {
        if node.right.is_none() {
            Some(node)
        } else {
            find_max(&node.right)
        }
    } else {
        None
    }
}

fn insert<T>(x: T, tree: &mut Option<Node<T>>) -> &mut Option<Node<T>>
where
    T: Ord + Copy,
{
    if tree.is_none() {
        *tree = Some(Node::new(x));
    } else {
        if x < tree.as_ref().unwrap().element {
            insert(x, &mut tree.as_mut().unwrap().left);
        } else if x > tree.as_ref().unwrap().element {
            insert(x, &mut tree.as_mut().unwrap().right);
        }
    }
    tree
}

fn delete<T>(x: T, tree: &mut Option<Node<T>>) -> &mut Option<Node<T>>
where
    T: Ord + Copy,
{
    if let Some(node) = tree {
        if x < node.element {
            delete(x, &mut node.left);
        } else if x > node.element {
            delete(x, &mut node.right);
        } else if node.left.is_some() && node.right.is_some() {
            let min_right = find_min(&node.right);
            node.element = min_right.unwrap().element;
            delete(node.element, &mut node.right);
        } else {
            *tree = node.left.or(node.right);
        }
    }
    tree
}

fn retrieve<T>(p: &Node<T>) -> T
where
    T: Ord + Copy,
{
    p.element
}

fn print_in_order<T>(tree: &Option<Node<T>>) -> &Option<Node<T>>
where
    T: Ord + Copy,
{
    if let Some(node) = tree {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
    tree
}

fn delete_even_numbers<T>(tree: &mut Option<Node<T>>) -> &mut Option<Node<T>>
where
    T: Ord + Copy,
{
    if let Some(node) = tree {
        delete_even_numbers(&mut node.left);
        delete_even_numbers(&mut node.right);
        if node.element % 2 == 0 {
            delete(node.element, tree);
        }
    }
    tree
}

fn main() {
    let mut tree: Option<Node<i32>> = None;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                insert(num, &mut tree);
            }
        }
    }

    if tree.is_none() {
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

    make_empty(&mut tree);
}