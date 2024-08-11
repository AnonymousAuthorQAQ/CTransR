use std::rc::Rc;
use std::cell::RefCell;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    element: i32,
    left: Link,
    right: Link,
}

impl Node {
    fn new(element: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            element,
            left: None,
            right: None,
        }))
    }
}

struct Tree {
    root: Link,
}

impl Tree {
    fn new() -> Tree {
        Tree { root: None }
    }

    fn make_empty(&mut self) {
        self.root.take();
    }

    fn find(&self, x: i32) -> Option<Rc<RefCell<Node>>> {
        self.root.as_ref().and_then(|root| {
            let mut current = root.clone();
            while let Some(node) = current.as_ref() {
                if x < node.borrow().element {
                    current = node.borrow_mut().left.take();
                } else if x > node.borrow().element {
                    current = node.borrow_mut().right.take();
                } else {
                    return Some(node.clone());
                }
            }
            None
        })
    }

    fn find_min(&self) -> Option<Rc<RefCell<Node>>> {
        self.root.as_ref().and_then(|root| {
            let mut current = root.clone();
            while let Some(node) = current.as_ref() {
                if node.borrow().left.is_none() {
                    return Some(node.clone());
                }
                current = node.borrow_mut().left.take();
            }
            None
        })
    }

    fn find_max(&self) -> Option<Rc<RefCell<Node>>> {
        self.root.as_ref().and_then(|root| {
            let mut current = root.clone();
            while let Some(node) = current.as_ref() {
                if node.borrow().right.is_none() {
                    return Some(node.clone());
                }
                current = node.borrow_mut().right.take();
            }
            None
        })
    }

    fn insert(&mut self, x: i32) {
        let new_node = Node::new(x);
        if let Some(root) = self.root.take() {
            self.root = Some(insert_into_tree(new_node, root));
        } else {
            self.root = Some(new_node);
        }
    }

    fn delete(&mut self, x: i32) {
        self.root = delete_from_tree(x, self.root.take());
    }
}

fn insert_into_tree(new_node: Rc<RefCell<Node>>, mut root: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    if new_node.borrow().element < root.borrow().element {
        if let Some(left) = root.borrow_mut().left.take() {
            root.borrow_mut().left = Some(insert_into_tree(new_node, left));
        } else {
            root.borrow_mut().left = Some(new_node);
        }
    } else {
        if let Some(right) = root.borrow_mut().right.take() {
            root.borrow_mut().right = Some(insert_into_tree(new_node, right));
        } else {
            root.borrow_mut().right = Some(new_node);
        }
    }
    root
}

fn delete_from_tree(x: i32, mut root: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    if let Some(node) = root.as_mut() {
        if x < node.borrow().element {
            node.borrow_mut().left = delete_from_tree(x, node.borrow_mut().left.take());
        } else if x > node.borrow().element {
            node.borrow_mut().right = delete_from_tree(x, node.borrow_mut().right.take());
        } else {
            if let Some(left) = node.borrow_mut().left.take() {
                if let Some(right) = node.borrow_mut().right.take() {
                    let mut min_right = find_min_node(right);
                    min_right.borrow_mut().right = Some(merge_trees(left, right));
                    return Some(min_right);
                } else {
                    return Some(left);
                }
            } else {
                return node.borrow_mut().right.take();
            }
        }
    }
    root
}

fn find_min_node(root: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    let mut current = root;
    while let Some(node) = current.borrow_mut().left.take() {
        current = node;
    }
    current
}

fn merge_trees(left: Rc<RefCell<Node>>, mut right: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    if let Some(min_right) = find_min_node(right.clone()) {
        let mut left = left;
        while let Some(node) = left.borrow_mut().right.take() {
            left = node;
        }
        left.borrow_mut().right = Some(right);
    } else {
        right
    }
}

fn main() {
    let mut tree = Tree::new();

    // Test insert and find
    for i in 0..50 {
        tree.insert(i);
    }
    for i in 0..50 {
        assert!(tree.find(i).is_some());
    }

    // Test delete
    for i in 0..25 {
        tree.delete(i * 2);
    }
    for i in 0..25 {
        assert!(tree.find(i * 2).is_none());
    }
    for i in 1..25 {
        assert!(tree.find(i * 2).is_some());
    }

    // Test find_min and find_max
    assert_eq!(tree.find_min().unwrap().borrow().element, 1);
    assert_eq!(tree.find_max().unwrap().borrow().element, 49);
}