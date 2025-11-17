impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let n=haystack.len();
        let m=needle.len();
        if m==0 { 
            return 0; 
        }
        if m > n {
            return -1;
        }
        let h_bytes=haystack.as_bytes();
        let n_bytes=needle.as_bytes();
        for i in 0..=n-m{
            let mut found = true;
            for j in 0..m{
                if !(h_bytes[i+j]==n_bytes[j]){
                    found=false;
                }   
            }
            if found{
                return i as i32;

            }
        }
        -1
    }
}