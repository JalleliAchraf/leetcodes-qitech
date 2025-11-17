impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy1=i32::MIN;
        let mut buy2=i32::MIN;
        let mut sell1=0;
        let mut sell2=0;
        for i in 0..prices.len(){
            buy1=std::cmp::max(buy1,-prices[i]);
            sell1=std::cmp::max(sell1,buy1+prices[i]);
            buy2=std::cmp::max(buy2,sell1-prices[i]);
            sell2=std::cmp::max(sell2,buy2+prices[i]);
        }
        sell2
    }
}