// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
struct Solution{}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return if let Some(node) = root {
            1 + Solution::count_nodes(node.borrow().left.clone()) + Solution::count_nodes(node.borrow().right.clone())
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use super::TreeNode;
    use std::rc::Rc;
    use std::cell::RefCell;

    // Test case for an empty tree.
    #[test]
    fn test_empty_tree() {
        let root: Option<Rc<RefCell<TreeNode>>> = None;
        assert_eq!(Solution::count_nodes(root), 0);
    }

    // Test case for a tree with only one node.
    #[test]
    fn test_single_node() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(Solution::count_nodes(root), 1);
    }

    // Test case for a tree with a left child only.
    #[test]
    fn test_left_subtree_only() {
        let mut root_node = TreeNode::new(1);
        root_node.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let root = Some(Rc::new(RefCell::new(root_node)));
        // Expected count: 2 nodes (1 and 2)
        assert_eq!(Solution::count_nodes(root), 2);
    }

    // Test case for a tree with a right child only.
    #[test]
    fn test_right_subtree_only() {
        let mut root_node = TreeNode::new(1);
        root_node.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let root = Some(Rc::new(RefCell::new(root_node)));
        // Expected count: 2 nodes (1 and 3)
        assert_eq!(Solution::count_nodes(root), 2);
    }

    // Test case for a full tree.
    // The tree structure is:
    //         1
    //        / \
    //       2   3
    //      / \
    //     4   5
    #[test]
    fn test_full_tree() {
        // Create nodes for the left subtree.
        let left_left = Rc::new(RefCell::new(TreeNode::new(4)));
        let left_right = Rc::new(RefCell::new(TreeNode::new(5)));
        let left = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(left_left),
            right: Some(left_right),
        }));

        // Create node for the right subtree.
        let right = Rc::new(RefCell::new(TreeNode::new(3)));

        // Create the root and attach left and right subtrees.
        let root = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(left),
            right: Some(right),
        }));

        // Expected count: 5 nodes.
        assert_eq!(Solution::count_nodes(Some(root)), 5);
    }

    // Test case for a complete binary tree.
    // The tree structure is:
    //          1
    //         / \
    //        2   3
    //       / \   \
    //      4   5   6
    #[test]
    fn test_complete_tree() {
        let node4 = Rc::new(RefCell::new(TreeNode::new(4)));
        let node5 = Rc::new(RefCell::new(TreeNode::new(5)));
        let node6 = Rc::new(RefCell::new(TreeNode::new(6)));

        let node2 = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(node4),
            right: Some(node5),
        }));
        let node3 = Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(node6),
        }));

        let root = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(node2),
            right: Some(node3),
        }));

        // Expected count: 6 nodes.
        assert_eq!(Solution::count_nodes(Some(root)), 6);
    }
}