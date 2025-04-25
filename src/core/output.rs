use std::{fs::{self, File}, path::Path, time::Duration};
use std::io::Write;
use ahash::AHashMap;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use petgraph::{algo::{is_isomorphic_matching, tarjan_scc}, dot::Dot, prelude::*};
use itertools::Itertools;

use super::{args::Args, atom::{Atom, BondGraph}};

pub struct OutputPlugin;

// Output two things
// 1. A folder containing all the molecules in a familiar graph format (like graphviz)

// 2. A file that is just the number of different molecule types at every time point that is recorded
//      Eg.
//         time,molecule_name,number_of_molecules
//         0.1,{name1},2
//         0.4,{name1},3


impl Plugin for OutputPlugin {
    fn build(&self, app: &mut App) {
        let output_interval = app.world().get_resource::<Args>().unwrap().output_every;

        app
            .insert_resource(Output::default())
            .add_systems(Update, output_molecules.run_if(on_timer(Duration::from_secs_f32(output_interval))));
    }
}

#[derive(Resource)]
pub struct Output {
    struct_dir: String,
    countsfile: File,
    moltypes: Vec<Graph<u8,u8,Undirected>>
}

impl Default for Output{
    fn default() -> Self {
        // Create the output directory in ./output/structures
        if !Path::new("output/structures").exists() {
            fs::create_dir_all("output/structures").expect("Failed to create output and structures directory");
        }
        // Create the output file at ./output/molecule_counts.csv
        // Create the data file
        let mut countsfile = File::create("output/molecule_counts.csv").expect("Failed to create molecule data file");
        writeln!(countsfile, "time,molecule_name,counts").expect("Failed to write to data file");

        // Create a molecule dictionary (used to lookup if a molecule already exists)
        let moltypes: Vec<Graph<u8,u8,Undirected>> = Vec::new();

        Output {struct_dir:String::from("output/structures"), countsfile: countsfile, moltypes: moltypes}
    }
}

fn output_molecules(
    bgraph: ResMut<BondGraph>,
    atoms: Query<&Atom>,
    mut output: ResMut<Output>
){
    println!("Outputting molecules...");
    // Dictionary of molecule numbers at this update
    let mut counts: AHashMap<usize, u16> = AHashMap::new();

    // Get vector of components in bondgraph
    let components = tarjan_scc(&bgraph.0);

    for component in components.iter(){ // for each component (vector of entity ids)
        // Create a new graph for the component, using atom species as attributes
        let mut component_species_graph: Graph<u8, u8, Undirected>  = Graph::new_undirected();
        for entityid_pair in component.into_iter().combinations(2){ // Get all combinations of 2 entity ides for each component
            // Get the atom species from the entity id
            if bgraph.0.contains_edge(*entityid_pair[0], *entityid_pair[1]){ // Check if the edge exists in original graph
                let species1 = atoms.get(*entityid_pair[0]).unwrap().0; // Get species from the atom entity 1
                let species2 = atoms.get(*entityid_pair[1]).unwrap().0; // Get species from the atom entity 2

                let node1 = component_species_graph.add_node(species1); // Add the node to the component graph
                let node2 = component_species_graph.add_node(species2); // Add the node to the component graph
                // Add the edge to this component graph
                component_species_graph.add_edge(node1,node2, 1); // Add the edge to the component graph
            }
        }

        let mut component_exists_as_molecule = false;
        // Check if the component is isomorphic to any existing molecule in the dictionary
        for (molid, molecule) in output.moltypes.iter().enumerate(){
            if is_isomorphic_matching(molecule, &component_species_graph, |n1, n2| *n1==*n2, |_, _| true){
                // If it is, increment the count of this molecule
                counts.entry(molid).and_modify(|count| *count += 1); 
                component_exists_as_molecule = true;
                break;
            }
        }

        // If we went through all molecules and did not find a match for this component
        if ! component_exists_as_molecule{
            output.moltypes.push(component_species_graph);
            let molid = output.moltypes.len() - 1; // Get the id of the new molecule
            counts.insert(molid, 1); // Add the new molecule to the dictionary
            // Create a new graphviz file for the new molecule

            let dotfile = Dot::new(&output.moltypes[molid]);

            let filename = format!("{}/molecule_{}.dot", output.struct_dir, molid);
            let mut file = File::create(&filename).expect("Failed to create graphviz file");
            writeln!(file, "{}", dotfile).expect("Failed to write to graphviz file");
        }
    }

    // Write the counts to the output file
    for (molid, count) in counts.iter(){
        // Get the name of the molecule
        let molname = format!("molecule_{}", molid);
        // Write to the output file
        writeln!(output.countsfile, "{},{},{}", 0, molname, count).expect("Failed to write to data file");
    }
}


