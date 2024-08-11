use std::cmp::Ord;
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use std::option::Option;

type ElementType = i32;

#[derive(Debug)]
struct TreeNode {
    element: ElementType,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
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
        Self::find_helper(&self.root, x)
    }

    fn find_helper(node: &Option<Rc<RefCell<TreeNode>>>, x: ElementType) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            Some(n) => {
                let n_borrowed = n.borrow();
                if x < n_borrowed.element {
                    Self::find_helper(&n_borrowed.left, x)
                } else if x > n_borrowed.element {
                    Self::find_helper(&n_borrowed.right, x)
                } else {
                    Some(n.clone())
                }
            }
            None => None,
        }
    }

    fn find_min(&self) -> Option<Rc<RefCell<TreeNode>>> {
        Self::find_min_helper(&self.root)
    }

    fn find_min_helper(node: &Option<Rc<RefCell<TreeNode>>>()) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            Some(n) => {
                let n_borrowed = n.borrow();
                if n_borrowed.left.is_none() {
                    Some(n.clone())
                } else {
                    Self::find_min_helper(&n_borrowed.left)
                }
            }
            None => None,
        }
    }

    fn find_max(&self) -> Option<Rc<RefCell<TreeNode>>> {
        Self::find_max_helper(&self.root)
    }

    fn find_max_helper(node: &Option<Rc<RefCell<TreeNode>>>()) -> Option<Rc<RefCell<TreeNode>>> {
        let mut current = node;
        while let Some(n) = current {
            let n_borrowed = n.borrow();
            current = &n_borrowed.right;
        }
        current.map(|n| n.clone())
    }

    fn insert(&mut self, x: ElementType) {
        self.root = Self::insert_helper(self.root.clone(), x)
    }

    fn insert_helper(node: Option<Rc<RefCell<TreeNode>>>, x: ElementType) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = node {
            let mut n_borrowed = n.borrow_mut();
            if x < n_borrowed.element {
                n_borrowed.left = Self::insert_helper(n_borrowed.left.clone(), x);
            } else if x > n_borrowed.element {
                n_borrowed.right = Self::insert_helper(n_borrowed.right.clone(), x);
            }
            Some(n.clone())
        } else {
            let new_node = Rc::new(RefCell::new(TreeNode {
                element: x,
                left: None,
                right: None,
            }));
            Some(new_node)
        }
    }

    fn delete(&mut self, x: ElementType) {
        self.root = Self::delete_helper(self.root.clone(), x);
    }

    fn delete_helper(node: Option<Rc<RefCell<TreeNode>>>, x: ElementType) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = node {
            let mut n_borrowed = n.borrow_mut();
            if x < n_borrowed.element {
                n_borrowed.left = Self::delete_helper(n_borrowed.left.clone(), x);
            } else if x > n_borrowed.element {
                n_borrowed.right = Self::delete_helper(n_borrowed.right.clone(), x);
            } else {
                // Node found, now handle the different cases
                if n_borrowed.left.is_some() && n_borrowed.right.is_some() {
                    let min_right = Self::find_min_helper(&n_borrowed.right).unwrap();
                    n_borrowed.element = min_right.borrow().element;
                    n_borrowed.right = Self::delete_helper(n_borrowed.right.clone(), n_borrowed.element);
                } else {
                    let temp = n_borrowed.clone();
                    if n_borrowed.left.is_none() {
                        return n_borrowed.right.clone();
                    } else {
                        return n_borrowed.left.clone();
                    }
                }
            }
            Some(n.clone())
        } else {
            None
        }
    }

    fn retrieve(&self, node: &Option<Rc<RefCell<TreeNode>>>) -> Option<ElementType> {
        node.as_ref().map(|n| n.borrow().element)
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
        if let None = tree.find(i) {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    for i in (1..50).step_by(2) {
        if let None = tree.find(i) {
            println!("Error at {}", i);
        }
    }
    
    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    if let (Some(min), Some(max)) = (tree.find_min(), tree.find_max()) {
        println!("Min is {}, Max is {}", min.borrow().element, max.borrow().element);
    }
}