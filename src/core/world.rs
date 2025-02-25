//! World module
//! Contains definitions for the simulation area, included geometries, and any spatially varying parameters.

use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;
use crate::core::{Grid, Atom};

pub struct World{
    size: (f32,f32) ,
    temperature: f32,
    num_atoms: u32,

    /// Vector containing the atoms, each index is an atom in the world
    // atoms: Vec<Atom>,
    atom_grid: Grid<Atom>,
    rng: Xoshiro256Plus,
}

impl World{
    // ************ PUBLIC ***********

    /// Create a new world with a given size and temperature
    pub fn new(size_x: f32, size_y: f32, temperature: f32, seed: u64, particle_size: f32) -> World {
        let world_rng = if seed==0 { Xoshiro256Plus::from_os_rng() } else { Xoshiro256Plus::seed_from_u64(seed) };
        
        World{ 
            size: (size_x,size_y), 
            temperature:temperature,
            num_atoms: 0,
            atom_grid: Grid::new(size_x, size_y, particle_size, particle_size), 
            rng: world_rng}
    }

    /// Add an atom to the world at a given x, y position and a given species and state.
    pub fn add_atom_at(&mut self, x: f32, y: f32, species: u8, state: u8, dia: f32, atomid: u32){
        
        let new_atom = Atom::new(species, state, x, y, dia, atomid);

        self.atom_grid.push(new_atom);
        self.num_atoms +=1 ;
    }

    /// Initialize the world to a random initial state
    pub fn init_random(&mut self, n_atoms: u32, dia: f32){
        for atomid in 0..n_atoms{

            // Initialize atom with random x, y, species and state
            let x: f32 = self.rng.random();
            let y: f32= self.rng.random();
            let spec: u8 = self.rng.random();
            let state: u8 = self.rng.random(); 

            self.add_atom_at(x*self.size.0, y*self.size.1, spec, state, dia, atomid);
        }
    }

    /// Step the world by one unit of time
    pub fn step(&mut self){

        // First, move all atoms diffusively
        for atomid in 0..self.num_atoms{
            let x_inc = self.temperature * (self.rng.random::<f32>() * 2.0 - 1.0) ;
            let y_inc = self.temperature * (self.rng.random::<f32>() * 2.0 - 1.0);
            self.atom_grid.move_obj(atomid, x_inc, y_inc);
        }
        // Resolve all collisions that are created during the diffusive moves
        for _ in 0..8{
            self.atom_grid.detect_and_resolve_collisions();
        }
    }


    /// Get the atoms in the world
    pub fn atom_iter(&self) -> Vec<&Atom> {
        self.atom_grid.iter()
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
}
