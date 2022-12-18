use crate::parse_input::get_edge_info;

pub const INFINITE_DIST: usize = 100000000;

#[derive(Debug, PartialEq)]
pub struct Edge {
    pub index_first: usize,
    pub index_second: usize,
    pub weight: usize,
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

impl Graph {

    pub fn new(number_of_nodes: usize, edges: Vec<Vec<Edge>>) -> Self {
        Self { number_of_nodes, edges }
    }

    pub fn get_edge_weight(&self, start_index: usize, end_index: usize) -> usize {
        self.edges[start_index][end_index].weight
    }
    pub fn get_number_of_nodes(&self) -> usize {
        self.number_of_nodes
    }
    pub fn get_edges_from_node(&self, starting_node_index :usize) -> &Vec<Edge> {
        &self.edges[starting_node_index]
    }
}

impl GraphNode {
    pub fn new(index: usize, node_name: String) -> Self {
        Self { index, node_name }
    }
    pub fn get_index(&self) -> usize {
        self.index
    }
    pub fn get_name(&self) -> &str {
        &self.node_name
    }
}

pub fn create_new_edge(start_index: usize, end_index: usize, weight: usize) -> Edge {
    let new_edge = Edge {
        index_first: start_index,
        index_second: end_index,
        weight,
    };
    return new_edge;
}

fn update_existing_edge(graph: &mut Graph, new_edge: Edge) -> bool {

    let start_index = new_edge.index_first;
    let end_index = new_edge.index_second;
    let new_weight = new_edge.weight;
    let edge_index = graph.edges[start_index]
        .iter()
        .position(|x| x.index_second == end_index);
    let mut edge_was_updated = true;
    match edge_index {
        None => {}
        Some(idx_into_edge_list) => {
            let old_edge_weight = graph.edges[start_index][idx_into_edge_list].weight;
            if old_edge_weight >= new_weight {
                graph.edges[start_index].remove(idx_into_edge_list);
            } else {
                edge_was_updated = false;
            }
        }
    }
    graph.edges[start_index].push(
        new_edge
    );
    return edge_was_updated;

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
        let new_reverse_edge = create_new_edge(end_index, start_index, weight);

        let new_edge_is_updated = update_existing_edge(&mut graph, new_edge);
        // same in reverse, assuming bidirectionality of edges
        if new_edge_is_updated {
            update_existing_edge(&mut graph, new_reverse_edge);
        }

    }

    return Ok(graph);
}


pub fn get_node_index_from_node_name(
    node_name: &str,
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

#[cfg(test)]
mod graph_only_tests {
    use crate::parse_input::get_nodes;
    use super::*;

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
    fn test_multiple_start_edges_input() {
        let (contents, expected_graph, _) = set_up_tests();
        let data: Vec<&str> = contents.split("\n\n").collect();

        let node_data = data[0].to_string();
        let edge_data = data[1].to_string();

        let graph_nodes: Vec<GraphNode> = get_nodes(&node_data);
        let graph = construct_graph_from_edges(&graph_nodes, &edge_data);

        assert_eq!(Ok(expected_graph), graph);

        // graph should not contain the I->G 167 path, as this should be updated by the I->G 17 path.
        assert_eq!(graph.unwrap().get_edge_weight(0, 1), 17);
    }
    #[test]
    fn test_route_finding_with_incorrect_number_of_nodes() {
        let (_, _, graph_nodes) = set_up_tests();
        let edge_data = "4\nI G 167\nI E 158\nG E 45\nI G 17\nE I 1".to_string();

        assert_eq!(
            Err("Unexpected number of edges. Expected: 4, actual: 5".to_string()),
            construct_graph_from_edges(&graph_nodes, &edge_data)
        )
    }
    #[test]
    fn test_route_finding_with_incorrect_nodes() {
        let (_, _, graph_nodes) = set_up_tests();
        let edge_data = "4\nI G 167\nI E 158\nG E 45\nI N 17".to_string();

        assert_eq!(
            Err("Nodes in edges should be present in node list. N not found.".to_string()),
            construct_graph_from_edges(&graph_nodes, &edge_data)
        )
    }
}
