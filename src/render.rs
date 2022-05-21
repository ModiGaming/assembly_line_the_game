//render.rs loads the different screen elements and displays them 
//to the terminal.

//load quadrants based off player_pos putting the tiles that will
//be visible into a 2d array with the size of the resolution
fn load_terrain(player_pos: [i64; 2]) {
    //not implemented
}

//call a function for each structure in the game that loads a
//quadrant for that structure  and puts the info on the sprites
//of the structures into a 2d array with the size of the render
//resolution
fn load_structures(player_pos: [i64; 2]) {
    //not implemented
}

//renders loaded tiles from the load_terrain and load_structures 
//functions to the terminal all surrounded by a border
fn render_screen(terrain_data: [[char; 32]; 18], structure_data: [[char; 32]; 18]) {
    let render_resolution: [i8; 2] = [33, 19];
    //not implemented
}