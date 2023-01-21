mod construct_graph;
mod find_path;
mod parse_input;

use crate::construct_graph::Graph;
use crate::find_path::PathFinder;
use crate::parse_input::{parse_graph_nodes_from_string, split_contents_into_nodes_edges_routes};

use log::debug;
use std::{env, fs};

fn main() -> Result<(), String> {
    env_logger::init();
    // read input
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(
            "Please provide relative file path as input arg, i.e. `$ cargo run <src/test/uk.txt>`"
                .to_string(),
        );
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename.to_string());
    match contents {
        Err(_) => {
            let current_dir =
                env::current_dir().expect("Path provided was incorrect. File not found.");
            println!(
                "Path provided ({}/{}) was incorrect. File not found.",
                current_dir.display(),
                filename
            )
        }
        Ok(file_path) => {
            let mut path_finder = PathFinder::new_from_string(&file_path)?;
            path_finder.dijkstra_multiple_routes()?;
            println!("{:?}", path_finder.solutions);
        }
    }

    Ok(())
}
