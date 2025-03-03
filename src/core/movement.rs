// use bevy::prelude::*;

// /// Bevy Plugin for initializing positions etc to move objects
// pub struct MovementPlugin;

// impl Plugin for MovementPlugin {
//     fn build(&self, app: &mut App){
//         //app.add_systems(Update, project_positions);
//     }
// }


// /// Position component
// #[derive(Component, Default)]
// #[require(Transform)]
// pub struct Position(pub Vec2);

// /// Move all objects that have a position and transform
// fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
//     // Here we are iterating over the query to get the
//     // components from our game world
//     for (mut transform, position) in &mut positionables {
//       transform.translation = position.0.extend(0.);
//     }
// }


