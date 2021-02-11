class Solution{
    function restoreString($s, $indices) {
        $s = str_split($s);
        $str_arr = [];
        for($i=0; $i<count($indices);$i+=1){
            $str_arr[$indices[$i]] = $s[$i];
        }
        //$str_arr = leetcdoe
        //why not $str_arr = leetcode?
        ksort($str_arr);
        return implode($str_arr);
    }
}
