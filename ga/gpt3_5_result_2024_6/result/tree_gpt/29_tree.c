use std::ptr;

type ElementType = i32;

struct TreeNode {
    element: ElementType,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
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

type SearchTree = Option<Box<TreeNode>>;

impl SearchTree {
    fn make_empty(t: &mut Self) {
        *t = None;
    }

    fn insert(x: ElementType, t: &mut Self) {
        if let Some(node) = t {
            if x < node.element {
                Self::insert(x, &mut node.left);
            } else if x > node.element {
                Self::insert(x, &mut node.right);
            }
        } else {
            let new_node = Box::new(TreeNode::new(x));
            *t = Some(new_node);
        }
    }

    fn delete(x: ElementType, t: &mut Self) {
        if let Some(node) = t {
            if x < node.element {
                Self::delete(x, &mut node.left);
            } else if x > node.element {
                Self::delete(x, &mut node.right);
            } else {
                if node.left.is_some() && node.right.is_some() {
                    let mut tmp_cell = Self::find_min(&node.right);
                    node.element = tmp_cell.element;
                    Self::delete(tmp_cell.element, &mut node.right);
                } else {
                    let tmp_cell = Box::new(unsafe { ptr::read(&*node) });
                    if tmp_cell.left.is_none() {
                        *t = tmp_cell.right;
                    } else if tmp_cell.right.is_none() {
                        *t = tmp_cell.left;
                    }
                }
            }
        }
    }

    fn retrieve(p: &Position) -> ElementType {
        p.element
    }

    fn find_min(t: &SearchTree) -> &TreeNode {
        if let Some(node) = t {
            if node.left.is_none() {
                return node;
            }
            Self::find_min(&node.left)
        } else {
            panic!("Empty tree");
        }
    }

    fn find_max(t: &SearchTree) -> &TreeNode {
        if let Some(node) = t {
            if node.right.is_none() {
                return node;
            }
            Self::find_max(&node.right)
        } else {
            panic!("Empty tree");
        }
    }
}

impl TreeNode {
    fn find(x: ElementType, t: &SearchTree) -> Option<&TreeNode> {
        match t {
            Some(node) => {
                if x < node.element {
                    TreeNode::find(x, &node.left)
                } else if x > node.element {
                    TreeNode::find(x, &node.right)
                } else {
                    Some(node)
                }
            }
            None => None,
        }
    }
}

fn main() {
    let mut t = SearchTree::make_empty(&mut None);
    for i in 0..50 {
        let j = (i + 7) % 50;
        SearchTree::insert(j, &mut t);
    }

    for i in 0..50 {
        let p = TreeNode::find(i, &t);
        match p {
            Some(node) => {
                if node.element != i {
                    println!("Error at {}", i);
                }
            }
            None => println!("Error at {}", i),
        }
    }

    for i in (0..50).step_by(2) {
        SearchTree::delete(i, &mut t);
    }

    for i in (1..50).step_by(2) {
        let p = TreeNode::find(i, &t);
        match p {
            Some(node) => {
                if node.element != i {
                    println!("Error at {}", i);
                }
            }
            None => println!("Error at {}", i),
        }
    }

    for i in (0..50).step_by(2) {
        if let Some(_) = TreeNode::find(i, &t) {
            println!("Error at {}", i);
        }
    }

    if let Some(min) = SearchTree::find_min(&t) {
        if let Some(max) = SearchTree::find_max(&t) {
            println!("Min is {}, Max is {}", min.element, max.element);
        }
    }
}