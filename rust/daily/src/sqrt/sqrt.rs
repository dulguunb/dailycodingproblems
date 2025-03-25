struct Solution{}
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let max_iterations = 1000;
        let tolerance = 1e-10;
        let mut guess = x / 2; // Initial guess
        for _ in 0..max_iterations {
            let next_guess = (guess + x / guess) / 2;
            // Check for convergence
            if (next_guess - guess).abs() < tolerance as i32 {
                return next_guess;
            }
            guess = next_guess;
        }
        guess // Return the best guess after max_iterations
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn my_sqrt() {
        // Test case 1: Target exists in the array
        assert_eq!(Solution::my_sqrt(4),2);
    }
}


