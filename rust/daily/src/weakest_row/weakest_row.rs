
struct Solution{}
impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let n = mat.len();
        let m = mat[0].len();
        let mut tmp = Vec::new();
        let mut res = Vec::new();
        for i in 0..n {
            let mut num_of_soldiers = 0;
            for j in 0..m{
                if mat[i][j] >= 1{
                    num_of_soldiers+=1;
                }
            }
            tmp.push((i,num_of_soldiers));
        }
        tmp.sort_by(|a,b| a.1.cmp(&b.1));
        for i in 0 .. k{
            res.push(tmp[i as usize].0 as i32);
        }
        return res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn k_weakest_row() {
        // Test case 1: Target exists in the array
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 1],
        ];
        assert_eq!(Solution::k_weakest_rows(matrix,3),vec![2,0,3]);
    }
}
