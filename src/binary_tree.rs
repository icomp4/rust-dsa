use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct TreeNode<T> {
    pub value: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T>
where
    T: Ord + std::fmt::Display,
{
    // Create a new TreeNode.
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode {
            value,
            left: None,
            right: None,
        }))
    }

    // Insert a value into the tree.
    pub fn insert(root: &Option<Rc<RefCell<TreeNode<T>>>>, value: T) {
        match root {
            Some(node) => {
                let mut node_borrowed = node.borrow_mut();
                if value < node_borrowed.value {
                    if node_borrowed.left.is_some() {
                        Self::insert(&node_borrowed.left, value);
                    } else {
                        node_borrowed.left = Some(Self::new(value));
                    }
                } else if value > node_borrowed.value {
                    if node_borrowed.right.is_some() {
                        Self::insert(&node_borrowed.right, value);
                    } else {
                        node_borrowed.right = Some(Self::new(value));
                    }
                }
            }
            None => panic!("The root must be initialized!"),
        }
    }

    // In-order traversal.
    pub fn in_order_traversal(node: &Option<Rc<RefCell<TreeNode<T>>>>) {
        if let Some(node) = node {
            Self::in_order_traversal(&node.borrow().left);
            println!("{}", node.borrow().value);
            Self::in_order_traversal(&node.borrow().right);
        }
    }

    // Writes data into a Vector for testing purposes.
    pub fn in_order_traversal_vec(node: &Option<Rc<RefCell<TreeNode<T>>>>, result: &mut Vec<T>)
    where
        T: Copy,
    {
        if let Some(node) = node {
            Self::in_order_traversal_vec(&node.borrow().left, result);
            result.push(node.borrow().value);
            Self::in_order_traversal_vec(&node.borrow().right, result);
        }
    }
}