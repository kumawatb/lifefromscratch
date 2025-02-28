use clap::Parser;
use bevy::prelude::*;


/// Bevy Plugin for initializing CLI argument reading
pub struct ArgsPlugin;

impl Plugin for ArgsPlugin {
    fn build(&self, app: &mut App){
        app.insert_resource(Args::parse());
    }
}


/// The `Args` struct stores the command line arguments
#[derive(Parser, Debug, Resource)]
#[command(name = "LifeFromScratch Simulator")]
#[command(version, about, long_about = None)]
struct Args {
    /// Size of the world along x
    #[arg(long, default_value_t = 100.0)]
    size_x: f32, 

    /// Size of the world along y
    #[arg(long, default_value_t = 100.0)]
    size_y: f32, 

    /// Atom diameter
    #[arg(long, default_value_t = 5.0)]
    diameter: f32,

    /// Number of atom species (<=256)
    #[arg(long, default_value_t = 256)]
    num_species: u16,

    /// Number of atom states (<=256)
    #[arg(long, default_value_t = 256)]
    num_states: u16,

    /// Base temperature
    #[arg(long, default_value_t = 0.05)]
    temperature: f32,

    /// Initial number of atoms
    #[arg(long, default_value_t = 100)]
    init_atoms: u32,
    
    /// Number of time steps to run the simulation
    #[arg(long, default_value_t = 1000000)]
    t_max: u64,

    /// Visualize the space?
    #[arg(long, default_value_t=false)]
    draw: bool,

    /// Skip draw_skip steps between drawing
    #[arg(long, default_value_t=1)]
    draw_every: u64,

    /// Seed for random number generator
    #[arg(long, default_value_t=0)]
    seed: u64
}