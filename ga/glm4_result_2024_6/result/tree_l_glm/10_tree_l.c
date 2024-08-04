use std::io;
use std::str::FromStr;
use std::collections::binary_tree::Node;

type SearchTree = Option<Box<Node<i32>>>;

fn make_empty<T>(tree: &mut SearchTree) {
    if let Some(ref mut node) = *tree {
        make_empty(&mut node.left);
        make_empty(&mut node.right);
        *tree = None;
    }
}

fn find<T>(x: i32, tree: &SearchTree) -> Option<&Node<T>> {
    if let Some(ref node) = *tree {
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

fn find_min<T>(tree: &SearchTree) -> Option<&Node<T>> {
    if let Some(ref node) = *tree {
        if node.left.is_none() {
            Some(node)
        } else {
            find_min(&node.left)
        }
    } else {
        None
    }
}

fn find_max<T>(tree: &SearchTree) -> Option<&Node<T>> {
    if let Some(ref node) = *tree {
        if node.right.is_none() {
            Some(node)
        } else {
            find_max(&node.right)
        }
    } else {
        None
    }
}

fn insert(x: i32, tree: &mut SearchTree) {
    if let None = *tree {
        *tree = Some(Box::new(Node { element: x, left: None, right: None }));
    } else if x < tree.as_ref().unwrap().element {
        insert(x, &mut tree.as_mut().unwrap().left);
    } else if x > tree.as_ref().unwrap().element {
        insert(x, &mut tree.as_mut().unwrap().right);
    }
}

fn delete<T>(x: i32, tree: &mut SearchTree) {
    if let Some(ref mut node) = *tree {
        if x < node.element {
            delete(x, &mut node.left);
        } else if x > node.element {
            delete(x, &mut node.right);
        } else {
            if node.left.is_none() {
                *tree = node.right.take();
            } else if node.right.is_none() {
                *tree = node.left.take();
            } else {
                let min_node = find_min(&node.right);
                node.element = min_node.unwrap().element;
                delete(min_node.unwrap().element, &mut node.right);
            }
        }
    }
}

fn retrieve<T>(position: &Node<T>) -> T {
    position.element
}

fn print_in_order<T>(tree: &SearchTree) {
    if let Some(ref node) = *tree {
        print_in_order(&node.left);
        println!("{}", node.element);
        print_in_order(&node.right);
    }
}

fn delete_even_numbers(tree: &mut SearchTree) {
    if let Some(ref mut node) = *tree {
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
    io::stdin().read_line(&mut input).expect("Failed to read line");

    for token in input.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            insert(num, &mut tree);
        }
    }

    println!("Tree after insertion:");
    print_in_order(&tree);

    delete_even_numbers(&mut tree);

    println!("Tree after deleting even numbers:");
    print_in_order(&tree);

    if let Some(node) = find_min(&tree) {
        println!("Min is {}, Max is {}", retrieve(node), retrieve(find_max(&tree).unwrap()));
    } else {
        println!("Tree is empty");
    }

    make_empty(&mut tree);
}