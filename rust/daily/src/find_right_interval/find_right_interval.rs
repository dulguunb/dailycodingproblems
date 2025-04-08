struct Solution{}
impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sorted_starts = Vec::new();
        let n = intervals.len();
        let mut result = vec![-1; n];
        for (i,interval) in intervals.iter().enumerate(){
            sorted_starts.push((interval[0], i));
        }
        let mut result = vec![-1; n];
        sorted_starts.sort_unstable();
        for (i,interval) in intervals.iter().enumerate(){
            let end_i = interval[1];

            // Binary search for the smallest start_j >= end_i
            let pos = sorted_starts.binary_search_by_key(&end_i, |&(start,_)| start)
                .unwrap_or_else(|x| x);
            if pos < sorted_starts.len() {
                result[i] = sorted_starts[pos].1 as i32;
            }
        }

        result
    }

}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn find_right_interval() {
        // assert_eq!(Solution::find_right_interval(vec![vec![1,2]]),vec![-1]);
        assert_eq!(Solution::find_right_interval(vec![vec![3,4],vec![2,3],vec![1,2]]),vec![-1,0,1]);

        assert_eq!(Solution::find_right_interval(vec![vec![1,4],vec![2,3],vec![3,4]]),vec![-1,2,-1]);

    }
}