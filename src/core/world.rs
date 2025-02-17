//! World module
//! Contains definitions for the simulation area, included geometries, and any spatially varying parameters.

use crate::core::Atom;

pub struct World{
    size: (f32,f32) ,
    temperature: f32,

    diameter: f32,

    atoms: Vec<Atom>

}

impl World{
    // ************ PUBLIC ***********

    /// Create a new world with a given size and temperature
    pub fn new(size_x: f32, size_y: f32, diameter: f32, temperature: f32) -> World{
        World{ size:(size_x,size_y), temperature:temperature, diameter: diameter, atoms: Vec::new() }
    }

    /// Add an atom to the world at a given x, y position and a given species and state.
    pub fn add_atom_at(&mut self, x: f32, y: f32, species: u8, state: u8){
        let new_atom = Atom::new(species, state, x, y);
        self.atoms.push(new_atom);
    }

    pub fn step(&mut self){
    }

    // *********** PRIVATE ***********
}
