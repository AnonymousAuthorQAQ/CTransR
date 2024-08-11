use std::cmp::Ord;
use std::fmt;
use std::ptr;
use std::rc::Rc;
use std::cell::RefCell;

type ElementType = i32;

#[derive(Debug)]
struct TreeNode {
    element: ElementType,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(element: ElementType) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
}

struct SearchTree {
    root: Option<Rc<RefCell<TreeNode>>>,
}

impl SearchTree {
    fn new() -> Self {
        SearchTree { root: None }
    }

    fn make_empty(&mut self) {
        self.root = None;
    }

    fn find(&self, x: ElementType) -> Option<Rc<RefCell<TreeNode>>> {
        Self::find_rec(&self.root, x)
    }

    fn find_rec(node: &Option<Rc<RefCell<TreeNode>>>, x: ElementType) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            Some(n) => {
                let n = n.borrow();
                if x < n.element {
                    Self::find_rec(&n.left, x)
                } else if x > n.element {
                    Self::find_rec(&n.right, x)
                } else {
                    Some(n.clone())
                }
            }
            None => None,
        }
    }

    fn find_min(&self) -> Option<Rc<RefCell<TreeNode>>> {
        Self::find_min_rec(&self.root)
    }

    fn find_min_rec(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            Some(n) => {
                let n = n.borrow();
                if n.left.is_none() {
                    Some(n.clone())
                } else {
                    Self::find_min_rec(&n.left)
                }
            }
            None => None,
        }
    }

    fn find_max(&self) -> Option<Rc<RefCell<TreeNode>>> {
        Self::find_max_rec(&self.root)
    }

    fn find_max_rec(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut current = node;
        while let Some(n) = current {
            let n = n.borrow();
            if n.right.is_none() {
                return Some(n.clone());
            }
            current = &n.right;
        }
        None
    }

    fn insert(&mut self, x: ElementType) {
        self.root = Self::insert_rec(self.root.take(), x);
    }

    fn insert_rec(node: Option<Rc<RefCell<TreeNode>>>, x: ElementType) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            Some(n) => {
                let mut n_borrowed = n.borrow_mut();
                if x < n_borrowed.element {
                    n_borrowed.left = Self::insert_rec(n_borrowed.left.take(), x);
                } else if x > n_borrowed.element {
                    n_borrowed.right = Self::insert_rec(n_borrowed.right.take(), x);
                }
                Some(n)
            }
            None => {
                Some(Rc::new(RefCell::new(TreeNode::new(x))))
            }
        }
    }

    fn delete(&mut self, x: ElementType) {
        self.root = Self::delete_rec(self.root.take(), x);
    }

    fn delete_rec(node: Option<Rc<RefCell<TreeNode>>>, x: ElementType) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            Some(n) => {
                let mut n_borrowed = n.borrow_mut();
                if x < n_borrowed.element {
                    n_borrowed.left = Self::delete_rec(n_borrowed.left.take(), x);
                } else if x > n_borrowed.element {
                    n_borrowed.right = Self::delete_rec(n_borrowed.right.take(), x);
                } else {
                    if n_borrowed.left.is_some() && n_borrowed.right.is_some() {
                        let min_node = Self::find_min_rec(&n_borrowed.right).unwrap();
                        n_borrowed.element = min_node.borrow().element;
                        n_borrowed.right = Self::delete_rec(n_borrowed.right.take(), n_borrowed.element);
                    } else {
                        return if n_borrowed.left.is_none() {
                            n_borrowed.right.take()
                        } else {
                            n_borrowed.left.take()
                        };
                    }
                }
                Some(n)
            }
            None => {
                println!("Error: Element not found");
                None
            }
        }
    }

    fn retrieve(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<ElementType> {
        node.as_ref().map(|n| n.borrow().element)
    }
}

fn main() {
    let mut tree = SearchTree::new();
    
    let mut j = 0;
    for i in 0..50 {
        tree.insert(j);
        j = (j + 7) % 50;
    }
    
    for i in 0..50 {
        if let Some(pos) = tree.find(i) {
            assert_eq!(SearchTree::retrieve(&Some(pos)), Some(i));
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    for i in 1..50 {
        if i % 2 != 0 {
            if let Some(pos) = tree.find(i) {
                assert_eq!(SearchTree::retrieve(&Some(pos)), Some(i));
            } else {
                println!("Error at {}", i);
            }
        }
    }
    
    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    if let Some(min_node) = tree.find_min() {
        let min_value = SearchTree::retrieve(&Some(min_node)).unwrap();
        if let Some(max_node) = tree.find_max() {
            let max_value = SearchTree::retrieve(&Some(max_node)).unwrap();
            println!("Min is {}, Max is {}", min_value, max_value);
        }
    }
}