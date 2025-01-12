// pub struct Solution;


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_search_insert() {
        // Test case 1: Target exists in the array
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);

        // Test case 2: Target does not exist and would be inserted at the end
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);

        // Test case 3: Target does not exist and would be inserted at the beginning
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);

        // Test case 4: Target does not exist and would be inserted in the middle
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);

        // Test case 5: Empty array
        assert_eq!(Solution::search_insert(vec![], 5), 0);
    }
}
pub struct Solution{}

impl Solution {
    pub fn binary_search(nums: &Vec<i32>, target: i32, low: i32, high: i32) -> i32 {
        if low > high {
            return low;
        }
        let mid = low + (high - low) / 2;
        if nums[mid as usize] == target {
            mid
        } else if nums[mid as usize] < target {
            Self::binary_search(nums, target, mid + 1, high)
        } else {
            Self::binary_search(nums, target, low, mid - 1)
        }
    }
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let len = (nums.len() as i32);
        return Self::binary_search(&nums, target, 0, len - 1);
    }
}