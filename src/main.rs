mod construct_graph;
mod find_path;
mod parse_input;

use crate::construct_graph::Graph;
use crate::find_path::{dijkstra, get_human_readable_route, print_route, PathFinder};
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
            for start_end in path_finder.routes_to_find {
                // todo: parallelise this &learn how to do threading in rust, for loop is slower
                let (dist, route) = dijkstra(start_end.0, start_end.1, &mut path_finder.graph)?;
                let human_readable_route = get_human_readable_route(route, &mut path_finder.graph.graph_nodes)?;
                println!("{}, dist {}", print_route(human_readable_route), dist);

                path_finder.graph.mark_all_edges_as_not_traversed();
            }
        }
    }

    Ok(())
}
