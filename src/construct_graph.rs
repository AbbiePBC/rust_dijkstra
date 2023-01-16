use crate::get_nodes;
use crate::parse_input::get_edge_info;
pub const INFINITE_DIST: usize = 100000000;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Edge {
    pub index_first: usize,
    pub index_second: usize,
    pub weight: usize,
    pub is_traversed: bool,
}

#[derive(Debug, PartialEq)]
pub struct Graph {
    pub number_of_nodes: usize,
    pub edges: Vec<Vec<Edge>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GraphNode {
    pub index: usize,
    pub node_name: String,
}

impl Graph {
    pub(crate) fn new(number_of_nodes_: usize, edges_: Vec<Vec<Edge>>) -> Graph {
        // return is unnecessary but looks weird to me otherwise to have Graph { Graph {...}}
        return Graph {
            number_of_nodes: number_of_nodes_,
            edges: edges_,
        };
    }
    pub(crate) fn mark_edges_from_node_as_traversed(&mut self, node_idx: usize){
        for e in self.edges[node_idx].iter_mut(){
            e.is_traversed = true;
        }
    }
    pub(crate) fn mark_edges_from_node_not_as_traversed(&mut self, node_idx: usize){
        for e in self.edges[node_idx].iter_mut(){
            e.is_traversed = false;
        }
    }
}
impl Edge {
    pub(crate) fn new(start_index: usize, end_index: usize, weight: usize) -> Edge {
        return Edge {
            index_first: start_index,
            index_second: end_index,
            weight,
            is_traversed: false
        };
    }
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
    graph.edges[start_index].push(new_edge);
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
            "Unexpected number of edges. Expected: {}, actual: {}",
            num_edges,
            edges.len() - 1,
        ));
    }

    let num_nodes = graph_nodes.len();

    let mut vec: Vec<Vec<Edge>> = Vec::with_capacity(num_nodes);

    for _ in 0..num_nodes {
        vec.push(Vec::with_capacity(num_nodes));
    }
    let mut graph = Graph::new(graph_nodes.len(),vec);

    for i in 1..(num_edges + 1) {
        let (start_index, end_index, weight) = get_edge_info(edges[i], graph_nodes)?;
        if start_index == end_index {
            // self referential edge, discard
            continue;
        }
        let new_edge = Edge::new(start_index, end_index, weight);
        let new_reverse_edge = Edge::new(end_index, start_index, weight);

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
    use super::*;
    use crate::get_nodes;

    fn set_up_tests() -> (String, Graph, Vec<GraphNode>) {
        let contents =
            "3\nI\nG\nE\n\n4\nI G 167\nI E 158\nG E 45\nI G 17\n\nG E\nE I\n\n".to_string();
        let expected_graph = Graph::new(
            3,
            vec![
                vec![Edge::new(0, 2, 158), Edge::new(0, 1, 17)],
                vec![Edge::new(1, 2, 45), Edge::new(1, 0, 17)],
                vec![Edge::new(2, 0, 158), Edge::new(2, 1, 45)],
            ],
        );

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

        let graph_nodes: Vec<GraphNode> = get_nodes(&node_data).unwrap();
        let graph = construct_graph_from_edges(&graph_nodes, &edge_data);
        // graph should not contain the I->G 167 path, as this should be updated by the I->G 17 path.

        assert_eq!(Ok(expected_graph), graph);
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
