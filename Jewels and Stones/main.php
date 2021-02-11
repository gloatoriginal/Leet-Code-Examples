class Solution {

    /**
     * @param String $jewels
     * @param String $stones
     * @return Integer
     */
    function numJewelsInStones($jewels, $stones) {
        $count = 0;
        $jewels = str_split($jewels);
        $stones = str_split($stones);
        for($i=0;$i<=count($jewels);$i+=1){
            for($j=0;$j<count($stones);$j+=1){
                if($jewels[$i] == $stones[$j]){
                    $count+=1;
                }
            }
        }
        return $count;
    }
}