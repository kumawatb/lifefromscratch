use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::core::movement::Position;


/// Bevy Plugin to initialize and work with atoms
pub struct AtomsPlugin;

impl Plugin for AtomsPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_atoms);
        app.add_systems(Update, diffuse_atoms);
    }
}

/// Struct to represent an atom in the simulation (Field 0: Species, Field 1: State)
#[derive(Component, Default)]
#[require(Position)]
pub struct Atom{
    species: u8, 
    state: u8
}

fn spawn_atoms(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
)
{
    let shape = Circle::new(5.0);
    let color = Color::srgb(1., 0., 0.);

    let mesh = meshes.add(shape);
    let material = materials.add(color);

    commands.spawn( (Atom{species: 0, state: 0}, Mesh2d(mesh),  MeshMaterial2d(material)) );
}


fn diffuse_atoms(
    mut atom: Query<&mut Position, With<Atom>>
    ){
    if let Ok(mut pos) = atom.get_single_mut() {
        pos.0.x += 1.0;
        pos.0.y += 1.0;
    }
}

