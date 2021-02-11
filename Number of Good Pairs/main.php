class Solution {

    /**
     * @param Integer[] $nums
     * @return Integer
     */
    function numIdenticalPairs($nums) {
        $count = 0;
        for($i = 0; $i < count($nums); $i+=1){
            for($j=$i+1; $j <= count($nums); $j+=1){
                if($nums[$j] == $nums[$i]){
                    $count+=1;
                }
            }
        }
        //echo $count;
        return $count;
    }
}