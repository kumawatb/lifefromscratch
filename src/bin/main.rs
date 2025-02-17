use clap::Parser;
use lifefromscratch::core::{Canvas, World};
use minifb_geometry::GeometryDrawer;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use minifb_fonts::*;

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
    #[arg(long, default_value_t = 1.0)]
    diameter: f32,

    /// Number of atom species (<=256)
    #[arg(long, default_value_t = 256)]
    num_species: u16,

    /// Number of atom states (<=256)
    #[arg(long, default_value_t = 256)]
    num_states: u16,

    /// Initial number of atoms
    #[arg(long, default_value_t = 100)]
    init_atoms: u32,
    
    /// Number of time steps to run the simulation
    #[arg(long, default_value_t = 100000000)]
    t_max: usize,

    /// Random seed (-1 for random)
    #[arg(long)]
    seed: Option<u32>,

    /// Visualize the space?
    #[arg(long, default_value_t=false)]
    draw: bool,

    /// Skip draw_skip steps between drawing
    #[arg(long, default_value_t=1)]
    draw_every: usize
}

fn main(){
    let args = Cli::parse();

    let mut world = World::new(args.size_x, args.size_y, args.diameter, 1.0);
    let mut canvas = Canvas::new(800, 800);

    world.add_atom_at(50.0, 50.0, 0, 0);

    for t in 0..args.t_max{
        world.step();
        if args.draw && args.t_max%args.draw_every==0 {
            canvas.draw(&world, &t);
        }
    }
}








