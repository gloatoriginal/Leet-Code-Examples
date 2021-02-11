class ParkingSystem {
    /**
     * @param Integer $big
     * @param Integer $medium
     * @param Integer $small
     */
    function __construct($big, $medium, $small) {
        $this->big_slots = $big;
        $this->med_slots = $medium;
        $this->sma_slots = $small;
        return null;
    }
  
    /**
     * @param Integer $carType
     * @return Boolean
     */
    function addCar($carType) {
        switch($carType){
            case 1:
                if($this->big_slots<1){ return false; }
                else{ 
                    $this->big_slots-=1;
                    return true;
                }
            case 2:
                if($this->med_slots<1){ return false; }
                else{ 
                    $this->med_slots-=1;
                    return true;
                }
            case 3:
                if($this->sma_slots<1){ return false; }
                else{ 
                    $this->sma_slots-=1;
                    return true;
                }
                
        }
    }
}

/**
 * Your ParkingSystem object will be instantiated and called as such:
 * $obj = ParkingSystem($big, $medium, $small);
 * $ret_1 = $obj->addCar($carType);
 */