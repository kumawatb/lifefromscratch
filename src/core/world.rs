//! World module
//! Contains definitions for the simulation area, included geometries, and any spatially varying parameters.

use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;
use crate::core::Atom;

pub struct World{
    size: (f32,f32) ,
    temperature: f32,

    /// Vector containing the atoms, each index is an atom in the world
    atoms: Vec<Atom>,
    rng: Xoshiro256Plus,
}

impl World{
    // ************ PUBLIC ***********

    /// Create a new world with a given size and temperature
    pub fn new(size_x: f32, size_y: f32, temperature: f32, seed: u64) -> World {
        let world_rng = if seed==0 { Xoshiro256Plus::from_os_rng() } else { Xoshiro256Plus::seed_from_u64(seed) };
        
        World{ size:(size_x,size_y), temperature:temperature, atoms: Vec::new(), rng: world_rng}
    }

    /// Add an atom to the world at a given x, y position and a given species and state.
    pub fn add_atom_at(&mut self, x: f32, y: f32, species: u8, state: u8, dia: f32){
        let new_atom = Atom::new(species, state, x, y, dia);
        self.atoms.push(new_atom);
    }

    /// Initialize the world to a random initial state
    pub fn init_random(&mut self, n_atoms: u32, dia: f32){
        for _ in 0..n_atoms{
            let x: f32 = self.rng.random();
            let y: f32= self.rng.random();
            let spec: u8 = self.rng.random();
            self.add_atom_at(x*self.size.0, y*self.size.1, spec, 0, dia);
        }
    }

    /// Step the world by one unit of time
    pub fn step(&mut self){

        // Diffuse the particle proportional to world temperature
        for atom in self.atoms.iter_mut(){
            atom.y_inc(self.temperature * (self.rng.random::<f32>() * 2.0 - 1.0) , self.size.1);
            atom.x_inc(self.temperature * (self.rng.random::<f32>() * 2.0 - 1.0), self.size.0);
        }

        // Resolve any collisions created during individual atom moves.
        self.resolve_all_collisions();
    }


    /// Get the atoms in the world as an iterator
    pub fn atom_iter(&self) -> std::slice::Iter<'_, Atom>{
        self.atoms.iter()
    }

    /// Get size
    pub fn size_x(&self) -> f32{
        return self.size.0
    }

    /// Get size
    pub fn size_y(&self) -> f32{
        return self.size.1
    }

    // *********** PRIVATE ***********

    /// Resolves all collisions using a simple spatial hasing algorithm
    fn resolve_all_collisions(&mut self){
        // Find all pairs of atoms that intersect (use a spatial hash)
        todo!();
        // self.resolve_collision(atom1, atom2);
    }

    /// Resolve collisions between given atoms
    fn _resolve_collision(&mut self, _atom1: &Atom, _atom2: &Atom){
        // Resolve the collision of the given atoms
        todo!();
    }
}
