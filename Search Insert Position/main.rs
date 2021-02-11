impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if target <= nums[0]{ return 0; }
    let mut retVal = 0;
    for i in 0..nums.len() {
        if nums[i] == target { retVal = i; }
        else if i+1 < nums.len(){
            if nums[i] < target && nums[i+1] > target { 
                retVal = i+1;
            }
        }
    }
    if retVal == 0 { retVal = nums.len(); }
    retVal as i32
}


}