use crate::parse_input::{
    parse_edges_from_string, parse_graph_nodes_from_string, split_contents_into_nodes_edges_routes,
    GraphNode,
};
pub const INFINITE_DIST: usize = 100000000;

#[derive(Debug, Clone, PartialEq, Copy)]
pub(crate) struct Node {
    pub index: usize,
    pub parent_idx: usize,
    pub dist_to_node: usize,
}

impl Node {
    pub(crate) fn new(index_: usize, parent_idx_: usize, dist_to_node_: usize) -> Node {
        return Node {
            index: index_,
            parent_idx: parent_idx_,
            dist_to_node: dist_to_node_,
        };
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Edge {
    pub index_first: usize,
    pub index_second: usize,
    pub weight: usize,
    pub is_traversed: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Graph {
    pub number_of_nodes: usize,
    pub edges: Vec<Vec<Edge>>,
    pub graph_nodes: Vec<GraphNode>,
}

impl Graph {
    pub(crate) fn new(graph_nodes: Vec<GraphNode>, edges_: Vec<Edge>) -> Graph {
        // return is unnecessary but looks weird to me otherwise to have Graph { Graph {...}}

        let num_nodes = graph_nodes.len();
        let mut vec: Vec<Vec<Edge>> = Vec::with_capacity(num_nodes);
        for _ in 0..num_nodes {
            vec.push(Vec::with_capacity(num_nodes));
        }

        let mut graph = Graph {
            number_of_nodes: num_nodes,
            edges: vec,
            graph_nodes,
        };

        for edge in edges_ {
            graph.update_edge_in_both_directions(edge);
        }

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
    // todo now we're doing this twice?
    pub(crate) fn mark_edge_as_traversed(&mut self, edge: Edge) {
        for e in self.edges[edge.index_first].iter_mut() {
            //debug!("e  - {:?}",e);
            if e.index_second == edge.index_second && e.index_first == edge.index_first {
                e.is_traversed = true;
                println!("marked edge as traversed  - {:?}", e);
                break;
            }
        }
    }
    pub(crate) fn mark_all_edges_as_not_traversed(&mut self) {
        for node_idx in self.edges.iter_mut() {
            for edge in node_idx.iter_mut() {
                edge.is_traversed = false;
            }
        }
    }
    pub(crate) fn update_edge_in_both_directions(&mut self, new_edge: Edge) {
        let new_edge_is_updated = self.update_existing_edge(new_edge);
        // same in reverse, assuming bidirectionality of edges
        if new_edge_is_updated {
            let new_reverse_edge =
                Edge::new(new_edge.index_second, new_edge.index_first, new_edge.weight);
            self.update_existing_edge(new_reverse_edge);
        }
    }

    pub(crate) fn update_existing_edge(&mut self, new_edge: Edge) -> bool {
        let start_index = new_edge.index_first;
        let end_index = new_edge.index_second;
        let new_weight = new_edge.weight;
        let edge_index = self.edges[start_index]
            .iter()
            .position(|x| x.index_second == end_index);
        let mut edge_was_updated = true;
        match edge_index {
            None => {}
            Some(idx_into_edge_list) => {
                let old_edge_weight = self.edges[start_index][idx_into_edge_list].weight;
                if old_edge_weight >= new_weight {
                    self.edges[start_index].remove(idx_into_edge_list);
                } else {
                    edge_was_updated = false;
                }
            }
        }
        self.edges[start_index].push(new_edge);
        return edge_was_updated;
    }
}

impl Edge {
    pub(crate) fn new(start_index: usize, end_index: usize, weight: usize) -> Edge {
        return Edge {
            index_first: start_index,
            index_second: end_index,
            weight,
            is_traversed: false,
        };
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
