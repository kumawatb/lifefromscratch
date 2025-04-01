use std::{fs::File, io::{BufRead, BufReader}};

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


fn load_chemistry(mut commands: Commands, args: Res<Args>){
    // Load chemistry from a file
    let chemistry = Chemistry::new(args.chempath.clone());

    for rxn in chemistry.iter_rxn(){
     commands.spawn(*rxn);
    }

}

/// Enum to represent a reaction in the simulation
#[derive(Component, Clone, Copy)]
pub enum Reaction{

    /// Combine atoms on collision
    Combine((u8,u8), (u8,u8), ((u8,u8),(u8,u8))), // (species1, state1) + (species2, state2) -> ((species1, state1')=(species2, state2'))

    /// Decompose atoms
    Decompose(((u8,u8), (u8,u8)), (u8,u8), (u8,u8)), // ((species1, state1)=(species2, state2)) -> (species1, state1') + (species2, state2')

    /// Excite atoms on collision
    Excite((u8,u8), (u8,u8), (u8,u8), (u8,u8)) // (species1, state1) + (species2, state2) -> (species1, state1') + (species2, state2')
}

fn check_collisions_and_react(
    mut commands: Commands,
    mut collision_events: EventReader<Collision>,
    mut atoms: Query<&mut Atom>,
    rxns: Query<&Reaction>,
    mut bmap: ResMut<BondMap>,
    args: Res<Args>
) {
    for Collision(contacts) in collision_events.read() {
        let [mut atom1, mut atom2] = atoms.get_many_mut([contacts.entity1, contacts.entity2]).unwrap();

        if ! ( bmap.bonds.contains(&(contacts.entity1, contacts.entity2))
             || bmap.bonds.contains(&(contacts.entity2, contacts.entity1))) 
        {
            for rxn in rxns.iter(){
                match rxn {
                    Reaction::Combine(
                        (reac1, reac1_state),(reac2, reac2_state), 
                        ((_, prod1_state) , (_, prod2_state)) ) 
                        if *reac1 == atom1.0 && *reac2 == atom2.0 && *reac1_state == atom1.1 && *reac2_state == atom2.1  => 
                        {
                            commands.spawn(DistanceJoint::new(contacts.entity1, contacts.entity2).with_rest_length(args.diameter*1.1).with_linear_velocity_damping(20.0));

                            atom1.1 = *prod1_state;
                            atom2.1 = *prod2_state;

                            bmap.bonds.insert((contacts.entity1, contacts.entity2));

                            //println!("{:?}", atoms.get(contacts.entity1).unwrap().1);

                        }, 
                    Reaction::Excite( 
                        (reac1, reac1_state), (reac2, reac2_state) ,
                        (_, prod1_state), (_, prod2_state) ) 
                        if *reac1 == atom1.0 && *reac2 == atom2.0  && *reac1_state == atom1.1 && *reac2_state == atom2.1 => {
                            atom1.1 = *prod1_state;
                            atom2.1 = *prod2_state;
                        },
                    _ => {}               
                }
            }
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

pub struct Chemistry{
    rxns: Vec<Reaction>
}

impl Chemistry{
    pub fn new(chempath: String) -> Self {

        let mut all_reactions: Vec<Reaction> = Vec::new();

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

                    if let Some((reac1, reac2, prod1, prod2)) = rxn_components {
                        all_reactions.push(Reaction::Combine(reac1, reac2, (prod1, prod2)));
                    } else {
                        println!("Invalid reaction format, skipping line: {}", line);
                    }

                } else if reactant_substr.contains("=") && product_substr.contains("+") {
                    // Decomposition reaction

                    let rxn_components = Chemistry::get_reactants_and_products(reactant_substr, product_substr, "=", "+");

                    if let Some((reac1, reac2, prod1, prod2)) = rxn_components {
                        all_reactions.push(Reaction::Decompose((reac1, reac2), prod1, prod2));
                    } else {
                        println!("Invalid reaction format, skipping line: {}", line);
                    }

                } else if reactant_substr.contains("+") && product_substr.contains("+") {
                    // Excitation reaction
                    let rxn_components = Chemistry::get_reactants_and_products(reactant_substr, product_substr, "+", "+");

                    if let Some((reac1, reac2, prod1, prod2)) = rxn_components {
                        all_reactions.push(Reaction::Excite(reac1, reac2, prod1, prod2));
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

    pub fn iter_rxn(&self) -> impl Iterator<Item = &Reaction> {
        self.rxns.iter()
    }

    pub fn get_reactants_and_products(reactant_substr: &str, product_substr: &str, char1: &str, char2: &str) -> Option<((u8,u8), (u8,u8), (u8,u8),(u8,u8))> {
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
            Some(
                (
                    reac1,
                    reac2,
                    prod1,
                    prod2
                )
            )
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