use bevy::prelude::*;
use avian2d::prelude::*;

use super::args::Args;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_walls);
    }
}


fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
    args: Res<Args>
){
    let window_width = window.single().width();
    let window_height = window.single().height();


    let wall_bundle_e = (
        Transform{
            translation: Vec3::new((window_width/2.0) + args.diameter, 0.0, 0.0),
            ..Default::default()
        },
        Mesh2d(meshes.add(Rectangle::new(window_width/200.0,window_height + 2.0 * args.diameter))),
        MeshMaterial2d(materials.add(Color::BLACK)),
        RigidBody::Static,
        Collider::rectangle(window_width/200.0, window_height + 2.0 * args.diameter),
    );
    let wall_bundle_w = (
        Transform{
            translation: Vec3::new(-(window_width/2.0) - args.diameter, 0.0, 0.0),
            ..Default::default()
        },
        Mesh2d(meshes.add(Rectangle::new(window_width/200.0,window_height + 2.0 * args.diameter))),
        MeshMaterial2d(materials.add(Color::BLACK)),
        RigidBody::Static,
        Collider::rectangle(window_width/200.0, window_height+ 2.0 * args.diameter),
    );

    let wall_bundle_n = (
        Transform{
            translation: Vec3::new(0.0, (window_height/2.0) + args.diameter, 0.0),
            ..Default::default()
        },
        Mesh2d(meshes.add(Rectangle::new(window_width + 2.0 * args.diameter, window_height/200.0))),
        MeshMaterial2d(materials.add(Color::BLACK)),
        RigidBody::Static,
        Collider::rectangle(window_width + 2.0 * args.diameter, window_height/200.0),
    );

    let wall_bundle_s = (
        Transform{
            translation: Vec3::new(0.0, -(window_height/2.0) - args.diameter, 0.0),
            ..Default::default()
        },
        Mesh2d(meshes.add(Rectangle::new(window_width+ 2.0 * args.diameter, window_height/200.0))),
        MeshMaterial2d(materials.add(Color::BLACK)),
        RigidBody::Static,
        Collider::rectangle(window_width + 2.0 * args.diameter, window_height/200.0),
    );



    commands.spawn(wall_bundle_e);
    commands.spawn(wall_bundle_w);
    commands.spawn(wall_bundle_n);
    commands.spawn(wall_bundle_s);
}
