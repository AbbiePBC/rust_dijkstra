use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions

include!("construct_graph.rs");
include!("find_path.rs");


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
    let contents =
        fs::read_to_string(filename.to_string()).expect("Should have been able to read the file");
    let (node_data, edge_data, routes_to_find) = read_input(contents)?;
    let graph_nodes: Vec<GraphNode> = get_nodes(&node_data);
    let graph = construct_graph_from_edges(&graph_nodes, &edge_data)?;

    debug!("graph: {:?}", graph);

    let routes: Vec<&str> = routes_to_find.trim().split("\n").collect();
    for route in routes {
        // todo: parallelise this &learn how to do threading in rust, for loop is slower
        let result = get_human_readable_solution(route, &graph_nodes, &graph);
        match result {
            Err(err) => println!("An error occured on the path {}. Error: {}", route, err),
            Ok(_) => println!("{}", result.unwrap())
        }
    }

    Ok(())
}
