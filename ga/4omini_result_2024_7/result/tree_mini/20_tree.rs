use std::ptr;
use std::cell::RefCell;
use std::rc::Rc;

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
        self.find_recursive(x, self.root.as_ref())
    }

    fn find_recursive(
        &self,
        x: ElementType,
        node: Option<&Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            Some(n) => {
                let n_ref = n.borrow();
                if x < n_ref.element {
                    self.find_recursive(x, n_ref.left.as_ref())
                } else if x > n_ref.element {
                    self.find_recursive(x, n_ref.right.as_ref())
                } else {
                    Some(n.clone())
                }
            }
            None => None,
        }
    }

    fn find_min(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.find_min_recursive(self.root.as_ref())
    }

    fn find_min_recursive(
        &self,
        node: Option<&Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            Some(n) => {
                let n_ref = n.borrow();
                if n_ref.left.is_none() {
                    Some(n.clone())
                } else {
                    self.find_min_recursive(n_ref.left.as_ref())
                }
            }
            None => None,
        }
    }

    fn find_max(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.find_max_recursive(self.root.as_ref())
    }

    fn find_max_recursive(
        &self,
        node: Option<&Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut current = node;
        while let Some(n) = current {
            let n_ref = n.borrow();
            if n_ref.right.is_some() {
                current = n_ref.right.as_ref();
            } else {
                break;
            }
        }
        current.map(|n| n.clone())
    }

    fn insert(&mut self, x: ElementType) {
        self.root = self.insert_recursive(x, self.root.clone());
    }

    fn insert_recursive(
        &self,
        x: ElementType,
        node: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            Some(n) => {
                let mut n_ref = n.borrow_mut();
                if x < n_ref.element {
                    n_ref.left = self.insert_recursive(x, n_ref.left.clone());
                } else if x > n_ref.element {
                    n_ref.right = self.insert_recursive(x, n_ref.right.clone());
                }
                Some(n.clone())
            }
            None => {
                let new_node = Rc::new(RefCell::new(TreeNode {
                    element: x,
                    left: None,
                    right: None,
                }));
                Some(new_node)
            }
        }
    }

    fn delete(&mut self, x: ElementType) {
        self.root = self.delete_recursive(x, self.root.clone());
    }

    fn delete_recursive(
        &self,
        x: ElementType,
        node: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            Some(n) => {
                let mut n_ref = n.borrow_mut();
                if x < n_ref.element {
                    n_ref.left = self.delete_recursive(x, n_ref.left.clone());
                    Some(n.clone())
                } else if x > n_ref.element {
                    n_ref.right = self.delete_recursive(x, n_ref.right.clone());
                    Some(n.clone())
                } else {
                    // Found node to delete
                    match (n_ref.left.as_ref(), n_ref.right.as_ref()) {
                        (None, None) => None,
                        (Some(_), None) => n_ref.left.take(),
                        (None, Some(_)) => n_ref.right.take(),
                        (Some(_), Some(_)) => {
                            if let Some(min_node) = self.find_min_recursive(n_ref.right.as_ref()) {
                                n_ref.element = min_node.borrow().element;
                                n_ref.right = self.delete_recursive(n_ref.element, n_ref.right.clone());
                            }
                            Some(n.clone())
                        }
                    }
                }
            }
            None => {
                println!("Error: Element not found");
                None
            }
        }
    }
}

fn retrieve(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<ElementType> {
    node.as_ref().map(|n| n.borrow().element)
}

fn main() {
    let mut tree = SearchTree::new();
    let mut j = 0;

    for i in 0..50 {
        j = (j + 7) % 50;
        tree.insert(j);
    }

    for i in 0..50 {
        let found = tree.find(i);
        if found.is_none() || retrieve(&found) != Some(i) {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        tree.delete(i);
    }

    for i in 1..50 {
        if i % 2 != 0 {
            let found = tree.find(i);
            if found.is_none() || retrieve(&found) != Some(i) {
                println!("Error at {}", i);
            }
        }
    }

    for i in (0..50).step_by(2) {
        if tree.find(i).is_some() {
            println!("Error at {}", i);
        }
    }

    if let (Some(min), Some(max)) = (tree.find_min(), tree.find_max()) {
        println!("Min is {}, Max is {}", retrieve(&Some(min)), retrieve(&Some(max)));
    }
}