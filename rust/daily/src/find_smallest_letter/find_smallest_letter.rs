struct Solution{}
impl Solution {
    pub fn search_internal(letters: Vec<char>,target: char, low :i32, high: i32) -> char {
        if high < 0{
            return letters[0]
        }
        let n = letters.clone().len();
        if low >= n as i32 {
            return letters[0]
        }
        if low > high {
            if letters[low as usize] > target{
                return letters[low as usize]
            }
            return letters[(low - 1) as usize]
        }
        let mid = low + (high - low) / 2;
        if  letters[mid as usize] == target {
            return letters[(mid) as usize];
        } else if letters[mid as usize] < target{
            Self::search_internal(letters,target,mid+1,high)
        } else {
            Self::search_internal(letters,target,low,mid-1)
        }
    }
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let n = letters.len() as i32;
        let next_target = std::char::from_u32((target as u32) + 1).unwrap();
        return Self::search_internal(letters,next_target,0,n-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_missing_number_middle() {
        assert_eq!(Solution::next_greatest_letter(vec!['c','f','j'],'a'), 'c');
        assert_eq!(Solution::next_greatest_letter(vec!['c','f','j'],'c'), 'f');
        // assert_eq!(Solution::next_greatest_letter(vec!['c','f','j'],'d'), 'c');
        assert_eq!(Solution::next_greatest_letter(vec!['c','f','j'],'g'), 'j');
        assert_eq!(Solution::next_greatest_letter(vec!['c','f','j'],'j'), 'c');
        // ["e","e","g","g"], g
        assert_eq!(Solution::next_greatest_letter(vec!['e','e','g','g'],'g'), 'e');
        assert_eq!(Solution::next_greatest_letter(vec!['x','x','y','y'],'z'), 'x');
        // output: e
        assert_eq!(Solution::next_greatest_letter(vec!['e','e','e','k','q','q','q','v','v','y'],'x'),'y')
    }
}