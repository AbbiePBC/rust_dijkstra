mod construct_graph;
mod find_path;
mod parse_input;

use crate::construct_graph::Graph;
use crate::find_path::{dijkstra, get_human_readable_route, print_route};
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
        Ok(file_path) => {
            let mut graph = Graph::new_from_string(&file_path)?;

            debug!("graph: {:?}", graph);
            let graph_nodes = graph.graph_nodes.clone();
            let routes = graph.routes_to_find.clone();
            for start_end in routes {
                // todo: parallelise this &learn how to do threading in rust, for loop is slower
                let (dist, route) = dijkstra(start_end.0, start_end.1, &mut graph)?;
                let human_readable_route = get_human_readable_route(route, &graph_nodes)?;
                println!("{}", print_route(human_readable_route));

                graph.mark_all_edges_as_not_traversed();
            }
        }
    }

    Ok(())
}
