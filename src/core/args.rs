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
pub struct Args {
    /// Size of the world along x
    #[arg(long, default_value_t = 100.0)]
    pub size_x: f32, 

    /// Size of the world along y
    #[arg(long, default_value_t = 100.0)]
    pub size_y: f32, 

    /// Atom diameter
    #[arg(long, default_value_t = 20.0)]
    pub diameter: f32,

    /// Number of atom species (<=256)
    #[arg(long, default_value_t = 4)]
    pub num_species: u16,

    /// Number of atom states (<=256)
    #[arg(long, default_value_t = 4)]
    pub num_states: u16,

    /// Base temperature
    #[arg(long, default_value_t = 100.0)]
    pub temperature: f32,

    /// Initial number of atoms
    #[arg(long, default_value_t = 100)]
    pub init_atoms: u32,
    
    /// Number of time steps to run the simulation
    #[arg(long, default_value_t = 1000000)]
    pub t_max: u64,

    /// Visualize the space?
    #[arg(long, default_value_t=false)]
    pub draw: bool,

    /// Skip draw_skip steps between drawing
    #[arg(long, default_value_t=1)]
    pub draw_every: u64,

    /// Seed for random number generator
    #[arg(long, default_value_t=0)]
    pub seed: u64,

    /// Chemistry file
    #[arg(long, default_value_t=String::from("./chemistry.cfg"))]
    pub chempath: String
}