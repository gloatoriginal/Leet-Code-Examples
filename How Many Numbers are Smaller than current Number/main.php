class Solution {

    /**
     * @param Integer[] $nums
     * @return Integer[]
     */
    function smallerNumbersThanCurrent($nums) {
        $_arr = array();
        foreach($nums as &$num){
            $count = 0;
            for($i=0; $i<count($nums);$i+=1){
                if($num > $nums[$i]) { $count+=1; }
            }
            $_arr[] = $count;
        }
        return $_arr;
    }
}