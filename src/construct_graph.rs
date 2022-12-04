use rstest::*;

pub const INFINITE_DIST: usize = 100000000;

#[derive(Debug, PartialEq)]
pub struct Edge {
    index_first: usize,
    index_second: usize,
    weight: usize,
}

#[derive(Debug, PartialEq)]
pub struct Graph {
    number_of_nodes: usize,
    edges: Vec<Vec<Edge>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GraphNode {
    index: usize,
    node_name: String,
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

fn create_new_edge(start_index: usize, end_index: usize, weight: usize) -> Edge {
    let new_edge = Edge {
        index_first: start_index,
        index_second: end_index,
        weight,
    };
    return new_edge;
}

fn update_existing_edge(graph: &mut Graph, new_edge: &Edge, is_reverse: bool) {
    
    let mut start_index = new_edge.index_first;
    let mut end_index = new_edge.index_second;
    if is_reverse{
        start_index = new_edge.index_second;
        end_index = new_edge.index_first; 
    }

    let new_weight = new_edge.weight;
    let edge_index = graph.edges[start_index]
        .iter()
        .position(|x| x.index_second == end_index);
    match edge_index {
        None => {
            graph.edges[start_index].push(
                create_new_edge(start_index, end_index, new_weight)
            );
        }
        Some(idx) => {
            let old_edge_weight = graph.edges[start_index][idx].weight;
            if old_edge_weight >= new_weight {
                graph.edges[start_index].remove(idx);
            }
            graph.edges[start_index].push(
                create_new_edge(start_index, end_index, new_weight)
            );
        }
    }
}



pub fn construct_graph_from_edges(
    graph_nodes: &Vec<GraphNode>,
    edge_data: &str,
) -> Result<Graph, String> {
    let edges: Vec<&str> = edge_data.split("\n").collect();
    let num_edges: usize = edges[0]
        .parse::<usize>()
        .expect("Expect an integer number of edges.");

    if num_edges != edges.len() - 1 {
        return Err(format!(
            "Unexpected number of edges. Expected: {num_edges}, actual: {edges_len}",
            num_edges = num_edges,
            edges_len = edges.len() - 1,
        ));
    }

    let num_nodes = graph_nodes.len();

    let mut vec: Vec<Vec<Edge>> = Vec::with_capacity(num_nodes);

    for _ in 0..num_nodes {
        vec.push(Vec::with_capacity(num_nodes));
    }
    let mut graph = Graph {
        number_of_nodes: graph_nodes.len(),
        edges: vec,
    };

    for i in 1..(num_edges + 1) {
        let (start_index, end_index, weight) = get_edge_info(edges[i], graph_nodes)?;
        if start_index == end_index {
            // self referential edge, discard
            continue;
        }
        let new_edge = create_new_edge(start_index, end_index, weight);
        update_existing_edge(&mut graph, &new_edge, false);
        // same in reverse, assuming bidirectionality of edges
        update_existing_edge(&mut graph, &new_edge, true);
    }

    return Ok(graph);
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

fn get_node_index_from_node_name(
    node_name: String,
    graph_nodes: &Vec<GraphNode>,
) -> Result<usize, String> {
    let graph_node = graph_nodes.iter().find(|&x| x.node_name == node_name);
    match graph_node {
        None => {
            return Err(format!(
                "Nodes in edges should be present in node list. {} not found.",
                node_name
            ))
        }
        Some(node) => return Ok(node.index),
    }
}

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
mod graph_only_tests {
    use super::*;

    #[fixture]
    fn set_up_tests() -> (String, Graph, Vec<GraphNode>) {
        let contents =
            "3\nI\nG\nE\n\n4\nI G 167\nI E 158\nG E 45\nI G 17\n\nG E\nE I\n\n".to_string();
        let expected_graph = Graph {
            number_of_nodes: 3,
            edges: vec![
                vec![
                    create_new_edge(0, 2, 158),
                    create_new_edge(0, 1, 17),
                ],
                vec![
                    create_new_edge(1, 2, 45),
                    create_new_edge(1, 0, 17),
                ],
                vec![
                    create_new_edge(2, 0, 158),
                    create_new_edge(2, 1, 45),
                ],
            ],
        };

        let graph_nodes = vec![
            GraphNode {
                index: 0,
                node_name: "I".to_string(),
            },
            GraphNode {
                index: 1,
                node_name: "G".to_string(),
            },
            GraphNode {
                index: 2,
                node_name: "E".to_string(),
            },
        ];
        return (contents, expected_graph, graph_nodes);
    }

    #[test]
    fn test_basic_input() {
        let contents =
            "4\nI\nG\nE\nN\n\n5\nI G 167\nI E 158\nG E 45\nG N 145\nE N 107\n\nG E\nE I\n\n";
        let data: Vec<&str> = contents.split("\n\n").collect();

        let routes_to_find = data[2].to_string();

        assert_eq!(routes_to_find, "G E\nE I");
    }
    #[rstest]
    fn test_multiple_start_edges_input() {
        let (contents, expected_graph, _) = set_up_tests();
        let data: Vec<&str> = contents.split("\n\n").collect();

        let node_data = data[0].to_string();
        let edge_data = data[1].to_string();

        let graph_nodes: Vec<GraphNode> = get_nodes(&node_data);
        let graph = construct_graph_from_edges(&graph_nodes, &edge_data);
        // graph should not contain the I->G 167 path, as this should be updated by the I->G 17 path.

        assert_eq!(Ok(expected_graph), graph);
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
    #[rstest]
    fn test_route_finding_with_incorrect_number_of_nodes() {
        let (_, _, graph_nodes) = set_up_tests();
        let edge_data = "4\nI G 167\nI E 158\nG E 45\nI G 17\nE I 1".to_string();

        assert_eq!(
            Err("Unexpected number of edges. Expected: 4, actual: 5".to_string()),
            construct_graph_from_edges(&graph_nodes, &edge_data)
        )
    }
    #[rstest]
    fn test_route_finding_with_incorrect_nodes() {
        let (_, _, graph_nodes) = set_up_tests();
        let edge_data = "4\nI G 167\nI E 158\nG E 45\nI N 17".to_string();

        assert_eq!(
            Err("Nodes in edges should be present in node list. N not found.".to_string()),
            construct_graph_from_edges(&graph_nodes, &edge_data)
        )
    }
    #[rstest]
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
}
