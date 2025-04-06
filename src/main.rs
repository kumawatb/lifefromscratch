use core::args::RunMode;
use bevy::{prelude::*, window::WindowResolution};
use avian2d::prelude::*;
use rand::SeedableRng;
mod core;
use crate::core::{args::ArgsPlugin, atom::AtomsPlugin, args::Args, chemistry::ChemistryPlugin};
use rand_xoshiro::Xoshiro256Plus;


const WINDOW_SIZE_X: f32 = 800.0 ;
const WINDOW_SIZE_Y: f32 = 800.0 ;

fn main(){
    
    let mut app = App::new();
    app
        .add_plugins(ArgsPlugin)
        .add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, setup_sim)                        
        .add_plugins(AtomsPlugin)
        .add_plugins(ChemistryPlugin)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(
                Window {
                resolution: WindowResolution::new(WINDOW_SIZE_X, WINDOW_SIZE_Y),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::WHITE))
        .add_systems(Startup, spawn_camera);

    let runmode = app.world().get_resource::<Args>().unwrap().mode;
    match runmode{
        RunMode::Debug => {
            app.add_plugins(PhysicsDebugPlugin::default());
        },
        RunMode::Visual => {},
    }
    
    app.run();

}



fn spawn_camera(mut commands: Commands) {
    commands
      .spawn_empty()
      .insert(Camera2d);
}


/// RANDOM NUMBER GENERATOR RESOURCE ##################
#[derive(Resource)]
struct SimRng(Xoshiro256Plus);

fn setup_sim(
    mut commands: Commands,
    args: Res<Args>
){
    let world_rng = if args.seed==0 { Xoshiro256Plus::from_os_rng() } else { Xoshiro256Plus::seed_from_u64(args.seed) };
    commands.insert_resource( SimRng(world_rng) );
}

