struct Solution{}
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut num = 0;
        for a in s.chars(){
            if a == 'I'{
                num+=1;
            }
            if a == 'V'{
                num+=5;
            }
            if a == 'X'{
                num+=10;
            }
            if a == 'L'{
                num+=50;
            }
            if a == 'C'{
                num+=100;
            }
            if a == 'D'{
                num+=500;
            }
            if a == 'M'{
                num+=1000;
            }
        }
        return num;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn roman_to_int() {
        // assert_eq!(Solution::min_cost_climbing_stairs(vec![10,15,20]),15);
        // Solution::min_cost_climbing_stairs(vec![1,100,1,1,1,100]);
        assert_eq!(Solution::roman_to_int(String::from("III")),3);
        assert_eq!(Solution::roman_to_int(String::from("LVIII")),58);
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")),1994);
    }
}