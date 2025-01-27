mod linked_list;
mod binary_tree;
use crate::binary_tree::TreeNode;

pub fn list_test() {
    let mut list = linked_list::LinkedList::new();
    list.add(2);
    list.add(3);
    list.add_to_front(24);
    list.add(10);
    list.print();
    println!("List contains 4: {}", list.contains(4));
    println!("List contains 2: {}", list.contains(2));
}

pub fn binary_tree_test() {
    let root = TreeNode::new(10);
    TreeNode::insert(&Some(root.clone()), 5);
    TreeNode::insert(&Some(root.clone()), 15);
    TreeNode::insert(&Some(root.clone()), 3);
    TreeNode::insert(&Some(root.clone()), 7);
    TreeNode::insert(&Some(root.clone()), 12);
    TreeNode::insert(&Some(root.clone()), 18);

    // Print the tree in order.
    println!("In-order traversal:");
    TreeNode::in_order_traversal(&Some(root));
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use super::*;

    #[test]
    fn test_insertion() {
        let root = TreeNode::new(10);
        TreeNode::insert(&Some(root.clone()), 5);
        TreeNode::insert(&Some(root.clone()), 15);

        assert_eq!(root.borrow().value, 10);
        assert_eq!(root.borrow().left.as_ref().unwrap().borrow().value, 5);
        assert_eq!(root.borrow().right.as_ref().unwrap().borrow().value, 15);
    }

    #[test]
    fn test_in_order_traversal() {
        let root = TreeNode::new(10);
        TreeNode::insert(&Some(root.clone()), 5);
        TreeNode::insert(&Some(root.clone()), 15);
        TreeNode::insert(&Some(root.clone()), 3);
        TreeNode::insert(&Some(root.clone()), 7);
        TreeNode::insert(&Some(root.clone()), 12);
        TreeNode::insert(&Some(root.clone()), 18);

        let mut result = Vec::new();
        TreeNode::in_order_traversal_vec(&Some(root.clone()), &mut result);

        assert_eq!(result, vec![3, 5, 7, 10, 12, 15, 18]);
    }

    #[test]
    fn test_empty_tree() {
        let root: Option<Rc<RefCell<TreeNode<i32>>>> = None;
        let mut result = Vec::new();
        TreeNode::in_order_traversal_vec(&root, &mut result);

        assert!(result.is_empty());
    }

    #[test]
    #[should_panic(expected = "The root must be initialized!")]
    fn test_insert_into_empty_tree() {
        let root: Option<Rc<RefCell<TreeNode<i32>>>> = None;
        TreeNode::insert(&root, 10);
    }

    #[test]
    fn test_duplicate_insertion() {
        let root = TreeNode::new(10);
        TreeNode::insert(&Some(root.clone()), 10);

        let mut result = Vec::new();
        TreeNode::in_order_traversal_vec(&Some(root), &mut result);

        // Duplicates are ignored: only one "10" remains.
        assert_eq!(result, vec![10]);
    }
}