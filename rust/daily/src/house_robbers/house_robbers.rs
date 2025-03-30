use std::cmp;
use cmp::max;
struct Solution{}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len() ;
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return nums[0];
        }
        let mut dp : Vec<i32> = Vec::with_capacity(n);
        dp.resize(n,0);
        dp[0] = nums[0];
        dp[1] =  cmp::max(nums[0], nums[1]);
        for i in 2 .. n {
            dp[i] = max(dp[i-1],nums[i]+dp[i-2]);
         }
        return dp[n-1]
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn rob() {
        assert_eq!(Solution::rob(vec![2,7,9,3,1]),12);
    }
}