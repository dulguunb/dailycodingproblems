struct Solution{}
impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let n = arr.len();
        for i in 0 .. n{
            for j in 0 .. n{
                if i != j {
                    if arr[i] == arr[j] * 2{
                        return true
                    }
                }
            }
        }
        return false
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn check_if_exist() {
        let arr = vec![10,2,5,3];
        assert_eq!(Solution::check_if_exist(arr),true);
        assert_eq!(Solution::check_if_exist(vec![3,1,7,11]),false);
    }
}
