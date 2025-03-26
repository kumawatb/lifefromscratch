use ahash::AHashSet;
use bevy::prelude::*;
use avian2d::prelude::*;

use super::{atom::Atom, args::Args};

/// Bevy Plugin to initialize and work with atoms
pub struct ChemistryPlugin;

impl Plugin for ChemistryPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, load_chemistry);
        app.insert_resource(BondMap::default());
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
    mut collision_events: EventReader<Collision>,
    atoms: Query<(&Atom, &Transform)>,
    rxns: Query<&ReactionTypes>,
    mut bmap: ResMut<BondMap>,
    args: Res<Args>
) {
    for Collision(contacts) in collision_events.read() {
        let (Atom(species1, state1), Transform{ translation: pos1, rotation: rot1, ..}) = atoms.get(contacts.entity1).unwrap();
        let (Atom(species2, state2), Transform{ translation: pos2, rotation: rot2, ..}) = atoms.get(contacts.entity2).unwrap();

        if ! ( bmap.bonds.contains(&(contacts.entity1, contacts.entity2))
             || bmap.bonds.contains(&(contacts.entity2, contacts.entity1))) 
        {
            for rxn in rxns.iter(){
                match rxn {
                    ReactionTypes::Combine(
                        (reac1, reac1_state),(reac2, reac2_state), 
                        ((prod1, prod1_state) , (prod2, prod2_state)) ) 
                        if *reac1 == *species1 && *reac2 == *species2 && *reac1_state == *state1 && *reac2_state == *state2  => 
                        {
                            
                            commands.spawn(DistanceJoint::new(contacts.entity1, contacts.entity2).with_rest_length(args.diameter*1.2));

                    
                        }, 
                    
                    _ => {}               
                }
            }
        }
    }

        // if let  CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
        //     let (Atom(species1, state1), Transform{ translation: pos1, rotation: rot1, ..}) = atoms.get(*entity1).unwrap();
        //     let (Atom(species2, state2), Transform{ translation: pos2, rotation: rot2, ..}) = atoms.get(*entity2).unwrap();

        //     if ! (bmap.bonds.contains(&(*entity1, *entity2))
        //     || bmap.bonds.contains(&(*entity2, *entity1))) {
        //         // Search reactions for a ReactionTypes::Combine match
        //         for rxn in rxns.iter(){
        //             match rxn {

        //                 // If colliding atoms' species and states match a reaction's reactants
        //                 ReactionTypes::Combine(
        //                     (reac1, reac1_state),(reac2, reac2_state), 
        //                     ((prod1, prod1_state) , (prod2, prod2_state)) ) 
        //                     if *reac1 == *species1 && *reac2 == *species2 && *reac1_state == *state1 && *reac2_state == *state2 => 
        //                 {
                            
        //                     let vec12 = (pos2 - pos1).truncate();
        //                     let midpoint = (pos2+pos1)/2.0;

        //                     // Revolute trial 300
        //                     // let revjoint1 = RevoluteJointBuilder::new()
        //                     // .local_anchor1(Vec2::new(0.0, 0.0))
        //                     // .local_anchor2(Vec2::new(-args.diameter*0.6, 0.0));



        //                     // let revjoint2 = RevoluteJointBuilder::new()
        //                     // .local_anchor1(Vec2::new(0.0, 0.0))
        //                     // .local_anchor2(Vec2::new(args.diameter*0.6, 0.0));


        //                     // let bond = commands.spawn((
        //                     //     RigidBody::Dynamic,
        //                     //     Transform {
        //                     //         translation: midpoint,
        //                     //         rotation: Quat::from_rotation_z(vec12.y.atan2(vec12.x)),
        //                     //         ..Default::default()
        //                     //     },
        //                     //     AdditionalMassProperties::MassProperties(MassProperties { mass: 0.00001, principal_inertia: 0.00001, ..Default::default() }),
        //                     //     Friction::coefficient(0.0),
        //                     //     GravityScale(0.0)
        //                     // )).id();

        //                     // commands.entity(bond).with_children(|cmd| {
        //                     //     cmd.spawn(ImpulseJoint::new(*entity1, revjoint1));
        //                     //     cmd.spawn(ImpulseJoint::new(*entity2, revjoint2));
        //                     // });

        //                     // bmap.bonds.insert((*entity1, *entity2));                        
        //                 },
        //                 _ => {}
        //             }
        //         }
        //     }
        // }        
}


#[derive(Resource)]
struct BondMap{
    bonds: AHashSet<(Entity, Entity)>
}

impl Default for BondMap{
    fn default() -> Self{
        Self{
            bonds: AHashSet::new()
        }
    }
}