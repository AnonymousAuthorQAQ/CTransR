use std::cmp::Ord;
use std::fmt;

// Define an error type for the binary search tree operations
#[derive(Debug)]
enum BSTError {
    ElementNotFound,
    OutOfMemory,
}

// Implement the Display trait for custom error messages
impl fmt::Display for BSTError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

type ElementType = i32; // Define the type for the elements in the tree

// Define a struct for a tree node
#[derive(Debug)]
struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>, // Using Option and Box for dynamic memory management
    right: Option<Box<TreeNode>>,
}

// Define a struct for the binary search tree
#[derive(Debug)]
struct SearchTree {
    root: Option<Box<TreeNode>>,
}

impl SearchTree {
    // Create a new empty tree
    fn new() -> Self {
        SearchTree { root: None }
    }

    // Insert an element into the tree
    fn insert(&mut self, x: ElementType) {
        self.root = Self::insert_recursive(x, self.root.take());
    }

    fn insert_recursive(x: ElementType, node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            None => Some(Box::new(TreeNode {
                element: x,
                left: None,
                right: None,
            })),
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::insert_recursive(x, n.left);
                } else if x > n.element {
                    n.right = Self::insert_recursive(x, n.right);
                }
                Some(n)
            }
        }
    }

    // Find an element in the tree
    fn find(&self, x: ElementType) -> Option<&TreeNode> {
        Self::find_recursive(x, self.root.as_deref())
    }

    fn find_recursive(x: ElementType, node: Option<&TreeNode>) -> Option<&TreeNode> {
        match node {
            None => None,
            Some(n) => {
                if x < n.element {
                    Self::find_recursive(x, n.left.as_deref())
                } else if x > n.element {
                    Self::find_recursive(x, n.right.as_deref())
                } else {
                    Some(n)
                }
            }
        }
    }

    // Find the minimum element
    fn find_min(&self) -> Option<&TreeNode> {
        self.root.as_deref().and_then(Self::find_min_recursive)
    }

    fn find_min_recursive(node: &TreeNode) -> Option<&TreeNode> {
        match node.left {
            None => Some(node),
            Some(ref left_node) => Self::find_min_recursive(left_node)
        }
    }

    // Find the maximum element
    fn find_max(&self) -> Option<&TreeNode> {
        self.root.as_deref().and_then(Self::find_max_recursive)
    }

    fn find_max_recursive(node: &TreeNode) -> Option<&TreeNode> {
        match node.right {
            None => Some(node),
            Some(ref right_node) => Self::find_max_recursive(right_node)
        }
    }

    // Delete an element
    fn delete(&mut self, x: ElementType) -> Result<(), BSTError> {
        self.root = Self::delete_recursive(x, self.root.take()).map(Box::new);
        Ok(())
    }

    fn delete_recursive(x: ElementType, node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            None => return None,
            Some(mut n) => {
                if x < n.element {
                    n.left = Self::delete_recursive(x, n.left);
                } else if x > n.element {
                    n.right = Self::delete_recursive(x, n.right);
                } else { // Found the node to delete
                    if n.left.is_some() && n.right.is_some() {
                        // Node with two children
                        let min_right = Self::find_min_recursive(n.right.as_deref().unwrap());
                        n.element = min_right.unwrap().element;
                        n.right = Self::delete_recursive(n.element, n.right);
                    } else {
                        // Node with one or no children
                        return if n.left.is_some() {
                            n.left
                        } else {
                            n.right
                        };
                    }
                }
                Some(n)
            }
        }
    }
}

fn main() {
    let mut tree = SearchTree::new();

    // Insert elements
    for i in 0..50 {
        let j = (i * 7) % 50;
        tree.insert(j);
    }

    // Check if each number from 0 to 49 can be found
    for i in 0..50 {
        if tree.find(i).is_none() {
            println!("Error at {}", i);
        }
    }

    // Delete even numbers
    for i in (0..50).step_by(2) {
        tree.delete(i).unwrap();
    }

    // Check if odd numbers can be found
    for i in (1..50).step_by(2) {
        if tree.find(i).is_none() {
            println!("Error at {}", i);
        }
    }

    // Check if even numbers cannot be found
    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    // Print minimum and maximum elements
    if let Some(min) = tree.find_min() {
        println!("Min is {}", min.element);
    }
    if let Some(max) = tree.find_max() {
        println!("Max is {}", max.element);
    }
}