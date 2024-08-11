use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    element: i32,
    left: Link,
    right: Link,
}

#[derive(Debug)]
struct Tree {
    root: Link,
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }

    fn make_empty(&mut self) {
        self.root = None;
    }

    fn find(&self, x: i32) -> Link {
        let mut current = self.root.clone();
        while let Some(node) = current {
            if x < node.borrow().element {
                current = node.borrow_mut().left.take();
            } else if x > node.borrow().element {
                current = node.borrow_mut().right.take();
            } else {
                return Some(node);
            }
        }
        None
    }

    fn find_min(&self) -> Link {
        let mut current = self.root.clone();
        while let Some(node) = current {
            if node.borrow().left.is_none() {
                return Some(node);
            }
            current = node.borrow_mut().left.take();
        }
        None
    }

    fn find_max(&self) -> Link {
        let mut current = self.root.clone();
        while let Some(node) = current {
            if node.borrow().right.is_none() {
                return Some(node);
            }
            current = node.borrow_mut().right.take();
        }
        None
    }

    fn insert(&mut self, x: i32) {
        let mut current = self.root.clone();
        if current.is_none() {
            self.root = Some(Rc::new(RefCell::new(Node {
                element: x,
                left: None,
                right: None,
            })));
        } else {
            while let Some(node) = current {
                if x < node.borrow().element {
                    if node.borrow().left.is_none() {
                        node.borrow_mut().left = Some(Rc::new(RefCell::new(Node {
                            element: x,
                            left: None,
                            right: None,
                        })));
                        return;
                    }
                    current = node.borrow_mut().left.take();
                } else {
                    if node.borrow().right.is_none() {
                        node.borrow_mut().right = Some(Rc::new(RefCell::new(Node {
                            element: x,
                            left: None,
                            right: None,
                        })));
                        return;
                    }
                    current = node.borrow_mut().right.take();
                }
            }
        }
    }

    fn delete(&mut self, x: i32) {
        let mut current = self.root.clone();
        let mut parent = None;
        while let Some(node) = current {
            if x < node.borrow().element {
                parent = Some(node);
                current = node.borrow_mut().left.take();
            } else if x > node.borrow().element {
                parent = Some(node);
                current = node.borrow_mut().right.take();
            } else {
                if let Some(left) = node.borrow_mut().left.take() {
                    if let Some(right) = node.borrow_mut().right.take() {
                        let mut min_right = right.borrow_mut().left.take();
                        if min_right.is_none() {
                            node.borrow_mut().element = right.borrow().element;
                            node.borrow_mut().right = Some(right);
                        } else {
                            loop {
                                parent = Some(Rc::clone(&min_right.unwrap()));
                                if let Some(next) = min_right.unwrap().borrow_mut().left.take() {
                                    min_right = Some(next);
                                } else {
                                    break;
                                }
                            }
                            node.borrow_mut().element = min_right.unwrap().borrow().element;
                            node.borrow_mut().right = Some(Rc::clone(&min_right.unwrap()));
                            min_right.unwrap().borrow_mut().left = Some(right);
                        }
                    } else {
                        node.borrow_mut().left = Some(left);
                    }
                } else if let Some(right) = node.borrow_mut().right.take() {
                    node.borrow_mut().right = Some(right);
                } else {
                    if let Some(p) = parent {
                        if x < p.borrow().element {
                            p.borrow_mut().left = None;
                        } else {
                            p.borrow_mut().right = None;
                        }
                    } else {
                        self.root = None;
                    }
                    return;
                }
            }
        }
    }

    fn retrieve(&self, p: &Link) -> i32 {
        p.as_ref().unwrap().borrow().element
    }
}

fn main() {
    let mut tree = Tree::new();
    for i in 0..50 {
        tree.insert(i);
    }

    for i in 0..50 {
        if tree.find(i).is_none() || tree.retrieve(&tree.find(i).unwrap()) != i {
            println!("Error at {}", i);
        }
    }

    for i in 0..50 {
        if i % 2 == 0 {
            tree.delete(i);
        }
    }

    for i in 1..50 {
        if i % 2 == 1 {
            if tree.find(i).is_none() || tree.retrieve(&tree.find(i).unwrap()) != i {
                println!("Error at {}", i);
            }
        } else {
            if tree.find(i).is_some() {
                println!("Error at {}", i);
            }
        }
    }

    println!("Min is {}, Max is {}", tree.retrieve(&tree.find_min().unwrap()), tree.retrieve(&tree.find_max().unwrap()));
}