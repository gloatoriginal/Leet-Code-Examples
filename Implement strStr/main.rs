impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
    if haystack.contains(&needle) {
        let haystackVec: Vec<char> = haystack.chars().collect();
        let needleVec: Vec<char> = needle.chars().collect();
        if needleVec.len() == 0 { return 0; }

        for i in 0..haystackVec.len() {
            let mut n = i;
            let mut j = 0;
            if haystackVec[i] == needleVec[j] {
                while j < needleVec.len() {
                    //println!("entered while i={} and j={}", n, j);
                    if haystackVec[n] == needleVec[j]{
                        //println!("haystacktarget -> {} needletarget -> {}", haystackVec[n], needleVec[j]);
                        j+=1;
                        n+=1;
                        if j >= needleVec.len() { return i as i32; }
                    } else { j = needleVec.len()+1; }
                }
            }
        }
        return 0;
    } else { return -1; }
}
}