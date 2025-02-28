//use clap::Parser;
//use lifefromscratch::core::{Canvas, World};
use bevy::prelude::*;
mod core;
use crate::core::args::ArgsPlugin;

fn main(){
    let mut app = App::new()
                        // Custom plugins
                        .add_plugins(ArgsPlugin).run(); // Get command line arguments using clap and add as resource


    // ## NON BEVY CODE
    // let mut world = World::new(args.size_x, args.size_y, args.temperature, args.seed, args.diameter);
    // let mut canvas = Canvas::new(800, 800);

    // world.init_random(args.init_atoms, args.diameter);

    // for t in 0..args.t_max{
    //     world.step();
    //     if args.draw && t%args.draw_every==0 {
    //         canvas.draw(&world, &t);
    //     }
    // }

}
