class Solution {

    /**
     * @param String $time
     * @return String
     */
    function maximumTime($time) {
        $time=str_split($time);
        for($i=0; $i<count($time); $i++){
            if($time[$i] == '?'){
                echo $i;
                switch($i){
                    case 0:
                        $time[$i] = ($time[$i+1] >= 4) ? 1 : 2;
                        break;              
                    case 1:
                        $time[$i] = ($time[$i-1] == 2) ? 3 : 9;
                        break;
                    case 3:
                        $time[$i] = 5;
                        break;
                    case 4:
                        $time[$i] = 9;
                        break;
                }

            }
            
        }
        return implode($time);
    }
}