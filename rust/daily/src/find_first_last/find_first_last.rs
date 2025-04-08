struct Solution{}
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        fn find_first(nums: &Vec<i32>, target: i32) -> i32 {
            let (mut left, mut right) = (0, nums.len() as i32 - 1);
            let mut first = -1;
            while left <= right {
                let mid = left + (right - left) / 2;
                if nums[mid as usize] >= target {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
                if nums[mid as usize] == target {
                    first = mid;
                }
            }
            first
        }

        fn find_last(nums: &Vec<i32>, target: i32) -> i32 {
            let (mut left, mut right) = (0, nums.len() as i32 - 1);
            let mut last = -1;
            while left <= right {
                let mid = left + (right - left) / 2;
                if nums[mid as usize] <= target {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
                if nums[mid as usize] == target {
                    last = mid;
                }
            }
            last
        }

        let first = find_first(&nums, target);
        if first == -1 {
            return vec![-1, -1];
        }
        let last = find_last(&nums, target);

        vec![first, last]
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn count_negatives() {
        // assert_eq!(Solution::search_range(vec![5,7,7,8,8,10],8),vec![3,4]);
        // assert_eq!(Solution::search_range(vec![1],1),vec![0,0]);
        // assert_eq!(Solution::search_range(vec![2,2],3),vec![-1,-1]);
        // assert_eq!(Solution::search_range(vec![5,7,7,8,8,10],8),vec![3,4]);
        assert_eq!(Solution::search_range(vec![1],1),vec![0,0]);
        assert_eq!(Solution::search_range(vec![2,2],2),vec![0,1]);
    }
}