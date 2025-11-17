impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        if n==1{
            return [0,1].to_vec();
        }
        else{
            let mut first_half=Solution::gray_code(n-1);
            let mut second_half=first_half.clone();
            second_half.reverse();
            let y=2_i32.pow(n as u32 - 1);
            second_half=second_half.iter().map(|&x| x+y ).collect::<Vec<_>>();
            first_half.extend(second_half);
            first_half
            
        }
    }
}