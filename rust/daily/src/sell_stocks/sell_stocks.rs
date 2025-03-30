use std::cmp::max;

struct Solution{}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let n = prices.len();
        for i in 0..n {
            for j in i+1 .. n{
                let diff = prices[j] - prices[i];
                if diff > max_profit {
                    max_profit = diff;
                }
            }
        }
        return max_profit
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn max_profit() {
        assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]),5);
    }
}