struct Solution{}
impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut cnt = 0;
        for g in grid {
            for i in (0..g.len()).rev() {
                if g[i] < 0 {
                    cnt+=1
                }
            }
        }
        return cnt
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn count_negatives() {
        // assert_eq!(Solution::count_negatives(vec![vec![4,3,2,-1],vec![3,2,1,-1],vec![1,1,-1,-2],vec![-1,-1,-2,-3]]),8);
        assert_eq!(Solution::count_negatives(vec![vec![5,1,0],vec![-5,-5,-5]]),3);
    }
}