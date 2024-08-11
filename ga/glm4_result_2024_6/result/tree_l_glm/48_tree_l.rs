use std::io::{self, Read};
use std::collections::binary_tree::Node;
use std::collections::binary_tree::Tree;

fn make_empty<T: Ord>(tree: &mut Tree<T>) {
    if let Some(root) = tree.root() {
        make_empty(&mut root.left());
        make_empty(&mut root.right());
        tree.clear();
    }
}

fn find<T: Ord>(tree: &Tree<T>, value: T) -> Option<Node<T>> {
    if let Some(root) = tree.root() {
        if value == root.value() {
            return Some(root);
        }
        if value < root.value() {
            return find(&root.left(), value);
        } else {
            return find(&root.right(), value);
        }
    }
    None
}

fn find_min<T: Ord>(tree: &Tree<T>) -> Option<Node<T>> {
    if let Some(root) = tree.root() {
        if let None = root.left() {
            return Some(root);
        }
        return find_min(&root.left());
    }
    None
}

fn find_max<T: Ord>(tree: &Tree<T>) -> Option<Node<T>> {
    if let Some(root) = tree.root() {
        if let None = root.right() {
            return Some(root);
        }
        return find_max(&root.right());
    }
    None
}

fn insert<T: Ord>(tree: &mut Tree<T>, value: T) {
    if let None = tree.root() {
        tree.insert(value);
    } else {
        if value < tree.root().value() {
            insert(&mut tree.root().left(), value);
        } else {
            insert(&mut tree.root().right(), value);
        }
    }
}

fn delete<T: Ord>(tree: &mut Tree<T>, value: T) {
    if let Some(root) = tree.root() {
        if value < root.value() {
            delete(&mut root.left(), value);
        } else if value > root.value() {
            delete(&mut root.right(), value);
        } else {
            if let Some(left) = root.left() {
                if let Some(right) = root.right() {
                    let min = find_min(&right);
                    root.set_value(min.value());
                    delete(&mut right, min.value());
                } else {
                    tree.remove(root);
                }
            } else {
                if let Some(right) = root.right() {
                    tree.replace(root, right);
                } else {
                    tree.remove(root);
                }
            }
        }
    }
}

fn retrieve<T: Ord>(node: Node<T>) -> T {
    node.value()
}

fn print_in_order<T: Ord>(tree: &Tree<T>) {
    if let Some(root) = tree.root() {
        print_in_order(&root.left());
        println!("{}", root.value());
        print_in_order(&root.right());
    }
}

fn delete_even_numbers<T: Ord + Copy>(tree: &mut Tree<T>) {
    if let Some(root) = tree.root() {
        delete_even_numbers(&mut root.left());
        delete_even_numbers(&mut root.right());
        if root.value() % 2 == 0 {
            delete(tree, root.value());
        }
    }
}

fn main() {
    let mut tree = Tree::new();
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    for line in input.lines() {
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                tree.insert(num);
            }
        }
    }
    print_in_order(&tree);
    delete_even_numbers(&mut tree);
    print_in_order(&tree);
}