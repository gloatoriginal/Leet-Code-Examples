impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let most_candy = candies.iter().max().unwrap();
        let mut return_vec = Vec::new();
        for candy in candies.iter(){
            if candy + extra_candies >= *most_candy { return_vec.push(true); }
            else { return_vec.push(false) }
        }
        
        return_vec
    }
}