use std::cmp::min;

struct Solution{}
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut dp : Vec<i32> = Vec::new();
        dp.resize(n+1,0);
        for i in 2 .. n+1 {
            dp[i] = min(dp[i-2]+cost[i-2],dp[i-1]+cost[i-1]);
        }
        return dp[n];
    }
    pub fn min_cost_climbing_stairs_prev_curr(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut prev = 0;
        let mut curr = 0;
        for i in 2 .. n+1 {
            prev = curr;
            curr = min(cost[i-1] + curr,cost[i-2]+prev);
        }
        return curr;
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn min_cost_climbing_stairs() {
        // assert_eq!(Solution::min_cost_climbing_stairs(vec![10,15,20]),15);
        // Solution::min_cost_climbing_stairs(vec![1,100,1,1,1,100]);
        assert_eq!(Solution::min_cost_climbing_stairs_prev_curr(vec![1,100,1,1,1,100,1,1,100,1]),6);
    }
}