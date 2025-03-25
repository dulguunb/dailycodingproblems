struct Solution{}
impl Solution {

    pub fn climb_stairs(n:i32) -> i32{
        let mut mem:Vec<i32> = Vec::new();
        mem.resize((n+2) as usize,0);
        mem[1] = 1;
        mem[2] = 2;
        for i in 3..n+1{
           let idx = i as usize;
           mem[idx] = mem[idx-2] + mem[idx-1];
        }
        return mem[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn climb_stairs() {
        assert_eq!(Solution::climb_stairs(1),1);
        assert_eq!(Solution::climb_stairs(2),2);
        assert_eq!(Solution::climb_stairs(3),3);
        assert_eq!(Solution::climb_stairs(4),5);
        assert_eq!(Solution::climb_stairs(6),13);
    }
}