use std::ops::Deref;

#[derive(Debug,Clone)]
pub struct BSTNode {
    val: i32,
    visited: i32,
    left: Option<Box<BSTNode>>,
    right: Option<Box<BSTNode>>,
}
impl BSTNode {
    fn new(val: i32) -> Self {
        BSTNode {
            val,
            visited: 0,
            right: None,
            left: None,
        }
    }
    // pub fn contains(self, mut other: BSTNode) -> Vec<i32> {
    //     let mut stack = Vec::new();
    //     let mut res = Vec::new();
    //     let mut curr_node = Some(Box::new(self));
    //     while !stack.is_empty() || curr_node.is_some() {
    //         if curr_node.is_some() {
    //             stack.push(curr_node.clone());
    //             curr_node = curr_node.unwrap().left
    //         } else {
    //             curr_node = stack.pop().unwrap();
    //             if other.search(curr_node.clone().unwrap().val){
    //                 res.push(curr_node.clone().unwrap().val);
    //             }
    //             curr_node = curr_node.unwrap().right;
    //         }
    //     }
    //     res
    // }
    pub fn search(&mut self, searched_val: i32) -> (bool,i32){
        if self.val == searched_val{
            self.visited+=1;
            return (true,self.visited)
        }
        return if self.val > searched_val {
            match self.left {
                None => {
                    return   (false,self.visited)
                }
                Some(ref mut left_node) => {
                    left_node.search(searched_val)
                }
            }
        } else {
            match self.right {
                None => {
                    return (false,self.visited)
                }
                Some(ref mut right_node) => {
                    right_node.search(searched_val)
                }
            }
        }
    }
    fn insert(&mut self, new_val: i32) {
        if self.val > new_val {
            match self.left {
                None => {
                    self.left = Some(Box::new(BSTNode::new(new_val)));
                }
                Some(ref mut left_node) => {
                    left_node.insert(new_val);
                }
            }
        } else if self.val < new_val {
            match self.right {
                None => {
                    self.right = Some(Box::new(BSTNode::new(new_val)));
                }
                Some(ref mut right_node) => {
                    right_node.insert(new_val)
                }
            }
        }
    }
}
struct Solution{}
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        if nums1.len() < nums2.len(){

        }
        let mut tree1 = BSTNode::new(nums1[0]);
        let mut res = Vec::new();
        for i in 1 .. nums1.len(){
            tree1.insert(nums1[i]);
        }
        for i in 0 .. nums2.len(){
            let (exists,num_visit) = tree1.search(nums2[i]);
            if exists && num_visit == 1{
                res.push(nums2[i])
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::{BSTNode, Solution};
    #[test]
    fn test_search_insert() {
        // Test case 1: Target exists in the array
        assert_eq!(Solution::intersection(vec![1,2,2,1], vec![2,2]), vec![2]);
        assert_eq!(Solution::intersection(vec![1], vec![1]), vec![1]);
        assert_eq!(Solution::intersection(vec![4,9,5], vec![9,4,9,8,4]), vec![4,9]);


    }
}