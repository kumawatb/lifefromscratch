use std::f32::consts::PI;

use bevy::prelude::*;
use avian2d::prelude::*;
use colorgrad::Gradient;
use rand::Rng;
use crate::core::args::Args;
use crate::{setup_sim, SimRng};


/// Bevy Plugin to initialize and work with atoms
pub struct AtomsPlugin;

impl Plugin for AtomsPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_atoms.after(setup_sim));
        app.add_systems(Update, diffuse_atoms);
    }
}

/// Struct to represent an atom in the simulation (Field 0: Species, Field 1: State)
#[derive(Component, Default)]
pub struct Atom(pub u8, pub u8);


fn spawn_atoms(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rng: ResMut<SimRng>,
    args: Res<Args>,
    window: Query<&Window>
)
{
    let shape = Circle::new(args.diameter/2.0);
    let window_width = window.single().width();
    let window_height = window.single().height();

    // Add atoms to random positions inside the world
    for _ in 0..args.init_atoms{
        let species = 0; //rng.0.random::<u8>();
        let state: u8 = 0; //rng.0.random::<u8>();

        let rgb = colorgrad::preset::rainbow().colors(256)[species as usize].to_linear_rgba();
        let color = Color::srgb(rgb[0] as f32, rgb[1] as f32, rgb[2] as f32);

        let mesh = meshes.add(shape);
        let material = materials.add(color);

        let atombundle = (
            Atom(species, state), 
            Transform{
                translation: Vec3::new(window_width * (rng.0.random::<f32>() * 1. - 0.5),  window_height * (rng.0.random::<f32>() * 1. - 0.5), 0.0),
                ..Default::default()
            },
            LinearVelocity(Vec2::new(0.0,0.0)),
            AngularVelocity(0.0),
            Mesh2d(mesh),  
            MeshMaterial2d(material), 
            RigidBody::Dynamic,
            Collider::circle(args.diameter/2.0),
        );
        commands.spawn(atombundle);
        
    }
}


fn diffuse_atoms(
    mut atoms: Query<&mut LinearVelocity,With<Atom>>,
    args: Res<Args>,
    mut rng: ResMut<SimRng>,
    time: Res<Time>
    ){

    for mut vel in &mut atoms{
        let ang = rng.0.random::<f32>() * 2.0 * PI;
        vel.x = args.temperature * args.diameter * ang.cos() * time.delta_secs();
        vel.y = args.temperature * args.diameter * ang.sin() * time.delta_secs();
    }
}
