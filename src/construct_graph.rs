use crate::parse_input::{
    parse_edges_from_string, parse_graph_nodes_from_string, split_contents_into_nodes_edges_routes,
    Edge, GraphNode,
};
pub const INFINITE_DIST: usize = 100000000;

#[derive(Debug, PartialEq, Clone)]
pub struct Graph {
    pub number_of_nodes: usize,
    pub edges: Vec<Edge>,
    pub graph_nodes: Vec<GraphNode>,
}

impl Graph {
    pub(crate) fn new(graph_nodes: Vec<GraphNode>, edges_: Vec<Edge>) -> Graph {

        let num_nodes = graph_nodes.len();

        let graph = Graph {
            number_of_nodes: num_nodes,
            edges: edges_,
            graph_nodes,
        };

        return graph;
    }

    pub(crate) fn new_from_string(contents: &str) -> Result<Graph, String> {
        let (node_data, edge_data, _) =
            split_contents_into_nodes_edges_routes(contents.to_string())?;
        let graph_nodes = parse_graph_nodes_from_string(&node_data)?;
        let edges = parse_edges_from_string(&edge_data, &graph_nodes)?;
        let graph = Graph::new(graph_nodes, edges);

        return Ok(graph);
    }

}

#[cfg(test)]
mod graph_only_tests {
    use crate::construct_graph::Graph;

    #[test]
    fn test_route_finding_with_incorrect_number_of_nodes() {
        let graph = Graph::new_from_string(
            "4\nI\nG\nE\nN\n\n4\nI G 167\nI E 158\nG E 45\nI G 17\nE I 1\n\nI E",
        );

        assert_eq!(
            Err("Unexpected number of edges. Expected: 4, actual: 5".to_string()),
            graph
        )
    }
    #[test]
    fn test_route_finding_with_incorrect_nodes() {
        let graph =
            Graph::new_from_string("4\nA\nB\nC\nD\n\n4\nI G 167\nI E 158\nG E 45\nI N 17\n\nA B");
        assert_eq!(
            Err("Nodes in edges should be present in node list. Node I (possibly others) not found.".to_string()),
            graph
        )
    }
}
