use std::io;
use std::ptr;

#[derive(Debug)]
struct TreeNode {
    element: i32,
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

    fn find(&self, x: i32) -> Option<&TreeNode> {
        self.find_helper(x, self.root.as_deref())
    }

    fn find_helper(&self, x: i32, node: Option<&Box<TreeNode>>) -> Option<&TreeNode> {
        match node {
            Some(n) => {
                if x < n.element {
                    self.find_helper(x, n.left.as_deref())
                } else if x > n.element {
                    self.find_helper(x, n.right.as_deref())
                } else {
                    Some(n)
                }
            },
            None => None,
        }
    }

    fn find_min(&self) -> Option<i32> {
        self.find_min_helper(&self.root)
    }

    fn find_min_helper(&self, node: &Option<Box<TreeNode>>) -> Option<i32> {
        match node {
            Some(n) => {
                if n.left.is_none() {
                    Some(n.element)
                } else {
                    self.find_min_helper(&n.left)
                }
            },
            None => None,
        }
    }

    fn find_max(&self) -> Option<i32> {
        self.find_max_helper(&self.root)
    }

    fn find_max_helper(&self, node: &Option<Box<TreeNode>>) -> Option<i32> {
        match node {
            Some(n) => {
                if n.right.is_none() {
                    Some(n.element)
                } else {
                    self.find_max_helper(&n.right)
                }
            },
            None => None,
        }
    }

    fn insert(&mut self, x: i32) {
        self.root = self.insert_helper(x, self.root.take());
    }

    fn insert_helper(&self, x: i32, node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            None => {
                Some(Box::new(TreeNode {
                    element: x,
                    left: None,
                    right: None,
                }))
            },
            Some(mut n) => {
                if x < n.element {
                    n.left = self.insert_helper(x, n.left);
                } else if x > n.element {
                    n.right = self.insert_helper(x, n.right);
                }
                Some(n)
            }
        }
    }

    fn delete(&mut self, x: i32) {
        self.root = self.delete_helper(x, self.root.take());
    }

    fn delete_helper(&self, x: i32, node: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        match node {
            None => {
                println!("Error: Element not found");
                None
            },
            Some(mut n) => {
                if x < n.element {
                    n.left = self.delete_helper(x, n.left);
                } else if x > n.element {
                    n.right = self.delete_helper(x, n.right);
                } else {
                    // Node found
                    if n.left.is_some() && n.right.is_some() {
                        // Node with two children
                        let min_value = self.find_min_helper(&n.right);
                        n.element = min_value.unwrap_or(n.element); // Use existing value if none exists
                        n.right = self.delete_helper(n.element, n.right);
                    } else {
                        // Node with one or no child
                        return if n.left.is_none() { n.right } else { n.left };
                    }
                }
                Some(n)
            }
        }
    }
}

fn main() {
    let mut t = SearchTree::new();
    for i in 0..50 {
        let j = (i + 7) % 50;
        t.insert(j);
    }

    for i in 0..50 {
        if t.find(i).is_none() {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        t.delete(i);
    }

    for i in (1..50).step_by(2) {
        if t.find(i).is_none() {
            println!("Error at {}", i);
        }
    }
    
    for i in (0..50).step_by(2) {
        if t.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    if let (Some(min), Some(max)) = (t.find_min(), t.find_max()) {
        println!("Min is {}, Max is {}", min, max);
    }
}