impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut new_nums = Vec::new();
        for i in 0..n {
            new_nums.push(nums[i as usize]);
            let n_index = (i as i32)+n;
            new_nums.push(nums[n_index as usize]);
        }
        new_nums
    }
}