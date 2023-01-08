
fn read_input(contents: String) -> Result<(String, String, String), String> {
    let data: Vec<&str> = contents.split("\n\n").collect();
    if data.len() != 3 {
        return Err("Invalid file format.".to_string());
    }
    let node_data = data[0].to_string();
    let edge_data = data[1].to_string();
    let routes_to_find = data[2].to_string();

    return Ok((node_data, edge_data, routes_to_find));
}

fn get_nodes(node_data: &str) -> Vec<GraphNode> {
    let nodes: Vec<&str> = node_data.split("\n").collect();
    let num_nodes: usize = nodes[0]
        .parse::<usize>()
        .expect("Expect an integer number of nodes.");

    if nodes.len() != num_nodes + 1 {
        println!("Unexpected number of nodes");
    }

    let mut graph_nodes = Vec::with_capacity(num_nodes);

    for i in 1..(num_nodes + 1) {
        graph_nodes.push(GraphNode {
            index: i - 1,
            node_name: nodes[i].to_string(),
        });
    }

    debug!("graph nodes: {:?}", graph_nodes);

    return graph_nodes;
}

fn get_edge_info(
    edge: &str,
    graph_nodes: &Vec<GraphNode>,
) -> Result<(usize, usize, usize), String> {
    let edge_info: Vec<&str> = edge.split(" ").collect();
    if edge_info.len() != 3 {
        return Err(format!(
            "Route {edge:?} is invalid. Please check the input.",
            edge = edge_info
        ));
    }
    let start_edge = edge_info[0];
    let end_edge = edge_info[1];
    let edge_weight = edge_info[2].parse::<usize>().expect(&format!(
        "Distance between edges should be an integer, {edge_weight} found.",
        edge_weight = edge_info[2]
    ));

    let start_index = get_node_index_from_node_name(start_edge.to_string(), graph_nodes)?;
    let end_index = get_node_index_from_node_name(end_edge.to_string(), graph_nodes)?;

    return Ok((start_index, end_index, edge_weight));
}


fn get_route(
    first_route: Vec<&str>,
    graph_nodes: &Vec<GraphNode>,
) -> Result<(usize, usize), String> {
    if first_route.len() != 2 {
        return Err(format!(
            "Route {route:?} is invalid. Please check the input.",
            route = first_route
        ));
    }
    let start_str = first_route[0];
    let end_str = first_route[1];
    if start_str == end_str {
        return Err(format!(
            "Route is self referential. Dist from {} to {} = 0",
            start_str, end_str
        ));
    }

    let start_idx = get_node_index_from_node_name(start_str.to_string(), graph_nodes)?;
    let end_idx = get_node_index_from_node_name(end_str.to_string(), graph_nodes)?;

    return Ok((start_idx, end_idx));
}

#[cfg(test)]
mod input_tests {
    use super::*;

    #[test]
    fn test_parsing_data_from_incorrect_format() {
        let incorrect_contents: String = "incorrectly formatted input".to_string();
        assert_eq!(
            Err("Invalid file format.".to_string()),
            read_input(incorrect_contents)
        );
        let contents_no_routes: String = "2\nA\nB\n\n1\nA B 1".to_string();
        assert_eq!(
            Err("Invalid file format.".to_string()),
            read_input(contents_no_routes)
        );
        let contents_wrong_delimiters_edge =
            "3\nI\nG\nE\n\n4\nI G 167\nI E 158\nG,E,45\nI G 17\n\nG E\nE I\n\n".to_string();
        assert_eq!(
            Err("Invalid file format.".to_string()),
            read_input(contents_wrong_delimiters_edge)
        );
        let contents_wrong_delimiters_route =
            "3\nI\nG\nE\n\n4\nI G 167\nI E 158\nG E 45\nI G 17\n\nG,E\nE I\n\n".to_string();
        assert_eq!(
            Err("Invalid file format.".to_string()),
            read_input(contents_wrong_delimiters_route)
        );
    }
    #[test]
    fn test_route_extraction() {
        let graph_nodes = vec![
            GraphNode {
                index: 0,
                node_name: "Inverness".to_string(),
            },
            GraphNode {
                index: 1,
                node_name: "Glasgow".to_string(),
            },
            GraphNode {
                index: 2,
                node_name: "Edinburgh".to_string(),
            },
        ];

        let (start_idx, end_idx) = get_route(vec!["Glasgow", "Edinburgh"], &graph_nodes).expect("");
        assert_eq!(start_idx, 1);
        assert_eq!(end_idx, 2);
    }
}
