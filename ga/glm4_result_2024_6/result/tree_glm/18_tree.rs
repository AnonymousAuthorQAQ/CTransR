use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

type SearchResult = Option<Rc<RefCell<TreeNode>>>;

#[derive(Clone)]
struct TreeNode {
    element: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(element: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            element,
            left: None,
            right: None,
        }))
    }

    fn insert(&mut self, element: i32) {
        if element < self.borrow().element {
            if let Some(left) = &mut self.borrow_mut().left {
                left.insert(element);
            } else {
                self.borrow_mut().left = Some(TreeNode::new(element));
            }
        } else if element > self.borrow().element {
            if let Some(right) = &mut self.borrow_mut().right {
                right.insert(element);
            } else {
                self.borrow_mut().right = Some(TreeNode::new(element));
            }
        }
    }

    fn find(&self, element: i32) -> SearchResult {
        if element == self.borrow().element {
            Some(self.clone())
        } else if element < self.borrow().element {
            self.borrow().left.as_ref().and_then(|left| left.find(element))
        } else {
            self.borrow().right.as_ref().and_then(|right| right.find(element))
        }
    }

    fn find_min(&self) -> SearchResult {
        if let Some(left) = &self.borrow().left {
            left.find_min()
        } else {
            Some(self.clone())
        }
    }

    fn find_max(&self) -> SearchResult {
        if let Some(right) = &self.borrow().right {
            right.find_max()
        } else {
            Some(self.clone())
        }
    }

    fn delete(&mut self, element: i32) {
        if element < self.borrow().element {
            if let Some(left) = &mut self.borrow_mut().left {
                left.delete(element);
            }
        } else if element > self.borrow().element {
            if let Some(right) = &mut self.borrow_mut().right {
                right.delete(element);
            }
        } else {
            if let Some(left) = self.borrow_mut().left.take() {
                if let Some(right) = self.borrow_mut().right.take() {
                    let min_right = right.find_min().unwrap();
                    self.replace(min_right.borrow().clone());
                    self.borrow_mut().right = Some(right);
                    self.borrow_mut().left = left;
                    self.borrow_mut().delete(min_right.borrow().element);
                } else {
                    self.replace(left.borrow().clone());
                }
            } else if let Some(right) = self.borrow_mut().right.take() {
                self.replace(right.borrow().clone());
            } else {
                self.take();
            }
        }
    }

    fn retrieve(&self) -> i32 {
        self.borrow().element
    }
}

fn main() {
    let mut root: Option<Rc<RefCell<TreeNode>>> = None;
    for i in 0..50 {
        if let Some(root) = &mut root {
            root.borrow_mut().insert(i);
        } else {
            root = Some(TreeNode::new(i));
        }
    }

    for i in 0..50 {
        if let Some(node) = root.as_ref().and_then(|r| r.find(i)) {
            if node.borrow().retrieve() != i {
                println!("Error at {}", i);
            }
        } else {
            println!("Error at {}", i);
        }
    }

    for i in (0..50).step_by(2) {
        if let Some(root) = &mut root {
            root.borrow_mut().delete(i);
        }
    }

    for i in 1..50 {
        if i % 2 == 0 {
            if let Some(node) = root.as_ref().and_then(|r| r.find(i)) {
                if node.borrow().retrieve() != i {
                    println!("Error at {}", i);
                }
            } else {
                println!("Error at {}", i);
            }
        } else {
            if root.as_ref().and_then(|r| r.find(i)).is_some() {
                println!("Error at {}", i);
            }
        }
    }

    if let Some(root) = &root {
        if let Some(min) = root.find_min() {
            println!("Min is {}, ", min.borrow().retrieve());
        }

        if let Some(max) = root.find_max() {
            println!("Max is {}", max.borrow().retrieve());
        }
    }
}