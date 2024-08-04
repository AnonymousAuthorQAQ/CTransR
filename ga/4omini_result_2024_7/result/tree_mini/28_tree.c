use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

struct SearchTree {
    root: Option<Box<TreeNode>>,
}

impl SearchTree {
    fn new() -> Self {
        SearchTree { root: None }
    }

    fn make_empty(&mut self) {
        self.root = None;
    }

    fn find(&self, x: ElementType) -> Option<&TreeNode> {
        self.find_node(x, &self.root)
    }

    fn find_node<'a>(&'a self, x: ElementType, node: &'a Option<Box<TreeNode>>) -> Option<&'a TreeNode> {
        match node {
            Some(n) => {
                if x < n.element {
                    self.find_node(x, &n.left)
                } else if x > n.element {
                    self.find_node(x, &n.right)
                } else {
                    Some(n)
                }
            }
            None => None,
        }
    }

    fn find_min(&self) -> Option<&TreeNode> {
        self.find_min_node(&self.root)
    }

    fn find_min_node<'a>(&'a self, node: &'a Option<Box<TreeNode>>) -> Option<&'a TreeNode> {
        match node {
            Some(n) => {
                if n.left.is_none() {
                    Some(n)
                } else {
                    self.find_min_node(&n.left)
                }
            }
            None => None,
        }
    }

    fn find_max(&self) -> Option<&TreeNode> {
        self.find_max_node(&self.root)
    }

    fn find_max_node<'a>(&'a self, node: &'a Option<Box<TreeNode>>) -> Option<&'a TreeNode> {
        let mut current = node;
        while let Some(n) = current {
            if n.right.is_none() {
                return Some(n);
            }
            current = &n.right;
        }
        None
    }

    fn insert(&mut self, x: ElementType) {
        self.root = Self::insert_node(x, self.root.take());
    }

    fn insert_node(node: ElementType, tree: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match tree {
            None => Some(Box::new(TreeNode {
                element: node,
                left: None,
                right: None,
            })),
            Some(mut n) => {
                if node < n.element {
                    n.left = Self::insert_node(node, n.left);
                } else if node > n.element {
                    n.right = Self::insert_node(node, n.right);
                }
                Some(n)
            }
        }
    }

    fn delete(&mut self, x: ElementType) {
        self.root = Self::delete_node(x, self.root.take());
    }

    fn delete_node(tree: ElementType, node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            None => {
                eprintln!("Element not found");
                None
            }
            Some(mut n) => {
                if tree < n.element {
                    n.left = Self::delete_node(tree, n.left);
                } else if tree > n.element {
                    n.right = Self::delete_node(tree, n.right);
                } else {
                    match (n.left.take(), n.right.take()) {
                        (None, None) => return None,
                        (Some(left), None) => return Some(left),
                        (None, Some(right)) => return Some(right),
                        (Some(left), Some(right)) => {
                            let min_right = Self::find_min_node(&Some(right.clone()));
                            n.element = min_right.unwrap().element; // Replace with min of right subtree
                            n.right = Self::delete_node(n.element, Some(right));
                        }
                    }
                }
                Some(n)
            }
        }
    }

    fn retrieve(&self, node: &Option<Box<TreeNode>>) -> Option<ElementType> {
        node.as_ref().map(|n| n.element)
    }
}

fn main() {
    let mut tree = SearchTree::new();

    let mut j = 0;
    for i in 0..50 {
        j = (j + 7) % 50;
        tree.insert(j);
    }

    for i in 0..50 {
        match tree.find(i) {
            Some(node) if node.element != i => {
                println!("Error at {}", i);
            }
            None => {
                println!("Error at {}", i);
            }
            _ => {}
        }
    }

    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    for i in (1..50).step_by(2) {
        match tree.find(i) {
            Some(node) if node.element != i => {
                println!("Error at {}", i);
            }
            None => {
                println!("Error at {}", i);
            }
            _ => {}
        }
    }

    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    if let (Some(min), Some(max)) = (tree.find_min(), tree.find_max()) {
        println!("Min is {}, Max is {}", min.element, max.element);
    }
}