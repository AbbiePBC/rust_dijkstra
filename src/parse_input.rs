use crate::construct_graph::Edge;
use log::debug;

#[derive(Debug, Clone, PartialEq)]
pub struct GraphNode {
    pub index: usize,
    pub node_name: String,
}

impl GraphNode {
    pub(crate) fn new(idx_: usize, name_: String) -> GraphNode {
        return GraphNode {
            index: idx_,
            node_name: name_,
        };
    }
}

pub fn split_contents_into_nodes_edges_routes(
    contents: String,
) -> Result<(String, String, String), String> {
    let data: Vec<&str> = contents.split("\n\n").collect();
    if data.len() != 3 {
        return Err("Invalid file format.".to_string());
    }
    let node_data = data[0].to_string();
    let edge_data = data[1].to_string();
    let routes_to_find = data[2].to_string();

    return Ok((node_data, edge_data, routes_to_find));
}

pub fn get_node_index_from_node_name(
    node_name: &str,
    graph_nodes: &Vec<GraphNode>,
) -> Result<usize, String> {
    let graph_node = graph_nodes.iter().find(|&x| x.node_name == node_name);
    match graph_node {
        None => {
            return Err(format!(
            "Nodes in edges should be present in node list. Node {} (possibly others) not found.",
            node_name
        ))
        }
        Some(node) => return Ok(node.index),
    }
}

pub fn parse_graph_nodes_from_string(node_data: &str) -> Result<Vec<GraphNode>, String> {
    let nodes: Vec<&str> = node_data.split("\n").collect();
    let num_nodes: usize = nodes[0]
        .parse::<usize>()
        .expect("Expect an integer number of nodes.");

    if nodes.len() != num_nodes + 1 {
        return Err("Unexpected number of nodes".to_string());
    }

    let mut graph_nodes = Vec::with_capacity(num_nodes);

    for i in 1..(num_nodes + 1) {
        graph_nodes.push(GraphNode::new(i - 1, nodes[i].to_string()));
    }

    debug!("graph nodes: {:?}", graph_nodes);

    return Ok(graph_nodes);
}

pub fn parse_edges_from_string(
    edge_data: &str,
    graph_nodes: &Vec<GraphNode>,
) -> Result<Vec<Edge>, String> {
    let edges: Vec<&str> = edge_data.split("\n").collect();
    let num_edges: usize = edges[0]
        .parse::<usize>()
        .expect("Expect an integer number of edges.");

    if num_edges != edges.len() - 1 {
        return Err(format!(
            "Unexpected number of edges. Expected: {}, actual: {}",
            num_edges,
            edges.len() - 1,
        ));
    }

    let mut useful_edges = Vec::with_capacity(num_edges - 1);

    for idx in 1..edges.len() {
        let edge_info: Vec<&str> = edges[idx].split(" ").collect();
        if edge_info.len() != 3 {
            return Err(format!(
                "Edge {:?} is invalid. Please check the input.",
                edge_info
            ));
        }
        let start_edge = edge_info[0];
        let end_edge = edge_info[1];
        let edge_weight = edge_info[2].parse::<usize>().expect(&format!(
            "Distance between edges should be an integer, {} found.",
            edge_info[2]
        ));

        let start_index = get_node_index_from_node_name(start_edge, graph_nodes)?;
        let end_index = get_node_index_from_node_name(end_edge, graph_nodes)?;

        if start_index != end_index {
            useful_edges.push(Edge::new(start_index, end_index, edge_weight));
        }
    }

    return Ok(useful_edges);
}

pub fn parse_routes_from_string(
    routes_to_find: &str,
    graph_nodes: &Vec<GraphNode>,
) -> Result<Vec<(usize, usize)>, String> {

    let routes: Vec<&str> = routes_to_find.trim().split("\n").collect();
    let mut parsed_routes = Vec::with_capacity(4);
    for route in routes {
        let route_names: Vec<&str> = route.split(" ").collect();
        if route_names.len() != 2 {
            return Err(format!(
                "Route {:?} is invalid. Please check the input.",
                route_names
            ));
        }
        let start_str = route_names[0];
        let end_str = route_names[1];
        if start_str == end_str {
            return Err(format!(
                "Route is self referential. Dist from {} to {} = 0",
                start_str, end_str
            ));
        }

        let start_idx = get_node_index_from_node_name(start_str, graph_nodes)?;
        let end_idx = get_node_index_from_node_name(end_str, graph_nodes)?;
        parsed_routes.push((start_idx, end_idx));
    }

    return Ok(parsed_routes);
}

#[cfg(test)]
mod input_tests {
    use super::*;

    #[test]
    fn test_parsing_data_from_incorrect_format() {
        let incorrect_contents: String = "incorrectly formatted input".to_string();
        assert_eq!(
            Err("Invalid file format.".to_string()),
            split_contents_into_nodes_edges_routes(incorrect_contents)
        );
        let contents_no_routes: String = "2\nA\nB\n\n1\nA B 1".to_string();
        assert_eq!(
            Err("Invalid file format.".to_string()),
            split_contents_into_nodes_edges_routes(contents_no_routes)
        );
        let contents_wrong_delimiters_edge =
            "3\nI\nG\nE\n\n4\nI G 167\nI E 158\nG,E,45\nI G 17\n\nG E\nE I\n\n".to_string();
        assert_eq!(
            Err("Invalid file format.".to_string()),
            split_contents_into_nodes_edges_routes(contents_wrong_delimiters_edge)
        );
        let contents_wrong_delimiters_route =
            "3\nI\nG\nE\n\n4\nI G 167\nI E 158\nG E 45\nI G 17\n\nG,E\nE I\n\n".to_string();
        assert_eq!(
            Err("Invalid file format.".to_string()),
            split_contents_into_nodes_edges_routes(contents_wrong_delimiters_route)
        );
    }
    #[test]
    fn test_route_extraction() {
        let graph_nodes = vec![
            GraphNode::new(0, "Inverness".to_string()),
            GraphNode::new(1, "Glasgow".to_string()),
            GraphNode::new(2, "Edinburgh".to_string()),
        ];

        let (start_idx, end_idx) =
            parse_routes_from_string("Glasgow Edinburgh", &graph_nodes).expect("")[0];
        assert_eq!(start_idx, 1);
        assert_eq!(end_idx, 2);
    }
}
