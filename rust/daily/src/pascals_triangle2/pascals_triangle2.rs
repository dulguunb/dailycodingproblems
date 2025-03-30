struct Solution{}
impl Solution {
    pub fn get_row(num_rows: i32) -> Vec<i32> {
        let mut mem: Vec<Vec<i32>> = Vec::new();
        let num_rows_ = num_rows + 1;
        for i in 1..num_rows_+1 {
            let mut row: Vec<i32> = Vec::new();
            row.resize((i as usize),1);
            mem.push(row);
        }

        for i in 2 .. num_rows_+1 {
            let i_idx = i as usize;
            for j in 1 .. (i-1) {
                let j_idx = j as usize;
                mem[i_idx-1][j_idx] = mem[i_idx-2][j_idx-1] + mem[i_idx-2][j_idx];
            }
        }
        return mem[num_rows as usize].clone();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn climb_stairs() {
        assert_eq!(Solution::get_row(0),vec![1]);
        assert_eq!(Solution::get_row(1),vec![1,1]);
        assert_eq!(Solution::get_row(2),vec![1,2,1]);
        assert_eq!(Solution::get_row(3),vec![1,3,3,1]);
        assert_eq!(Solution::get_row(4),vec![1,4,6,4,1]);
    }
}