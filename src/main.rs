mod construct_graph;
mod find_path;
mod parse_input;

use crate::construct_graph::{construct_graph_from_edges, GraphNode};
use crate::find_path::get_human_readable_solution;
use crate::parse_input::{get_nodes, read_input};
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
        Ok(_) => {}
    }
    let (node_data, edge_data, routes_to_find) = read_input(contents.unwrap())?;
    let graph_nodes: Vec<GraphNode> = get_nodes(&node_data)?;
    let mut graph = construct_graph_from_edges(&graph_nodes, &edge_data)?;

    debug!("graph: {:?}", graph);

    let routes: Vec<&str> = routes_to_find.trim().split("\n").collect();
    for route in routes {
        // todo: parallelise this &learn how to do threading in rust, for loop is slower
        let result = get_human_readable_solution(route, &graph_nodes, &mut graph);
        match result {
            Err(err) => println!("An error occured on the path {}. Error: {}", route, err),
            Ok(_) => println!("{}", result.unwrap()),
        }
        graph.mark_all_edges_as_not_traversed();
    }

    Ok(())
}
