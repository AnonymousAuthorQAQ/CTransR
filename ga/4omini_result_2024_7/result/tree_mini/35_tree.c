use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
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

    fn insert(&mut self, x: ElementType) {
        self.root = Self::insert_node(self.root.take(), x);
    }

    fn insert_node(node: Option<Rc<RefCell<TreeNode>>>, x: ElementType) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            None => {
                let new_node = Rc::new(RefCell::new(TreeNode {
                    element: x,
                    left: None,
                    right: None,
                }));
                Some(new_node)
            }
            Some(n) => {
                let mut n_borrowed = n.borrow_mut();
                if x < n_borrowed.element {
                    n_borrowed.left = Self::insert_node(n_borrowed.left.clone(), x);
                } else if x > n_borrowed.element {
                    n_borrowed.right = Self::insert_node(n_borrowed.right.clone(), x);
                }
                Some(n)
            }
        }
    }

    fn find(&self, x: ElementType) -> Option<Rc<RefCell<TreeNode>>> {
        Self::find_node(&self.root, x)
    }

    fn find_node(node: &Option<Rc<RefCell<TreeNode>>>, x: ElementType) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            None => None,
            Some(n) => {
                let n_borrowed = n.borrow();
                if x < n_borrowed.element {
                    Self::find_node(&n_borrowed.left, x)
                } else if x > n_borrowed.element {
                    Self::find_node(&n_borrowed.right, x)
                } else {
                    Some(n.clone())
                }
            }
        }
    }

    fn find_min(&self) -> Option<Rc<RefCell<TreeNode>>> {
        Self::find_min_node(&self.root)
    }

    fn find_min_node(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            None => None,
            Some(n) => {
                let mut current = n.clone();
                while let Some(ref next) = current.borrow().left {
                    current = next.clone();
                }
                Some(current)
            }
        }
    }

    fn find_max(&self) -> Option<Rc<RefCell<TreeNode>>> {
        Self::find_max_node(&self.root)
    }

    fn find_max_node(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            None => None,
            Some(n) => {
                let mut current = n.clone();
                while let Some(ref next) = current.borrow().right {
                    current = next.clone();
                }
                Some(current)
            }
        }
    }

    fn delete(&mut self, x: ElementType) {
        self.root = Self::delete_node(self.root.take(), x);
    }

    fn delete_node(node: Option<Rc<RefCell<TreeNode>>>, x: ElementType) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            None => {
                println!("Error: Element not found");
                None
            }
            Some(n) => {
                let mut n_borrowed = n.borrow_mut();
                if x < n_borrowed.element {
                    n_borrowed.left = Self::delete_node(n_borrowed.left.clone(), x);
                } else if x > n_borrowed.element {
                    n_borrowed.right = Self::delete_node(n_borrowed.right.clone(), x);
                } else {
                    // Node to delete found
                    if n_borrowed.left.is_some() && n_borrowed.right.is_some() {
                        let min_node = Self::find_min_node(&n_borrowed.right).unwrap();
                        n_borrowed.element = min_node.borrow().element;
                        n_borrowed.right = Self::delete_node(n_borrowed.right.clone(), n_borrowed.element);
                    } else {
                        // One child or no child
                        let tmp = if n_borrowed.left.is_none() {
                            n_borrowed.right.take()
                        } else {
                            n_borrowed.left.take()
                        };
                        return tmp;
                    }
                }
                Some(n)
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
        j = (j + 7) % 50;
        tree.insert(j);
    }

    for i in 0..50 {
        if let Some(p) = tree.find(i) {
            if SearchTree::retrieve(&Some(p)) != Some(i) {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    for i in (1..50).step_by(2) {
        if let Some(p) = tree.find(i) {
            if SearchTree::retrieve(&Some(p)) != Some(i) {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }
    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    if let Some(min) = tree.find_min() {
        if let Some(max) = tree.find_max() {
            println!("Min is {}, Max is {}", SearchTree::retrieve(&Some(min)), SearchTree::retrieve(&Some(max)));
        }
    }
}