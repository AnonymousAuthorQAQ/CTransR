use std::cmp::Ord;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

struct SearchTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Copy + Debug> SearchTree<T> {
    fn new() -> Self {
        SearchTree { root: None }
    }

    fn make_empty(&mut self) {
        self.root = None;
    }

    fn find(&self, x: T) -> Option<&TreeNode<T>> {
        self._find(x, &self.root)
    }

    fn _find(&self, x: T, node: &Option<Box<TreeNode<T>>>) -> Option<&TreeNode<T>> {
        match node {
            Some(n) => {
                if x < n.element {
                    self._find(x, &n.left)
                } else if x > n.element {
                    self._find(x, &n.right)
                } else {
                    Some(n)
                }
            }
            None => None,
        }
    }

    fn find_min(&self) -> Option<T> {
        self._find_min(&self.root)
    }

    fn _find_min(&self, node: &Option<Box<TreeNode<T>>>) -> Option<T> {
        match node {
            Some(n) => {
                if n.left.is_none() {
                    Some(n.element)
                } else {
                    self._find_min(&n.left)
                }
            }
            None => None,
        }
    }

    fn find_max(&self) -> Option<T> {
        self._find_max(&self.root)
    }

    fn _find_max(&self, node: &Option<Box<TreeNode<T>>>) -> Option<T> {
        let mut current = node;
        while let Some(n) = current {
            if n.right.is_none() {
                return Some(n.element);
            }
            current = &n.right;
        }
        None
    }

    fn insert(&mut self, x: T) {
        self.root = self._insert(x, self.root.take());
    }

    fn _insert(&mut self, x: T, node: Option<Box<TreeNode<T>>>) -> Option<Box<TreeNode<T>>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = self._insert(x, n.left);
                } else if x > n.element {
                    n.right = self._insert(x, n.right);
                }
                Some(n)
            }
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
        }
    }

    fn delete(&mut self, x: T) {
        self.root = self._delete(x, self.root.take());
    }

    fn _delete(&mut self, x: T, node: Option<Box<TreeNode<T>>>) -> Option<Box<TreeNode<T>>> {
        match node {
            Some(mut n) => {
                if x < n.element {
                    n.left = self._delete(x, n.left);
                    Some(n)
                } else if x > n.element {
                    n.right = self._delete(x, n.right);
                    Some(n)
                } else {
                    // Node with only one child or no child
                    if n.left.is_none() {
                        return n.right;
                    } else if n.right.is_none() {
                        return n.left;
                    }

                    // Node with two children, get the inorder successor (smallest in the right subtree)
                    let min = self._find_min(&n.right).unwrap();
                    n.element = min;
                    n.right = self._delete(min, n.right);
                    Some(n)
                }
            }
            None => {
                panic!("Element not found");
            }
        }
    }
}

fn main() {
    let mut t = SearchTree::new();

    for i in 0..50 {
        let j = (i * 7) % 50;
        t.insert(j);
    }

    for i in 0..50 {
        match t.find(i) {
            Some(node) if node.element != i => {
                println!("Error at {}", i);
            },
            None => {
                println!("Error at {}", i);
            },
            _ => {}
        }
    }

    for i in (0..50).step_by(2) {
        t.delete(i);
    }

    for i in (1..50).step_by(2) {
        match t.find(i) {
            Some(node) if node.element != i => {
                println!("Error at {}", i);
            },
            None => {
                println!("Error at {}", i);
            },
            _ => {}
        }
    }

    for i in (0..50).step_by(2) {
        if t.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    let min = t.find_min().unwrap();
    let max = t.find_max().unwrap();
    println!("Min is {:?}, Max is {:?}", min, max);
}