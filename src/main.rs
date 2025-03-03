use std::num::{NonZero, NonZeroUsize};

use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier2d::{prelude::*, rapier::prelude::IntegrationParameters};
use rand::SeedableRng;
mod core;
use crate::core::{args::ArgsPlugin, atom::AtomsPlugin, args::Args};
use rand_xoshiro::Xoshiro256Plus;


const PIXELS_PER_METER: f32 = 1.0;
const WINDOW_SIZE_X: f32 = 800.0 ;
const WINDOW_SIZE_Y: f32 = 800.0 ;

fn main(){
    let _ = App::new()
        .add_plugins(ArgsPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER))
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_sim)                        
        .add_plugins(AtomsPlugin)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(
                Window {
                resolution: WindowResolution::new(WINDOW_SIZE_X, WINDOW_SIZE_Y),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, spawn_camera)
        .run();

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
    args: Res<Args>,
    mut context: Query<&mut RapierContextSimulation>
){
    let world_rng = if args.seed==0 { Xoshiro256Plus::from_os_rng() } else { Xoshiro256Plus::seed_from_u64(args.seed) };
    commands.insert_resource( SimRng(world_rng) );

    context.single_mut().integration_parameters = IntegrationParameters{  
        //length_unit: args.diameter/100.0,
        normalized_max_corrective_velocity: args.diameter*100.0,
        ..Default::default() 
    };
}

