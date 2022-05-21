//calculate.rs calculates all resource consumption and production
//for the whole map and every structure every frame asyncronously

//find the position of all resource collection structures and
//determine where the resource goes conveyor/ground
fn find_raw_resource_collection() {
    //not implemented
}

//calculates the size of a string of conveyors from start of a set
//to end of a set of conveyors stores the amount of time in frames
//that items on the conveyor have been on it and what type of items
//are there if an item is in front of the item dont update frame value
//and keep it on the highest frame value that puts the item behind the
//full conveyor belt
fn conveyor_follower() {
    //not implemented
}

//check for insertors along conveyors, check if insertor is on a
//cooldown, then check if an item is in front of the insertor, if 
//there is an item start the moving process and once the moving process
//is done check if there is space, put the item into the structure being 
//inserted to if there is space, or if there is no structure put the 
//item on the ground
fn insertor_conveyor_check() {
    //not implemented
}

//check all items on the ground and check if there is an insertor next to
//it if there is calculate when it will be grabbed through the same
//instructions as insertor_conveyor_check
fn check_ground_items() {
    //not implemented
}