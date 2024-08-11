use std::cmp::Ordering;
use std::io;

#[derive(Debug)]
struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

struct BST {
    root: Option<Box<TreeNode>>,
}

impl BST {
    fn new() -> Self {
        BST { root: None }
    }

    fn insert(&mut self, x: i32) {
        self.root = Self::insert_into(self.root.take(), x);
    }

    fn insert_into(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                match x.cmp(&n.element) {
                    Ordering::Less => {
                        n.left = Self::insert_into(n.left, x);
                    }
                    Ordering::Greater => {
                        n.right = Self::insert_into(n.right, x);
                    }
                    Ordering::Equal => {}
                }
                Some(n)
            }
            None => {
                Some(Box::new(TreeNode {
                    element: x,
                    left: None,
                    right: None,
                }))
            }
        }
    }

    fn find(&self, x: i32) -> Option<&TreeNode> {
        Self::find_in(&self.root, x)
    }

    fn find_in(node: &Option<Box<TreeNode>>, x: i32) -> Option<&TreeNode> {
        match node {
            Some(n) => {
                match x.cmp(&n.element) {
                    Ordering::Less => Self::find_in(&n.left, x),
                    Ordering::Greater => Self::find_in(&n.right, x),
                    Ordering::Equal => Some(n),
                }
            }
            None => None,
        }
    }

    fn delete(&mut self, x: i32) {
        self.root = Self::delete_from(self.root.take(), x);
    }

    fn delete_from(node: Option<Box<TreeNode>>, x: i32) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                match x.cmp(&n.element) {
                    Ordering::Less => {
                        n.left = Self::delete_from(n.left, x);
                        Some(n)
                    }
                    Ordering::Greater => {
                        n.right = Self::delete_from(n.right, x);
                        Some(n)
                    }
                    Ordering::Equal => {
                        match (n.left, n.right) {
                            (None, None) => None,  // No children
                            (Some(l), None) => Some(l),  // Only left child
                            (None, Some(r)) => Some(r),  // Only right child
                            (Some(l), Some(r)) => {
                                let min_node = Self::find_min(r);
                                n.element = min_node.element;
                                n.right = Self::delete_from(n.right, min_node.element);
                                Some(n)
                            }
                        }
                    }
                }
            }
            None => None,
        }
    }

    fn find_min(&self) -> Option<&TreeNode> {
        Self::find_min_node(&self.root)
    }

    fn find_min_node(node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            Some(n) => {
                Self::find_min_node(&n.left).unwrap_or_else(|| n.as_ref())
            }
            None => None,
        }
    }

    fn find_max(&self) -> Option<&TreeNode> {
        Self::find_max_node(&self.root)
    }

    fn find_max_node(node: &Option<Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            Some(n) => Self::find_max_node(&n.right).unwrap_or_else(|| n.as_ref()),
            None => None,
        }
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

fn main() {
    let mut bst = BST::new();

    // Insert values into the BST
    let mut j = 0;
    for i in 0..50 {
        j = (j + 7) % 50;
        bst.insert(j);
    }

    // Validate all integers from 0 to 49 are present
    for i in 0..50 {
        if bst.find(i).is_none() {
            println!("Error at {}", i);
        }
    }

    // Delete even integers
    for i in (0..50).step_by(2) {
        bst.delete(i);
    }

    // Validate odd integers remain
    for i in (1..50).step_by(2) {
        if bst.find(i).is_none() {
            println!("Error at {}", i);
        }
    }
    for i in (0..50).step_by(2) {
        if bst.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    // Display min and max
    if let (Some(min), Some(max)) = (bst.find_min(), bst.find_max()) {
        println!("Min is {}, Max is {}", min.element, max.element);
    } else {
        println!("Tree is empty.");
    }
}