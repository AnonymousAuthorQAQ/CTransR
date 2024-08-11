use std::cmp::Ord;

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

struct SearchTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Copy> SearchTree<T> {
    fn new() -> Self {
        SearchTree { root: None }
    }

    fn make_empty(&mut self) {
        self.root = None;
    }

    fn insert(&mut self, x: T) {
        self.root = Self::insert_node(self.root.take(), x);
    }

    fn insert_node(node: Option<Box<TreeNode<T>>>, x: T) -> Option<Box<TreeNode<T>>> {
        match node {
            None => {
                Some(Box::new(TreeNode {
                    element: x,
                    left: None,
                    right: None,
                }))
            }
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_node(n.left, x);
                } else if x > n.element {
                    n.right = Self::insert_node(n.right, x);
                }
                Some(n)
            }
        }
    }

    fn find(&self, x: T) -> Option<&TreeNode<T>> {
        Self::find_node(&self.root, x)
    }

    fn find_node(node: &Option<Box<TreeNode<T>>>, x: T) -> Option<&TreeNode<T>> {
        match node {
            None => None,
            Some(n) => {
                if x < n.element {
                    Self::find_node(&n.left, x)
                } else if x > n.element {
                    Self::find_node(&n.right, x)
                } else {
                    Some(n)
                }
            }
        }
    }

    fn find_min(&self) -> Option<&TreeNode<T>> {
        Self::find_min_node(&self.root)
    }

    fn find_min_node(node: &Option<Box<TreeNode<T>>>) -> Option<&TreeNode<T>> {
        match node {
            None => None,
            Some(n) => {
                if n.left.is_none() {
                    Some(n)
                } else {
                    Self::find_min_node(&n.left)
                }
            }
        }
    }

    fn find_max(&self) -> Option<&TreeNode<T>> {
        Self::find_max_node(&self.root)
    }

    fn find_max_node(node: &Option<Box<TreeNode<T>>>) -> Option<&TreeNode<T>> {
        match node {
            None => None,
            Some(n) => {
                if n.right.is_none() {
                    Some(n)
                } else {
                    Self::find_max_node(&n.right)
                }
            }
        }
    }

    fn delete(&mut self, x: T) {
        self.root = Self::delete_node(self.root.take(), x);
    }

    fn delete_node(node: Option<Box<TreeNode<T>>>, x: T) -> Option<Box<TreeNode<T>>> {
        match node {
            None => {
                println!("Error: Element not found");
                None
            }
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete_node(n.left, x);
                } else if x > n.element {
                    n.right = Self::delete_node(n.right, x);
                } else {
                    // Node with two children
                    if n.left.is_some() && n.right.is_some() {
                        let min_node = Self::find_min_node(&n.right);
                        if let Some(min_node) = min_node {
                            n.element = min_node.element;
                            n.right = Self::delete_node(n.right, min_node.element);
                        }
                    } else {
                        return if n.left.is_none() {
                            n.right
                        } else {
                            n.left
                        };
                    }
                }
                Some(n)
            }
        }
    }

    fn retrieve(node: &Option<Box<TreeNode<T>>>) -> Option<T> {
        node.as_ref().map(|n| n.element)
    }
}

fn main() {
    let mut tree = SearchTree::new();

    for i in 0..50 {
        let j = (i * 7) % 50; // Generating the same sequence as in the C code.
        tree.insert(j);
    }

    for i in 0..50 {
        if let Some(node) = tree.find(i) {
            assert_eq!(tree.retrieve(&Some(node)), Some(i));
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    for i in (1..50).step_by(2) {
        if let Some(node) = tree.find(i) {
            assert_eq!(tree.retrieve(&Some(node)), Some(i));
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    if let Some(min_node) = tree.find_min() {
        if let Some(max_node) = tree.find_max() {
            println!(
                "Min is {}, Max is {}",
                min_node.element, max_node.element
            );
        }
    }
}