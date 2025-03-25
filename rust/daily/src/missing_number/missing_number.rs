
pub struct Solution{}
#[derive(Debug)]
pub struct BSTNode {
    val: i32,
    left: Option<Box<BSTNode>>,
    right: Option<Box<BSTNode>>,
}

impl BSTNode {
    fn new(val: i32) -> Self {
        BSTNode {
            val,
            right: None,
            left: None,
        }
    }
    pub(crate) fn insert(&mut self, new_val: i32) {
        if self.val > new_val {
            match self.left {
                None => {
                    self.left = Some(Box::new(BSTNode::new(new_val)));
                }
                Some(ref mut left_node) => {
                    left_node.insert(new_val)
                }
            }
        } else {
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
    pub fn search(&mut self, searched_val: i32) -> bool{
        if self.val == searched_val{
            return true
        }
        return if self.val > searched_val {
            match self.left {
                None => {
                    false
                }
                Some(ref mut left_node) => {
                    left_node.search(searched_val)
                }
            }
        } else {
            match self.right {
                None => {
                    false
                }
                Some(ref mut right_node) => {
                    right_node.search(searched_val)
                }
            }
        }
    }
}
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        // Your original implementation (note: this implementation may need adjustments to compile and work as expected)
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        let min = sorted_nums.first().copied();
        let max = sorted_nums.last().copied();

        // If there are no elements, assume 0 is missing.
        if min.is_none() || max.is_none() {
            return 0;
        }
        // This implementation attempts to use binary search to find a missing number in the range.
        // (This is just one approach and may not cover all edge cases.)
        let mut result = -1;
        for i in 0..=nums.len() as i32 {
            // We use binary_search to check if `i` is in the sorted list.
            match sorted_nums.binary_search(&i) {
                Ok(_) => continue,
                Err(_) => {
                    result = i;
                    break;
                }
            }
        }
        if result == -1 {
            return nums.len() as i32
        }
        // If no number in the range [min, max] was missing, then check for missing 0 or the next number.
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_number_middle() {
        // Example: missing number is in the middle.
        let nums = vec![3, 0, 1];
        let expected = 2;
        assert_eq!(Solution::missing_number(nums), expected);
    }

    #[test]
    fn test_missing_number_last() {
        // Example: the missing number is after the highest element.
        let nums = vec![0, 1];
        let expected = 2;
        assert_eq!(Solution::missing_number(nums), expected);
    }

    #[test]
    fn test_missing_number_first() {
        // Example: the missing number is 0.
        let nums = vec![1, 2, 3];
        let expected = 0;
        assert_eq!(Solution::missing_number(nums), expected);
    }

    #[test]
    fn test_missing_number_unsorted() {
        // Unsorted array with a missing number.
        let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        let expected = 8;
        assert_eq!(Solution::missing_number(nums), expected);
    }

    #[test]
    fn test_missing_number_empty() {
        // When the input is empty, the missing number is 0.
        let nums: Vec<i32> = vec![];
        let expected = 0;
        assert_eq!(Solution::missing_number(nums), expected);
    }
}