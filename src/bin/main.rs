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

// fn draw(world: &World){
//     //const WIDTH: usize = 800;
//     //const HEIGHT: usize = 800;

//     //let mut noise;
//     //let mut carry;
//     //let mut seed = 0xbeefu32;

//     //let mut buffer = vec![0u32; WIDTH * HEIGHT];
//     //let geometry = GeometryDrawer::new(WIDTH);


//     // let mut window = Window::new(
//     //     "Noise Test - Press ESC to exit",
//     //     WIDTH,
//     //     HEIGHT,
//     //     WindowOptions {
//     //         resize: true,
//     //         scale_mode: ScaleMode::UpperLeft,
//     //         ..WindowOptions::default()
//     //     },
//     // )
//     // .expect("Unable to create the window");

//     //window.set_target_fps(0);

//     let mut size = (0, 0);

//     while window.is_open() && !window.is_key_down(Key::Escape) {
        // let new_size = window.get_size();
        // if new_size != size {
        //     size = new_size;
        //     buffer.resize(size.0 * size.1, 0);
        // }

//         for pixel in buffer.iter_mut() {
//             noise = seed;
//             noise >>= 3;
//             noise ^= seed;
//             carry = noise & 1;
//             noise >>= 1;
//             seed >>= 1;
//             seed |= carry << 30;
//             noise &= 0xFF;

//             *pixel = (noise << 16) | (noise << 8) | noise;
//         }

//         let _ = geometry.draw_box(&mut buffer, 120, 130, 220, 230, 0xffff00);
//         let mut font_renderer = font6x8::new_renderer(WIDTH, HEIGHT, 0xffff00);

//         font_renderer.draw_text(&mut buffer, 10, 40, "Font-6x8");
//         font_renderer.set_color(0xff_ff_ff);
//         font_renderer.draw_text(&mut buffer, 10, 50, "Rusty claws play Dm on a xylophone, quickly bizarre, just for given vibes. ");

//         window
//             .update_with_buffer(&buffer, new_size.0, new_size.1)
//             .unwrap();
//     }
// }







