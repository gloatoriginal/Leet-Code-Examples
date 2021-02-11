impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut run_vec = Vec::new();
        for (i, num) in nums.iter().enumerate() {
            let mut new_num = 0;
            for j in 0..i+1 {
                new_num = nums[j] + new_num;
            }
            run_vec.push(new_num);
        }
        run_vec
    }
}