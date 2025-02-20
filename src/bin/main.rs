use clap::Parser;
use lifefromscratch::core::{Canvas, World};

/// The `Cli` struct stores the command line arguments
#[derive(Parser)]
#[command(name = "LifeFromScratch Simulator")]
#[command(version, about, long_about = None)]
#[derive(Debug)]
struct Cli {
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

fn main(){
    let args = Cli::parse();

    let mut world = World::new(args.size_x, args.size_y, args.temperature, args.seed, args.diameter);
    let mut canvas = Canvas::new(800, 800);

    world.init_random(args.init_atoms, args.diameter);

    for t in 0..args.t_max{
        world.step();
        if args.draw && t%args.draw_every==0 {
            canvas.draw(&world, &t);
        }
    }
}