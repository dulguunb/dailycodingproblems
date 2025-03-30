use std::cmp::min;

struct Solution{}
impl Solution {
    fn knapsack(w :i32, profit: Vec<i32>, weight: Vec<i32>) -> i32{
        
        return 0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn min_cost_climbing_stairs() {
        // assert_eq!(Solution::min_cost_climbing_stairs(vec![10,15,20]),15);
        // Solution::min_cost_climbing_stairs(vec![1,100,1,1,1,100]);
        assert_eq!(Solution::knapsack(4,vec![1,2,3],vec![4,5,6]),3);
    }
}