use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{atom::Atom, args::Args};

/// Bevy Plugin to initialize and work with atoms
pub struct ChemistryPlugin;

impl Plugin for ChemistryPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, load_chemistry);
        app.add_systems(Update,check_collisions_and_react);
    }
}


fn load_chemistry(mut commands: Commands){
    // Load chemistry from a file later
    // For now, create a simple reaction
    commands.spawn( ReactionTypes::Combine((0,0), (0,0), ((0,1),(0,1))) );
}

/// Enum to represent a reaction in the simulation
#[derive(Component)]
enum ReactionTypes{
    Combine((u8,u8), (u8,u8), ((u8,u8),(u8,u8))), // (species1, state1) + (species2, state2) -> ((species1, state1')=(species2, state2'))
    Decompose(((u8,u8), (u8,u8)), (u8,u8), (u8,u8)) // ((species1, state1)=(species2, state2)) -> (species1, state1') + (species2, state2')
}

fn check_collisions_and_react(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    atoms: Query<(&Atom, &Transform)>,
    rxns: Query<&ReactionTypes>,
    args: Res<Args>
) {
    for collision_event in collision_events.read() {
        if let  CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
                let (Atom(species1, state1), Transform{ translation: pos1, ..}) = atoms.get(*entity1).unwrap();
                let (Atom(species2, state2), Transform{ translation: pos2, ..}) = atoms.get(*entity2).unwrap();

                // Search reactions for a ReactionTypes::Combine match
                for rxn in rxns.iter(){
                    match rxn {

                        // If colliding atoms' species and states match a reaction's reactants
                        ReactionTypes::Combine(
                            (reac1, reac1_state),(reac2, reac2_state), 
                            ((prod1, prod1_state) , (prod2, prod2_state)) ) 
                            if *reac1 == *species1 && *reac2 == *species2 && *reac1_state == *state1 && *reac2_state == *state2 => 
                        {
                            //println!("Reaction found: Combine | {} {} + {} {} -> {} {} = {} {}", reac1, reac1_state, reac2, reac2_state, prod1, prod1_state, prod2, prod2_state);
                            
                            let vec12 = (pos2 - pos1).truncate();
                            let direction = vec12.normalize();
                            let midpoint = ((pos1 + pos2)/2.0).truncate();

                            // Spawn a joint connecting the atoms
                            commands
                            .spawn((
                                Bond{},
                                RigidBody::Dynamic,
                                AdditionalMassProperties::Mass(0.01),
                                Transform::from_xyz(midpoint.x, midpoint.y, 0.0),
                                Velocity{
                                    linvel: Vec2::new(0.0, 0.0),
                                    angvel: 0.0
                                }
                            ))
                            .with_children(|children| {
                                children.spawn(ImpulseJoint::new(*entity1, RevoluteJointBuilder::new()
                                        .local_anchor1(Vec2::new(0.0,0.0)).local_anchor2(-direction * (args.diameter/2.0) * 1.2)));
                                children.spawn(ImpulseJoint::new(*entity2, RevoluteJointBuilder::new()
                                        .local_anchor1(Vec2::new(0.0,0.0)).local_anchor2(direction * (args.diameter/2.0) * 1.2)));
                            });
                        },
                        _ => {}
                    }
                }

                // Move atoms to a distance = 2*r

                // Create a fixed joint between the two atoms


        }        
    }
}


/// Struct to represent a bond in the simulation
#[derive(Component)]
pub struct Bond;