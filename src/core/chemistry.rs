use std::{fs::File, io::{BufRead, BufReader}};

use ahash::{AHashMap, AHashSet};
use bevy::prelude::*;
use avian2d::{dynamics::solver::xpbd::XpbdConstraint, prelude::*};
use super::{atom::Atom, args::Args};

/// Bevy Plugin to initialize and work with atoms
pub struct ChemistryPlugin;

impl Plugin for ChemistryPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, load_chemistry);
        app.insert_resource(BondMap::default());
        app.add_systems(Update,check_collisions_and_react);
        app.add_systems(Update, decompose);
    }
}


fn load_chemistry(mut commands: Commands, args: Res<Args>){
    // Load chemistry from a file
    let chemistry = Chemistry::new(args.chempath.clone());

    commands.insert_resource(chemistry);

}


fn check_collisions_and_react(
    mut commands: Commands,
    mut collision_events: EventReader<Collision>,
    mut atoms: Query<&mut Atom>,
    chem: Res<Chemistry>,
    mut bmap: ResMut<BondMap>,
    args: Res<Args>
) {
    for Collision(contacts) in collision_events.read() {

        // Get colliding atoms
        let [mut atom1, mut atom2] = atoms.get_many_mut([contacts.entity1, contacts.entity2]).unwrap();

        // Check if the atoms are already bonded
        if ! ( bmap.bonds.contains(&(contacts.entity1, contacts.entity2))
             || bmap.bonds.contains(&(contacts.entity2, contacts.entity1))) 
        {
            // Check if atoms are candidates for a combination or excitation reaction
            if let Some(rxn) = chem.get_products(Reactant(atom1.0, atom1.1), Reactant(atom2.0, atom2.1)) {
                match rxn {
                    ReactionResult::Combine(prod1_state, prod2_state) => {
                        commands.spawn(DistanceJoint::new(contacts.entity1, contacts.entity2).with_rest_length(args.diameter*1.1).with_linear_velocity_damping(20.0));
                        atom1.1 = prod1_state;
                        atom2.1 = prod2_state;
                        bmap.bonds.insert((contacts.entity1, contacts.entity2));
                    },
                    ReactionResult::Excite(prod1_state, prod2_state) => {
                        atom1.1 = prod1_state;
                        atom2.1 = prod2_state;
                    },
                    _ => {}
                }
            }
        }
    }
}

fn decompose(
    mut commands: Commands,
    mut joints: Query<(Entity, &mut DistanceJoint)>,
    mut atoms: Query<&mut Atom>,
    chem: Res<Chemistry>,
    mut bmap: ResMut<BondMap>,
) {
    // Iterate over all bonds 
    // check if the entities on the bond satisfy a decomposition reaction
    for joint in joints.iter_mut(){
        let atom_entities = joint.1.entities();
        let [mut atom1, mut atom2] = atoms.get_many_mut(atom_entities).unwrap();

        if let Some(ReactionResult::Decompose(prod1_state, prod2_state)) = chem.get_products(Reactant(atom1.0, atom1.1), Reactant(atom2.0, atom2.1)) {
            atom1.1 = prod1_state;
            atom2.1 = prod2_state;
            bmap.bonds.remove(&(atom_entities[0], atom_entities[1]));
            bmap.bonds.remove(&(atom_entities[1], atom_entities[0]));
            commands.entity(joint.0).despawn();
        }
    }
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

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub struct Reactant(u8,u8); // Reactant.0 = species, Reactant.1 = state

/// Enum to represent the result of a reaction
#[derive(Clone, Copy)]
pub enum ReactionResult{

    /// Combine atoms on collision
    Combine(u8, u8), // (species1, state1) + (species2, state2) -> ((species1, state1')=(species2, state2')), (u8, u8) = state1', state2'

    /// Decompose atoms
    Decompose(u8, u8), // ((species1, state1)=(species2, state2)) -> (species1, state1') + (species2, state2'), (u8, u8) = state1', state2'

    /// Excite atoms on collision
    Excite(u8, u8) // (species1, state1) + (species2, state2) -> (species1, state1') + (species2, state2'), (u8, u8) = state1', state2'
}

#[derive(Resource)]
pub struct Chemistry{
    rxns: AHashMap<(Reactant, Reactant), ReactionResult>
}

impl Chemistry{
    pub fn new(chempath: String) -> Self {

        let mut all_reactions: AHashMap<(Reactant, Reactant), ReactionResult> = AHashMap::new();

        // Use chemfile specification to create reactions
        let f = File::open(chempath).expect("Unable to open chemistry file");
        let buffer = BufReader::new(f); 
        
        for line in buffer.lines(){
            let line = line.expect("Unable to read line from chemistry file");
            let mut line = line.trim().replace(" ", "").replace("\t", "").replace("\r", "");
            let offset = line.find("#").unwrap_or(line.len());
            line.truncate(offset);

            if !(line.is_empty() || line.starts_with('#')) {
                let parts: Vec<&str> = line.split("->").collect();
                if parts.len() != 2 {
                    println!("Invalid reaction format, skipping line: {}", line);
                    continue;
                }
                let reactant_substr = parts[0];
                let product_substr = parts[1];

                if reactant_substr.contains("+") && product_substr.contains("=") {
                    // Combination reaction
                    let rxn_components = Chemistry::get_reactants_and_products(reactant_substr, product_substr, "+", "=");

                    if let Some((reac1, reac2, result)) = rxn_components {
                        all_reactions.insert((reac1, reac2), ReactionResult::Combine(result.0,result.1));
                    } else {
                        println!("Invalid reaction format, skipping line: {}", line);
                    }

                } else if reactant_substr.contains("=") && product_substr.contains("+") {
                    // Decomposition reaction

                    let rxn_components = Chemistry::get_reactants_and_products(reactant_substr, product_substr, "=", "+");

                    if let Some((reac1, reac2, result)) = rxn_components {
                        all_reactions.insert((reac1, reac2), ReactionResult::Decompose(result.0,result.1));
                    } else {
                        println!("Invalid reaction format, skipping line: {}", line);
                    }

                } else if reactant_substr.contains("+") && product_substr.contains("+") {
                    // Excitation reaction
                    let rxn_components = Chemistry::get_reactants_and_products(reactant_substr, product_substr, "+", "+");

                    if let Some((reac1, reac2, result)) = rxn_components {
                        all_reactions.insert((reac1, reac2), ReactionResult::Excite(result.0,result.1));
                    } else {
                        println!("Invalid reaction format, skipping line: {}", line);
                    }

                }
            } 

        }

        Chemistry {
            rxns: all_reactions
        }
    }

    pub fn get_products(&self, reac1: Reactant, reac2: Reactant) -> Option<ReactionResult> {

        // Check reactants in chemistry in both orders (reac1 first vs reac2 first)
        let order_0 = self.rxns.get(&(reac1, reac2)).copied();
        let order_1 = self.rxns.get(&(reac2, reac1)).copied();

        if let Some(rxn) = order_0 {
            Some(rxn)
        } else if let Some(rxn) = order_1 { // Flip product order if reactants found in reverse order
            match rxn {
                ReactionResult::Combine(prod1_state, prod2_state) => {
                    Some(ReactionResult::Combine(prod2_state, prod1_state))
                },
                ReactionResult::Decompose(prod1_state, prod2_state) => {
                    Some(ReactionResult::Decompose(prod2_state, prod1_state))
                },
                ReactionResult::Excite(prod1_state, prod2_state) => {
                    Some(ReactionResult::Excite(prod2_state, prod1_state))
                }
            }
        } else {
            None
        }
    }

    pub fn get_reactants_and_products(reactant_substr: &str, product_substr: &str, char1: &str, char2: &str) -> Option<(Reactant, Reactant, (u8,u8))> {
        let reactants = reactant_substr.split(char1).collect::<Vec<&str>>();
        let products= product_substr.split(char2).collect::<Vec<&str>>();

        if reactants.len() != 2 || products.len() != 2 {
            return None;
        }

        let reac1 = Chemistry::parse_species_and_state(reactants[0]);
        let reac2 = Chemistry::parse_species_and_state(reactants[1]);
        let prod1 = Chemistry::parse_species_and_state(products[0]);
        let prod2 = Chemistry::parse_species_and_state(products[1]);

        if let (Some(reac1), Some(reac2), Some(prod1), Some(prod2)) = (reac1, reac2, prod1, prod2) {
            if reac1.0 != prod1.0 || reac2.0 != prod2.0 {
                // If species of reactants and products do not match, return None (species changing reactions are not allowed)
                return None            
            }

            Some((                    
                Reactant(reac1.0, reac1.1),
                Reactant(reac2.0, reac2.1),
                (prod1.1, prod2.1)
            ))
        } else{
            None
        }
    }

    pub fn parse_species_and_state(instr: &str) -> Option<(u8,u8)> {
        let parts = instr.split(['{','}']).collect::<Vec<&str>>();
        let spec = u8::from_str_radix(parts[0], 16);
        let state = u8::from_str_radix(parts[1], 16);

        if let (Ok(spec), Ok(state)) = (spec, state) {
            Some((spec, state))
        } else {
            println!("Invalid species or state: {} {}", parts[0], parts[1]);
            None
        }
    }
}