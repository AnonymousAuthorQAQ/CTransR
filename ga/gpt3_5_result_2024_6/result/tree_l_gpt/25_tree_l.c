use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

type SearchTree = Option<Box<TreeNode>>;

fn make_empty(mut t: SearchTree) -> SearchTree {
    if let Some(mut tree) = t {
        tree.left = make_empty(tree.left.take());
        tree.right = make_empty(tree.right.take());
    }
    t
}

fn find(x: i32, t: &SearchTree) -> Option<&TreeNode> {
    if let Some(ref tree) = t {
        match x.cmp(&tree.element) {
            Ordering::Less => find(x, &tree.left),
            Ordering::Greater => find(x, &tree.right),
            Ordering::Equal => Some(&**tree),
        }
    } else {
        None
    }
}

fn find_min(t: &SearchTree) -> Option<&TreeNode> {
    t.as_ref().and_then(|tree| tree.left.as_ref().map_or(Some(&**tree), |left| find_min(&Some(left))))
}

fn find_max(t: &SearchTree) -> Option<&TreeNode> {
    t.as_ref().and_then(|tree| tree.right.as_ref().map_or(Some(&**tree), |right| find_max(&Some(right))))
}

fn insert(x: i32, mut t: SearchTree) -> SearchTree {
    match t {
        None => {
            t = Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            }));
        }
        Some(ref mut tree) => {
            match x.cmp(&tree.element) {
                Ordering::Less => tree.left = insert(x, tree.left.take()),
                Ordering::Greater => tree.right = insert(x, tree.right.take()),
                Ordering::Equal => (), // duplicate, do nothing
            }
        }
    }
    t
}

fn delete(mut t: SearchTree, x: i32) -> SearchTree {
    if let Some(mut tree) = t.take() {
        match x.cmp(&tree.element) {
            Ordering::Less => tree.left = delete(tree.left.take(), x),
            Ordering::Greater => tree.right = delete(tree.right.take(), x),
            Ordering::Equal => {
                if let left = tree.left.take() {
                    let right = tree.right.take();
                    tree = *find_min(&right).unwrap();
                    tree.right = right;
                    tree.left = Some(left);
                } else {
                    tree = *tree.right.unwrap();
                }
            }
        }
        t = Some(tree);
    }
    t
}

fn retrieve(p: &TreeNode) -> i32 {
    p.element
}

fn print_in_order(t: &SearchTree) {
    if let Some(ref tree) = t {
        print_in_order(&tree.left);
        println!("{}", tree.element);
        print_in_order(&tree.right);
    }
}

fn delete_even_numbers(t: SearchTree) -> SearchTree {
    if let Some(tree) = &t {
        let mut new_tree = Some(Box::new(TreeNode {
            element: tree.element,
            left: delete_even_numbers(tree.left.take()),
            right: delete_even_numbers(tree.right.take()),
        });

        if new_tree.as_ref().unwrap().element % 2 == 0 {
            new_tree = delete(new_tree, new_tree.as_ref().unwrap().element);
        }

        new_tree
    } else {
        None
    }
}

fn main() {
    let mut t: SearchTree = None;
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    for token in input.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            t = insert(num, t);
        }
    }
    
    if let Some(tree) = &t {
        println!("Tree after insertion:");
        print_in_order(&t);

        t = delete_even_numbers(t);

        println!("Tree after deleting even numbers:");
        print_in_order(&t);

        if let Some(min_node) = find_min(&t) {
            if let Some(max_node) = find_max(&t) {
                println!("Min is {}, Max is {}", retrieve(min_node), retrieve(max_node));
            }
        }
    } else {
        println!("Tree is empty");
    }

    t = make_empty(t);
}