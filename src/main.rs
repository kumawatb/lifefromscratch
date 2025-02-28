use bevy::prelude::*;
mod core;
use crate::core::{args::ArgsPlugin, atom::AtomsPlugin, movement::MovementPlugin};

fn main(){
    let mut app = App::new()
                        .add_plugins(DefaultPlugins)
                        // Custom plugins
                        .add_plugins(ArgsPlugin)
                        .add_plugins(MovementPlugin)
                        .add_plugins(AtomsPlugin)
                        .add_systems(Startup, spawn_camera)
                        .run(); // Get command line arguments using clap and add as resource

    

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


fn spawn_camera(mut commands: Commands) {
    commands
      .spawn_empty()
      .insert(Camera2d);
  }