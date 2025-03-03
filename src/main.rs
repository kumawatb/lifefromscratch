use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier2d::prelude::*;
mod core;
use crate::core::{args::ArgsPlugin, atom::AtomsPlugin, args::Args};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;


const PIXELS_PER_METER: f32 = 1.0;
const WINDOW_SIZE_X: f32 = 800.0 ;
const WINDOW_SIZE_Y: f32 = 800.0 ;

fn main(){
    let mut app = App::new()
                        .add_plugins(DefaultPlugins.set(WindowPlugin {
                            primary_window: Some(
                                Window {
                                resolution: WindowResolution::new(WINDOW_SIZE_X, WINDOW_SIZE_Y),
                                ..default()
                            }),
                            ..default()
                        }))
                        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
                        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER))
                        //.add_plugins(RapierDebugRenderPlugin::default())
                        .add_systems(Startup, setup_rng)
                        // Custom plugins
                        .add_plugins(ArgsPlugin)
                        //.add_plugins(MovementPlugin)
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

#[derive(Resource)]
struct SimRng(Xoshiro256Plus);

fn setup_rng(
    mut commands: Commands,
    args: Res<Args>
){
    let world_rng = if args.seed==0 { Xoshiro256Plus::from_os_rng() } else { Xoshiro256Plus::seed_from_u64(args.seed) };
    commands.insert_resource( SimRng(world_rng) );
}

