
/// Struct to represent an atom in the simulation
pub struct Atom{
    /// Species of the atom (originally type in Squirm3), max 255
    species: u8,
    
    /// State of the atom, max 255
    state: u8,

    /// Tuple of 2D position of the atom in the world, 32-bit precision
    pos: (f32, f32),

    /// Diameter of the atom
    dia: f32

}

impl Atom{
    // *********** PUBLIC ***********

    /// Create a new atom with a given species, state, and positions
    pub fn new(species: u8, state: u8, pos_x: f32, pos_y: f32, dia: f32) -> Atom {
        Atom{ species: species, state: state, pos: (pos_x, pos_y), dia: dia}
    }

    /// Get position of atom
    pub fn x(&self) -> f32 {
        return self.pos.0
    }

    /// Get position of atom
    pub fn y(&self) -> f32 {
        return self.pos.1
    }

    /// Get radii of atom
    pub fn r(&self) -> f32 {
        return self.dia/2.0
    }


    // *********** PRIVATE ***********

    /// Move atom to a new position `(new_x, new_y)` in the world
    fn move_to(&mut self, new_x: f32, new_y: f32){
        self.pos = (new_x, new_y);
    }
    

    /// Change state of the atom to `new_state`
    fn state_to(&mut self, new_state: u8){
        self.state = new_state;
    }


}


